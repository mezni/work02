#!/bin/bash

set -e

echo "Generating authentication service project structure..."

# Create main project directory
mkdir -p auth-service
cd auth-service

# Create source directory structure
mkdir -p src/{domain/{entities,value_objects,enums,repositories},application/{commands,queries,dto,services},infrastructure/{config,database,auth,audit},interfaces/{controllers,routes,openapi}}
mkdir -p tests/{integration,unit}
mkdir -p config
mkdir -p migrations

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"
description = "Authentication and Authorization Microservice with Keycloak"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
readme = "README.md"

[dependencies]
actix-web = "4.4"
sqlx = { version = "0.7", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
validator = { version = "0.16", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = "0.3"
jsonwebtoken = "9.0"
chrono = { version = "0.4", features = ["serde"] }
bcrypt = "0.15"
config = "0.13"
utoipa = { version = "3.0", features = ["actix_extras", "chrono"] }
utoipa-swagger-ui = { version = "3.0", features = ["actix-web"] }
reqwest = { version = "0.11", features = ["json"] }
futures = "0.3"
async-trait = "0.1"

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
wiremock = "0.5"
testcontainers = "0.15"

[build-dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "macros"] }
EOF

# Create main.rs
cat > src/main.rs << 'EOF'
use auth_service::{
    infrastructure::config::Config,
    interfaces::routes::configure_routes,
    prelude::*,
};
use tracing_subscriber;

#[actix_web::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::init();
    
    // Load configuration
    let config = Config::load().expect("Failed to load configuration");
    
    // Create database pool
    let db_pool = auth_service::infrastructure::database::create_pool(&config.database)
        .await
        .expect("Failed to create database pool");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");
    
    // Configure and start server
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(configure_routes)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run();
    
    tracing::info!("Server running on {}:{}", config.server.host, config.server.port);
    server.await?;
    
    Ok(())
}
EOF

# Create lib.rs
cat > src/lib.rs << 'EOF'
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

// Re-exports for common types
pub use domain::entities::{User, Company};
pub use application::dto::{UserDto, CompanyDto};
EOF

# Create mod.rs files for each module
for dir in src/domain src/application src/infrastructure src/interfaces; do
    cat > $dir/mod.rs << 'EOF'
// Module declarations will be added by specific generators
EOF
done

# Create test directory mod files
for dir in tests/integration tests/unit; do
    cat > $dir/mod.rs << 'EOF'
// Test modules
EOF
done

# Create configuration files
cat > config/default.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 8080

[database]
host = "localhost"
port = 5432
username = "postgres"
password = "password"
database_name = "auth_service"
max_connections = 10

[keycloak]
server_url = "http://localhost:8080"
realm = "auth-service-realm"
client_id = "auth-service-client"
client_secret = "your-client-secret"
admin_username = "admin"
admin_password = "admin"

[jwt]
secret = "your-jwt-secret"
issuer = "auth-service"
audience = "auth-service-users"
expiration_days = 7
EOF

cat > config/development.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 8080

[database]
host = "localhost"
port = 5432
username = "postgres"
password = "password"
database_name = "auth_service_dev"

[keycloak]
server_url = "http://localhost:8080"
EOF

cat > config/production.toml << 'EOF'
[server]
host = "0.0.0.0"
port = 8080

[database]
max_connections = 50

[keycloak]
# Production Keycloak settings
EOF

echo "Project structure generated successfully!"
echo "Run the specific generator scripts for each module:"
echo "1. ./generate_domain.sh"
echo "2. ./generate_application.sh" 
echo "3. ./generate_infrastructure.sh"
echo "4. ./generate_interfaces.sh"