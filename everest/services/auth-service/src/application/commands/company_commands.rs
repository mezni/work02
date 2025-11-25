use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCompanyCommand {
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Acme Corporation")]
    pub name: String,

    #[validate(length(max = 1000))]
    #[schema(example = "A leading technology company specializing in innovative solutions")]
    pub description: Option<String>,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCompanyCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Uuid,

    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Acme Corporation Ltd.")]
    pub name: Option<String>,

    #[validate(length(max = 1000))]
    #[schema(example = "A global leader in innovative technology solutions")]
    pub description: Option<String>,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub updated_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCompanyCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub deleted_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransferCompanyOwnershipCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub new_owner_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub transferred_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AddCompanyUserCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Uuid,
    
    #[validate(email)]
    #[schema(example = "new.user@example.com")]
    pub user_email: String,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub added_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RemoveCompanyUserCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub user_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub removed_by: Uuid,
}