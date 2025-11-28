use crate::domain::models::User;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{Username, Email};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn get_by_username(&self, username: &Username) -> Result<Option<User>, DomainError>;
    async fn get_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<User, DomainError>;
    async fn update(&self, user: &User) -> Result<User, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn list_by_organisation(&self, organisation_id: Uuid) -> Result<Vec<User>, DomainError>;
    async fn list_by_station(&self, station_id: Uuid) -> Result<Vec<User>, DomainError>;
}
