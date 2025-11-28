#!/bin/bash
# create_application_layer_with_stubs.sh
# Creates the Application layer structure and starter Rust files with Swagger support

SERVICE_DIR="auth-service"

echo "Creating Application layer structure with starter code..."

cd $SERVICE_DIR

# Create directories
mkdir -p src/application/commands
mkdir -p src/application/queries
mkdir -p src/application/dto
mkdir -p src/application/handlers

# Create mod.rs
cat > src/application/mod.rs <<EOL
pub mod commands;
pub mod queries;
pub mod dto;
pub mod handlers;
pub mod errors;
EOL

# Create errors.rs
cat > src/application/errors.rs <<EOL
use thiserror::Error;
use crate::domain::errors::DomainError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Keycloak client error: {0}")]
    Keycloak(String),
    #[error("JWT error: {0}")]
    Jwt(String),
}
EOL

# Create DTOs
cat > src/application/dto/user_dto.rs <<EOL
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub role: String,
    pub organisation_name: Option<String>,
    pub station_name: Option<String>,
}
EOL

cat > src/application/dto/auth_response.rs <<EOL
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub organisation_name: Option<String>,
    pub station_name: Option<String>,
    pub role: String,
}
EOL

# Create starter commands
touch src/application/commands/register_user.rs
touch src/application/commands/create_partner.rs
touch src/application/commands/create_operator.rs

# Create starter queries
touch src/application/queries/get_user.rs
touch src/application/queries/list_users.rs

# Create handlers with starter code
cat > src/application/handlers/register_handler.rs <<EOL
use actix_web::{post, web, HttpResponse, Responder};
use crate::application::dto::user_dto::UserDTO;
use tracing::{info, error};

#[post("/auth/register")]
pub async fn handle(payload: web::Json<UserDTO>) -> impl Responder {
    info!("Register request for user: {}", payload.username);

    // Placeholder: implement actual registration logic
    let result: Result<UserDTO, crate::application::errors::ApplicationError> = Ok(payload.0.clone());

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            error!("Registration failed: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        }
    }
}
EOL

cat > src/application/handlers/login_handler.rs <<EOL
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::application::dto::auth_response::AuthResponse;
use tracing::{info, error};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[post("/auth/login")]
pub async fn handle(payload: web::Json<LoginRequest>) -> impl Responder {
    info!("Login attempt: {}", payload.username);

    // Placeholder: implement actual authentication logic with Keycloak
    let response = AuthResponse {
        access_token: "fake-jwt-token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        organisation_name: Some("Org A".to_string()),
        station_name: Some("Station 1".to_string()),
        role: "operator".to_string(),
    };

    HttpResponse::Ok().json(response)
}
EOL

echo "✅ Application layer structure and starter code created successfully!"
echo "Directories and files:"
echo "- src/application/mod.rs"
echo "- src/application/errors.rs"
echo "- src/application/dto/user_dto.rs"
echo "- src/application/dto/auth_response.rs"
echo "- src/application/commands/register_user.rs"
echo "- src/application/commands/create_partner.rs"
echo "- src/application/commands/create_operator.rs"
echo "- src/application/queries/get_user.rs"
echo "- src/application/queries/list_users.rs"
echo "- src/application/handlers/register_handler.rs"
echo "- src/application/handlers/login_handler.rs"
