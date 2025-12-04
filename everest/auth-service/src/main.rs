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
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    expires_in: Option<i32>,
    #[serde(default)]
    token_type: Option<String>,
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
        .map_err(|e| format!("Failed to get admin token: {}", e))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        return Err(format!("Failed to authenticate admin client: {}", text));
    }

    let token_response: TokenResponse = serde_json::from_str(&text)
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    Ok(token_response.access_token)
}

async fn create_user_with_role(client: &Client, req: &RegisterRequest) -> Result<(), String> {
    let admin_token = get_admin_token(client).await?;

    // Default attributes - using unmanaged attributes
    let mut attributes = HashMap::new();
    attributes.insert("company_name".to_string(), vec!["Default Company".to_string()]);
    attributes.insert("station_name".to_string(), vec!["Default Station".to_string()]);

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
        attributes,
    };

    let create_resp = client
        .post("http://localhost:5080/admin/realms/myrealm/users")
        .bearer_auth(&admin_token)
        .header("Content-Type", "application/json")
        .json(&new_user)
        .send()
        .await
        .map_err(|e| format!("Failed to send create user request: {}", e))?;

    let status = create_resp.status();
    
    // 201 Created is the expected success status
    if status.as_u16() == 409 {
        return Err("User already exists".to_string());
    }
    
    if !status.is_success() && status.as_u16() != 201 {
        let text = create_resp.text().await.unwrap_or_default();
        return Err(format!("Failed to create user (status {}): {}", status, text));
    }

    // Give Keycloak a moment to process
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Fetch user ID
    let users_resp = client
        .get("http://localhost:5080/admin/realms/myrealm/users")
        .bearer_auth(&admin_token)
        .query(&[("username", &req.username), ("exact", &"true".to_string())])
        .send()
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?;

    let users_status = users_resp.status();
    let users_text = users_resp.text().await.map_err(|e| e.to_string())?;
    
    if !users_status.is_success() {
        return Err(format!("Failed to query users (status {}): {}", users_status, users_text));
    }

    let users: Vec<KeycloakUser> =
        serde_json::from_str(&users_text).map_err(|e| format!("Failed to parse users: {}", e))?;
    
    let user_id = users
        .first()
        .ok_or("User not found after creation")?
        .id
        .clone();

    // Assign role "user"
    let role_resp = client
        .get("http://localhost:5080/admin/realms/myrealm/roles/user")
        .bearer_auth(&admin_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch role: {}", e))?;

    let role_status = role_resp.status();
    let role_text = role_resp.text().await.map_err(|e| e.to_string())?;
    
    if !role_status.is_success() {
        return Err(format!("Failed to fetch role 'user' (status {}): {}", role_status, role_text));
    }

    let role = serde_json::from_str::<serde_json::Value>(&role_text)
        .map_err(|e| format!("Failed to parse role: {}", e))?;

    let assign_resp = client
        .post(format!(
            "http://localhost:5080/admin/realms/myrealm/users/{}/role-mappings/realm",
            user_id
        ))
        .bearer_auth(&admin_token)
        .header("Content-Type", "application/json")
        .json(&[role])
        .send()
        .await
        .map_err(|e| format!("Failed to assign role: {}", e))?;

    let assign_status = assign_resp.status();
    
    if !assign_status.is_success() && assign_status.as_u16() != 204 {
        let text = assign_resp.text().await.unwrap_or_default();
        return Err(format!("Failed to assign 'user' role (status {}): {}", assign_status, text));
    }

    // Remove unwanted default roles
    let unwanted_roles = vec!["default-roles-myrealm", "offline_access", "uma_authorization"];
    
    for role_name in unwanted_roles {
        // Fetch the role
        if let Ok(role_resp) = client
            .get(format!("http://localhost:5080/admin/realms/myrealm/roles/{}", role_name))
            .bearer_auth(&admin_token)
            .send()
            .await
        {
            if let Ok(role_text) = role_resp.text().await {
                if let Ok(role_json) = serde_json::from_str::<serde_json::Value>(&role_text) {
                    // Remove the role from user
                    let _ = client
                        .delete(format!(
                            "http://localhost:5080/admin/realms/myrealm/users/{}/role-mappings/realm",
                            user_id
                        ))
                        .bearer_auth(&admin_token)
                        .header("Content-Type", "application/json")
                        .json(&[role_json])
                        .send()
                        .await;
                }
            }
        }
    }

    Ok(())
}

async fn login_user(client: &Client, req: &LoginRequest) -> Result<serde_json::Value, String> {
    let resp = client
        .post("http://localhost:5080/realms/myrealm/protocol/openid-connect/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "password"),
            ("client_id", "auth-client"),
            ("username", &req.username),
            ("password", &req.password),
        ])
        .send()
        .await
        .map_err(|e| format!("Login request failed: {}", e))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        return Err(format!("Authentication failed (status {}): {}", status, text));
    }

    let token: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("Failed to parse token: {}", e))?;
    
    Ok(token)
}

// ---------------- REGISTER ----------------
#[post("/register")]
async fn register(body: web::Json<RegisterRequest>) -> impl Responder {
    let client = Client::new();

    match create_user_with_role(&client, &body).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User created and role assigned successfully",
            "username": body.username
        })),
        Err(e) => {
            eprintln!("Registration error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e
            }))
        }
    }
}

// ---------------- LOGIN ----------------
#[post("/login")]
async fn login(body: web::Json<LoginRequest>) -> impl Responder {
    let client = Client::new();

    match login_user(&client, &body).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => {
            eprintln!("Login error: {}", e);
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials"
            }))
        }
    }
}

// ---------------- MAIN ----------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Backend running on http://localhost:3000");
    println!("📋 Endpoints:");
    println!("   POST /register - Create new user");
    println!("   POST /login - Authenticate user");

    HttpServer::new(|| {
        App::new()
            .service(register)
            .service(login)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}