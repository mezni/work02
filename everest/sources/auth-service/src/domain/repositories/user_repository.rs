// src/domain/repositories/user_repository.rs
use super::super::entities::User;
use super::super::value_objects::{Email, UserId, Username};
use crate::domain::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;

    // Query methods
    async fn list_users(
        &self,
        page: u32,
        page_size: u32,
        active_only: bool,
    ) -> Result<Vec<User>, DomainError>;

    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError> {
        Ok(self.find_by_email(email).await?.is_some())
    }

    async fn exists_by_username(&self, username: &Username) -> Result<bool, DomainError> {
        Ok(self.find_by_username(username).await?.is_some())
    }
}
