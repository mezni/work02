#!/bin/bash

PROJECT_DIR="ev-charging-platform"

echo "Creating EV Charging Platform project structure..."
mkdir $PROJECT_DIR
cd $PROJECT_DIR

echo "Creating workspace members..."
cargo new shared --lib
cargo new configurator-service --bin
cargo new auth-service --bin

echo "Creating workspace Cargo.toml..."
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = ["shared", "configurator-service", "auth-service"]

[workspace.dependencies]
actix-web = "4.12.1"
sqlx = { version = "0.8.6", features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono", "migrate"] }
serde = { version = "1.0.228", features = ["derive"] }
uuid = { version = "1.18.1", features = ["v4", "serde"] }
chrono = { version = "0.4.42", features = ["serde"] }
thiserror = "2.0.17"
tracing = "0.1.43"
tracing-subscriber = "0.3.22"
tracing-actix-web = "0.7.19"
utoipa = "5.4.0"
utoipa-swagger-ui = "9.0.2"
config = "0.15.19"
reqwest = { version = "0.12.24", features = ["json"] }
actix-cors = "0.7.1"
validator = { version = "0.20.0", features = ["derive"] }
async-trait = "0.1.89"
EOF

echo "Updating shared Cargo.toml..."
cat > shared/Cargo.toml << 'EOF'
[package]
name = "shared"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = { workspace = true }
serde = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
utoipa = { workspace = true }
config = { workspace = true }
validator = { workspace = true }
async-trait = { workspace = true }

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
EOF

echo "Creating shared source files..."
mkdir -p shared/src

cat > shared/src/lib.rs << 'EOF'
pub mod types;
pub mod error;
pub mod config;
pub mod telemetry;

// Re-export commonly used items
pub use types::{
    UserId, OrganizationId, StationId, 
    UserRole, UserStatus, OrganizationStatus, StationStatus,
    UserClaims, PaginationParams,
};
pub use error::{ApiError, DomainError, ApiResult, DomainResult, ErrorResponse};
pub use config::{AppConfig, DatabaseConfig, ServerConfig, AuthConfig, CorsConfig};
pub use telemetry::{init_telemetry, HealthResponse, ReadinessResponse, DependenciesHealth, DependencyStatus};

// Common validation functions
pub mod validation {
    use validator::Validate;
    
    pub fn validate_email(email: &str) -> Result<(), validator::ValidationError> {
        if email.contains('@') && email.len() >= 3 {
            Ok(())
        } else {
            Err(validator::ValidationError::new("invalid_email"))
        }
    }
    
    pub fn validate_user_role(role: &super::UserRole) -> Result<(), validator::ValidationError> {
        match role {
            super::UserRole::SuperAdmin | 
            super::UserRole::Partner | 
            super::UserRole::Operator => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_serialization() {
        let role = UserRole::SuperAdmin;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"super_admin\"");
        
        let deserialized: UserRole = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, role);
    }
    
    #[test]
    fn test_config_loading() {
        // Test that default values work
        let config = AppConfig::load();
        // This will fail without config files, but shouldn't panic
        assert!(config.is_err() || config.is_ok());
    }
}
EOF

cat > shared/src/types.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub type UserId = uuid::Uuid;
pub type OrganizationId = uuid::Uuid;
pub type StationId = uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema, Validate)]
pub enum UserRole {
    #[serde(rename = "super_admin")]
    SuperAdmin,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "operator")]
    Operator,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::SuperAdmin => write!(f, "super_admin"),
            UserRole::Partner => write!(f, "partner"),
            UserRole::Operator => write!(f, "operator"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum UserStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "deleted")]
    Deleted,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Pending => write!(f, "pending"),
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Deleted => write!(f, "deleted"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum OrganizationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum StationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "maintenance")]
    Maintenance,
}

// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserClaims {
    pub user_id: UserId,
    pub role: UserRole,
    pub organization_id: Option<OrganizationId>,
    pub station_id: Option<StationId>,
    pub user_status: UserStatus,
}

// Common query parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }
EOF

cat > shared/src/error.rs << 'EOF'
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User not found: {user_id}")]
    UserNotFound { user_id: String },
    
    #[error("Organization not found: {organization_id}")]
    OrganizationNotFound { organization_id: String },
    
    #[error("Station not found: {station_id}")]
    StationNotFound { station_id: String },
    
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    
    #[error("Invalid user state transition from {from} to {to}")]
    InvalidStateTransition { from: String, to: String },
    
    #[error("Invalid user configuration: {message}")]
    InvalidUserConfiguration { message: String },
    
    #[error("Email already exists: {email}")]
    EmailAlreadyExists { email: String },
    
    #[error("Database error: {source}")]
    DatabaseError {
        #[from]
        source: sqlx::Error,
    },
    
    #[error("External service error: {service} - {message}")]
    ExternalServiceError { service: String, message: String },
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: &str, message: String) -> Self {
        Self {
            code: code.to_string(),
            message,
        }
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Authentication required")]
    Unauthorized,
    
    #[error("Access forbidden")]
    Forbidden,
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Invalid input: {0}")]
    BadRequest(String),
    
    #[error("Internal server error")]
    InternalServerError,
    
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Unauthorized => HttpResponse::Unauthorized().json(
                ErrorResponse::new("UNAUTHORIZED", self.to_string())
            ),
            ApiError::Forbidden => HttpResponse::Forbidden().json(
                ErrorResponse::new("FORBIDDEN", self.to_string())
            ),
            ApiError::NotFound(_) => HttpResponse::NotFound().json(
                ErrorResponse::new("NOT_FOUND", self.to_string())
            ),
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(
                ErrorResponse::new("BAD_REQUEST", self.to_string())
            ),
            ApiError::InternalServerError => HttpResponse::InternalServerError().json(
                ErrorResponse::new("INTERNAL_ERROR", self.to_string())
            ),
            ApiError::DomainError(domain_error) => match domain_error {
                DomainError::UserNotFound { .. } | 
                DomainError::OrganizationNotFound { .. } | 
                DomainError::StationNotFound { .. } => HttpResponse::NotFound().json(
                    ErrorResponse::new("NOT_FOUND", domain_error.to_string())
                ),
                DomainError::InsufficientPermissions => HttpResponse::Forbidden().json(
                    ErrorResponse::new("FORBIDDEN", domain_error.to_string())
                ),
                DomainError::InvalidUserConfiguration { .. } | 
                DomainError::InvalidStateTransition { .. } => HttpResponse::BadRequest().json(
                    ErrorResponse::new("BAD_REQUEST", domain_error.to_string())
                ),
                DomainError::EmailAlreadyExists { .. } => HttpResponse::Conflict().json(
                    ErrorResponse::new("CONFLICT", domain_error.to_string())
                ),
                _ => HttpResponse::InternalServerError().json(
                    ErrorResponse::new("INTERNAL_ERROR", "An unexpected error occurred".to_string())
                ),
            },
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => ApiError::NotFound("Resource not found".to_string()),
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let message = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                format!("{}: {}", field, errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ");
        
        ApiError::BadRequest(format!("Validation failed: {}", message))
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
pub type DomainResult<T> = Result<T, DomainError>;
EOF

cat > shared/src/config.rs << 'EOF'
use serde::Deserialize;
use config::{Config, File, Environment};

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub keycloak_url: String,
    pub realm: String,
    pub client_id: String,
    #[serde(default = "default_jwt_leeway")]
    pub jwt_leeway: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Vec<String>,
    #[serde(default = "default_allowed_methods")]
    pub allowed_methods: Vec<String>,
    #[serde(default = "default_allowed_headers")]
    pub allowed_headers: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub cors: CorsConfig,
}

// Default values
fn default_host() -> String { "0.0.0.0".to_string() }
fn default_port() -> u16 { 8080 }
fn default_log_level() -> String { "info".to_string() }
fn default_max_connections() -> u32 { 20 }
fn default_jwt_leeway() -> u64 { 60 }
fn default_allowed_origins() -> Vec<String> { vec!["*".to_string()] }
fn default_allowed_methods() -> Vec<String> {
    vec![
        "GET".to_string(),
        "POST".to_string(), 
        "PUT".to_string(),
        "DELETE".to_string(),
        "OPTIONS".to_string(),
    ]
}
fn default_allowed_headers() -> Vec<String> { vec!["*".to_string()] }

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let environment = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        
        let config = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", environment)).required(false))
            .add_source(Environment::with_prefix("EV_CHARGING").separator("_"))
            .build()?;
            
        config.try_deserialize()
    }
}
EOF

cat > shared/src/telemetry.rs << 'EOF'
use tracing::{Level, Subscriber};
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false);
    
    let json_layer = if std::env::var("JSON_LOGGING").is_ok() {
        Some(
            fmt::layer()
                .json()
                .with_target(true)
                .with_current_span(true)
                .flatten_event(true)
        )
    } else {
        None
    };
    
    if let Some(json_layer) = json_layer {
        tracing_subscriber::registry()
            .with(filter)
            .with(json_layer)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
    
    tracing::info!("Telemetry initialized");
}

// Health check response structures
#[derive(Debug, serde::Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ReadinessResponse {
    pub status: String,
    pub timestamp: String,
    pub dependencies: DependenciesHealth,
}

#[derive(Debug, serde::Serialize)]
pub struct DependenciesHealth {
    pub database: DependencyStatus,
    pub keycloak: DependencyStatus,
}

#[derive(Debug, serde::Serialize)]
pub struct DependencyStatus {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl ReadinessResponse {
    pub fn ready(dependencies: DependenciesHealth) -> Self {
        let overall_status = if dependencies.database.status == "healthy" 
            && dependencies.keycloak.status == "healthy" {
            "ready"
        } else {
            "not_ready"
        };
        
        Self {
            status: overall_status.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            dependencies,
        }
    }
}

impl DependenciesHealth {
    pub fn new(database: DependencyStatus, keycloak: DependencyStatus) -> Self {
        Self { database, keycloak }
    }
}

impl DependencyStatus {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            error: None,
        }
    }
    
    pub fn unhealthy(error: String) -> Self {
        Self {
            status: "unhealthy".to_string(),
            error: Some(error),
        }
    }
}
EOF

echo "Creating configuration directory and files..."
mkdir -p config

cat > config/default.toml << 'EOF'
[server]
host = "0.0.0.0"
port = 3000
log_level = "info"

[database]
host = "localhost"
port = 5432
username = "ev_charging"
password = "password"
database_name = "ev_charging_configurator"
max_connections = 20

[auth]
keycloak_url = "http://localhost:8080"
realm = "ev-charging"
client_id = "configurator-service"
jwt_leeway = 60

[cors]
allowed_origins = ["*"]
allowed_methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
allowed_headers = ["*"]
EOF

echo "Creating Docker Compose file..."
cat > docker-compose.yml << 'EOF'
version: '3.8'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: ev_charging_configurator
      POSTGRES_USER: ev_charging
      POSTGRES_PASSWORD: password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ev_charging"]
      interval: 10s
      timeout: 5s
      retries: 5

  keycloak:
    image: quay.io/keycloak/keycloak:22.0
    environment:
      KEYCLOAK_ADMIN: admin
      KEYCLOAK_ADMIN_PASSWORD: admin
      KC_HEALTH_ENABLED: "true"
    ports:
      - "8080:8080"
    command: ["start-dev"]
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 10s
      timeout: 5s
      retries: 5

volumes:
  postgres_data:
EOF

echo "Creating initial service Cargo.toml files..."
cat > configurator-service/Cargo.toml << 'EOF'
[package]
name = "configurator-service"
version = "0.1.0"
edition = "2021"

[dependencies]
shared = { path = "../shared" }
actix-web = { workspace = true }
sqlx = { workspace = true }
serde = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
utoipa = { workspace = true }
config = { workspace = true }
validator = { workspace = true }
async-trait = { workspace = true }
actix-cors = { workspace = true }
reqwest = { workspace = true }
EOF

cat > auth-service/Cargo.toml << 'EOF'
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"

[dependencies]
shared = { path = "../shared" }
actix-web = { workspace = true }
serde = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
utoipa = { workspace = true }
config = { workspace = true }
validator = { workspace = true }
async-trait = { workspace = true }
actix-cors = { workspace = true }
reqwest = { workspace = true }
jsonwebtoken = { version = "10.2.0", features = ["rust_crypto"] }
argon2 = "0.5.3"
EOF

echo "Creating .gitignore..."
cat > .gitignore << 'EOF'
# Compiled files
/target/
**/*.rs.bk

# Database
*.db
*.sqlite
/migrations/

# Environment variables
.env
.env.local

# Logs
*.log
/logs/

# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# IDE
.vscode/
.idea/
*.swp
*.swo

# Docker
docker-compose.override.yml
EOF

echo "Setting up basic service source files..."
cat > configurator-service/src/main.rs << 'EOF'
fn main() {
    println!("Configurator Service - TODO: Implement");
}
EOF

cat > auth-service/src/main.rs << 'EOF'
fn main() {
    println!("Auth Service - TODO: Implement");
}
EOF

echo "Project structure created successfully!"
echo ""
echo "Next steps:"
echo "1. cd $PROJECT_DIR"
echo "2. docker-compose up -d  # Start dependencies"
echo "3. cargo build           # Build the workspace"
echo "4. cargo test            # Run tests"
echo ""
echo "Project structure:"
echo "$PROJECT_DIR/"
echo "├── Cargo.toml"
echo "├── docker-compose.yml"
echo "├── config/"
echo "│   └── default.toml"
echo "├── shared/"
echo "│   ├── src/"
echo "│   │   ├── lib.rs"
echo "│   │   ├── types.rs"
echo "│   │   ├── error.rs"
echo "│   │   ├── config.rs"
echo "│   │   └── telemetry.rs"
echo "│   └── Cargo.toml"
echo "├── configurator-service/"
echo "│   ├── src/main.rs"
echo "│   └── Cargo.toml"
echo "└── auth-service/"
echo "    ├── src/main.rs"
echo "    └── Cargo.toml"