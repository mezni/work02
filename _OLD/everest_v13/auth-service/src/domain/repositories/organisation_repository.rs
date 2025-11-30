use async_trait::async_trait;
use crate::domain::entities::organisation::Organisation;
use crate::domain::value_objects::OrganisationId;
use super::RepositoryError;

#[async_trait]
pub trait OrganisationRepository: Send + Sync {
    async fn create(&self, organisation: &Organisation) -> Result<OrganisationId, RepositoryError>;
    async fn find_by_id(&self, id: &OrganisationId) -> Result<Option<Organisation>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Organisation>, RepositoryError>;
    async fn update(&self, organisation: &Organisation) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &OrganisationId) -> Result<(), RepositoryError>;
    async fn find_live_organisations(&self) -> Result<Vec<Organisation>, RepositoryError>;
}