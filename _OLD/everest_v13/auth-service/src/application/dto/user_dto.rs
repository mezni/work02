use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 50))]
    pub first_name: String,

    #[validate(length(min = 1, max = 50))]
    pub last_name: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AssignRoleDto {
    #[validate(length(min = 1))]
    pub role_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserRolesDto {
    pub user_id: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SuccessResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserResponse {
    pub id: String,
    pub message: String,
}
