use actix_web::{App, HttpResponse, HttpServer, Responder, post, web, ResponseError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

use anyhow::Result;
use thiserror::Error;
use dotenvy::dotenv;

// ---------------- ERRORS ----------------

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Keycloak request error: {0}")]
    Keycloak(String),

    #[error("JSON parse error: {0}")]
    Json(String),

    #[error("Unexpected error: {0}")]
    Other(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}

// ---------------- REQUEST MODELS ----------------

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct CreateUserDto {
    username: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    email: String,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    enabled: bool,
    credentials: Vec<Credential>,
    attributes: HashMap<String, Vec<String>>,
}

#[derive(Serialize)]
struct Credential {
    #[serde(rename = "type")]
    cred_type: String,
    value: String,
    temporary: bool,
}

#[derive(Deserialize)]
struct KeycloakUser {
    id: String,
    username: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

// ---------------- HELPER FUNCTIONS ----------------

async fn get_admin_token(client: &Client) -> Result<String, AppError> {
    let url = env::var("KC_TOKEN_URL")
        .unwrap_or("http://localhost:5080/realms/myrealm/protocol/openid-connect/token".into());

    let client_id = env::var("KC_ADMIN_CLIENT").unwrap_or("backend-admin".into());
    let client_secret = env::var("KC_ADMIN_SECRET").unwrap_or("backend-admin-secret".into());

    let resp = client
        .post(url)
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
        ])
        .send()
        .await
        .map_err(|e| AppError::Keycloak(e.to_string()))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| AppError::Keycloak(e.to_string()))?;

    if !status.is_success() {
        return Err(AppError::Keycloak(format!("Admin login failed: {}", text)));
    }

    let token_response: TokenResponse =
        serde_json::from_str(&text).map_err(|e| AppError::Json(e.to_string()))?;

    Ok(token_response.access_token)
}

async fn create_user_with_role(client: &Client, req: &RegisterRequest) -> Result<(), AppError> {
    let admin_token = get_admin_token(client).await?;

    let mut attributes = HashMap::new();
    attributes.insert("company_name".into(), vec!["Default Company".into()]);
    attributes.insert("station_name".into(), vec!["Default Station".into()]);

    let user_dto = CreateUserDto {
        username: req.username.clone(),
        first_name: req.first_name.clone(),
        last_name: req.last_name.clone(),
        email: req.email.clone(),
        email_verified: true,
        enabled: true,
        credentials: vec![Credential {
            cred_type: "password".into(),
            value: req.password.clone(),
            temporary: false,
        }],
        attributes,
    };

    let url = env::var("KC_USER_URL")
        .unwrap_or("http://localhost:5080/admin/realms/myrealm/users".into());

    let resp = client
        .post(&url)
        .bearer_auth(&admin_token)
        .json(&user_dto)
        .send()
        .await
        .map_err(|e| AppError::Keycloak(e.to_string()))?;

    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();

    if status.as_u16() == 409 {
        return Err(AppError::Keycloak("User already exists".into()));
    }

    if !status.is_success() {
        return Err(AppError::Keycloak(format!(
            "Create user failed: {}",
            text
        )));
    }

    Ok(())
}

async fn login_user(client: &Client, req: &LoginRequest) -> Result<serde_json::Value, AppError> {
    let url = env::var("KC_TOKEN_URL")
        .unwrap_or("http://localhost:5080/realms/myrealm/protocol/openid-connect/token".into());

    let resp = client
        .post(url)
        .form(&[
            ("grant_type", "password"),
            ("client_id", "auth-client"),
            ("username", req.username.as_str()),
            ("password", req.password.as_str()),
        ])
        .send()
        .await
        .map_err(|e| AppError::Other(e.to_string()))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| AppError::Other(e.to_string()))?;

    if !status.is_success() {
        return Err(AppError::Keycloak(format!("Login failed: {}", text)));
    }

    let token_json: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| AppError::Json(e.to_string()))?;

    Ok(token_json)
}

// ---------------- ENDPOINTS ----------------

#[post("/register")]
async fn register(body: web::Json<RegisterRequest>) -> impl Responder {
    let client = Client::new();

    match create_user_with_role(&client, &body).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User registered successfully",
            "username": body.username
        })),
        Err(e) => e.error_response(),
    }
}

#[post("/login")]
async fn login(body: web::Json<LoginRequest>) -> impl Responder {
    let client = Client::new();

    match login_user(&client, &body).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid username or password"
        })),
    }
}

// ---------------- MAIN ----------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("🚀 Server running at http://localhost:3000");
    println!("📌 POST /register");
    println!("📌 POST /login");

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(register)
            .service(login)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
