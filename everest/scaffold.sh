#!/bin/bash

set -e

cargo new auth-service 

cd auth-service 
cargo add thiserror 

# Layers
layers=("domain" "application" "infrastructure" "interfaces")

# Create directories and files for each layer
for layer in "${layers[@]}"; do
    mkdir -p "src/$layer"
    touch "src/$layer/errors.rs"
    touch "src/$layer/mod.rs"

    # Populate mod.rs
    echo "pub mod errors;" > "src/$layer/mod.rs"

    # Populate errors.rs with minimal thiserror example
    case $layer in
        "domain")
            cat > "src/$layer/errors.rs" <<EOL
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),
}
EOL
            ;;
        "application")
            cat > "src/$layer/errors.rs" <<EOL
use thiserror::Error;
use crate::domain::errors::DomainError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error(transparent)]
    DomainError(#[from] DomainError),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}
EOL
            ;;
        "infrastructure")
            cat > "src/$layer/errors.rs" <<EOL
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Network error: {0}")]
    NetworkError(String),
}
EOL
            ;;
        "interfaces")
            cat > "src/$layer/errors.rs" <<EOL
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterfacesError {
    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),
}
EOL
            ;;
    esac
done

# Create top-level AppError
touch src/error.rs
cat > src/error.rs <<EOL
use thiserror::Error;
use crate::domain::errors::DomainError;
use crate::application::errors::ApplicationError;
use crate::infrastructure::errors::InfrastructureError;
use crate::interfaces::errors::InterfacesError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Application(#[from] ApplicationError),

    #[error(transparent)]
    Infrastructure(#[from] InfrastructureError),

    #[error(transparent)]
    Interfaces(#[from] InterfacesError),
}
EOL

echo "DDD layers created with pre-populated errors.rs and top-level AppError."
