#!/bin/bash
# create_domain_layer_fixed.sh
# Creates domain layer folders and files with FIXED and enhanced starter Rust code

SERVICE_DIR="auth-service"

echo "Applying fixes and enhancements to the Domain layer structure..."

# Assuming we are already inside the $SERVICE_DIR from the previous context

# --- 1. Create/Fix directories (ensuring all are present)
mkdir -p src/domain/models
mkdir -p src/domain/value_objects
mkdir -p src/domain/repositories
mkdir -p src/domain/services

# --- 2. Create mod.rs (Correct)
cat > src/domain/mod.rs <<EOL
pub mod models;
pub mod value_objects;
pub mod repositories;
pub mod services;
pub mod errors;
EOL

# --- 3. Fix errors.rs
# FIX: Use parameterized errors (Uuid, String) as suggested by best practices.
# FIX: Remove KeycloakError and PermissionDenied to keep the layer pure.
#      PermissionDenied should be checked by Domain Services using Roles, 
#      and Keycloak is an Infrastructure concern.
cat > src/domain/errors.rs <<EOL
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    #[error("Invalid username: {0}")]
    InvalidUsername(String),
    #[error("User with ID {0} not found.")]
    UserNotFound(Uuid),
    #[error("User with username '{0}' already exists.")]
    UsernameAlreadyExists(String),
    #[error("Organisation with ID {0} not found.")]
    OrganisationNotFound(Uuid),
    #[error("Station with ID {0} not found.")]
    StationNotFound(Uuid),
    #[error("A critical internal domain consistency error occurred: {0}")]
    InternalError(String),
}
EOL

# --- 4. Create Role Value Object (MISSING in original prompt/stubs)
cat > src/domain/value_objects/role.rs <<EOL
use serde::{Serialize, Deserialize};
use std::fmt::{self, Display};
use std::str::FromStr;
use crate::domain::errors::DomainError;

// FIX: Implement the Role as an Enum with validation/conversion traits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Partner,
    Operator,
    RegisteredUser,
    Public,
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Role::Admin => "admin",
            Role::Partner => "partner",
            Role::Operator => "operator",
            Role::RegisteredUser => "registered_user",
            Role::Public => "public",
        })
    }
}

impl FromStr for Role {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "partner" => Ok(Role::Partner),
            "operator" => Ok(Role::Operator),
            "registered_user" => Ok(Role::RegisteredUser),
            "public" => Ok(Role::Public),
            _ => Err(DomainError::InternalError(format!("Invalid role string: {}", s))),
        }
    }
}
EOL
touch src/domain/value_objects/mod.rs

# --- 5. Fix Value Objects to use the `From` trait and `Deref` (for convenience)
cat > src/domain/value_objects/email.rs <<EOL
use regex::Regex;
use crate::domain::errors::DomainError;
use std::ops::Deref;
use serde::{Serialize, Deserialize}; // Added for external serialization

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self, DomainError> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if email_regex.is_match(&email) {
            Ok(Self(email))
        } else {
            Err(DomainError::InvalidEmail(email))
        }
    }
}

// Allow treating Email as a String reference for convenience
impl Deref for Email {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
EOL

cat > src/domain/value_objects/username.rs <<EOL
use crate::domain::errors::DomainError;
use std::ops::Deref;
use serde::{Serialize, Deserialize}; // Added for external serialization

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn parse(username: String) -> Result<Self, DomainError> {
        // Simple length check, but could include alphanumeric/character set rules
        if username.len() >= 3 && username.len() <= 50 && username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Ok(Self(username))
        } else {
            Err(DomainError::InvalidUsername(username))
        }
    }
}

impl Deref for Username {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
EOL

# --- 6. Fix User Entity to use the Role Value Object
cat > src/domain/models/user.rs <<EOL
use crate::domain::value_objects::{Email, Username, Role};
use uuid::Uuid;
use serde::{Serialize, Deserialize}; // Added for external serialization

// FIX: Use the Role enum/Value Object instead of a raw String for 'role'
// FIX: Add Keycloak ID as a core property of the User Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub keycloak_id: Uuid, // Infrastructure detail promoted to Entity ID (Aggregate Root)
    pub username: Username,
    pub email: Email,
    pub role: Role,
    pub organisation_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
}

impl User {
    // Factory method for creating a new user (Aggregate Root creation)
    pub fn register(
        keycloak_id: Uuid,
        username: Username,
        email: Email,
    ) -> Self {
        User {
            id: Uuid::new_v4(),
            keycloak_id,
            username,
            email,
            role: Role::RegisteredUser,
            organisation_id: None,
            station_id: None,
        }
    }
    
    // Example domain behavior
    pub fn promote_to_partner(&mut self, organisation_id: Uuid) {
        self.role = Role::Partner;
        self.organisation_id = Some(organisation_id);
        self.station_id = None;
    }
}
EOL

# --- 7. Create missing repository traits (Organisation and Station)
cat > src/domain/repositories/organisation_repository.rs <<EOL
use crate::domain::models::Organisation;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait OrganisationRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Organisation, DomainError>;
    async fn get_name_by_id(&self, id: Uuid) -> Result<String, DomainError>;
}
EOL

cat > src/domain/repositories/station_repository.rs <<EOL
use crate::domain::models::Station;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait StationRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Station, DomainError>;
    async fn get_name_by_id(&self, id: Uuid) -> Result<String, DomainError>;
}
EOL

# FIX: Update User Repository trait for consistency and better querying
cat > src/domain/repositories/user_repository.rs <<EOL
use crate::domain::models::User;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{Username, Email};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
// FIX: Add Send + Sync bounds for Actix/multithreading
pub trait UserRepository: Send + Sync {
    // FIX: Return Option<User> to clearly denote 'not found' instead of a blanket error
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn get_by_keycloak_id(&self, keycloak_id: Uuid) -> Result<Option<User>, DomainError>;
    async fn get_by_username(&self, username: &Username) -> Result<Option<User>, DomainError>;
    async fn get_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: User) -> Result<User, DomainError>; // Save returns the persisted instance
}
EOL

# --- 8. Fix Domain Service for Purity
cat > src/domain/services/user_domain_service.rs <<EOL
use crate::domain::models::User;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::Role;

pub struct UserDomainService;

impl UserDomainService {
    // FIX: Takes the Role Value Object, ensuring purity (no string comparison needed)
    pub fn check_permission(user: &User, required_role: Role) -> Result<(), DomainError> {
        if user.role as i32 >= required_role as i32 {
             // NOTE: In a real system, a permission matrix is needed. 
             // This simple check stub assumes higher enum values = more privilege.
            Ok(())
        } else {
            Err(DomainError::InternalError("User does not have required permissions.".to_string()))
        }
    }
}
EOL

echo "✅ Domain layer fixed, enhanced with Role VO, Keycloak ID, and robust traits."