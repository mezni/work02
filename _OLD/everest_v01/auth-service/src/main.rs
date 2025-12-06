mod errors;
mod config;

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use errors::AppError;
use config::*;

// ---------------- OPENAPI DOCUMENTATION ----------------

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
        register,
        login
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            HealthResponse,
            SuccessResponse,
            ErrorResponse,
            TokenResponse,
            KeycloakError,
            Credential,
            CreateUserDto,
            ErrorDetail
        )
    ),
    tags(
        (name = "Authentication", description = "User authentication endpoints"),
        (name = "Health", description = "Service health check")
    ),
    info(
        title = "Authentication Service API",
        version = "1.0.0",
        description = "REST API for user registration and login using Keycloak",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        contact(
            name = "API Support",
            email = "support@example.com",
            url = "https://example.com/support"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Development server"),
        (url = "/api/v1", description = "API base path")
    ),
    external_docs(
        url = "https://keycloak.org/docs",
        description = "Keycloak authentication documentation"
    )
)]
struct ApiDoc;

// ---------------- REQUEST/RESPONSE MODELS ----------------

/// Register a new user
#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "username": "john_doe",
    "password": "StrongPassword123!",
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com"
}))]
struct RegisterRequest {
    /// Unique username for the user (3-50 characters)
    #[schema(min_length = 3, max_length = 50)]
    username: String,
    
    /// User's password (minimum 8 characters)
    #[schema(min_length = 8)]
    password: String,
    
    /// First name
    first_name: String,
    
    /// Last name
    last_name: String,
    
    /// Email address
    #[schema(format = "email")]
    email: String,
}

/// Login request
#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "username": "john_doe",
    "password": "StrongPassword123!"
}))]
struct LoginRequest {
    /// Username
    username: String,
    
    /// Password
    password: String,
}

/// Token response from Keycloak
#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJ...",
    "expires_in": 300,
    "refresh_expires_in": 1800,
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICI...",
    "token_type": "Bearer",
    "not-before-policy": 0,
    "session_state": "ccea4c1c-3c3c-4b3b-8b3b-3c3c3c3c3c3c",
    "scope": "email profile"
}))]
struct TokenResponse {
    /// Access token for API requests
    access_token: String,
    
    /// Token expiration time in seconds
    expires_in: Option<i32>,
    
    /// Refresh token expiration time in seconds
    refresh_expires_in: Option<i32>,
    
    /// Refresh token
    refresh_token: Option<String>,
    
    /// Token type (usually Bearer)
    token_type: Option<String>,
    
    /// Session state
    session_state: Option<String>,
    
    /// Scope of the token
    scope: Option<String>,
}

/// Health check response
#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "status": "UP",
    "timestamp": "2024-01-15T10:30:00Z",
    "service": "authentication-service",
    "version": "1.0.0"
}))]
struct HealthResponse {
    /// Service status
    #[schema(example = "UP")]
    status: String,
    
    /// Timestamp of the check
    timestamp: Option<String>,
    
    /// Service name
    service: Option<String>,
    
    /// Service version
    version: Option<String>,
}

/// Success response
#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "success": true,
    "message": "User registered successfully",
    "data": {
        "username": "john_doe"
    },
    "timestamp": "2024-01-15T10:30:00Z"
}))]
struct SuccessResponse {
    /// Success flag
    success: bool,
    
    /// Response message
    message: String,
    
    /// Response data
    data: serde_json::Value,
    
    /// Response timestamp
    timestamp: String,
}

/// Error response
#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "success": false,
    "error": {
        "code": "USER_EXISTS",
        "message": "User already exists",
        "details": "A user with username 'john_doe' already exists"
    },
    "timestamp": "2024-01-15T10:30:00Z"
}))]
struct ErrorResponse {
    /// Success flag
    success: bool,
    
    /// Error details
    error: ErrorDetail,
    
    /// Response timestamp
    timestamp: String,
}

/// Error detail
#[derive(Serialize, ToSchema)]
struct ErrorDetail {
    /// Error code
    code: String,
    
    /// Error message
    message: String,
    
    /// Additional error details
    details: Option<String>,
}

/// Keycloak error
#[derive(Serialize, ToSchema)]
struct KeycloakError {
    /// Error type
    error: String,
    
    /// Error description
    error_description: Option<String>,
}

// ---------------- INTERNAL MODELS (for Keycloak) ----------------

#[derive(Serialize, ToSchema)]
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

#[derive(Serialize, ToSchema)]
struct Credential {
    #[serde(rename = "type")]
    #[schema(example = "password")]
    cred_type: String,
    
    #[schema(example = "password123")]
    value: String,
    
    #[schema(example = "false")]
    temporary: bool,
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

async fn login_user(client: &Client, req: &LoginRequest) -> Result<TokenResponse, AppError> {
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

    let token_response: TokenResponse =
        serde_json::from_str(&text).map_err(|e| AppError::Json(e.to_string()))?;

    Ok(token_response)
}

// Helper to create timestamp
fn current_timestamp() -> String {
    // Simple timestamp without external dependencies
    let now = std::time::SystemTime::now();
    let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    format!("{}", since_epoch.as_secs())
}

// ---------------- ENDPOINTS ----------------

/// Health check endpoint
/// 
/// Returns the current health status of the service.
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unavailable")
    )
)]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "UP".to_string(),
        timestamp: Some(current_timestamp()),
        service: Some("authentication-service".to_string()),
        version: Some("1.0.0".to_string()),
    })
}

/// Register a new user
/// 
/// Creates a new user account in Keycloak with the 'user' role assigned.
#[utoipa::path(
    post,
    path = "/api/v1/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = SuccessResponse),
        (status = 400, description = "Invalid request data", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/register")]
async fn register(body: web::Json<RegisterRequest>) -> impl Responder {
    let client = Client::new();

    match create_user_with_role(&client, &body).await {
        Ok(_) => {
            let response = SuccessResponse {
                success: true,
                message: "User registered successfully".to_string(),
                data: serde_json::json!({
                    "username": body.username,
                    "email": body.email,
                    "first_name": body.first_name,
                    "last_name": body.last_name
                }),
                timestamp: current_timestamp(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let error_response = ErrorResponse {
                success: false,
                error: ErrorDetail {
                    code: match e {
                        AppError::Keycloak(ref msg) if msg.contains("already exists") => "USER_EXISTS".to_string(),
                        AppError::Keycloak(_) => "KEYCLOAK_ERROR".to_string(),
                        AppError::Json(_) => "JSON_ERROR".to_string(),
                        AppError::Other(_) => "OTHER_ERROR".to_string(),
                    },
                    message: e.to_string(),
                    details: Some(match e {
                        AppError::Keycloak(ref msg) => msg.clone(),
                        AppError::Json(ref msg) => msg.clone(),
                        AppError::Other(ref msg) => msg.clone(),
                    }),
                },
                timestamp: current_timestamp(),
            };
            
            let status_code = match e {
                AppError::Keycloak(ref msg) if msg.contains("already exists") => actix_web::http::StatusCode::CONFLICT,
                AppError::Keycloak(_) => actix_web::http::StatusCode::BAD_GATEWAY,
                AppError::Json(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                AppError::Other(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            
            HttpResponse::build(status_code).json(error_response)
        }
    }
}

/// User login
/// 
/// Authenticates a user and returns Keycloak tokens.
#[utoipa::path(
    post,
    path = "/api/v1/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/login")]
async fn login(body: web::Json<LoginRequest>) -> impl Responder {
    let client = Client::new();

    match login_user(&client, &body).await {
        Ok(token_response) => HttpResponse::Ok().json(token_response),
        Err(e) => {
            let error_response = ErrorResponse {
                success: false,
                error: ErrorDetail {
                    code: "AUTH_FAILED".to_string(),
                    message: "Authentication failed".to_string(),
                    details: Some(match e {
                        AppError::Keycloak(ref msg) => {
                            if msg.contains("Invalid user") || msg.contains("invalid credentials") {
                                "Invalid username or password".to_string()
                            } else {
                                msg.clone()
                            }
                        }
                        AppError::Json(ref msg) => msg.clone(),
                        AppError::Other(ref msg) => msg.clone(),
                    }),
                },
                timestamp: current_timestamp(),
            };
            
            HttpResponse::Unauthorized().json(error_response)
        }
    }
}

// ---------------- MAIN ----------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();

    // Enable logging - wrap in unsafe block
    unsafe {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                AUTHENTICATION SERVICE                    ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║ 🚀 Server running at: http://localhost:3000              ║");
    println!("║ 📚 Swagger UI: http://localhost:3000/swagger-ui/         ║");
    println!("║ 📋 OpenAPI Spec: http://localhost:3000/api-docs/openapi.json ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║ 📌 Endpoints:                                            ║");
    println!("║   POST /api/v1/register - Register new user             ║");
    println!("║   POST /api/v1/login    - Login user                    ║");
    println!("║   GET  /api/v1/health   - Health check                  ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    HttpServer::new(move || {
        App::new()
            // Add request logging
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            // API endpoints
            .service(
                web::scope("/api/v1")
                    .service(
                        web::resource("/health")
                            .route(web::get().to(health))
                    )
                    .service(register)
                    .service(login)
            )
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
            // Serve OpenAPI JSON
            .service(
                web::resource("/api-docs/openapi.json")
                    .route(web::get().to(|| async {
                        HttpResponse::Ok()
                            .content_type("application/json")
                            .json(ApiDoc::openapi())
                    }))
            )
            // Default 404 handler
            .default_service(
                web::route().to(|| async {
                    let error_response = ErrorResponse {
                        success: false,
                        error: ErrorDetail {
                            code: "NOT_FOUND".to_string(),
                            message: "Endpoint not found".to_string(),
                            details: Some("Check the API documentation at /swagger-ui/".to_string()),
                        },
                        timestamp: current_timestamp(),
                    };
                    HttpResponse::NotFound().json(error_response)
                })
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}