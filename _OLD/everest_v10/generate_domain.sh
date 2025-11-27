#!/bin/bash

set -e

echo "Generating domain layer..."

cd auth-service

# Create domain directories
mkdir -p src/domain #/{entities,value_objects,enums,repositories}

# Domain mod.rs
cat > src/domain/mod.rs << 'EOF'
pub mod entities;
pub mod value_objects;
pub mod enums;
pub mod repositories;
pub mod errors;

// Re-exports
pub use entities::{User, Company, AuditLog};
pub use value_objects::{Email, Password};
pub use enums::{UserRole, AuditAction};
pub use errors::DomainError;
EOF

# Domain errors
cat > src/domain/errors.rs << 'EOF'
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Company not found")]
    CompanyNotFound,
    
    #[error("Invalid user role: {0}")]
    InvalidUserRole(String),
    
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),
    
    #[error("Company already exists: {0}")]
    CompanyAlreadyExists(String),
    
    #[error("Invalid company assignment")]
    InvalidCompanyAssignment,
    
    #[error("Unauthorized operation")]
    UnauthorizedOperation,
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl DomainError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidEmail(_) => "DOMAIN_INVALID_EMAIL",
            Self::InvalidPassword(_) => "DOMAIN_INVALID_PASSWORD",
            Self::UserNotFound => "DOMAIN_USER_NOT_FOUND",
            Self::CompanyNotFound => "DOMAIN_COMPANY_NOT_FOUND",
            Self::InvalidUserRole(_) => "DOMAIN_INVALID_USER_ROLE",
            Self::UserAlreadyExists(_) => "DOMAIN_USER_ALREADY_EXISTS",
            Self::CompanyAlreadyExists(_) => "DOMAIN_COMPANY_ALREADY_EXISTS",
            Self::InvalidCompanyAssignment => "DOMAIN_INVALID_COMPANY_ASSIGNMENT",
            Self::UnauthorizedOperation => "DOMAIN_UNAUTHORIZED_OPERATION",
            Self::ValidationError(_) => "DOMAIN_VALIDATION_ERROR",
        }
    }
}
EOF

cat > src/domain/enums.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum UserRole {
    Admin,
    Partner,
    Operator,
    User,
    Guest,
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "partner" => Ok(UserRole::Partner),
            "operator" => Ok(UserRole::Operator),
            "user" => Ok(UserRole::User),
            "guest" => Ok(UserRole::Guest),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::Partner => write!(f, "Partner"),
            UserRole::Operator => write!(f, "Operator"),
            UserRole::User => write!(f, "User"),
            UserRole::Guest => write!(f, "Guest"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum AuditAction {
    UserCreated,
    UserUpdated,
    UserDeleted,
    CompanyCreated,
    CompanyUpdated,
    CompanyDeleted,
    Login,
    Logout,
    PasswordReset,
}

impl std::str::FromStr for AuditAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UserCreated" => Ok(AuditAction::UserCreated),
            "UserUpdated" => Ok(AuditAction::UserUpdated),
            "UserDeleted" => Ok(AuditAction::UserDeleted),
            "CompanyCreated" => Ok(AuditAction::CompanyCreated),
            "CompanyUpdated" => Ok(AuditAction::CompanyUpdated),
            "CompanyDeleted" => Ok(AuditAction::CompanyDeleted),
            "Login" => Ok(AuditAction::Login),
            "Logout" => Ok(AuditAction::Logout),
            "PasswordReset" => Ok(AuditAction::PasswordReset),
            _ => Err(format!("Invalid audit action: {}", s)),
        }
    }
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::UserCreated => write!(f, "UserCreated"),
            AuditAction::UserUpdated => write!(f, "UserUpdated"),
            AuditAction::UserDeleted => write!(f, "UserDeleted"),
            AuditAction::CompanyCreated => write!(f, "CompanyCreated"),
            AuditAction::CompanyUpdated => write!(f, "CompanyUpdated"),
            AuditAction::CompanyDeleted => write!(f, "CompanyDeleted"),
            AuditAction::Login => write!(f, "Login"),
            AuditAction::Logout => write!(f, "Logout"),
            AuditAction::PasswordReset => write!(f, "PasswordReset"),
        }
    }
}
EOF

# Value Objects
cat > src/domain/value_objects.rs << 'EOF'
use validator::Validate;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Self, crate::domain::errors::DomainError> {
        let email = Email { value };
        email.validate()
            .map_err(|e| crate::domain::errors::DomainError::InvalidEmail(e.to_string()))?;
        Ok(email)
    }
    
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn new(value: String) -> Result<Self, crate::domain::errors::DomainError> {
        if value.len() < 8 {
            return Err(crate::domain::errors::DomainError::InvalidPassword(
                "Password must be at least 8 characters long".to_string(),
            ));
        }
        
        Ok(Password { value })
    }
    
    pub fn value(&self) -> &str {
        &self.value
    }
    
    pub fn into_inner(self) -> String {
        self.value
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "***") // Don't expose password in logs
    }
}
EOF

# Entities
cat > src/domain/entities.rs << 'EOF'
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::domain::enums::{UserRole, AuditAction};
use crate::domain::errors::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub company_id: Option<Uuid>,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        keycloak_id: String,
        username: String,
        email: String,
        role: UserRole,
        company_id: Option<Uuid>,
    ) -> Result<Self, DomainError> {
        let now = Utc::now();
        let user = User {
            id: Uuid::new_v4(),
            keycloak_id,
            username,
            email,
            role,
            company_id,
            email_verified: false,
            created_at: now,
            updated_at: now,
        };
        
        user.validate()?;
        Ok(user)
    }
    
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
    
    pub fn is_partner(&self) -> bool {
        matches!(self.role, UserRole::Partner)
    }
    
    pub fn is_operator(&self) -> bool {
        matches!(self.role, UserRole::Operator)
    }
    
    pub fn is_regular_user(&self) -> bool {
        matches!(self.role, UserRole::User)
    }
    
    pub fn is_guest(&self) -> bool {
        matches!(self.role, UserRole::Guest)
    }
    
    pub fn can_manage_company(&self, company_id: Uuid) -> bool {
        if self.is_admin() {
            return true;
        }
        
        if self.is_partner() || self.is_operator() {
            return self.company_id == Some(company_id);
        }
        
        false
    }
    
    pub fn can_manage_user(&self, target_user: &User) -> bool {
        if self.is_admin() {
            return true;
        }
        
        if (self.is_partner() || self.is_operator()) && self.company_id.is_some() {
            return self.company_id == target_user.company_id;
        }
        
        self.id == target_user.id
    }
}

impl Validate for User {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        if self.username.is_empty() {
            errors.add("username", validator::ValidationError::new("required"));
        }
        
        if self.email.is_empty() || !validator::ValidateEmail::validate_email(&self.email) {
            errors.add("email", validator::ValidationError::new("invalid"));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl From<ValidationErrors> for DomainError {
    fn from(err: ValidationErrors) -> Self {
        DomainError::ValidationError(err.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Company {
    pub fn new(name: String, description: Option<String>, created_by: Uuid) -> Self {
        let now = Utc::now();
        Company {
            id: Uuid::new_v4(),
            name,
            description,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Validate for Company {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        if self.name.is_empty() {
            errors.add("name", validator::ValidationError::new("required"));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl AuditLog {
    pub fn new(
        user_id: Option<Uuid>,
        action: AuditAction,
        resource_type: String,
        resource_id: Option<String>,
        details: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        AuditLog {
            id: Uuid::new_v4(),
            user_id,
            action,
            resource_type,
            resource_id,
            details,
            ip_address,
            user_agent,
            created_at: Utc::now(),
        }
    }
}
EOF

# Repository traits
cat > src/domain/repositories.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{User, Company, AuditLog};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    async fn update(&self, user: &User) -> Result<User, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn list_by_company(&self, company_id: Uuid) -> Result<Vec<User>, DomainError>;
    async fn list_all(&self) -> Result<Vec<User>, DomainError>;
}

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn create(&self, company: &Company) -> Result<Company, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Company>, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Company>, DomainError>;
    async fn update(&self, company: &Company) -> Result<Company, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn list_all(&self) -> Result<Vec<Company>, DomainError>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Company>, DomainError>;
}

#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn create(&self, audit_log: &AuditLog) -> Result<(), DomainError>;
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<AuditLog>, DomainError>;
    async fn find_by_company(&self, company_id: Uuid) -> Result<Vec<AuditLog>, DomainError>;
    async fn list_recent(&self, limit: u32) -> Result<Vec<AuditLog>, DomainError>;
}
EOF

# Create domain test directory and tests
mkdir -p tests/unit/domain

cat > tests/unit/domain/mod.rs << 'EOF'
pub mod domain_tests;
EOF

cat > tests/unit/domain/domain_tests.rs << 'EOF'
use auth_service::domain::entities::User;
use auth_service::domain::enums::UserRole;
use auth_service::domain::value_objects::{Email, Password};
use uuid::Uuid;

#[test]
fn test_email_validation() {
    // Valid email
    let email = Email::new("test@example.com".to_string());
    assert!(email.is_ok());
    
    // Invalid email
    let email = Email::new("invalid-email".to_string());
    assert!(email.is_err());
}

#[test]
fn test_password_validation() {
    // Valid password
    let password = Password::new("password123".to_string());
    assert!(password.is_ok());
    
    // Too short password
    let password = Password::new("short".to_string());
    assert!(password.is_err());
}

#[test]
fn test_user_creation() {
    let user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        "test@example.com".to_string(),
        UserRole::User,
        None,
    );
    
    assert!(user.is_ok());
    let user = user.unwrap();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    assert!(matches!(user.role, UserRole::User));
    assert!(user.company_id.is_none());
}

#[test]
fn test_user_permissions() {
    let admin_user = User::new(
        "keycloak-admin".to_string(),
        "admin".to_string(),
        "admin@example.com".to_string(),
        UserRole::Admin,
        None,
    ).unwrap();
    
    let partner_user = User::new(
        "keycloak-partner".to_string(),
        "partner".to_string(),
        "partner@example.com".to_string(),
        UserRole::Partner,
        Some(Uuid::new_v4()),
    ).unwrap();
    
    let regular_user = User::new(
        "keycloak-user".to_string(),
        "user".to_string(),
        "user@example.com".to_string(),
        UserRole::User,
        None,
    ).unwrap();
    
    assert!(admin_user.is_admin());
    assert!(partner_user.is_partner());
    assert!(regular_user.is_regular_user());
    
    let test_company_id = Uuid::new_v4();
    assert!(admin_user.can_manage_company(test_company_id));
    assert!(partner_user.can_manage_company(partner_user.company_id.unwrap()));
    assert!(!regular_user.can_manage_company(test_company_id));
}

#[test]
fn test_user_validation() {
    // Test invalid user creation
    let invalid_user = User::new(
        "".to_string(),
        "".to_string(),
        "invalid-email".to_string(),
        UserRole::User,
        None,
    );
    
    assert!(invalid_user.is_err());
}

#[test]
fn test_enum_conversions() {
    // Test UserRole from string
    assert_eq!("admin".parse::<UserRole>().unwrap(), UserRole::Admin);
    assert_eq!("partner".parse::<UserRole>().unwrap(), UserRole::Partner);
    assert_eq!("operator".parse::<UserRole>().unwrap(), UserRole::Operator);
    assert_eq!("user".parse::<UserRole>().unwrap(), UserRole::User);
    assert_eq!("guest".parse::<UserRole>().unwrap(), UserRole::Guest);
    
    // Test invalid role
    assert!("invalid".parse::<UserRole>().is_err());
    
    // Test display
    assert_eq!(UserRole::Admin.to_string(), "Admin");
    assert_eq!(UserRole::Partner.to_string(), "Partner");
}
EOF

echo "Domain layer generated successfully!"
echo "You can test it with: cargo test unit::domain"