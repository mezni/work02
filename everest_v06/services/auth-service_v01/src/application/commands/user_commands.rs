use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use crate::domain::enums::UserRole;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserCommand {
    #[validate(length(min = 3, max = 50))]
    #[schema(example = "jane_smith")]
    pub username: String,

    #[validate(email)]
    #[schema(example = "jane@example.com")]
    pub email: String,

    #[schema(example = "User")]
    pub role: UserRole,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,

    #[validate(length(min = 3, max = 50))]
    #[schema(example = "jane_doe")]
    pub username: Option<String>,

    #[validate(email)]
    #[schema(example = "jane.doe@example.com")]
    pub email: Option<String>,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub updated_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangeUserRoleCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    
    #[schema(example = "Admin")]
    pub new_role: UserRole,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub changed_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AssignUserToCompanyCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub assigned_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RemoveUserFromCompanyCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub removed_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,

    #[validate(length(min = 3, max = 50))]
    #[schema(example = "john_doe_updated")]
    pub username: Option<String>,

    #[validate(email)]
    #[schema(example = "john.doe.updated@example.com")]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ChangeUserPasswordCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,

    #[validate(length(min = 8))]
    #[schema(example = "newSecurePassword123!")]
    pub new_password: String,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub changed_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteUserCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub deleted_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyEmailCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    
    #[schema(example = "email_verification_token_abc123")]
    pub verification_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResendVerificationEmailCommand {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
}