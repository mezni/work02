use crate::domain::entities::user::User;
use crate::domain::value_objects::UserId;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("User not found: {0}")]
    NotFound(String),
    #[error("User already exists: {0}")]
    AlreadyExists(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User, password: &str) -> Result<UserId, RepositoryError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepositoryError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<User>, RepositoryError>;
    async fn update(&self, user: &User) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError>;
    async fn assign_role(&self, user_id: &UserId, role_name: &str) -> Result<(), RepositoryError>;
    async fn get_roles(&self, user_id: &UserId) -> Result<Vec<String>, RepositoryError>;
}