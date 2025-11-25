#!/bin/bash

set -e

echo "🚀 Setting up Rust Auth Microservice Project Structure..."

# Project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

# Create main source directories
mkdir -p src/{domain,application,infrastructure,interfaces,common}
mkdir -p src/domain/{entities,value_objects,enums,repositories,events}
mkdir -p src/application/{commands,queries,handlers,dto,services,events}
mkdir -p src/infrastructure/{database,auth,config,logging,audit,events,cache}
mkdir -p src/infrastructure/database/repositories
mkdir -p src/infrastructure/events/brokers
mkdir -p src/interfaces/{routes,controllers,openapi}

# Create test directories
mkdir -p tests/{common,unit,integration}
mkdir -p tests/common/{fixtures,helpers}
mkdir -p tests/unit/{domain,application,infrastructure}
mkdir -p tests/unit/domain/{entities,value_objects,events}
mkdir -p tests/unit/application/{commands,handlers,services}
mkdir -p tests/unit/infrastructure/{auth,repositories}

# Create configuration files
touch .env.example
touch .env.test
touch .gitignore
touch Dockerfile
touch docker-compose.yml
touch docker-compose.test.yml
touch README.md

# Create Rust source files
create_rust_files() {
    # Main files
    touch src/main.rs
    touch src/lib.rs
    
    # Domain layer
    touch src/domain/mod.rs
    touch src/domain/errors.rs
    touch src/domain/entities/mod.rs
    touch src/domain/entities/user.rs
    touch src/domain/entities/company.rs
    touch src/domain/entities/role.rs
    touch src/domain/entities/audit_log.rs
    touch src/domain/value_objects/mod.rs
    touch src/domain/value_objects/email.rs
    touch src/domain/value_objects/password.rs
    touch src/domain/value_objects/user_id.rs
    touch src/domain/enums/mod.rs
    touch src/domain/enums/user_role.rs
    touch src/domain/enums/audit_action.rs
    touch src/domain/repositories/mod.rs
    touch src/domain/repositories/user_repository.rs
    touch src/domain/repositories/company_repository.rs
    touch src/domain/repositories/audit_repository.rs
    touch src/domain/events/mod.rs
    touch src/domain/events/domain_events.rs
    touch src/domain/events/user_events.rs
    touch src/domain/events/company_events.rs
    
    # Application layer
    touch src/application/mod.rs
    touch src/application/errors.rs
    touch src/application/commands/mod.rs
    touch src/application/commands/user_commands.rs
    touch src/application/commands/company_commands.rs
    touch src/application/commands/auth_commands.rs
    touch src/application/queries/mod.rs
    touch src/application/queries/user_queries.rs
    touch src/application/queries/company_queries.rs
    touch src/application/queries/audit_queries.rs
    touch src/application/handlers/mod.rs
    touch src/application/handlers/command_handlers.rs
    touch src/application/handlers/query_handlers.rs
    touch src/application/dto/mod.rs
    touch src/application/dto/user_dto.rs
    touch src/application/dto/company_dto.rs
    touch src/application/dto/auth_dto.rs
    touch src/application/dto/audit_dto.rs
    touch src/application/services/mod.rs
    touch src/application/services/audit_service.rs
    touch src/application/events/mod.rs
    touch src/application/events/event_bus.rs
    touch src/application/events/event_handlers.rs
    touch src/application/events/integration_events.rs
    
    # Infrastructure layer
    touch src/infrastructure/mod.rs
    touch src/infrastructure/errors.rs
    touch src/infrastructure/database/mod.rs
    touch src/infrastructure/database/connection.rs
    touch src/infrastructure/database/migrations.rs
    touch src/infrastructure/database/repositories/mod.rs
    touch src/infrastructure/database/repositories/user_repository_impl.rs
    touch src/infrastructure/database/repositories/company_repository_impl.rs
    touch src/infrastructure/database/repositories/audit_repository_impl.rs
    touch src/infrastructure/auth/mod.rs
    touch src/infrastructure/auth/keycloak.rs
    touch src/infrastructure/auth/jwt.rs
    touch src/infrastructure/auth/middleware.rs
    touch src/infrastructure/config/mod.rs
    touch src/infrastructure/config/settings.rs
    touch src/infrastructure/logging/mod.rs
    touch src/infrastructure/logging/middleware.rs
    touch src/infrastructure/audit/mod.rs
    touch src/infrastructure/audit/middleware.rs
    touch src/infrastructure/audit/auditor.rs
    touch src/infrastructure/events/mod.rs
    touch src/infrastructure/events/event_publisher.rs
    touch src/infrastructure/events/event_subscriber.rs
    touch src/infrastructure/events/brokers/mod.rs
    touch src/infrastructure/events/brokers/redis_broker.rs
    touch src/infrastructure/events/brokers/in_memory_broker.rs
    touch src/infrastructure/cache/mod.rs
    touch src/infrastructure/cache/manager.rs
    touch src/infrastructure/cache/keys.rs
    touch src/infrastructure/cache/types.rs
    
    # Interfaces layer
    touch src/interfaces/mod.rs
    touch src/interfaces/errors.rs
    touch src/interfaces/routes/mod.rs
    touch src/interfaces/routes/user_routes.rs
    touch src/interfaces/routes/company_routes.rs
    touch src/interfaces/routes/auth_routes.rs
    touch src/interfaces/routes/audit_routes.rs
    touch src/interfaces/controllers/mod.rs
    touch src/interfaces/controllers/user_controller.rs
    touch src/interfaces/controllers/company_controller.rs
    touch src/interfaces/controllers/auth_controller.rs
    touch src/interfaces/controllers/audit_controller.rs
    touch src/interfaces/openapi/mod.rs
    touch src/interfaces/openapi/spec.rs
    
    # Common
    touch src/common/mod.rs
    touch src/common/response.rs
    
    # Test files
    touch tests/common/mod.rs
    touch tests/common/test_utils.rs
    touch tests/common/fixtures/mod.rs
    touch tests/common/fixtures/users.rs
    touch tests/common/fixtures/companies.rs
    touch tests/common/helpers/mod.rs
    touch tests/common/helpers/database.rs
    touch tests/common/helpers/auth.rs
    
    # Unit tests
    touch tests/unit/mod.rs
    touch tests/unit/domain/mod.rs
    touch tests/unit/domain/entities/mod.rs
    touch tests/unit/domain/entities/user_tests.rs
    touch tests/unit/domain/entities/company_tests.rs
    touch tests/unit/domain/value_objects/mod.rs
    touch tests/unit/domain/value_objects/email_tests.rs
    touch tests/unit/domain/value_objects/password_tests.rs
    touch tests/unit/domain/events/mod.rs
    touch tests/unit/domain/events/user_events_tests.rs
    touch tests/unit/application/mod.rs
    touch tests/unit/application/commands/mod.rs
    touch tests/unit/application/commands/user_commands_tests.rs
    touch tests/unit/application/commands/company_commands_tests.rs
    touch tests/unit/application/handlers/mod.rs
    touch tests/unit/application/handlers/command_handlers_tests.rs
    touch tests/unit/application/services/mod.rs
    touch tests/unit/application/services/audit_service_tests.rs
    touch tests/unit/infrastructure/mod.rs
    touch tests/unit/infrastructure/auth/mod.rs
    touch tests/unit/infrastructure/auth/jwt_tests.rs
    touch tests/unit/infrastructure/auth/keycloak_tests.rs
    touch tests/unit/infrastructure/repositories/mod.rs
    touch tests/unit/infrastructure/repositories/user_repository_tests.rs
    touch tests/unit/infrastructure/repositories/company_repository_tests.rs
    
    # Integration tests
    touch tests/integration/mod.rs
    touch tests/integration/auth_api.rs
    touch tests/integration/user_api.rs
    touch tests/integration/company_api.rs
    touch tests/integration/audit_api.rs
    touch tests/integration/setup.rs
}

create_rust_files

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4"
actix-cors = "0.7"
actix-rt = "2.9"
sqlx = { version = "0.7", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls", "macros"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
anyhow = "1.0"
bcrypt = "0.15"
jsonwebtoken = "9.0"
reqwest = { version = "0.11", features = ["json"] }
config = "0.13"
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"
tracing-actix-web = "0.7"
utoipa = { version = "4.0", features = ["actix_extras", "chrono"] }
utoipa-swagger-ui = "4.0"
validator = { version = "0.16", features = ["derive"] }
async-trait = "0.1"
moka = "0.12"
strum = { version = "0.25", features = ["derive"] }

[dev-dependencies]
rstest = "0.18"
mockall = "0.11"
wiremock = "0.5"
testcontainers = "0.15"
testcontainers-modules = "0.1"
actix-web-httptest = "4.0"
tokio-test = "0.4"
EOF

# Create .gitignore
cat > .gitignore << 'EOF'
# Generated by Cargo
/target/
**/*.rs.bk

# Environment files
.env
.env.local
.env.*.local

# Database
*.db
*.sqlite
*.sqlite3

# Logs
*.log
logs/

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

# Create .env.example
cat > .env.example << 'EOF'
# -------------------------------
# Keycloak Configuration
# -------------------------------
KEYCLOAK_URL=http://localhost:5080
KEYCLOAK_ADMIN=admin
KEYCLOAK_ADMIN_PASSWORD=password
REALM_NAME=ev-realm
CLIENT_NAME=auth-service

# Initial Admin User
ADMIN_USERNAME=admin
ADMIN_EMAIL=admin@company.com
ADMIN_PASSWORD=secret

# -------------------------------
# Database Configuration (Postgres)
# -------------------------------
POSTGRES_HOST=localhost
POSTGRES_PORT=5433
POSTGRES_DB=auth_db
POSTGRES_USER=auth_user
POSTGRES_PASSWORD=password
DATABASE_URL=postgres://auth_user:password@localhost:5433/auth_db

# -------------------------------
# Service Configuration
# -------------------------------
SERVICE_HOST=0.0.0.0
SERVICE_PORT=3000

# JWT / Auth Config
JWT_SECRET=your_jwt_secret_key_here
JWT_EXPIRATION_SECONDS=3600

# -------------------------------
# Logging
# -------------------------------
RUST_LOG=info

# -------------------------------
# Cache Configuration
# -------------------------------
REDIS_URL=redis://localhost:6379

# -------------------------------
# Audit Configuration
# -------------------------------
AUDIT_RETENTION_DAYS=365
EOF

# Create docker-compose.yml
cat > docker-compose.yml << 'EOF'
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: auth_db
      POSTGRES_USER: auth_user
      POSTGRES_PASSWORD: password
    ports:
      - "5433:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U auth_user -d auth_db"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 5

  keycloak:
    image: quay.io/keycloak/keycloak:22.0
    environment:
      KEYCLOAK_ADMIN: admin
      KEYCLOAK_ADMIN_PASSWORD: password
      KC_HOSTNAME: localhost
      KC_HOSTNAME_PORT: 5080
      KC_HTTP_ENABLED: true
    ports:
      - "5080:8080"
    command: start-dev
    depends_on:
      - postgres
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health/ready"]
      interval: 10s
      timeout: 5s
      retries: 10

  auth-service:
    build: .
    ports:
      - "3000:3000"
    environment:
      - RUST_LOG=debug
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
      keycloak:
        condition: service_healthy
    volumes:
      - .:/app
    working_dir: /app

volumes:
  postgres_data:
  redis_data:
EOF
