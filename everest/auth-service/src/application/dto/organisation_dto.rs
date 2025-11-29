use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateOrganisationDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(max = 500))]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateOrganisationDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(max = 500))]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrganisationDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub is_live: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrganisationResponse {
    pub organisation: OrganisationDto,
    pub user_count: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AssignUserToOrganisationDto {
    pub user_id: String,
    pub organisation_id: i32,
}

// Re-export common types
pub use super::user_dto::{ErrorResponse, SuccessResponse};