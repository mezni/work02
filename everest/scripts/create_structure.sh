#!/bin/bash
# create_auth_service_full.sh
# Creates Auth-Service folder, files, migrations, .env, and layer scripts with Swagger support

SERVICE_DIR="auth-service"
echo "Creating Auth-Service project in $SERVICE_DIR ..."

# 1. Create main folders
mkdir -p $SERVICE_DIR/src
cd $SERVICE_DIR || exit

# Root files
cat > Cargo.toml <<EOL
[package]
name = "auth_service"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
actix-rt = "2"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
async-trait = "0.1"
uuid = { version = "1", features = ["v4"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenvy = "0.15"
jsonwebtoken = "9.0"
keycloak-rs = "0.2"
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter"] }
utoipa = { version = "3", features = ["macros", "serde-json"] }
utoipa-swagger-ui = "3"
EOL

cat > main.rs <<EOL
use auth_service::startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    startup::run().await
}
EOL

cat > lib.rs <<EOL
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
pub mod startup;
pub mod swagger;
EOL

touch .env

# Migrations folder
mkdir -p migrations
touch migrations/0001_init.sql

# 2. Create directories for layers
mkdir -p src/domain/models src/domain/value_objects src/domain/repositories src/domain/services
mkdir -p src/application/commands src/application/queries src/application/dto src/application/handlers
mkdir -p src/infrastructure/db src/infrastructure/keycloak src/infrastructure/jwt
mkdir -p src/interfaces/http
mkdir -p src/startup src/swagger

# 3. Create layer scripts
# Domain
cat > create_domain.sh <<'EOL'
#!/bin/bash
mkdir -p src/domain/models src/domain/value_objects src/domain/repositories src/domain/services
touch src/domain/mod.rs
touch src/domain/errors.rs
touch src/domain/models/user.rs
touch src/domain/models/organisation.rs
touch src/domain/models/station.rs
touch src/domain/value_objects/email.rs
touch src/domain/value_objects/username.rs
touch src/domain/repositories/user_repository.rs
touch src/domain/services/user_domain_service.rs
echo "Domain layer files created."
EOL
chmod +x create_domain.sh

# Application
cat > create_application.sh <<'EOL'
#!/bin/bash
mkdir -p src/application/commands src/application/queries src/application/dto src/application/handlers
touch src/application/mod.rs
touch src/application/errors.rs
touch src/application/dto/user_dto.rs
touch src/application/dto/auth_response.rs
touch src/application/handlers/register_handler.rs
touch src/application/handlers/login_handler.rs
touch src/application/commands/register_user.rs
touch src/application/commands/create_partner.rs
touch src/application/commands/create_operator.rs
touch src/application/queries/get_user.rs
touch src/application/queries/list_users.rs
echo "Application layer files created."
EOL
chmod +x create_application.sh

# Infrastructure
cat > create_infrastructure.sh <<'EOL'
#!/bin/bash
mkdir -p src/infrastructure/db src/infrastructure/keycloak src/infrastructure/jwt
touch src/infrastructure/mod.rs
touch src/infrastructure/logging.rs
touch src/infrastructure/config.rs
touch src/infrastructure/db/mod.rs
touch src/infrastructure/db/user_repository_pg.rs
touch src/infrastructure/keycloak/mod.rs
touch src/infrastructure/keycloak/keycloak_client.rs
touch src/infrastructure/jwt/mod.rs
touch src/infrastructure/jwt/token_enricher.rs
echo "Infrastructure layer files created."
EOL
chmod +x create_infrastructure.sh

# Interfaces
cat > create_interfaces.sh <<'EOL'
#!/bin/bash
mkdir -p src/interfaces/http
touch src/interfaces/mod.rs
touch src/interfaces/http/routes.rs
touch src/interfaces/http/middleware.rs
touch src/interfaces/http/mod.rs
echo "Interfaces layer files created."
EOL
chmod +x create_interfaces.sh

# Startup
cat > create_startup.sh <<'EOL'
#!/bin/bash
mkdir -p src/startup
touch src/startup.rs
echo "Startup files created."
EOL
chmod +x create_startup.sh

# Swagger
cat > create_swagger.sh <<'EOL'
#!/bin/bash
mkdir -p src/swagger
touch src/swagger.rs
echo "Swagger files created."
EOL
chmod +x create_swagger.sh

# 4. Notify
echo "All layer scripts created. Run each script to generate files per layer."
echo "Auth-Service base files, Cargo.toml, main.rs, lib.rs, migrations, and .env are created."
