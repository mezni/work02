#!/bin/bash
# create_infrastructure_layer_with_stubs.sh
# Creates the Infrastructure layer structure and starter Rust files
SERVICE_DIR="auth-service"

echo "Creating Infrastructure layer structure with starter code..."

cd $SERVICE_DIR
# Create directories
mkdir -p src/infrastructure/db
mkdir -p src/infrastructure/keycloak
mkdir -p src/infrastructure/jwt

# Create mod.rs
cat > src/infrastructure/mod.rs <<EOL
pub mod db;
pub mod keycloak;
pub mod jwt;
pub mod logging;
pub mod config;
EOL

# Logging module
cat > src/infrastructure/logging.rs <<EOL
use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();
}
EOL

# Config module
cat > src/infrastructure/config.rs <<EOL
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub keycloak_client_id: String,
    pub keycloak_client_secret: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            keycloak_url: env::var("KEYCLOAK_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
            keycloak_realm: env::var("KEYCLOAK_REALM").unwrap_or_else(|_| "master".to_string()),
            keycloak_client_id: env::var("KEYCLOAK_CLIENT_ID").unwrap_or_else(|_| "auth-service".to_string()),
            keycloak_client_secret: env::var("KEYCLOAK_CLIENT_SECRET").unwrap_or_else(|_| "".to_string()),
        }
    }
}
EOL

# DB repository stubs
cat > src/infrastructure/db/mod.rs <<EOL
pub mod user_repository_pg;
EOL

cat > src/infrastructure/db/user_repository_pg.rs <<EOL
use crate::domain::models::User;
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepositoryPg {
    pub pool: PgPool,
}

#[async_trait]
impl UserRepository for UserRepositoryPg {
    async fn get_by_id(&self, id: Uuid) -> Result<User, DomainError> {
        // TODO: implement database query
        Err(DomainError::UserNotFound)
    }

    async fn get_by_username(&self, username: &str) -> Result<User, DomainError> {
        // TODO: implement database query
        Err(DomainError::UserNotFound)
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        // TODO: implement database save
        Ok(())
    }
}
EOL

# Keycloak client stubs
cat > src/infrastructure/keycloak/mod.rs <<EOL
pub mod keycloak_client;
EOL

cat > src/infrastructure/keycloak/keycloak_client.rs <<EOL
use crate::domain::errors::DomainError;
use crate::application::dto::auth_response::AuthResponse;
use crate::infrastructure::config::Settings;

pub struct KeycloakClient {
    settings: Settings,
}

impl KeycloakClient {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    pub async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<(), DomainError> {
        // TODO: call Keycloak REST API to create user
        Ok(())
    }

    pub async fn assign_role(&self, username: &str, role: &str) -> Result<(), DomainError> {
        // TODO: call Keycloak REST API to assign role
        Ok(())
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Result<AuthResponse, DomainError> {
        // TODO: call Keycloak to authenticate and return token
        Ok(AuthResponse {
            access_token: "fake-token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            organisation_name: Some("Org A".to_string()),
            station_name: Some("Station 1".to_string()),
            role: "operator".to_string(),
        })
    }
}
EOL

# JWT token enricher stub
cat > src/infrastructure/jwt/mod.rs <<EOL
pub mod token_enricher;
EOL

cat > src/infrastructure/jwt/token_enricher.rs <<EOL
use crate::application::dto::auth_response::AuthResponse;

pub struct JwtTokenEnricher;

impl JwtTokenEnricher {
    pub fn enrich(token: &str, organisation_name: Option<String>, station_name: Option<String>, role: &str) -> AuthResponse {
        AuthResponse {
            access_token: token.to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            organisation_name,
            station_name,
            role: role.to_string(),
        }
    }
}
EOL

echo "✅ Infrastructure layer structure and starter code created successfully!"
echo "Directories and files:"
echo "- src/infrastructure/mod.rs"
echo "- src/infrastructure/logging.rs"
echo "- src/infrastructure/config.rs"
echo "- src/infrastructure/db/mod.rs"
echo "- src/infrastructure/db/user_repository_pg.rs"
echo "- src/infrastructure/keycloak/mod.rs"
echo "- src/infrastructure/keycloak/keycloak_client.rs"
echo "- src/infrastructure/jwt/mod.rs"
echo "- src/infrastructure/jwt/token_enricher.rs"
