#!/bin/bash
# create_domain_layer_with_stubs.sh
# Creates domain layer folders and files with starter Rust code
SERVICE_DIR="auth-service"

echo "Creating Domain layer structure with starter code..."

cd $SERVICE_DIR

# Create directories
mkdir -p src/domain/models
mkdir -p src/domain/value_objects
mkdir -p src/domain/repositories
mkdir -p src/domain/services

# Create mod.rs
cat > src/domain/mod.rs <<EOL
pub mod models;
pub mod value_objects;
pub mod repositories;
pub mod services;
pub mod errors;
EOL

# Create errors.rs
cat > src/domain/errors.rs <<EOL
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    #[error("Invalid username: {0}")]
    InvalidUsername(String),
    #[error("User not found")]
    UserNotFound,
    #[error("Organisation not found")]
    OrganisationNotFound,
    #[error("Station not found")]
    StationNotFound,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Keycloak error: {0}")]
    KeycloakError(String),
}
EOL

# Create models
cat > src/domain/models/user.rs <<EOL
use crate::domain::value_objects::{Email, Username};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: Username,
    pub email: Email,
    pub role: String,
    pub organisation_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
}
EOL

cat > src/domain/models/organisation.rs <<EOL
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Organisation {
    pub id: Uuid,
    pub name: String,
}
EOL

cat > src/domain/models/station.rs <<EOL
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Station {
    pub id: Uuid,
    pub name: String,
    pub organisation_id: Uuid,
}
EOL

# Create value objects
cat > src/domain/value_objects/email.rs <<EOL
use regex::Regex;
use crate::domain::errors::DomainError;

#[derive(Debug, Clone)]
pub struct Email(pub String);

impl Email {
    pub fn new(email: &str) -> Result<Self, DomainError> {
        let email_regex = Regex::new(r"^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$").unwrap();
        if email_regex.is_match(email) {
            Ok(Self(email.to_string()))
        } else {
            Err(DomainError::InvalidEmail(email.to_string()))
        }
    }
}
EOL

cat > src/domain/value_objects/username.rs <<EOL
use crate::domain::errors::DomainError;

#[derive(Debug, Clone)]
pub struct Username(pub String);

impl Username {
    pub fn new(username: &str) -> Result<Self, DomainError> {
        if username.len() >= 3 {
            Ok(Self(username.to_string()))
        } else {
            Err(DomainError::InvalidUsername(username.to_string()))
        }
    }
}
EOL

# Create repository traits
cat > src/domain/repositories/user_repository.rs <<EOL
use crate::domain::models::User;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<User, DomainError>;
    async fn get_by_username(&self, username: &str) -> Result<User, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
}
EOL

# Create domain services
cat > src/domain/services/user_domain_service.rs <<EOL
use crate::domain::models::User;
use crate::domain::errors::DomainError;

pub struct UserDomainService;

impl UserDomainService {
    pub fn validate_user_role(user: &User, allowed_roles: &[&str]) -> Result<(), DomainError> {
        if allowed_roles.contains(&user.role.as_str()) {
            Ok(())
        } else {
            Err(DomainError::PermissionDenied)
        }
    }
}
EOL

echo "✅ Domain layer structure and starter code created successfully!"
echo "Directories and files:"
echo "- src/domain/mod.rs"
echo "- src/domain/errors.rs"
echo "- src/domain/models/user.rs"
echo "- src/domain/models/organisation.rs"
echo "- src/domain/models/station.rs"
echo "- src/domain/value_objects/email.rs"
echo "- src/domain/value_objects/username.rs"
echo "- src/domain/repositories/user_repository.rs"
echo "- src/domain/services/user_domain_service.rs"
