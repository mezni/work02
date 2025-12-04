use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    attributes: Option<HashMap<String, Vec<String>>>,
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

// ---------------- HELPER FUNCTIONS ----------------
async fn get_admin_token(client: &Client) -> Result<String, String> {
    let resp = client
        .post("http://localhost:5080/realms/myrealm/protocol/openid-connect/token")
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", "backend-admin"),
            ("client_secret", "backend-admin-secret"),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    let token = serde_json::from_str::<serde_json::Value>(&text)
        .map_err(|e| e.to_string())?["access_token"]
        .as_str()
        .ok_or("Missing access_token")?
        .to_string();

    Ok(token)
}

async fn create_user_with_role(client: &Client, req: &RegisterRequest) -> Result<(), String> {
    let admin_token = get_admin_token(client).await?;

    // Default attributes
    let mut attributes = HashMap::new();
    attributes.insert("company_name".to_string(), vec!["X".to_string()]);
    attributes.insert("station_name".to_string(), vec!["X".to_string()]);

    let new_user = CreateUserDto {
        username: req.username.clone(),
        first_name: req.first_name.clone(),
        last_name: req.last_name.clone(),
        email: req.email.clone(),
        email_verified: true,
        enabled: true,
        credentials: vec![Credential {
            cred_type: "password".to_string(),
            value: req.password.clone(),
            temporary: false,
        }],
        attributes: Some(attributes),
    };

    let create_resp = client
        .post("http://localhost:5080/admin/realms/myrealm/users")
        .bearer_auth(&admin_token)
        .json(&new_user)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !create_resp.status().is_success() && create_resp.status().as_u16() != 409 {
        let text = create_resp.text().await.unwrap_or_default();
        return Err(format!("Failed to create user: {}", text));
    }

    // Fetch user ID
    let users_resp = client
        .get("http://localhost:5080/admin/realms/myrealm/users")
        .bearer_auth(&admin_token)
        .query(&[("username", &req.username)])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let users_text = users_resp.text().await.map_err(|e| e.to_string())?;
    let users: Vec<KeycloakUser> =
        serde_json::from_str(&users_text).map_err(|_| "Failed to parse user".to_string())?;
    let user_id = users
        .get(0)
        .ok_or("User not found after creation")?
        .id
        .clone();

    // Clear required actions & verify email
    let patch_resp = client
        .put(format!(
            "http://localhost:5080/admin/realms/myrealm/users/{}",
            user_id
        ))
        .bearer_auth(&admin_token)
        .json(&serde_json::json!({
            "enabled": true,
            "requiredActions": [],
            "emailVerified": true
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !patch_resp.status().is_success() {
        let text = patch_resp.text().await.unwrap_or_default();
        return Err(format!("Failed to clear required actions: {}", text));
    }

    // Assign role "user"
    let role_resp = client
        .get("http://localhost:5080/admin/realms/myrealm/roles/user")
        .bearer_auth(&admin_token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let role_text = role_resp.text().await.map_err(|e| e.to_string())?;
    let role = serde_json::from_str::<serde_json::Value>(&role_text).map_err(|e| e.to_string())?;

    let assign_resp = client
        .post(format!(
            "http://localhost:5080/admin/realms/myrealm/users/{}/role-mappings/realm",
            user_id
        ))
        .bearer_auth(&admin_token)
        .json(&[role])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !assign_resp.status().is_success() {
        let text = assign_resp.text().await.unwrap_or_default();
        return Err(format!("Failed to assign 'user' role: {}", text));
    }

    Ok(())
}

async fn login_user(client: &Client, req: &LoginRequest) -> Result<serde_json::Value, String> {
    let resp = client
        .post("http://localhost:5080/realms/myrealm/protocol/openid-connect/token")
        .form(&[
            ("grant_type", "password"),
            ("client_id", "auth-client"),
            ("username", &req.username),
            ("password", &req.password),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        return Err(format!("Authentication failed: {}", text));
    }

    let token: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    Ok(token)
}

// ---------------- REGISTER ----------------
#[post("/register")]
async fn register(body: web::Json<RegisterRequest>) -> impl Responder {
    let client = Client::new();

    match create_user_with_role(&client, &body).await {
        Ok(_) => HttpResponse::Ok().body("User created and role assigned successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// ---------------- LOGIN ----------------
#[post("/login")]
async fn login(body: web::Json<LoginRequest>) -> impl Responder {
    let client = Client::new();

    match login_user(&client, &body).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}

// ---------------- MAIN ----------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Backend running on http://localhost:3000");

    HttpServer::new(|| App::new().service(register).service(login))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
