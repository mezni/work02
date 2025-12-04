#!/bin/bash

set -e

PROJECT_NAME="auth-service"

# Create project
cargo new $PROJECT_NAME --bin
cd $PROJECT_NAME

# Install required crates
cargo add actix-web env_logger dotenvy serde_json async-trait thiserror anyhow

cargo add utoipa -F actix_extras
cargo add utoipa-swagger-ui -F actix-web
cargo add reqwest -F json
cargo add uuid -F v4,serde
cargo add serde -F derive
cargo add tokio -F full

# Create folder structure
mkdir -p src/{domain,infrastructure,application,interfaces/http}
touch .env

touch src/domain/mod.rs
touch src/domain/errors.rs
touch src/infrastructure/mod.rs
touch src/infrastructure/errors.rs
touch src/application/mod.rs
touch src/application/errors.rs
touch src/interfaces/mod.rs
touch src/application/errors.rs
touch src/interfaces/http/mod.rs
