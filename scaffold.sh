#!/bin/bash

set -e

# Configuration
PROJECT_ROOT="XXX"
AUTHOR="M.MEZNI <mamezni@gmail.com>"
SERVICE_NAME="ms1"

create_base_directories() {
    echo "Creating base project structure..."
    mkdir -p "${PROJECT_ROOT}"/{services,deploy,docs,scripts,.github/workflows}
    touch "${PROJECT_ROOT}"/{.gitignore,README.md,Cargo.toml,docker-compose.yml,.env.example}
}

add_service() {
    local service_name="$1"
    echo "Creating service: $service_name"
    
    # Base service directories
    mkdir -p "${PROJECT_ROOT}/services/${service_name}"/{src,tests,docs,scripts,migrations}
    
    # DDD source structure
    mkdir -p "${PROJECT_ROOT}/services/${service_name}/src"/{domain,application,infrastructure,interfaces}
    
    # Domain layer
    mkdir -p "${PROJECT_ROOT}/services/${service_name}/src/domain"/{entities,value_objects,events,services,common}
    touch "${PROJECT_ROOT}/services/${service_name}/src/domain"/{mod.rs,${service_name}.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/domain/entities"/{mod.rs,${service_name}_entity.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/domain/value_objects"/{mod.rs,common_vo.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/domain/events"/{mod.rs,${service_name}_events.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/domain/services"/{mod.rs,domain_services.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/domain/common"/{mod.rs,types.rs,errors.rs}
    
    # Application layer
    mkdir -p "${PROJECT_ROOT}/services/${service_name}/src/application"/{use_cases,commands,queries,dtos,common}
    touch "${PROJECT_ROOT}/services/${service_name}/src/application"/{mod.rs,application.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/application/use_cases"/{mod.rs,create_${service_name}.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/application/commands"/{mod.rs,create_${service_name}_command.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/application/queries"/{mod.rs,get_${service_name}_query.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/application/dtos"/{mod.rs,request.rs,response.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/application/common"/{mod.rs,errors.rs,traits.rs}
    
    # Infrastructure layer
    mkdir -p "${PROJECT_ROOT}/services/${service_name}/src/infrastructure"/{database,web,messaging,config,common}
    mkdir -p "${PROJECT_ROOT}/services/${service_name}/src/infrastructure/database"/{repositories,connections,migrations}
    touch "${PROJECT_ROOT}/services/${service_name}/src/infrastructure"/{mod.rs,infrastructure.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/infrastructure/database"/{mod.rs,connection.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/infrastructure/database/repositories"/{mod.rs,${service_name}_repository.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/infrastructure/web"/{mod.rs,routes.rs,middleware.rs,swagger.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/infrastructure/messaging"/{mod.rs,event_publisher.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/infrastructure/config"/{mod.rs,settings.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/infrastructure/common"/{mod.rs,errors.rs,utils.rs}
    
    # Interfaces layer
    mkdir -p "${PROJECT_ROOT}/services/${service_name}/src/interfaces"/{controllers,api,common}
    touch "${PROJECT_ROOT}/services/${service_name}/src/interfaces"/{mod.rs,interfaces.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/interfaces/controllers"/{mod.rs,${service_name}_controller.rs,health_controller.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/interfaces/api"/{mod.rs,openapi.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/src/interfaces/common"/{mod.rs,errors.rs,validation.rs}
    
    # Main files
    touch "${PROJECT_ROOT}/services/${service_name}/src"/{main.rs,lib.rs}
    
    # Test structure
    mkdir -p "${PROJECT_ROOT}/services/${service_name}/tests"/{unit,integration,fixtures}
    touch "${PROJECT_ROOT}/services/${service_name}/tests"/{mod.rs,test_utils.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/tests/unit"/{mod.rs,domain_tests.rs,application_tests.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/tests/integration"/{mod.rs,api_tests.rs,db_tests.rs}
    touch "${PROJECT_ROOT}/services/${service_name}/tests/fixtures"/{mod.rs,test_data.rs}
    
    # Service configuration files
    touch "${PROJECT_ROOT}/services/${service_name}"/{Cargo.toml,Dockerfile,.env.example,README.md}
    touch "${PROJECT_ROOT}/services/${service_name}/docs"/{api.yaml,architecture.md}
    touch "${PROJECT_ROOT}/services/${service_name}/scripts"/{dev.sh,test.sh,migrate.sh}
}

create_deployment_structure() {
    echo "Creating deployment structure..."
    
    # Kubernetes
    mkdir -p "${PROJECT_ROOT}/deploy"/{k8s,docker-compose,monitoring,scripts}
    mkdir -p "${PROJECT_ROOT}/deploy/k8s"/{infrastructure,services,config}
    mkdir -p "${PROJECT_ROOT}/deploy/k8s/services/${SERVICE_NAME}"
    mkdir -p "${PROJECT_ROOT}/deploy/k8s/infrastructure"/{postgresql,redis,rabbitmq}
    
    # K8s files
    touch "${PROJECT_ROOT}/deploy/k8s"/{namespace.yaml,configmap.yaml,secrets.yaml}
    touch "${PROJECT_ROOT}/deploy/k8s/services/${SERVICE_NAME}"/{deployment.yaml,service.yaml,configmap.yaml}
    touch "${PROJECT_ROOT}/deploy/k8s/infrastructure"/{postgresql.yaml,redis.yaml,rabbitmq.yaml}
    
    # Docker compose
    touch "${PROJECT_ROOT}/deploy/docker-compose"/{docker-compose.dev.yml,docker-compose.prod.yml,docker-compose.test.yml}
    
    # Monitoring
    touch "${PROJECT_ROOT}/deploy/monitoring"/{prometheus.yml,grafana-dashboard.json,loki-config.yaml}
    
    # Scripts
    touch "${PROJECT_ROOT}/deploy/scripts"/{deploy.sh,health-check.sh,migrate-db.sh}
}

create_documentation() {
    echo "Creating documentation structure..."
    
    mkdir -p "${PROJECT_ROOT}/docs"/{architecture,api,development,deployment}
    
    touch "${PROJECT_ROOT}/docs/architecture"/{microservices.md,data-flow.md,domain-model.md}
    touch "${PROJECT_ROOT}/docs/api"/{gateway-swagger.yaml,${SERVICE_NAME}-api.yaml}
    touch "${PROJECT_ROOT}/docs/development"/{setup.md,testing.md,coding-standards.md}
    touch "${PROJECT_ROOT}/docs/deployment"/{local.md,production.md,kubernetes.md}
    
    # Main documentation
    touch "${PROJECT_ROOT}/docs"/{README.md,api-guidelines.md,deployment-guide.md}
}

create_config_files() {
    echo "Creating configuration files..."
    
    # Root Cargo.toml (workspace)
    cat > "${PROJECT_ROOT}/Cargo.toml" << EOF
[workspace]
members = [
    "services/${SERVICE_NAME}",
]

resolver = "2"

[workspace.dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
tracing = "0.1"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"

[workspace.package]
authors = ["${AUTHOR}"]
version = "0.1.0"
edition = "2021"
EOF

    # Service Cargo.toml
    cat > "${PROJECT_ROOT}/services/${SERVICE_NAME}/Cargo.toml" << EOF
[package]
name = "${SERVICE_NAME}"
version = "0.1.0"
edition = "2021"
authors = ["${AUTHOR}"]

[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
tracing = "0.1"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"

# Database
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "macros"] }

# Web framework
warp = "0.3"

# API documentation
utoipa = { version = "4.0", features = ["warp"] }
utoipa-swagger-ui = { version = "4.0", features = ["warp"] }

# Validation
validator = { version = "0.16", features = ["derive"] }

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# Configuration
config = "0.13"

# Caching
redis = { version = "0.23", features = ["tokio-comp"] }

[dev-dependencies]
rstest = "0.18"
testcontainers = "0.15"
mockall = "0.11"
EOF

    # Docker Compose
    cat > "${PROJECT_ROOT}/docker-compose.yml" << EOF
version: '3.8'

services:
  ${SERVICE_NAME}:
    build: ./services/${SERVICE_NAME}
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://user:password@${SERVICE_NAME}-db:5432/${SERVICE_NAME}
      - REDIS_URL=redis://redis:6379
      - RUST_LOG=info
    depends_on:
      - ${SERVICE_NAME}-db
      - redis

  ${SERVICE_NAME}-db:
    image: postgres:15
    environment:
      - POSTGRES_DB=${SERVICE_NAME}
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=password
    ports:
      - "5432:5432"
    volumes:
      - ${SERVICE_NAME}_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

volumes:
  ${SERVICE_NAME}_data:
EOF

    # Service Dockerfile
    cat > "${PROJECT_ROOT}/services/${SERVICE_NAME}/Dockerfile" << EOF
FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \\
    ca-certificates \\
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/${SERVICE_NAME} /app/
COPY --from=builder /app/migrations /app/migrations

EXPOSE 8080
ENV RUST_LOG=info

CMD ["./${SERVICE_NAME}"]
EOF

    # Service environment example
    cat > "${PROJECT_ROOT}/services/${SERVICE_NAME}/.env.example" << EOF
# Database
DATABASE_URL=postgres://user:password@localhost:5432/${SERVICE_NAME}

# Redis
REDIS_URL=redis://localhost:6379

# Logging
RUST_LOG=info

# Server
PORT=8080
HOST=0.0.0.0

# Application specific
APP_ENV=development
EOF

    # Gitignore
    cat > "${PROJECT_ROOT}/.gitignore" << EOF
# Rust
/target/
**/*.rs.bk
Cargo.lock

# Environment
.env
.env.local
.env.production

# Databases
*.db
*.sqlite
**/migrations/*.sql

# Logs
*.log
logs/

# IDE
.vscode/
.idea/
*.swp
*.swo

# Docker
docker-compose.override.yml

# OS
.DS_Store
Thumbs.db

# Build artifacts
**/build/
**/dist/
EOF

    # Create basic lib.rs
    cat > "${PROJECT_ROOT}/services/${SERVICE_NAME}/src/lib.rs" << EOF
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;

pub use domain::*;
pub use application::*;
pub use infrastructure::*;
pub use interfaces::*;
EOF

    # Create basic main.rs
    cat > "${PROJECT_ROOT}/services/${SERVICE_NAME}/src/main.rs" << EOF
use ${SERVICE_NAME}::infrastructure::config::Settings;
use ${SERVICE_NAME}::interfaces::controllers::health_controller;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");
    
    // Start the server
    health_controller::start_server(settings.server.port).await?;
    
    Ok(())
}
EOF
}

create_github_workflows() {
    echo "Creating GitHub workflows..."
    
    # CI workflow
    cat > "${PROJECT_ROOT}/.github/workflows/ci.yml" << EOF
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          
      - name: Build
        run: cargo build --verbose
        
      - name: Run tests
        run: cargo test --verbose
        
      - name: Check format
        run: cargo fmt -- --check
        
      - name: Clippy
        run: cargo clippy -- -D warnings

  docker:
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: actions/checkout@v4
      
      - name: Build Docker image
        run: docker build -t ${SERVICE_NAME}:latest ./services/${SERVICE_NAME}
EOF
}

main() {
    echo "Creating DDD microservice structure for: $SERVICE_NAME"
    echo "Project root: $PROJECT_ROOT"
    echo "Author: $AUTHOR"
    
    create_base_directories
    add_service "$SERVICE_NAME"
    create_deployment_structure
    create_documentation
    create_config_files
    create_github_workflows
    
    echo ""
    echo "✅ DDD microservice structure created successfully!"
    echo "📁 Project root: $PROJECT_ROOT"
    echo "🚀 Service: $SERVICE_NAME"
    echo ""
    echo "Next steps:"
    echo "1. Update ${PROJECT_ROOT}/services/${SERVICE_NAME}/Cargo.toml with specific dependencies"
    echo "2. Define your domain models in ${PROJECT_ROOT}/services/${SERVICE_NAME}/src/domain/"
    echo "3. Configure database connections in ${PROJECT_ROOT}/services/${SERVICE_NAME}/src/infrastructure/config/"
    echo "4. Update deployment configurations in ${PROJECT_ROOT}/deploy/"
    echo ""
}

main