#!/bin/bash

# Project name
PROJECT_NAME="auth-service"

# Create main project folder
mkdir -p $PROJECT_NAME
cd $PROJECT_NAME || exit

# Cargo.toml
touch Cargo.toml

# src folder
mkdir -p src

# Domain layer
mkdir -p src/domain
touch src/domain/mod.rs
touch src/domain/user.rs
touch src/domain/claims.rs
touch src/domain/errors.rs

# Application layer
mkdir -p src/application/commands
mkdir -p src/application/queries
mkdir -p src/application/ports
touch src/application/mod.rs
touch src/application/commands/user_commands.rs
touch src/application/queries/user_queries.rs
touch src/application/ports/user_repository.rs
touch src/application/ports/auth_gateway.rs

# Infrastructure layer
mkdir -p src/infrastructure/db
mkdir -p src/infrastructure/keycloak
touch src/infrastructure/mod.rs
touch src/infrastructure/config.rs
touch src/infrastructure/db/sqlx_user_repository.rs
touch src/infrastructure/keycloak/keycloak_adapter.rs

# Interfaces layer
mkdir -p src/interfaces/http
touch src/interfaces/http/handlers.rs
touch src/interfaces/http/routes.rs
touch src/interfaces/http/dto.rs

# Root files
touch src/lib.rs
touch src/main.rs

# Migrations folder
mkdir -p migrations
touch migrations/0001_create_users.sql

# Optional assets folder (for Swagger UI if needed later)
mkdir -p assets/swagger

# Output
echo "Project skeleton created successfully!"
