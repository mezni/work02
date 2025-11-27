#!/bin/bash

# Script to generate a complete Rust DDD microservice template with Keycloak integration
set -e

PROJECT_NAME="${1:-auth-service}"

echo "🚀 Creating complete DDD microservice: $PROJECT_NAME"

# Create project structure
mkdir -p "$PROJECT_NAME"
cd "$PROJECT_NAME"

# Create main Cargo.toml
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
serde_json = "1.0.145"
chrono = { version = "0.4.42", features = ["serde"] }
validator = { version = "0.20.0", features = ["derive"] }
uuid = { version = "1.18.1", features = ["v4", "serde"] }
async-trait = "0.1.89"
sqlx = { version = "0.8.6", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls", "migrate"] }
jsonwebtoken = { version = "10.2.0", features = ["rust_crypto"] }
reqwest = { version = "0.12.24", features = ["json"] }
bcrypt = "0.17.1"
utoipa = { version = "5.4.0", features = ["actix_extras", "chrono", "uuid"] }
utoipa-swagger-ui = { version = "9.0.2", features = ["actix-web"] }
actix-cors = "0.8.0"
actix-web-httpauth = "1.0.0"

[dev-dependencies]
actix-web = "4.12.0"
tokio = { version = "1.48.0", features = ["full"] }

[[bin]]
name = "auth-service"
path = "src/main.rs"
EOF

# Create basic project structure
mkdir -p src/{domain,application,infrastructure,interfaces,shared}
mkdir -p src/domain/{entities,value_objects,aggregates,repositories,services,events,enums}
mkdir -p src/application/{services,commands,queries,dtos}
mkdir -p src/infrastructure/{persistence,external,auth,config,errors}
mkdir -p src/infrastructure/persistence/{database,repositories}
mkdir -p src/interfaces/{controllers,middleware,routes,models,swagger}
mkdir -p tests/{unit,integration}
mkdir -p config
mkdir -p migrations

# Create basic files
touch src/domain/{entities,value_objects,aggregates,repositories,services,events,enums}/mod.rs
touch src/application/{services,commands,queries,dtos}/mod.rs
touch src/infrastructure/{persistence,external,auth,config,errors}/mod.rs
touch src/infrastructure/persistence/{database,repositories}/mod.rs
touch src/interfaces/{controllers,middleware,routes,models,swagger}/mod.rs

# Create lib.rs
cat > src/lib.rs << 'EOF'
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
pub mod shared;

use actix_web::{web, App, HttpServer};
use infrastructure::persistence::database::{create_pool, run_migrations};
use shared::config::AppConfig;
use tracing::info;

pub async fn run() -> std::io::Result<()> {
    // Initialize logger
    shared::logger::init();
    
    // Load configuration
    let config = AppConfig::load().expect("Failed to load configuration");
    
    info!("🚀 {} service starting...", env!("CARGO_PKG_NAME"));
    
    // Create database pool and run migrations
    let pool = create_pool(&config.database)
        .await
        .expect("Failed to create database pool");
    
    run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");
    
    info!("✅ Database initialized successfully");
    
    // Create infrastructure services
    let jwt_service = infrastructure::auth::jwt::JwtService::new(config.jwt.clone());
    let keycloak_client = infrastructure::auth::keycloak::KeycloakClient::new(config.keycloak.clone());
    
    // Create repositories
    let user_repository = infrastructure::persistence::repositories::PostgresUserRepository::new(pool.clone());
    let company_repository = infrastructure::persistence::repositories::PostgresCompanyRepository::new(pool.clone());
    let audit_log_repository = infrastructure::persistence::repositories::PostgresAuditLogRepository::new(pool);
    
    // Create application services
    let user_app_service = application::services::UserApplicationServiceImpl::new(
        Box::new(user_repository),
        Box::new(company_repository),
        Box::new(audit_log_repository),
        keycloak_client.clone(),
    );
    
    let auth_app_service = application::services::AuthApplicationServiceImpl::new(
        Box::new(infrastructure::persistence::repositories::PostgresUserRepository::new(create_pool(&config.database).await.unwrap()),
        keycloak_client,
        jwt_service,
    );
    
    info!("🌐 Server running at http://{}:{}", config.server.host, config.server.port);
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_app_service.clone()))
            .app_data(web::Data::new(auth_app_service.clone()))
            .configure(interfaces::app_config::configure_app)
            .service(interfaces::swagger::configure_swagger_ui())
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
EOF

# Create main.rs
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

# Create basic shared files
cat > src/shared/logger.rs << 'EOF'
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    println!("Logger initialized");
}
EOF

# Create config files
cat > config/default.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 3000

[database]
host = "localhost"
port = 5432
username = "auth_user"
password = "password"
database_name = "auth_db"
max_connections = 10

[keycloak]
server_url = "http://localhost:8080"
realm = "master"
client_id = "auth-service"
client_secret = "your-client-secret"
admin_username = "admin"
admin_password = "admin"

[jwt]
secret = "your-super-secret-jwt-key-that-is-at-least-32-chars-long"
issuer = "auth-service"
audience = "auth-service-users"
expiration_days = 7

[logging]
level = "info"
EOF

# Create SQL migration
cat > migrations/001_initial_schema.sql << 'EOF'
-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    username