mod errors;
mod config;

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web, ResponseError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use errors::AppError;
use config::*;

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
struct TokenResponse {
    access_token: String,
}

// ---------------- HELPER FUNCTIONS ----------------

async fn get_admin_token(client: &Client) -> Result<String, AppError> {
    let url = token_url();
    let client_id = admin_client();
    let client_secret = admin_secret();

    let resp = client
        .post(url)
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
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

    // Create user DTO
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

    // Create user
    let url = user_url();
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

    // Small delay to allow Keycloak to process
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // ---------------- ASSIGN ROLE ----------------
    // 1. Fetch newly created user ID
    let users_resp = client
        .get(&format!("http://localhost:5080/admin/realms/myrealm/users"))
        .bearer_auth(&admin_token)
        .query(&[("username", &req.username), ("exact", &"true".to_string())])
        .send()
        .await
        .map_err(|e| AppError::Keycloak(e.to_string()))?;

    let users_text = users_resp.text().await.map_err(|e| AppError::Keycloak(e.to_string()))?;
    let users: Vec<serde_json::Value> =
        serde_json::from_str(&users_text).map_err(|e| AppError::Json(e.to_string()))?;

    let user_id = users
        .first()
        .ok_or(AppError::Keycloak("User not found after creation".into()))?["id"]
        .as_str()
        .unwrap()
        .to_string();

    // 2. Fetch 'user' role
    let role_resp = client
        .get("http://localhost:5080/admin/realms/myrealm/roles/user")
        .bearer_auth(&admin_token)
        .send()
        .await
        .map_err(|e| AppError::Keycloak(e.to_string()))?;

    let role_text = role_resp.text().await.map_err(|e| AppError::Keycloak(e.to_string()))?;
    let role_json: serde_json::Value =
        serde_json::from_str(&role_text).map_err(|e| AppError::Json(e.to_string()))?;

    // 3. Assign role to user
    let assign_resp = client
        .post(&format!(
            "http://localhost:5080/admin/realms/myrealm/users/{}/role-mappings/realm",
            user_id
        ))
        .bearer_auth(&admin_token)
        .json(&[role_json])
        .send()
        .await
        .map_err(|e| AppError::Keycloak(e.to_string()))?;

    if !assign_resp.status().is_success() && assign_resp.status().as_u16() != 204 {
        let text = assign_resp.text().await.unwrap_or_default();
        return Err(AppError::Keycloak(format!(
            "Failed to assign 'user' role: {}",
            text
        )));
    }

    Ok(())
}

async fn login_user(client: &Client, req: &LoginRequest) -> Result<serde_json::Value, AppError> {
    let url = token_url();
    let client_id = public_client();

    let resp = client
        .post(url)
        .form(&[
            ("grant_type", "password"),
            ("client_id", &client_id),
            ("username", &req.username),
            ("password", &req.password),
        ])
        .send()
        .await
        .map_err(|e| AppError::Keycloak(e.to_string()))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| AppError::Keycloak(e.to_string()))?;

    if !status.is_success() {
        return Err(AppError::Keycloak(format!("Login failed: {}", text)));
    }

    let token_json: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| AppError::Json(e.to_string()))?;

    Ok(token_json)
}

// ---------------- ENDPOINTS ----------------

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "UP"}))
}

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
    config::init();

    println!("🚀 Server running at http://localhost:3000");
    println!("📌 POST /api/v1/register");
    println!("📌 POST /api/v1/login");
    println!("📌 GET  /api/v1/health");

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::resource("/health").route(web::get().to(health))
                    )
                    .service(register)
                    .service(login)
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
