cargo new auth-service

cd auth-service

# Web Framework
cargo add actix-web
cargo add actix-cors
cargo add actix-rt
# Async
cargo add tokio -F full
cargo add async-trait
cargo add futures
# Serialization
cargo add serde -F derive
cargo add serde_json
cargo add serde_yaml
# Authentication & Cryptography
cargo add jsonwebtoken -F rust_crypto
cargo add argon2
cargo add secrecy -F serde
cargo add bcrypt 
# HTTP Client
cargo add reqwest -F json
cargo add reqwest -F rustls-tls
cargo add http
# Configuration & Environment
cargo add config
cargo add dotenvy
cargo add envy
# Error Handling
cargo add thiserror
cargo add anyhow
# Logging
cargo add log
cargo add env_logger
cargo add tracing
cargo add tracing-subscriber -F env-filter
cargo add tracing-subscriber -F json
cargo add tracing-actix-web
cargo add tracing-bunyan-formatter
# Utilities
cargo add uuid -F v4
cargo add uuid -F serde
cargo add chrono -F serde
cargo add validator -F derive
cargo add regex
cargo add lazy_static
cargo add url
# Database (if needed for caching/sessions)
cargo add sqlx -F postgres
cargo add sqlx -F runtime-tokio-native-tls
cargo add sqlx -F macros
cargo add sqlx -F migrate
cargo add redis -F tokio-comp
# API Documentation (Swagger/OpenAPI)
cargo add utoipa -F actix_extras
cargo add utoipa -F chrono
cargo add utoipa -F uuid
cargo add utoipa-swagger-ui -F actix-web
# Middleware & Utilities
cargo add actix-web-httpauth
cargo add headers
cargo add mime
cargo add cookie -F secure

cd ..

mkdir -p auth-service/src/domain
mkdir -p auth-service/src/application
mkdir -p auth-service/src/infrastructure
mkdir -p auth-service/src/interfaces
mkdir -p auth-service/src/shared

touch auth-service/Cargo.toml
touch auth-service/.env.example
touch auth-service/docker-compose.yml
touch auth-service/Dockerfile
touch auth-service/Makefile
touch auth-service/README.md
touch auth-service/src/main.rs
touch auth-service/src/lib.rs
touch auth-service/src/config.rs
touch auth-service/src/db.rs
touch auth-service/src/logger.rs

touch auth-service/src/domain/mod.rs
touch auth-service/src/domain/user.rs
touch auth-service/src/domain/token.rs
touch auth-service/src/domain/credentials.rs
touch auth-service/src/domain/registration.rs
touch auth-service/src/domain/events.rs
touch auth-service/src/domain/repository.rs
touch auth-service/src/domain/error.rs

touch auth-service/src/application/mod.rs
touch auth-service/src/application/auth_service.rs
touch auth-service/src/application/registration_service.rs
touch auth-service/src/application/token_service.rs
touch auth-service/src/application/commands.rs
touch auth-service/src/application/queries.rs
touch auth-service/src/application/error.rs

touch auth-service/src/infrastructure/keycloak_client.rs
touch auth-service/src/infrastructure/keycloak_repository.rs
touch auth-service/src/infrastructure/token_generator.rs
touch auth-service/src/infrastructure/http_server.rs
touch auth-service/src/infrastructure/cache.rs
touch auth-service/src/infrastructure/config_manager.rs
touch auth-service/src/infrastructure/health_check.rs
touch auth-service/src/infrastructure/error.rs
touch auth-service/src/infrastructure/mod.rs

touch auth-service/src/interfaces/mod.rs
touch auth-service/src/interfaces/http_routes.rs
touch auth-service/src/interfaces/handlers.rs
touch auth-service/src/interfaces/middleware.rs
touch auth-service/src/interfaces/dtos.rs
touch auth-service/src/interfaces/swagger.rs
touch auth-service/src/interfaces/openapi.rs
touch auth-service/src/interfaces/error.rs

touch auth-service/src/shared/mod.rs
touch auth-service/src/shared/constants.rs
touch auth-service/src/shared/utils.rs

touch auth-service/src/error.rs