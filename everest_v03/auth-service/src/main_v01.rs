use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Module-level constants
const ROLE_NAME: &str = "user";
const COMPANY_NAME: &str = "X";
const STATION_NAME: &str = "X";

#[derive(Serialize, Deserialize)]
struct CreateUserDto {
    username: String,
    email: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthDto {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct VerifyRequest {
    token: String,
}

#[derive(Serialize)]
struct VerifyResponse {
    valid: bool,
    username: Option<String>,
    email: Option<String>,
    company_name: Option<String>,
    station_name: Option<String>,
}

async fn create_user(
    user_dto: web::Json<CreateUserDto>,
    client: web::Data<reqwest::Client>,
) -> impl Responder {
    println!("=== CREATE USER REQUEST ===");
    println!("Username: {}", user_dto.username);

    let url = std::env::var("KEYCLOAK_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let realm_name = std::env::var("KEYCLOAK_REALM").unwrap_or_else(|_| "master".to_string());
    let client_id = std::env::var("KEYCLOAK_ADMIN_CLIENT_ID").unwrap_or_else(|_| "backend-admin".to_string());
    let client_secret = std::env::var("KEYCLOAK_ADMIN_CLIENT_SECRET").expect("KEYCLOAK_ADMIN_CLIENT_SECRET must be set");

    // Get admin token using service account (client credentials)
    let token_url = format!("{}/realms/{}/protocol/openid-connect/token", url, realm_name);
    
    let token_response = client
        .post(&token_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=client_credentials&client_id={}&client_secret={}",
            client_id, client_secret
        ))
        .send()
        .await;

    let access_token = match token_response {
        Ok(resp) if resp.status().is_success() => {
            match resp.json::<serde_json::Value>().await {
                Ok(token_data) => {
                    if let Some(access_token) = token_data.get("access_token").and_then(|v| v.as_str()) {
                        println!("✓ Admin token acquired via service account");
                        access_token.to_string()
                    } else {
                        eprintln!("✗ No access_token in response");
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "No access token in response"
                        }));
                    }
                }
                Err(e) => {
                    eprintln!("✗ Failed to parse token response: {:?}", e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to parse token response",
                        "details": format!("{:?}", e)
                    }));
                }
            }
        }
        Ok(resp) => {
            let status = resp.status();
            let error_text = resp.text().await.unwrap_or_default();
            eprintln!("✗ Failed to get admin token. Status: {}", status);
            eprintln!("  Response: {}", error_text);
            eprintln!("  Keycloak URL: {}", url);
            eprintln!("  Client ID: {}", client_id);
            eprintln!("  Realm: {}", realm_name);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to authenticate service account",
                "status": status.as_u16(),
                "details": error_text,
                "keycloak_url": url,
                "realm": realm_name,
                "client_id": client_id
            }));
        }
        Err(e) => {
            eprintln!("✗ Failed to connect to Keycloak: {:?}", e);
            eprintln!("  Keycloak URL: {}", url);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to connect to Keycloak",
                "details": format!("{:?}", e),
                "keycloak_url": url
            }));
        }
    };

    // Create user directly via REST API
    let users_url = format!("{}/admin/realms/{}/users", url, realm_name);

    // Create attributes HashMap
    let mut attributes = HashMap::new();
    attributes.insert("company_name".to_string(), vec![COMPANY_NAME.to_string()]);
    attributes.insert("station_name".to_string(), vec![STATION_NAME.to_string()]);

    let user_data = serde_json::json!({
        "username": user_dto.username,
        "email": user_dto.email,
        "firstName": user_dto.first_name,
        "lastName": user_dto.last_name,
        "enabled": true,
        "emailVerified": true,
        "attributes": attributes,
        "credentials": [{
            "type": "password",
            "value": user_dto.password,
            "temporary": false
        }]
    });

    let create_response = client
        .post(&users_url)
        .bearer_auth(&access_token)
        .header("Content-Type", "application/json")
        .json(&user_data)
        .send()
        .await;

    match create_response {
        Ok(response) => {
            let status = response.status();
            
            if status.is_success() {
                // Extract user ID from Location header if available
                let user_id = response
                    .headers()
                    .get("location")
                    .and_then(|loc| loc.to_str().ok())
                    .and_then(|loc| loc.split('/').last())
                    .map(String::from);

                match user_id {
                    Some(id) => {
                        println!("✓ User created with ID: {}", id);
                        println!("✓ Attributes added for company_name and station_name");

                        HttpResponse::Created().json(serde_json::json!({
                            "message": format!("User created successfully with '{}' role", ROLE_NAME),
                            "userId": id,
                            "role": ROLE_NAME,
                            "attributes": {
                                "company_name": COMPANY_NAME,
                                "station_name": STATION_NAME
                            }
                        }))
                    }
                    None => {
                        println!("✓ User created (no ID returned)");

                        HttpResponse::Created().json(serde_json::json!({
                            "message": format!("User created successfully with '{}' role", ROLE_NAME),
                            "role": ROLE_NAME,
                            "attributes": {
                                "company_name": COMPANY_NAME,
                                "station_name": STATION_NAME
                            }
                        }))
                    }
                }
            } else {
                let error_text = response.text().await.unwrap_or_default();
                eprintln!("✗ Failed to create user: {}", error_text);
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": format!("Failed to create user: {}", error_text)
                }))
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to create user: {:?}", e)
            }))
        }
    }
}

async fn authenticate_user(
    auth_dto: web::Json<AuthDto>,
    client: web::Data<reqwest::Client>,
) -> impl Responder {
    println!("=== AUTHENTICATE USER REQUEST ===");
    println!("Username: {}", auth_dto.username);

    let keycloak_url = std::env::var("KEYCLOAK_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let realm_name = std::env::var("KEYCLOAK_REALM").unwrap_or_else(|_| "master".to_string());
    let client_id = std::env::var("KEYCLOAK_CLIENT_ID").unwrap_or_else(|_| "admin-cli".to_string());

    let token_url = format!("{}/realms/{}/protocol/openid-connect/token", keycloak_url, realm_name);

    let res = client
        .post(&token_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=password&username={}&password={}&client_id={}",
            auth_dto.username, auth_dto.password, client_id
        ))
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();
            match response.text().await {
                Ok(body) => {
                    if status.is_success() {
                        match serde_json::from_str::<serde_json::Value>(&body) {
                            Ok(token_response) => {
                                println!("✓ Authentication successful");
                                HttpResponse::Ok().json(token_response)
                            }
                            Err(_) => {
                                eprintln!("✗ Invalid response format");
                                HttpResponse::InternalServerError().json(serde_json::json!({
                                    "error": "Invalid response format"
                                }))
                            }
                        }
                    } else {
                        println!("✗ Invalid credentials");
                        HttpResponse::Unauthorized().json(serde_json::json!({
                            "error": "Invalid credentials"
                        }))
                    }
                }
                Err(_) => {
                    eprintln!("✗ Failed to read response");
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to read response"
                    }))
                }
            }
        }
        Err(_) => {
            eprintln!("✗ Failed to connect to auth server");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to connect to auth server"
            }))
        }
    }
}

async fn verify_token(
    verify_req: web::Json<VerifyRequest>,
    client: web::Data<reqwest::Client>,
) -> impl Responder {
    println!("=== VERIFY TOKEN REQUEST ===");

    let keycloak_url = std::env::var("KEYCLOAK_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let realm_name = std::env::var("KEYCLOAK_REALM").unwrap_or_else(|_| "master".to_string());

    let userinfo_url = format!("{}/realms/{}/protocol/openid-connect/userinfo", keycloak_url, realm_name);

    let response = client
        .get(&userinfo_url)
        .bearer_auth(&verify_req.token)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                if let Ok(user_info) = resp.json::<serde_json::Value>().await {
                    println!("✓ Token verified successfully");
                    
                    let company_name = user_info
                        .get("company_name")
                        .and_then(|v| v.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|v| v.as_str())
                        .map(String::from);
                    
                    let station_name = user_info
                        .get("station_name")
                        .and_then(|v| v.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|v| v.as_str())
                        .map(String::from);

                    HttpResponse::Ok().json(VerifyResponse {
                        valid: true,
                        username: user_info
                            .get("preferred_username")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        email: user_info
                            .get("email")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        company_name,
                        station_name,
                    })
                } else {
                    println!("✗ Failed to parse user info");
                    HttpResponse::Ok().json(VerifyResponse {
                        valid: true,
                        username: None,
                        email: None,
                        company_name: None,
                        station_name: None,
                    })
                }
            } else {
                println!("✗ Invalid token");
                HttpResponse::Unauthorized().json(VerifyResponse {
                    valid: false,
                    username: None,
                    email: None,
                    company_name: None,
                    station_name: None,
                })
            }
        }
        Err(_) => {
            eprintln!("✗ Failed to verify token");
            HttpResponse::InternalServerError().json(VerifyResponse {
                valid: false,
                username: None,
                email: None,
                company_name: None,
                station_name: None,
            })
        }
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "keycloak-auth"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv::dotenv().ok();
    
    let keycloak_url = std::env::var("KEYCLOAK_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let realm = std::env::var("KEYCLOAK_REALM").unwrap_or_else(|_| "master".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .unwrap_or(8081);

    println!("=================================");
    println!("Starting Keycloak Auth Service...");
    println!("=================================");
    println!("Server URL: http://127.0.0.1:{}", port);
    println!("Keycloak URL: {}", keycloak_url);
    println!("Realm: {}", realm);
    println!("Default role: {}", ROLE_NAME);
    println!("=================================");

    let client = web::Data::new(reqwest::Client::new());

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::resource("/health").route(web::get().to(health_check)))
            .service(web::resource("/api/v1/register").route(web::post().to(create_user)))
            .service(web::resource("/api/v1/auth").route(web::post().to(authenticate_user)))
            .service(web::resource("/api/v1/verify").route(web::post().to(verify_token)))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}