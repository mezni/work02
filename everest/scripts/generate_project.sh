#!/bin/bash

# Script to generate a minimal Rust DDD microservice template
PROJECT_NAME="${1:-auth-service}"

echo "Creating DDD template: $PROJECT_NAME"

# Create project structure
mkdir -p "$PROJECT_NAME"
cd "$PROJECT_NAME"

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"
description = "Authentication and Authorization Microservice with Keycloak"
authors = ["M.MEZNI"]
license = "MIT"

[dependencies]
actix-web = "4.12.0"
serde = { version = "1.0.228", features = ["derive"] }
thiserror = "2.0.17"
tracing = "0.1.41"
tracing-subscriber = "0.3.20"
tokio = { version = "1.48.0", features = ["full"] }
config = "0.15.19"

[dev-dependencies]
actix-web = "4.12.0"
tokio = { version = "1.48.0", features = ["full"] }

[[bin]]
name = "auth-service"
path = "src/main.rs"
EOF

# Create source structure
mkdir -p src/{domain,application,infrastructure,interfaces,shared}
mkdir -p src/infrastructure/{persistence,external,auth}
mkdir -p tests/{unit,integration}
mkdir -p config

# Create tests mod.rs
cat > tests/mod.rs << 'EOF'
pub mod unit;
pub mod integration;
EOF

# Create unit tests mod.rs
cat > tests/unit/mod.rs << 'EOF'
// Unit tests will go here
#[cfg(test)]
mod basic_tests {
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}
EOF

# Create integration tests mod.rs
cat > tests/integration/mod.rs << 'EOF'
// Integration tests will go here
use actix_web::{test, App};

#[tokio::test]
async fn test_health_check() {
    use auth_service::run;
    
    // Test that the server can be started
    assert!(true);
}
EOF

# Create minimal lib.rs
cat > src/lib.rs << 'EOF'
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
pub mod shared;

use actix_web::{web, App, HttpServer, Responder};
use shared::{config::AppConfig, logger};

pub async fn run() -> std::io::Result<()> {
    logger::init();
    
    let config = AppConfig::load().unwrap();
    
    println!("✅ {} service starting...", env!("CARGO_PKG_NAME"));
    
    async fn health_check() -> impl Responder {
        "✅ Authentication Service is healthy"
    }
    
    println!("🚀 Server running at http://{}:{}", config.server.host, config.server.port);
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
EOF

# Create minimal main.rs
cat > src/main.rs << 'EOF'
#[tokio::main]
async fn main() -> std::io::Result<()> {
    auth_service::run().await
}
EOF

# Create shared modules
cat > src/shared/mod.rs << 'EOF'
pub mod error;
pub mod result;
pub mod logger;
pub mod config;
EOF

cat > src/shared/logger.rs << 'EOF'
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    println!("Logger initialized");
}
EOF

cat > src/shared/error.rs << 'EOF'
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal server error")]
    Internal,
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Internal => HttpResponse::InternalServerError().body("Internal server error"),
            AppError::ConfigError(msg) => HttpResponse::InternalServerError().body(format!("Configuration error: {}", msg)),
        }
    }
}

impl From<config::ConfigError> for AppError {
    fn from(e: config::ConfigError) -> Self {
        AppError::ConfigError(e.to_string())
    }
}
EOF

cat > src/shared/result.rs << 'EOF'
use crate::shared::error::AppError;

pub type AppResult<T> = Result<T, AppError>;
EOF

cat > src/shared/config.rs << 'EOF'
use serde::Deserialize;
use config::{Config, File};
use crate::shared::error::AppError;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self, AppError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default.toml"))
            .build()?;
        let app_config = config.try_deserialize()?;
        Ok(app_config)
    }
}
EOF

# Create config file
cat > config/default.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 3000
EOF

# Create infrastructure with keycloak directory
cat > src/infrastructure/mod.rs << 'EOF'
pub mod persistence;
pub mod external;
pub mod auth;
EOF

# Only create the keycloak directory structure
mkdir -p src/infrastructure/auth

cat > src/infrastructure/auth/mod.rs << 'EOF'
// Keycloak authentication infrastructure
// Add keycloak client implementation here
EOF

# Create interfaces
cat > src/interfaces/mod.rs << 'EOF'
pub mod http;
pub mod graphql;
EOF

# Create empty module files for other layers
cat > src/domain/mod.rs << 'EOF'
// Domain layer modules
pub mod entities;
pub mod value_objects;
EOF

cat > src/application/mod.rs << 'EOF'
// Application layer modules
pub mod commands;
pub mod queries;
EOF

# Create empty submodules
mkdir -p src/domain/{entities,value_objects}
mkdir -p src/application/{commands,queries}
mkdir -p src/infrastructure/{persistence,external}
mkdir -p src/interfaces/{http,graphql}

touch src/domain/entities/mod.rs
touch src/domain/value_objects/mod.rs
touch src/application/commands/mod.rs
touch src/application/queries/mod.rs
touch src/infrastructure/persistence/mod.rs
touch src/infrastructure/external/mod.rs
touch src/interfaces/http/mod.rs
touch src/interfaces/graphql/mod.rs

# Create .gitignore
cat > .gitignore << 'EOF'
/target
**/*.rs.bk
Cargo.lock
.env
.idea
.vscode
*.log
EOF

# Create README.md
cat > README.md << 'EOF'
# Authentication Service

Authentication and Authorization Microservice with Keycloak.

## Structure
EOF