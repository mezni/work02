#!/bin/bash
# create_structure.sh
# Creates Auth-Service folder, files, migrations, .env, and layer scripts with Swagger support

SERVICE_DIR="auth-service"
echo "Creating Auth-Service project in $SERVICE_DIR ..."

# 1. Create main folders and enter directory
mkdir -p $SERVICE_DIR/src
cd $SERVICE_DIR || { echo "Failed to enter $SERVICE_DIR"; exit 1; }

# Root files
echo "Creating Cargo.toml, main.rs, lib.rs, and .env..."
cat > Cargo.toml <<EOL
[package]
name = "auth_service"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web Framework
actix-web = "4"
actix-rt = "2"

# Database
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "uuid", "chrono"] }

# Utilities
async-trait = "0.1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
dotenvy = "0.15"
anyhow = "1.0"
thiserror = "1.0"

# Serialization/Deserialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Tracing/Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter"] }

# Auth/Keycloak
jsonwebtoken = "9.0"
# NOTE: keycloak-rs is a good starting point, but often needs custom HTTP logic for full features. 
# We'll use a custom KeycloakClient stub backed by reqwest for flexibility.
reqwest = { version = "0.11", features = ["json"] } 

# OpenAPI/Swagger
utoipa = { version = "4", features = ["macros", "serde_json"] }
utoipa-swagger-ui = "4"

# Dependency Injection - using simple struct/trait pattern for this example
# Injector logic will be in startup
EOL

cat > main.rs <<EOL
use auth_service::startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    auth_service::infrastructure::logging::setup_tracing();

    match startup::run().await {
        Ok(()) => {
            tracing::info!("Server shut down successfully.");
            Ok(())
        }
        Err(e) => {
            tracing::error!("Server startup failed: {}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        }
    }
}
EOL

cat > lib.rs <<EOL
// Core DDD Layers
pub mod domain;
pub mod application;
pub mod infrastructure;

// Interface/Presentation
pub mod interfaces;

// Startup and Utilities
pub mod startup;
pub mod swagger;
EOL

cat > .env <<EOL
# Infrastructure Configuration
DATABASE_URL=postgres://postgres:password@localhost/auth_db
JWT_SECRET=a_very_secret_key_that_is_at_least_32_bytes_long

# Keycloak Configuration
KEYCLOAK_REALM_URL=http://localhost:8080/realms/your_realm
KEYCLOAK_CLIENT_ID=auth-service-client
KEYCLOAK_CLIENT_SECRET=a_secret_for_keycloak
KEYCLOAK_MASTER_USER=admin_user
KEYCLOAK_MASTER_PASSWORD=admin_password

# Server
HOST=127.0.0.1
PORT=8000
EOL

# Migrations folder
mkdir -p migrations
cat > migrations/0001_init.sql <<EOL
-- Add sqlx migration files in the migrations directory.
-- Create the minimal tables needed for the Auth-Service (Organisation, Station, User)

CREATE TABLE organisations (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    keycloak_group_id UUID NOT NULL
);

CREATE TABLE stations (
    id UUID PRIMARY KEY,
    organisation_id UUID NOT NULL REFERENCES organisations(id),
    name VARCHAR(255) NOT NULL,
    keycloak_group_id UUID NOT NULL,
    UNIQUE (organisation_id, name)
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    organisation_id UUID REFERENCES organisations(id),
    station_id UUID REFERENCES stations(id),
    username VARCHAR(255) NOT NULL UNIQUE,
    keycloak_id UUID NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);

-- Initial Data (optional, but good for setup)
INSERT INTO organisations (id, name, keycloak_group_id) VALUES 
('00000000-0000-0000-0000-000000000001', 'Default Public', '00000000-0000-0000-0000-000000000001');

INSERT INTO stations (id, organisation_id, name, keycloak_group_id) VALUES 
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'Default Station', '00000000-0000-0000-0000-000000000001');
EOL

# 2. Create directories for layers
echo "Creating DDD layer directories..."
mkdir -p src/{domain/models,domain/value_objects,domain/repositories,domain/services}
mkdir -p src/{application/commands,application/queries,application/dto,application/handlers}
mkdir -p src/{infrastructure/db,infrastructure/keycloak,infrastructure/jwt}
mkdir -p src/interfaces/http
mkdir -p src/{startup,swagger}

# 3. Create layer files
echo "Creating layer files..."

# Domain Layer
touch src/domain/{mod.rs,errors.rs}
touch src/domain/models/{user.rs,organisation.rs,station.rs}
touch src/domain/value_objects/{email.rs,username.rs,role.rs}
touch src/domain/repositories/{user_repository.rs,organisation_repository.rs}
touch src/domain/services/user_domain_service.rs

# Application Layer
touch src/application/{mod.rs,errors.rs}
touch src/application/dto/{auth_request.rs,auth_response.rs,user_dto.rs}
touch src/application/commands/{register_user.rs,admin_create_user.rs}
touch src/application/handlers/{register_handler.rs,login_handler.rs,admin_create_user_handler.rs}

# Infrastructure Layer
touch src/infrastructure/{mod.rs,logging.rs,config.rs,ioc.rs}
touch src/infrastructure/db/{mod.rs,user_repository_pg.rs,organisation_repository_pg.rs}
touch src/infrastructure/keycloak/{mod.rs,keycloak_client.rs}
touch src/infrastructure/jwt/{mod.rs,token_enricher.rs,claims.rs}

# Interfaces Layer
touch src/interfaces/{mod.rs,errors.rs}
touch src/interfaces/http/{mod.rs,routes.rs,auth_controller.rs}

# Startup Layer
touch src/startup.rs
touch src/swagger.rs

echo "---"
echo "✅ Auth-Service project setup complete in $SERVICE_DIR."
echo "Please populate the environment variables in $SERVICE_DIR/.env"
echo "---"