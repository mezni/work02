#!/bin/bash

set -e

echo "Generating complete domain layer..."

cd auth-service

# Create complete domain directories
mkdir -p src/domain/{entities,value_objects,aggregates,repositories,services,events}

# Domain mod.rs
cat > src/domain/mod.rs << 'EOF'
pub mod entities;
pub mod value_objects;
pub mod aggregates;
pub mod repositories;
pub mod services;
pub mod events;
pub mod errors;
pub mod enums;

// Re-exports
pub use entities::{User, Company, AuditLog};
pub use value_objects::{Email, Password};
pub use aggregates::{UserAggregate, CompanyAggregate};
pub use services::{UserService, CompanyService};
pub use events::{DomainEvent, UserEvent, CompanyEvent};
pub use errors::DomainError;
pub use enums::{UserRole, AuditAction};
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
    
    #[error("Domain rule violation: {0}")]
    DomainRuleViolation(String),
    
    #[error("Insufficient permissions: {0}")]
    InsufficientPermissions(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
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
            Self::DomainRuleViolation(_) => "DOMAIN_RULE_VIOLATION",
            Self::InsufficientPermissions(_) => "DOMAIN_INSUFFICIENT_PERMISSIONS",
            Self::InvalidOperation(_) => "DOMAIN_INVALID_OPERATION",
        }
    }
}
EOF

# Enums
cat > src/domain/enums.rs << 'EOF'
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

# Value Objects - Email
cat > src/domain/value_objects/email.rs << 'EOF'
use serde::{Deserialize, Serialize};
use std::fmt;
use validator::Validate;

use crate::domain::errors::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Self, DomainError> {
        let email = Email { value };
        email.validate()
            .map_err(|e| DomainError::InvalidEmail(e.to_string()))?;
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
EOF

# Value Objects - Password
cat > src/domain/value_objects/password.rs << 'EOF'
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::domain::errors::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.len() < 8 {
            return Err(DomainError::InvalidPassword(
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

# Value Objects mod.rs
cat > src/domain/value_objects/mod.rs << 'EOF'
mod email;
mod password;

pub use email::Email;
pub use password::Password;
EOF

# Entities - User
cat > src/domain/entities/user.rs << 'EOF'
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::domain::enums::UserRole;
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
EOF

# Entities - Company
cat > src/domain/entities/company.rs << 'EOF'
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

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
EOF

# Entities - AuditLog
cat > src/domain/entities/audit_log.rs << 'EOF'
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::AuditAction;

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

# Entities mod.rs
cat > src/domain/entities/mod.rs << 'EOF'
mod user;
mod company;
mod audit_log;

pub use user::User;
pub use company::Company;
pub use audit_log::AuditLog;
EOF

# Aggregates - UserAggregate
cat > src/domain/aggregates/user_aggregate.rs << 'EOF'
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::enums::UserRole;
use crate::domain::errors::DomainError;
use crate::domain::events::{DomainEvent, UserEvent};

pub struct UserAggregate {
    pub user: User,
    pub events: Vec<DomainEvent>,
}

impl UserAggregate {
    pub fn new(
        keycloak_id: String,
        username: String,
        email: String,
        role: UserRole,
        company_id: Option<Uuid>,
    ) -> Result<Self, DomainError> {
        let user = User::new(keycloak_id, username, email, role, company_id)?;
        let mut aggregate = UserAggregate {
            user,
            events: Vec::new(),
        };
        
        aggregate.events.push(DomainEvent::UserCreated(UserEvent {
            user_id: aggregate.user.id,
            keycloak_id: aggregate.user.keycloak_id.clone(),
            username: aggregate.user.username.clone(),
            email: aggregate.user.email.clone(),
            role: aggregate.user.role,
            company_id: aggregate.user.company_id,
            timestamp: chrono::Utc::now(),
        }));
        
        Ok(aggregate)
    }
    
    pub fn update_email(&mut self, new_email: String) -> Result<(), DomainError> {
        let _old_email = self.user.email.clone();
        self.user.email = new_email;
        self.user.updated_at = chrono::Utc::now();
        
        self.events.push(DomainEvent::UserUpdated(UserEvent {
            user_id: self.user.id,
            keycloak_id: self.user.keycloak_id.clone(),
            username: self.user.username.clone(),
            email: self.user.email.clone(),
            role: self.user.role,
            company_id: self.user.company_id,
            timestamp: chrono::Utc::now(),
        }));
        
        Ok(())
    }
    
    pub fn update_role(&mut self, new_role: UserRole, actor: &User) -> Result<(), DomainError> {
        if !actor.is_admin() && !actor.can_manage_user(&self.user) {
            return Err(DomainError::InsufficientPermissions(
                "Only admins or managers can change user roles".to_string(),
            ));
        }
        
        self.user.role = new_role;
        self.user.updated_at = chrono::Utc::now();
        
        self.events.push(DomainEvent::UserRoleChanged(UserEvent {
            user_id: self.user.id,
            keycloak_id: self.user.keycloak_id.clone(),
            username: self.user.username.clone(),
            email: self.user.email.clone(),
            role: self.user.role,
            company_id: self.user.company_id,
            timestamp: chrono::Utc::now(),
        }));
        
        Ok(())
    }
    
    pub fn assign_to_company(&mut self, company_id: Uuid, actor: &User) -> Result<(), DomainError> {
        if !actor.can_manage_company(company_id) {
            return Err(DomainError::InsufficientPermissions(
                "User cannot assign to this company".to_string(),
            ));
        }
        
        self.user.company_id = Some(company_id);
        self.user.updated_at = chrono::Utc::now();
        
        self.events.push(DomainEvent::UserCompanyAssigned(UserEvent {
            user_id: self.user.id,
            keycloak_id: self.user.keycloak_id.clone(),
            username: self.user.username.clone(),
            email: self.user.email.clone(),
            role: self.user.role,
            company_id: self.user.company_id,
            timestamp: chrono::Utc::now(),
        }));
        
        Ok(())
    }
    
    pub fn take_events(&mut self) -> Vec<DomainEvent> {
        std::mem::take(&mut self.events)
    }
}
EOF

# Aggregates - CompanyAggregate
cat > src/domain/aggregates/company_aggregate.rs << 'EOF'
use uuid::Uuid;

use crate::domain::entities::{Company, User};
use crate::domain::errors::DomainError;
use crate::domain::events::{CompanyEvent, DomainEvent};

pub struct CompanyAggregate {
    pub company: Company,
    pub events: Vec<DomainEvent>,
}

impl CompanyAggregate {
    pub fn new(name: String, description: Option<String>, created_by: Uuid) -> Self {
        let company = Company::new(name, description, created_by);
        let mut aggregate = CompanyAggregate {
            company,
            events: Vec::new(),
        };
        
        aggregate.events.push(DomainEvent::CompanyCreated(CompanyEvent {
            company_id: aggregate.company.id,
            name: aggregate.company.name.clone(),
            created_by: aggregate.company.created_by,
            timestamp: chrono::Utc::now(),
        }));
        
        aggregate
    }
    
    pub fn update_info(&mut self, name: String, description: Option<String>, actor: &User) -> Result<(), DomainError> {
        if !actor.can_manage_company(self.company.id) {
            return Err(DomainError::InsufficientPermissions(
                "User cannot manage this company".to_string(),
            ));
        }
        
        self.company.name = name;
        self.company.description = description;
        self.company.updated_at = chrono::Utc::now();
        
        self.events.push(DomainEvent::CompanyUpdated(CompanyEvent {
            company_id: self.company.id,
            name: self.company.name.clone(),
            created_by: self.company.created_by,
            timestamp: chrono::Utc::now(),
        }));
        
        Ok(())
    }
    
    pub fn take_events(&mut self) -> Vec<DomainEvent> {
        std::mem::take(&mut self.events)
    }
}
EOF

# Aggregates mod.rs
cat > src/domain/aggregates/mod.rs << 'EOF'
mod user_aggregate;
mod company_aggregate;

pub use user_aggregate::UserAggregate;
pub use company_aggregate::CompanyAggregate;
EOF

# Repository traits - UserRepository
cat > src/domain/repositories/user_repository.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::User;
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
EOF

# Repository traits - CompanyRepository
cat > src/domain/repositories/company_repository.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Company;
use crate::domain::errors::DomainError;

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
EOF

# Repository traits - AuditLogRepository
cat > src/domain/repositories/audit_log_repository.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::AuditLog;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn create(&self, audit_log: &AuditLog) -> Result<(), DomainError>;
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<AuditLog>, DomainError>;
    async fn find_by_company(&self, company_id: Uuid) -> Result<Vec<AuditLog>, DomainError>;
    async fn list_recent(&self, limit: u32) -> Result<Vec<AuditLog>, DomainError>;
}
EOF

# Repository traits mod.rs
cat > src/domain/repositories/mod.rs << 'EOF'
mod user_repository;
mod company_repository;
mod audit_log_repository;

pub use user_repository::UserRepository;
pub use company_repository::CompanyRepository;
pub use audit_log_repository::AuditLogRepository;
EOF

# Domain Services - UserService
cat > src/domain/services/user_service.rs << 'EOF'
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::enums::UserRole;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{UserRepository, CompanyRepository};

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        UserService
    }
    
    pub async fn register_user(
        &self,
        user_repo: &impl UserRepository,
        company_repo: &impl CompanyRepository,
        keycloak_id: String,
        username: String,
        email: String,
        role: UserRole,
        company_id: Option<Uuid>,
    ) -> Result<User, DomainError> {
        // Check if user already exists
        if user_repo.find_by_keycloak_id(&keycloak_id).await?.is_some() {
            return Err(DomainError::UserAlreadyExists(
                format!("User with keycloak_id {} already exists", keycloak_id)
            ));
        }
        
        if user_repo.find_by_email(&email).await?.is_some() {
            return Err(DomainError::UserAlreadyExists(
                format!("User with email {} already exists", email)
            ));
        }
        
        // Validate company assignment
        if let Some(cid) = company_id {
            if company_repo.find_by_id(cid).await?.is_none() {
                return Err(DomainError::CompanyNotFound);
            }
        }
        
        // Create user
        let user = User::new(keycloak_id, username, email, role, company_id)?;
        user_repo.create(&user).await
    }
    
    pub async fn change_user_role(
        &self,
        user_repo: &impl UserRepository,
        actor_id: Uuid,
        target_user_id: Uuid,
        new_role: UserRole,
    ) -> Result<User, DomainError> {
        let actor = user_repo.find_by_id(actor_id).await?
            .ok_or(DomainError::UserNotFound)?;
        
        let mut target_user = user_repo.find_by_id(target_user_id).await?
            .ok_or(DomainError::UserNotFound)?;
        
        if !actor.can_manage_user(&target_user) {
            return Err(DomainError::InsufficientPermissions(
                "Cannot change role of this user".to_string()
            ));
        }
        
        target_user.role = new_role;
        user_repo.update(&target_user).await
    }
    
    pub async fn assign_user_to_company(
        &self,
        user_repo: &impl UserRepository,
        company_repo: &impl CompanyRepository,
        actor_id: Uuid,
        user_id: Uuid,
        company_id: Uuid,
    ) -> Result<User, DomainError> {
        let actor = user_repo.find_by_id(actor_id).await?
            .ok_or(DomainError::UserNotFound)?;
        
        let company = company_repo.find_by_id(company_id).await?
            .ok_or(DomainError::CompanyNotFound)?;
        
        if !actor.can_manage_company(company.id) {
            return Err(DomainError::InsufficientPermissions(
                "Cannot assign users to this company".to_string()
            ));
        }
        
        let mut user = user_repo.find_by_id(user_id).await?
            .ok_or(DomainError::UserNotFound)?;
        
        user.company_id = Some(company.id);
        user_repo.update(&user).await
    }
}
EOF

# Domain Services - CompanyService
cat > src/domain/services/company_service.rs << 'EOF'
use uuid::Uuid;

use crate::domain::entities::Company;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{CompanyRepository, UserRepository};

pub struct CompanyService;

impl CompanyService {
    pub fn new() -> Self {
        CompanyService
    }
    
    pub async fn create_company(
        &self,
        company_repo: &impl CompanyRepository,
        user_repo: &impl UserRepository,
        name: String,
        description: Option<String>,
        created_by: Uuid,
    ) -> Result<Company, DomainError> {
        // Check if company already exists
        if company_repo.find_by_name(&name).await?.is_some() {
            return Err(DomainError::CompanyAlreadyExists(
                format!("Company with name {} already exists", name)
            ));
        }
        
        // Validate creator exists
        if user_repo.find_by_id(created_by).await?.is_none() {
            return Err(DomainError::UserNotFound);
        }
        
        // Create company
        let company = Company::new(name, description, created_by);
        company_repo.create(&company).await
    }
    
    pub async fn update_company(
        &self,
        company_repo: &impl CompanyRepository,
        user_repo: &impl UserRepository,
        company_id: Uuid,
        name: String,
        description: Option<String>,
        actor_id: Uuid,
    ) -> Result<Company, DomainError> {
        let actor = user_repo.find_by_id(actor_id).await?
            .ok_or(DomainError::UserNotFound)?;
        
        let mut company = company_repo.find_by_id(company_id).await?
            .ok_or(DomainError::CompanyNotFound)?;
        
        if !actor.can_manage_company(company.id) {
            return Err(DomainError::InsufficientPermissions(
                "Cannot update this company".to_string()
            ));
        }
        
        company.name = name;
        company.description = description;
        company_repo.update(&company).await
    }
}
EOF

# Domain Services mod.rs
cat > src/domain/services/mod.rs << 'EOF'
mod user_service;
mod company_service;

pub use user_service::UserService;
pub use company_service::CompanyService;
EOF

# Domain Events
cat > src/domain/events.rs << 'EOF'
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    UserCreated(UserEvent),
    UserUpdated(UserEvent),
    UserRoleChanged(UserEvent),
    UserCompanyAssigned(UserEvent),
    UserDeleted(UserEvent),
    CompanyCreated(CompanyEvent),
    CompanyUpdated(CompanyEvent),
    CompanyDeleted(CompanyEvent),
}

impl DomainEvent {
    pub fn event_type(&self) -> &str {
        match self {
            Self::UserCreated(_) => "UserCreated",
            Self::UserUpdated(_) => "UserUpdated",
            Self::UserRoleChanged(_) => "UserRoleChanged",
            Self::UserCompanyAssigned(_) => "UserCompanyAssigned",
            Self::UserDeleted(_) => "UserDeleted",
            Self::CompanyCreated(_) => "CompanyCreated",
            Self::CompanyUpdated(_) => "CompanyUpdated",
            Self::CompanyDeleted(_) => "CompanyDeleted",
        }
    }
    
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Self::UserCreated(e) => e.timestamp,
            Self::UserUpdated(e) => e.timestamp,
            Self::UserRoleChanged(e) => e.timestamp,
            Self::UserCompanyAssigned(e) => e.timestamp,
            Self::UserDeleted(e) => e.timestamp,
            Self::CompanyCreated(e) => e.timestamp,
            Self::CompanyUpdated(e) => e.timestamp,
            Self::CompanyDeleted(e) => e.timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEvent {
    pub user_id: Uuid,
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub company_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyEvent {
    pub company_id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub timestamp: DateTime<Utc>,
}
EOF

cat > Cargo.toml << 'EOF'
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"
description = "Authentication and Authorization Microservice with Keycloak"
authors = ["M.MEZNI"]
license = "MIT"

[dependencies]
actix-web = "4.12.0"
serde = { version = "1.0.228", features = ["derive"] }
thiserror = "2.0.17"
tracing = "0.1.41"
tracing-subscriber = "0.3.20"
tokio = { version = "1.48.0", features = ["full"] }
config = "0.15.19"
serde_json = "1.0.145"
chrono = { version = "0.4.42", features = ["serde"] }
validator = { version = "0.20.0", features = ["derive"] }
uuid = { version = "1.18.1", features = ["v4", "serde"] }
async-trait = "0.1.89"

[dev-dependencies]
actix-web = "4.12.0"
tokio = { version = "1.48.0", features = ["full"] }

[[bin]]
name = "auth-service"
path = "src/main.rs"
EOF

echo "Domain layer generated successfully!"