use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{User, Company, AuditLog};
use crate::domain::enums::UserRole;
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
