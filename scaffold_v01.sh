#!/bin/bash

set -e

# Configuration
PROJECT_NAME="configurator-service"
AUTHOR="M.MEZNI <mamezni@gmail.com>"

# Define your entities here - add or remove as needed
ENTITIES=("network")

echo "Creating $PROJECT_NAME project structure..."

# Create main directories
mkdir -p ${PROJECT_NAME}/{src,tests,docs,scripts}
mkdir -p ${PROJECT_NAME}/src/{domain,application,infrastructure,api,utils}
mkdir -p ${PROJECT_NAME}/tests/{unit,integration}

# Create domain subdirectories
mkdir -p ${PROJECT_NAME}/src/domain/{models,value_objects,enums,events,services}
mkdir -p ${PROJECT_NAME}/src/domain/models
mkdir -p ${PROJECT_NAME}/src/domain/value_objects
mkdir -p ${PROJECT_NAME}/src/domain/enums
mkdir -p ${PROJECT_NAME}/src/domain/events
mkdir -p ${PROJECT_NAME}/src/domain/services

# Create application subdirectories
mkdir -p ${PROJECT_NAME}/src/application/{commands,queries,handlers,dtos}
mkdir -p ${PROJECT_NAME}/src/application/commands
mkdir -p ${PROJECT_NAME}/src/application/queries
mkdir -p ${PROJECT_NAME}/src/application/handlers
mkdir -p ${PROJECT_NAME}/src/application/dtos

# Create infrastructure subdirectories
mkdir -p ${PROJECT_NAME}/src/infrastructure/{database,config,external}
mkdir -p ${PROJECT_NAME}/src/infrastructure/database/{repositories,migrations}
mkdir -p ${PROJECT_NAME}/src/infrastructure/database/repositories

# Create API subdirectories
mkdir -p ${PROJECT_NAME}/src/api/{routes,handlers,middleware,responses}
mkdir -p ${PROJECT_NAME}/src/api/routes
mkdir -p ${PROJECT_NAME}/src/api/handlers
mkdir -p ${PROJECT_NAME}/src/api/middleware
mkdir -p ${PROJECT_NAME}/src/api/responses

# Create root files
touch ${PROJECT_NAME}/Cargo.toml
touch ${PROJECT_NAME}/Cargo.lock
touch ${PROJECT_NAME}/.env
touch ${PROJECT_NAME}/.gitignore
touch ${PROJECT_NAME}/README.md

# Create source files
touch ${PROJECT_NAME}/src/main.rs
touch ${PROJECT_NAME}/src/lib.rs

# Create domain model files for each entity
touch ${PROJECT_NAME}/src/domain/mod.rs
touch ${PROJECT_NAME}/src/domain/models/mod.rs

for entity in "${ENTITIES[@]}"; do
    echo "Creating files for entity: $entity"
    touch "${PROJECT_NAME}/src/domain/models/${entity}.rs"
done

# Create value objects
touch ${PROJECT_NAME}/src/domain/value_objects/mod.rs
touch ${PROJECT_NAME}/src/domain/value_objects/location.rs
touch ${PROJECT_NAME}/src/domain/value_objects/contact_info.rs
touch ${PROJECT_NAME}/src/domain/value_objects/tags.rs
touch ${PROJECT_NAME}/src/domain/value_objects/operational_status.rs
touch ${PROJECT_NAME}/src/domain/value_objects/verification_status.rs
touch ${PROJECT_NAME}/src/domain/value_objects/email.rs
touch ${PROJECT_NAME}/src/domain/value_objects/phone.rs

# Create enums
touch ${PROJECT_NAME}/src/domain/enums/mod.rs
touch ${PROJECT_NAME}/src/domain/enums/network_type.rs
touch ${PROJECT_NAME}/src/domain/enums/company_type.rs
touch ${PROJECT_NAME}/src/domain/enums/role_type.rs
touch ${PROJECT_NAME}/src/domain/enums/connector_status.rs

# Create event files for each entity
touch ${PROJECT_NAME}/src/domain/events/mod.rs

for entity in "${ENTITIES[@]}"; do
    if [[ "$entity" == "station" || "$entity" == "connector" || "$entity" == "network" ]]; then
        touch "${PROJECT_NAME}/src/domain/events/${entity}_events.rs"
    fi
done

# Create service files
touch ${PROJECT_NAME}/src/domain/services/mod.rs
touch ${PROJECT_NAME}/src/domain/services/station_service.rs
touch ${PROJECT_NAME}/src/domain/services/network_service.rs
touch ${PROJECT_NAME}/src/domain/services/verification_service.rs

# Create application files
touch ${PROJECT_NAME}/src/application/mod.rs

# Create command files for each entity
touch ${PROJECT_NAME}/src/application/commands/mod.rs

for entity in "${ENTITIES[@]}"; do
    if [[ "$entity" == "station" || "$entity" == "connector" || "$entity" == "network" ]]; then
        touch "${PROJECT_NAME}/src/application/commands/${entity}_commands.rs"
    fi
done

# Create query files for each entity
touch ${PROJECT_NAME}/src/application/queries/mod.rs

for entity in "${ENTITIES[@]}"; do
    if [[ "$entity" == "station" || "$entity" == "connector" || "$entity" == "network" ]]; then
        touch "${PROJECT_NAME}/src/application/queries/${entity}_queries.rs"
    fi
done

# Create handler files
touch ${PROJECT_NAME}/src/application/handlers/mod.rs
touch ${PROJECT_NAME}/src/application/handlers/command_handlers.rs
touch ${PROJECT_NAME}/src/application/handlers/query_handlers.rs

# Create DTO files for each entity
touch ${PROJECT_NAME}/src/application/dtos/mod.rs

for entity in "${ENTITIES[@]}"; do
    if [[ "$entity" == "station" || "$entity" == "connector" || "$entity" == "network" ]]; then
        touch "${PROJECT_NAME}/src/application/dtos/${entity}_dtos.rs"
    fi
done

# Create infrastructure files
touch ${PROJECT_NAME}/src/infrastructure/mod.rs
touch ${PROJECT_NAME}/src/infrastructure/database/mod.rs
touch ${PROJECT_NAME}/src/infrastructure/database/connection.rs

# Create repository files for each entity
touch ${PROJECT_NAME}/src/infrastructure/database/repositories/mod.rs

for entity in "${ENTITIES[@]}"; do
    if [[ "$entity" == "station" || "$entity" == "connector" || "$entity" == "network" || "$entity" == "person" ]]; then
        touch "${PROJECT_NAME}/src/infrastructure/database/repositories/${entity}_repository.rs"
    fi
done

touch ${PROJECT_NAME}/src/infrastructure/database/migrations/mod.rs
touch ${PROJECT_NAME}/src/infrastructure/database/migrations/001_initial_schema.sql

touch ${PROJECT_NAME}/src/infrastructure/config/mod.rs
touch ${PROJECT_NAME}/src/infrastructure/config/settings.rs

touch ${PROJECT_NAME}/src/infrastructure/external/mod.rs
touch ${PROJECT_NAME}/src/infrastructure/external/email_service.rs
touch ${PROJECT_NAME}/src/infrastructure/external/sms_service.rs

# Create API route files for each entity
touch ${PROJECT_NAME}/src/api/mod.rs
touch ${PROJECT_NAME}/src/api/routes/mod.rs

for entity in "${ENTITIES[@]}"; do
    if [[ "$entity" == "station" || "$entity" == "connector" || "$entity" == "network" || "$entity" == "person" ]]; then
        touch "${PROJECT_NAME}/src/api/routes/${entity}s.rs"
    fi
done

# Create API handler files for each entity
touch ${PROJECT_NAME}/src/api/handlers/mod.rs

for entity in "${ENTITIES[@]}"; do
    if [[ "$entity" == "station" || "$entity" == "connector" || "$entity" == "network" ]]; then
        touch "${PROJECT_NAME}/src/api/handlers/${entity}_handlers.rs"
    fi
done

# Create middleware files
touch ${PROJECT_NAME}/src/api/middleware/mod.rs
touch ${PROJECT_NAME}/src/api/middleware/auth.rs
touch ${PROJECT_NAME}/src/api/middleware/logging.rs
touch ${PROJECT_NAME}/src/api/middleware/error_handling.rs

# Create response files
touch ${PROJECT_NAME}/src/api/responses/mod.rs
touch ${PROJECT_NAME}/src/api/responses/api_response.rs
touch ${PROJECT_NAME}/src/api/responses/error_response.rs

# Create utils files
touch ${PROJECT_NAME}/src/utils/mod.rs
touch ${PROJECT_NAME}/src/utils/validators.rs
touch ${PROJECT_NAME}/src/utils/datetime.rs
touch ${PROJECT_NAME}/src/utils/id_generator.rs

# Create test files
touch ${PROJECT_NAME}/tests/mod.rs
touch ${PROJECT_NAME}/tests/unit/mod.rs
touch ${PROJECT_NAME}/tests/unit/domain_tests.rs
touch ${PROJECT_NAME}/tests/unit/service_tests.rs
touch ${PROJECT_NAME}/tests/integration/mod.rs

# Create integration test files for main entities
for entity in "${ENTITIES[@]}"; do
    if [[ "$entity" == "station" || "$entity" == "network" ]]; then
        touch "${PROJECT_NAME}/tests/integration/${entity}_tests.rs"
    fi
done

# Create docs files
touch ${PROJECT_NAME}/docs/api.md
touch ${PROJECT_NAME}/docs/architecture.md
touch ${PROJECT_NAME}/docs/deployment.md

# Create script files
touch ${PROJECT_NAME}/scripts/migrate.sh
touch ${PROJECT_NAME}/scripts/test.sh
touch ${PROJECT_NAME}/scripts/deploy.sh

# Make scripts executable
chmod +x ${PROJECT_NAME}/scripts/migrate.sh
chmod +x ${PROJECT_NAME}/scripts/test.sh
chmod +x ${PROJECT_NAME}/scripts/deploy.sh

echo "Creating module structure and mod.rs files..."

# Create lib.rs
cat > ${PROJECT_NAME}/src/lib.rs << 'EOF'
// src/lib.rs
pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod utils;

pub use api::*;
pub use application::*;
pub use domain::*;
pub use infrastructure::*;
pub use utils::*;
EOF

# Create main.rs
cat > ${PROJECT_NAME}/src/main.rs << 'EOF'
// src/main.rs
use configurator_service::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting {}...", env!("CARGO_PKG_NAME"));
    
    // Initialize logging
    env_logger::init();
    
    // Start HTTP server
    api::start_server().await
}
EOF

# Create domain mod.rs files
cat > ${PROJECT_NAME}/src/domain/mod.rs << 'EOF'
// src/domain/mod.rs
pub mod enums;
pub mod events;
pub mod models;
pub mod services;
pub mod value_objects;
EOF

cat > ${PROJECT_NAME}/src/domain/models/mod.rs << 'EOF'
// src/domain/models/mod.rs
pub mod network;
EOF

cat > ${PROJECT_NAME}/src/domain/value_objects/mod.rs << 'EOF'
// src/domain/value_objects/mod.rs
pub mod contact_info;
pub mod email;
pub mod location;
pub mod operational_status;
pub mod phone;
pub mod tags;
pub mod verification_status;
EOF

cat > ${PROJECT_NAME}/src/domain/enums/mod.rs << 'EOF'
// src/domain/enums/mod.rs
pub mod company_type;
pub mod connector_status;
pub mod network_type;
pub mod role_type;
EOF

cat > ${PROJECT_NAME}/src/domain/events/mod.rs << 'EOF'
// src/domain/events/mod.rs
pub mod network_events;
EOF

cat > ${PROJECT_NAME}/src/domain/services/mod.rs << 'EOF'
// src/domain/services/mod.rs
pub mod network_service;
pub mod station_service;
pub mod verification_service;
EOF

# Create application mod.rs files
cat > ${PROJECT_NAME}/src/application/mod.rs << 'EOF'
// src/application/mod.rs
pub mod commands;
pub mod dtos;
pub mod handlers;
pub mod queries;
EOF

cat > ${PROJECT_NAME}/src/application/commands/mod.rs << 'EOF'
// src/application/commands/mod.rs
pub mod network_commands;
EOF

cat > ${PROJECT_NAME}/src/application/queries/mod.rs << 'EOF'
// src/application/queries/mod.rs
pub mod network_queries;
EOF

cat > ${PROJECT_NAME}/src/application/handlers/mod.rs << 'EOF'
// src/application/handlers/mod.rs
pub mod command_handlers;
pub mod query_handlers;
EOF

cat > ${PROJECT_NAME}/src/application/dtos/mod.rs << 'EOF'
// src/application/dtos/mod.rs
pub mod network_dtos;
EOF

# Create infrastructure mod.rs files
cat > ${PROJECT_NAME}/src/infrastructure/mod.rs << 'EOF'
// src/infrastructure/mod.rs
pub mod config;
pub mod database;
pub mod external;
EOF

cat > ${PROJECT_NAME}/src/infrastructure/database/mod.rs << 'EOF'
// src/infrastructure/database/mod.rs
pub mod connection;
pub mod migrations;
pub mod repositories;
EOF

cat > ${PROJECT_NAME}/src/infrastructure/database/repositories/mod.rs << 'EOF'
// src/infrastructure/database/repositories/mod.rs
pub mod network_repository;
EOF

cat > ${PROJECT_NAME}/src/infrastructure/config/mod.rs << 'EOF'
// src/infrastructure/config/mod.rs
pub mod settings;
EOF

cat > ${PROJECT_NAME}/src/infrastructure/external/mod.rs << 'EOF'
// src/infrastructure/external/mod.rs
pub mod email_service;
pub mod sms_service;
EOF

# Create API mod.rs files
cat > ${PROJECT_NAME}/src/api/mod.rs << 'EOF'
// src/api/mod.rs
pub mod handlers;
pub mod middleware;
pub mod responses;
pub mod routes;

use actix_web::{web, App, HttpServer};

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
EOF

cat > ${PROJECT_NAME}/src/api/routes/mod.rs << 'EOF'
// src/api/routes/mod.rs
pub mod networks;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(networks::config)
    );
}
EOF

cat > ${PROJECT_NAME}/src/api/routes/networks.rs << 'EOF'
// src/api/routes/networks.rs
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/networks")
            // Routes will be added here
    );
}
EOF

cat > ${PROJECT_NAME}/src/api/handlers/mod.rs << 'EOF'
// src/api/handlers/mod.rs
pub mod network_handlers;
EOF

cat > ${PROJECT_NAME}/src/api/middleware/mod.rs << 'EOF'
// src/api/middleware/mod.rs
pub mod auth;
pub mod error_handling;
pub mod logging;
EOF

cat > ${PROJECT_NAME}/src/api/responses/mod.rs << 'EOF'
// src/api/responses/mod.rs
pub mod api_response;
pub mod error_response;
EOF

# Create utils mod.rs
cat > ${PROJECT_NAME}/src/utils/mod.rs << 'EOF'
// src/utils/mod.rs
pub mod datetime;
pub mod id_generator;
pub mod validators;
EOF

# Create test mod.rs files
cat > ${PROJECT_NAME}/tests/mod.rs << 'EOF'
// tests/mod.rs
pub mod integration;
pub mod unit;
EOF

cat > ${PROJECT_NAME}/tests/unit/mod.rs << 'EOF'
// tests/unit/mod.rs
pub mod domain_tests;
pub mod service_tests;
EOF

cat > ${PROJECT_NAME}/tests/integration/mod.rs << 'EOF'
// tests/integration/mod.rs
pub mod network_tests;
EOF

# Create basic Cargo.toml content
cat > ${PROJECT_NAME}/Cargo.toml << EOF
[package]
name = "${PROJECT_NAME}"
version = "0.1.0"
edition = "2021"
description = "EV Station Management System"
authors = ["${AUTHOR}"]
license = "MIT"

[[bin]]
name = "${PROJECT_NAME}"
path = "src/main.rs"

[dependencies]
actix-web = "4.12.0"
actix-cors = "0.7.1"
tokio = { version = "1.48.0", features = ["full"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"
sqlx = { version = "0.8.6", features = ["postgres", "runtime-tokio-rustls"] }
chrono = { version = "0.4.42", features = ["serde"] }
uuid = { version = "1.18.1", features = ["v4", "serde"] }
config = "0.15.19"
dotenvy = "0.15.7"
thiserror = "2.0.17"
log = "0.4.28"
env_logger = "0.11.8"

[dev-dependencies]
rstest = "0.26.1"
mockall = "0.13.1"
testcontainers = "0.25.2"
EOF

# Create basic .gitignore
cat > ${PROJECT_NAME}/.gitignore << 'EOF'
/target/
**/*.rs.bk
.env
!.env.example
*.log
.DS_Store
.idea/
.vscode/
EOF

# Create basic README.md
cat > ${PROJECT_NAME}/README.md << EOF
# ${PROJECT_NAME}

A comprehensive EV station management system built with Rust, Actix-web, and SQLx.

## Features

- Station management
- Connector management
- Network management
- User management
- Real-time status updates
- RESTful API with Swagger documentation

## Entities

$(for entity in "${ENTITIES[@]}"; do
  echo "- ${entity}"
done)

## Getting Started

1. Clone the repository
2. Set up environment variables (copy .env.example to .env)
3. Run database migrations: \`./scripts/migrate.sh\`
4. Start the server: \`cargo run\`

## Project Structure

The project follows Domain-Driven Design (DDD) principles:

- \`domain/\`: Core business logic, entities, value objects
- \`application/\`: Use cases, commands, queries
- \`infrastructure/\`: Database, external services, configuration
- \`api/\`: Web API, routes, handlers
- \`utils/\`: Shared utilities

## Build

\`\`\`bash
cargo build
cargo run
\`\`\`

## API Documentation

Once running, access the API at: http://localhost:8080/api/
EOF

# Create basic .env file
cat > ${PROJECT_NAME}/.env << EOF
DATABASE_URL=postgres://user:password@localhost:5432/${PROJECT_NAME}
DATABASE_URL_TEST=postgres://user:password@localhost:5432/${PROJECT_NAME}_test
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
RUST_LOG=debug
JWT_SECRET=your_jwt_secret_key_here
EOF

# Create basic script files
cat > ${PROJECT_NAME}/scripts/migrate.sh << 'EOF'
#!/bin/bash
set -e

echo "Running database migrations..."
# Add your migration commands here
# sqlx migrate run --database-url your_database_url
echo "Migrations completed!"
EOF

cat > ${PROJECT_NAME}/scripts/test.sh << 'EOF'
#!/bin/bash
set -e

echo "Running tests..."
cargo test --verbose
echo "Tests completed!"
EOF

cat > ${PROJECT_NAME}/scripts/deploy.sh << 'EOF'
#!/bin/bash
set -e

echo "Deploying application..."
# Add your deployment commands here
echo "Deployment completed!"
EOF

# Create empty module files with basic structure
for file in ${PROJECT_NAME}/src/domain/models/*.rs; do
    if [ -f "$file" ]; then
        cat > "$file" << 'EOF'
// Placeholder for domain model
pub struct Model {
    // Will be implemented
}

impl Model {
    // Will be implemented
}
EOF
    fi
done

echo "Project structure for '${PROJECT_NAME}' created successfully!"
echo "Entities created: ${ENTITIES[*]}"
echo ""
echo "To build and run:"
echo "  cd ${PROJECT_NAME} && cargo build && cargo run"
echo ""
echo "The project should compile successfully with all modules wired."