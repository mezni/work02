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
