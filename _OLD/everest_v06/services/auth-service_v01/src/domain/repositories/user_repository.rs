use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::Email;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), DomainError>;

    async fn update(&self, user: &User) -> Result<(), DomainError>;

    async fn delete(&self, user_id: &Uuid) -> Result<(), DomainError>;

    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, DomainError>;

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError>;

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;

    async fn find_by_company(&self, company_id: &Uuid) -> Result<Vec<User>, DomainError>;

    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError>;

    async fn exists_by_username(&self, username: &str) -> Result<bool, DomainError>;
}
