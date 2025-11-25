use async_trait::async_trait;
use crate::domain::models::{User, NewUser};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    async fn create(&self, new_user: NewUser) -> Result<User, DomainError>;
    async fn update(&self, user: User) -> Result<User, DomainError>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError>;
}