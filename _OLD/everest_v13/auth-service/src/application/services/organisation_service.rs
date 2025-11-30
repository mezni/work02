use crate::application::dto::organisation_dto::{
    CreateOrganisationDto, OrganisationDto, UpdateOrganisationDto, OrganisationResponse,
    AssignUserToOrganisationDto,
};
use crate::domain::entities::organisation::Organisation;
use crate::domain::repositories::organisation_repository::OrganisationRepository;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::value_objects::{OrganisationId, UserId};
use std::sync::Arc;
use tracing::{info, error};

#[derive(Debug, thiserror::Error)]
pub enum OrganisationError {
    #[error("Organisation not found")]
    NotFound,
    #[error("Organisation name already exists")]
    NameExists,
    #[error("Invalid organisation data: {0}")]
    InvalidData(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct OrganisationService {
    organisation_repository: Arc<dyn OrganisationRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl OrganisationService {
    pub fn new(
        organisation_repository: Arc<dyn OrganisationRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            organisation_repository,
            user_repository,
        }
    }

    pub async fn create_organisation(
        &self,
        dto: CreateOrganisationDto,
    ) -> Result<OrganisationDto, OrganisationError> {
        info!("Creating organisation: {}", dto.name);

        let organisation = Organisation::new(dto.name, dto.description);
        
        let organisation_id = self.organisation_repository.create(&organisation)
            .await
            .map_err(|e| OrganisationError::DatabaseError(e.to_string()))?;

        let organisation = organisation.with_id(organisation_id);
        
        Ok(self.to_dto(&organisation))
    }

    pub async fn get_organisation(&self, id: i32) -> Result<OrganisationResponse, OrganisationError> {
        let organisation_id = OrganisationId::new(id);
        let organisation = self.organisation_repository.find_by_id(&organisation_id)
            .await
            .map_err(|e| OrganisationError::DatabaseError(e.to_string()))?
            .ok_or(OrganisationError::NotFound)?;

        // Get user count for this organisation
        let user_count = self.get_organisation_user_count(id).await?;

        Ok(OrganisationResponse {
            organisation: self.to_dto(&organisation),
            user_count,
        })
    }

    pub async fn list_organisations(&self) -> Result<Vec<OrganisationDto>, OrganisationError> {
        let organisations = self.organisation_repository.find_live_organisations()
            .await
            .map_err(|e| OrganisationError::DatabaseError(e.to_string()))?;

        Ok(organisations.iter().map(|o| self.to_dto(o)).collect())
    }

    pub async fn update_organisation(
        &self,
        id: i32,
        dto: UpdateOrganisationDto,
    ) -> Result<OrganisationDto, OrganisationError> {
        let organisation_id = OrganisationId::new(id);
        let mut organisation = self.organisation_repository.find_by_id(&organisation_id)
            .await
            .map_err(|e| OrganisationError::DatabaseError(e.to_string()))?
            .ok_or(OrganisationError::NotFound)?;

        organisation.name = dto.name;
        organisation.description = dto.description;

        self.organisation_repository.update(&organisation)
            .await
            .map_err(|e| OrganisationError::DatabaseError(e.to_string()))?;

        Ok(self.to_dto(&organisation))
    }

    pub async fn delete_organisation(&self, id: i32) -> Result<(), OrganisationError> {
        let organisation_id = OrganisationId::new(id);
        
        // Soft delete the organisation
        let mut organisation = self.organisation_repository.find_by_id(&organisation_id)
            .await
            .map_err(|e| OrganisationError::DatabaseError(e.to_string()))?
            .ok_or(OrganisationError::NotFound)?;

        organisation.soft_delete();
        
        self.organisation_repository.update(&organisation)
            .await
            .map_err(|e| OrganisationError::DatabaseError(e.to_string()))?;

        info!("Organisation soft deleted: {}", id);
        Ok(())
    }

    pub async fn assign_user_to_organisation(
        &self,
        dto: AssignUserToOrganisationDto,
    ) -> Result<(), OrganisationError> {
        let organisation_id = OrganisationId::new(dto.organisation_id);
        
        // Verify organisation exists and is live
        let organisation = self.organisation_repository.find_by_id(&organisation_id)
            .await
            .map_err(|e| OrganisationError::DatabaseError(e.to_string()))?
            .ok_or(OrganisationError::NotFound)?;

        if !organisation.is_live {
            return Err(OrganisationError::NotFound);
        }

        // Update user's organisation
        let user_id = UserId::new(dto.user_id);
        self.user_repository.assign_to_organisation(&user_id, &organisation_id)
            .await
            .map_err(|e| OrganisationError::DatabaseError(e.to_string()))?;

        info!("User {} assigned to organisation {}", user_id, organisation_id);
        Ok(())
    }

    async fn get_organisation_user_count(&self, _organisation_id: i32) -> Result<i32, OrganisationError> {
        // This would need to be implemented in the UserRepository
        // For now, return a placeholder
        Ok(0)
    }

    fn to_dto(&self, organisation: &Organisation) -> OrganisationDto {
        OrganisationDto {
            id: organisation.id.as_ref().map(|id| id.as_i32()).unwrap_or(0),
            name: organisation.name.clone(),
            description: organisation.description.clone(),
            is_live: organisation.is_live,
        }
    }
}