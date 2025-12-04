use crate::domain::user::User;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
}
