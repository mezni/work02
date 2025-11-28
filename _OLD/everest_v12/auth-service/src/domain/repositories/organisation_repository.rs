use crate::domain::models::Organisation;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait OrganisationRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Organisation>, DomainError>;
    async fn save(&self, organisation: &Organisation) -> Result<Organisation, DomainError>;
    async fn update(&self, organisation: &Organisation) -> Result<Organisation, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn list_all(&self) -> Result<Vec<Organisation>, DomainError>;
}
