#!/bin/bash

# Create deployment directories
mkdir -p deployment/docker/scripts

# Create auth_service directories
mkdir -p services/auth_service/src/{domain,application,infrastructure,interfaces/http,interfaces/events,tests/{unit,integration}}

# Create auth_service files
touch services/auth_service/{Cargo.toml,.env,Dockerfile}
touch services/auth_service/src/{main.rs,lib.rs,config.rs}

# Domain files
touch services/auth_service/src/domain/{mod.rs,models.rs,value_objects.rs,events.rs,enums.rs}

# Application files
touch services/auth_service/src/application/{mod.rs,commands.rs,queries.rs,services.rs}

# Infrastructure files
touch services/auth_service/src/infrastructure/{mod.rs,keycloak.rs,repository.rs,postgres.rs}

# Interfaces files
touch services/auth_service/src/interfaces/{mod.rs}
touch services/auth_service/src/interfaces/http/{mod.rs,handlers.rs,routes.rs,swagger.rs}
touch services/auth_service/src/interfaces/events/{mod.rs,event_handlers.rs}

# Tests files
touch services/auth_service/src/tests/{mod.rs}
touch services/auth_service/src/tests/unit/mod.rs
touch services/auth_service/src/tests/integration/mod.rs

# Deployment scripts
touch deployment/docker/scripts/keycloak_setup.py

# README
touch README.md

echo "Project structure with mod.rs files created successfully!"
