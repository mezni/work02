use async_trait::async_trait;
use uuid::Uuid;
use super::entities::User;
use super::errors::DomainError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<User, DomainError>;
    async fn update(&self, user: &User) -> Result<User, DomainError>;
    async fn list_by_organisation(&self, org_name: &str) -> Result<Vec<User>, DomainError>;
    async fn list_all(&self) -> Result<Vec<User>, DomainError>;
}
