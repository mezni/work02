use crate::domain::models::Station;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait StationRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Station>, DomainError>;
    async fn get_by_organisation(&self, organisation_id: Uuid) -> Result<Vec<Station>, DomainError>;
    async fn save(&self, station: &Station) -> Result<Station, DomainError>;
    async fn update(&self, station: &Station) -> Result<Station, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
