#!/bin/bash

# Root directory
PROJECT_NAME="auth_service"
mkdir -p $PROJECT_NAME/src

cd $PROJECT_NAME || exit

# Cargo.toml placeholder
touch Cargo.toml
touch .env

# SRC directories
mkdir -p src/{domain/{value_objects,events,enums},application/{commands,queries,handlers},interfaces/{http,keycloak},infrastructure/{repositories,middleware},utils}

# Main files
touch src/main.rs src/lib.rs src/config.rs

# Domain files
touch src/domain/models.rs
touch src/domain/value_objects/{mod.rs,email.rs,password.rs,company_name.rs}
touch src/domain/enums/{mod.rs,role.rs}
touch src/domain/events/{mod.rs,user_events.rs,company_events.rs}

# Application files
touch src/application/mod.rs
touch src/application/dtos.rs
touch src/application/commands/{mod.rs,create_user.rs,assign_role.rs,assign_company.rs}
touch src/application/queries/{mod.rs,get_user.rs,list_users.rs,get_company_users.rs}
touch src/application/handlers/{mod.rs,user_handlers.rs,company_handlers.rs}

# Interfaces / HTTP
touch src/interfaces/mod.rs
touch src/interfaces/http/{mod.rs,handlers.rs,routes.rs,swagger.rs}
touch src/interfaces/keycloak/{mod.rs,client.rs}

# Infrastructure
touch src/infrastructure/mod.rs
touch src/infrastructure/repositories/{mod.rs,user_repo.rs,company_repo.rs}
touch src/infrastructure/middleware/{mod.rs,auth.rs,role_access.rs,service_access.rs}

# Utils
touch src/utils/{mod.rs,jwt.rs}

# Scripts, migrations, docs, tests
mkdir -p scripts migrations docs tests/{unit,integration}
touch scripts/keycloak_setup.py

echo "✅ Project structure for $PROJECT_NAME created successfully!"
