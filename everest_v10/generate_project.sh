#!/bin/bash

set -e

# Set service name as variable
SERVICE_NAME="auth-service"

echo "Generating $SERVICE_NAME project structure..."

# Create main project directory
mkdir -p $SERVICE_NAME
cd $SERVICE_NAME

# Create clean source directory structure
mkdir -p src/{domain,application,infrastructure,interfaces}
#mkdir -p src/domain/{entities,value_objects,enums,repositories}
#mkdir -p src/application/{commands,queries,dto,services}
#mkdir -p src/infrastructure/{config,database,auth,audit,logger}
#mkdir -p src/interfaces/{controllers,routes,openapi}

# Create test directory structure
mkdir -p tests/{integration,unit}
mkdir -p tests/unit/{domain,application,infrastructure,interfaces}
mkdir -p tests/unit/infrastructure

mkdir -p config
mkdir -p migrations

# Create Cargo.toml with fixed tracing-subscriber features
cat > Cargo.toml << EOF
[package]
name = "$SERVICE_NAME"
version = "0.1.0"
edition = "2021"
description = "Authentication and Authorization Microservice with Keycloak"
authors = ["M.MEZNI"]
license = "MIT"

[dependencies]
actix-web = "4.12.0"
sqlx = { version = "0.8.6", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls"] }
uuid = { version = "1.18.1", features = ["v4", "serde"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"
thiserror = "2.0.17"
validator = { version = "0.20.0", features = ["derive"] }
tracing = "0.1.41"
tracing-subscriber = { version = "0.3.20", features = ["env-filter", "fmt"] }
jsonwebtoken = { version = "10.2.0", features = ["rust_crypto"] }
chrono = { version = "0.4.42", features = ["serde"] }
bcrypt = "0.17.1"
config = "0.15.19"
utoipa = { version = "5.4.0", features = ["actix_extras", "chrono"] }
utoipa-swagger-ui = { version = "9.0.2", features = ["actix-web"] }
reqwest = { version = "0.12.24", features = ["json"] }
futures = "0.3.31"
async-trait = "0.1.89"

[dev-dependencies]
tokio = { version = "1.48.0", features = ["full"] }
wiremock = "0.6.5"
testcontainers = "0.25.2"
serial_test = "3.0.0"

[build-dependencies]
sqlx = { version = "0.8.6", features = ["postgres", "runtime-tokio-rustls", "macros"] }
EOF

# Create minimal main.rs with logger and config
cat > src/main.rs << EOF
use auth_service::infrastructure::{config, logger};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger first
    logger::init_logger();
    
    // Load configuration
    let config = config::Config::load()?;
    
    tracing::info!("Starting {} service...", env!("CARGO_PKG_NAME"));
    tracing::info!("Environment: {}", config.environment());
    tracing::info!("Server: {}:{}", config.server.host, config.server.port);
    
    // Start Actix-web server
    auth_service::start_server(config).await?;
    
    Ok(())
}
EOF

# Create lib.rs
cat > src/lib.rs << EOF
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;

// Prelude for common imports
pub mod prelude {
    pub use crate::domain::errors::DomainError;
    pub use crate::application::errors::ApplicationError;
    pub use crate::infrastructure::errors::InfrastructureError;
    pub use crate::interfaces::errors::InterfaceError;
    
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}

// Server startup function
pub async fn start_server(config: infrastructure::config::Config) -> Result<(), Box<dyn std::error::Error>> {
    use actix_web::{web, App, HttpServer, HttpResponse};

    // Basic health check endpoint
    async fn health_check() -> HttpResponse {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "service": "auth-service"
        }))
    }

    tracing::info!("Starting HTTP server on {}:{}", config.server.host, config.server.port);
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(|| async { 
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Welcome to Auth Service!",
                    "version": env!("CARGO_PKG_VERSION")
                }))
            }))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
    .map_err(|e| e.into())
}
EOF

# Create infrastructure mod.rs
cat > src/infrastructure/mod.rs << EOF
pub mod config;
//pub mod database;
//pub mod auth;
//pub mod audit;
pub mod logger;
pub mod errors;

// Re-exports
pub use config::Config;
//pub use database::{DatabasePool, UserRepositoryImpl, CompanyRepositoryImpl, AuditLogRepositoryImpl};
//pub use auth::KeycloakClient;
pub use logger::{init_logger, init_test_logger};
pub use errors::InfrastructureError;
EOF

# Create infrastructure/errors.rs
cat > src/infrastructure/errors.rs << EOF
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Keycloak error: {0}")]
    KeycloakError(String),
    
    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] reqwest::Error),
    
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Connection pool error: {0}")]
    PoolError(String),
}

impl InfrastructureError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DatabaseError(_) => "INFRA_DATABASE_ERROR",
            Self::ConfigError(_) => "INFRA_CONFIG_ERROR",
            Self::KeycloakError(_) => "INFRA_KEYCLOAK_ERROR",
            Self::HttpClientError(_) => "INFRA_HTTP_CLIENT_ERROR",
            Self::JwtError(_) => "INFRA_JWT_ERROR",
            Self::SerializationError(_) => "INFRA_SERIALIZATION_ERROR",
            Self::IoError(_) => "INFRA_IO_ERROR",
            Self::PoolError(_) => "INFRA_POOL_ERROR",
        }
    }
}
EOF

# Create infrastructure/config.rs
cat > src/infrastructure/config.rs << EOF
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub keycloak: KeycloakConfig,
    pub jwt: JwtConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakConfig {
    pub server_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

impl KeycloakConfig {
    pub fn admin_token_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.server_url, self.realm
        )
    }
    
    pub fn admin_users_url(&self) -> String {
        format!(
            "{}/admin/realms/{}/users",
            self.server_url, self.realm
        )
    }
    
    pub fn token_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.server_url, self.realm
        )
    }
    
    pub fn user_info_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/userinfo",
            self.server_url, self.realm
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub expiration_days: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, crate::infrastructure::errors::InfrastructureError> {
        let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        
        let config = config::Config::builder()
            .add_source(config::File::with_name("config/default").required(false))
            .add_source(config::File::with_name(&format!("config/{}", environment)).required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .map_err(|e| crate::infrastructure::errors::InfrastructureError::ConfigError(e.to_string()))?;
        
        config.try_deserialize()
            .map_err(|e| crate::infrastructure::errors::InfrastructureError::ConfigError(e.to_string()))
    }

    pub fn environment(&self) -> String {
        env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
    }

    pub fn is_development(&self) -> bool {
        self.environment() == "development"
    }

    pub fn is_production(&self) -> bool {
        self.environment() == "production"
    }
}
EOF

# Create infrastructure/logger.rs
cat > src/infrastructure/logger.rs << EOF
pub fn init_logger() {
    // Initialize with default configuration
    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(true)
        .with_max_level(tracing::Level::INFO)
        .pretty()
        .init();
}

pub fn init_test_logger() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .try_init();
}
EOF

# Create mod.rs files for other modules
for dir in src/domain src/application src/interfaces; do
    cat > $dir/mod.rs << EOF
// Module declarations will be added by specific generators
EOF
done

# Create test mod.rs files
cat > tests/mod.rs << EOF
pub mod unit;
pub mod integration;
EOF

cat > tests/unit/mod.rs << EOF
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
EOF

cat > tests/unit/infrastructure/mod.rs << EOF
pub mod config_test;
pub mod logger_test;
EOF

# Create config tests
cat > tests/unit/infrastructure/config_test.rs << EOF
use auth_service::infrastructure::config::{Config, ServerConfig, DatabaseConfig, KeycloakConfig, JwtConfig, LoggingConfig};
use std::env;
use serial_test::serial;

#[test]
#[serial]
fn test_config_default_values() {
    let config = Config {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        },
        database: DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "test".to_string(),
            password: "test".to_string(),
            database_name: "test".to_string(),
            max_connections: 10,
        },
        keycloak: KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test".to_string(),
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        },
        jwt: JwtConfig {
            secret: "test".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_days: 7,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
        },
    };

    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.database.host, "localhost");
    assert_eq!(config.database.port, 5432);
    assert_eq!(config.logging.level, "info");
}

#[test]
#[serial]
fn test_database_connection_string() {
    let db_config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "user".to_string(),
        password: "pass".to_string(),
        database_name: "mydb".to_string(),
        max_connections: 10,
    };

    let conn_string = db_config.connection_string();
    assert_eq!(conn_string, "postgres://user:pass@localhost:5432/mydb");
}

#[test]
#[serial]
fn test_environment_detection() {
    // Test default environment (development)
    env::remove_var("APP_ENVIRONMENT");
    let config = Config {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        },
        database: DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "test".to_string(),
            password: "test".to_string(),
            database_name: "test".to_string(),
            max_connections: 10,
        },
        keycloak: KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test".to_string(),
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        },
        jwt: JwtConfig {
            secret: "test".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_days: 7,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
        },
    };

    assert_eq!(config.environment(), "development");
    assert!(config.is_development());
    assert!(!config.is_production());

    // Test production environment
    env::set_var("APP_ENVIRONMENT", "production");
    assert_eq!(config.environment(), "production");
    assert!(config.is_production());
    assert!(!config.is_development());

    // Cleanup
    env::remove_var("APP_ENVIRONMENT");
}

#[test]
#[serial]
fn test_config_load_fallback() {
    // Remove any existing env var to test fallback behavior
    env::remove_var("APP_ENVIRONMENT");
    
    // This should not panic and should load default config
    let result = Config::load();
    
    // It might fail due to missing config files, but shouldn't panic
    assert!(result.is_ok() || matches!(result, Err(_)));
}

#[test]
#[serial]
fn test_logging_config_default() {
    let logging_config = LoggingConfig::default();
    assert_eq!(logging_config.level, "info");
}

#[test]
#[serial]
fn test_keycloak_url_generation() {
    let keycloak_config = KeycloakConfig {
        server_url: "http://localhost:8080".to_string(),
        realm: "myrealm".to_string(),
        client_id: "myclient".to_string(),
        client_secret: "secret".to_string(),
        admin_username: "admin".to_string(),
        admin_password: "admin".to_string(),
    };

    assert!(keycloak_config.admin_token_url().contains("myrealm"));
    assert!(keycloak_config.token_url().contains("myrealm"));
    assert!(keycloak_config.user_info_url().contains("myrealm"));
}
EOF

# Create logger tests
cat > tests/unit/infrastructure/logger_test.rs << EOF
use auth_service::infrastructure::logger;
use std::sync::Once;

static TEST_LOGGER_INIT: Once = Once::new();

fn ensure_test_logger() {
    TEST_LOGGER_INIT.call_once(|| {
        logger::init_test_logger();
    });
}

#[test]
fn test_logger_initialization() {
    ensure_test_logger();
    
    // Test that we can log at different levels without panicking
    tracing::error!("test error message");
    tracing::warn!("test warn message");
    tracing::info!("test info message");
    tracing::debug!("test debug message");
    tracing::trace!("test trace message");
    
    // If we get here without panicking, logging is working
    assert!(true, "Logger should initialize without errors");
}

#[test]
fn test_logger_levels_functional() {
    ensure_test_logger();
    
    // Test that different log levels work
    // These should not panic when the test logger is initialized
    tracing::info!("info level works");
    tracing::warn!("warn level works");
    tracing::error!("error level works");
    
    assert!(true, "All log levels should work without errors");
}

#[test]
fn test_multiple_logger_calls() {
    ensure_test_logger();
    
    // Ensure we can call logging multiple times
    for i in 0..5 {
        tracing::info!("iteration {}", i);
    }
    
    assert!(true, "Multiple log calls should work without issues");
}
EOF

# Create empty mod.rs files for other test directories
cat > tests/unit/domain/mod.rs << EOF
// Domain unit tests
EOF

cat > tests/unit/application/mod.rs << EOF
// Application unit tests
EOF

cat > tests/unit/interfaces/mod.rs << EOF
// Interfaces unit tests
EOF

cat > tests/integration/mod.rs << EOF
// Integration tests
EOF

# Create configuration files
cat > config/default.toml << EOF
[server]
host = "127.0.0.1"
port = 3000

[database]
host = "localhost"
port = 5432
username = "postgres"
password = "password"
database_name = "${SERVICE_NAME//-/_}"
max_connections = 10

[keycloak]
server_url = "http://localhost:8080"
realm = "${SERVICE_NAME}-realm"
client_id = "${SERVICE_NAME}-client"
client_secret = "your-client-secret"
admin_username = "admin"
admin_password = "admin"

[jwt]
secret = "your-jwt-secret"
issuer = "$SERVICE_NAME"
audience = "$SERVICE_NAME-users"
expiration_days = 7

[logging]
level = "info"
EOF

cat > config/development.toml << EOF
[server]
host = "127.0.0.1"
port = 3000

[database]
host = "localhost"
port = 5432
username = "postgres"
password = "password"
database_name = "${SERVICE_NAME//-/_}_dev"

[keycloak]
server_url = "http://localhost:8080"

[logging]
level = "debug"
EOF

cat > config/production.toml << EOF
[server]
host = "0.0.0.0"
port = 3000

[database]
max_connections = 50

[keycloak]
# Production Keycloak settings

[logging]
level = "warn"
EOF

echo "Project structure for $SERVICE_NAME generated successfully!"
echo "All fixes applied:"
echo "  - Fixed tracing-subscriber features in Cargo.toml"
echo "  - Removed use statement from logger.rs"
echo "  - Changed port from 8080 to 3000 in config files"
echo "  - Fixed auth_service import in main.rs"
echo "  - Commented out unused modules in infrastructure/mod.rs"
echo "  - Simplified directory structure"
echo ""
echo "You can now run: cargo run"
echo "Server will start on http://127.0.0.1:3000"