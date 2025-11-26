use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use crate::domain::enums::UserRole;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    #[schema(example = "john_doe")]
    pub username: String,
    
    #[schema(example = "john@example.com")]
    pub email: String,
    
    #[schema(example = "User")]
    pub role: UserRole,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,
    
    #[schema(example = true)]
    pub email_verified: bool,
    
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserProfileDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    #[schema(example = "john_doe")]
    pub username: String,
    
    #[schema(example = "john@example.com")]
    pub email: String,
    
    #[schema(example = "User")]
    pub role: UserRole,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,
    
    #[schema(example = true)]
    pub email_verified: bool,
    
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    #[schema(example = "jane_smith")]
    pub username: String,
    
    #[schema(example = "jane@example.com")]
    pub email: String,
    
    #[schema(example = "User")]
    pub role: UserRole,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    #[schema(example = "jane_doe")]
    pub username: Option<String>,
    
    #[schema(example = "jane.doe@example.com")]
    pub email: Option<String>,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangeUserRoleRequest {
    #[schema(example = "Admin")]
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AssignUserToCompanyRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateProfileRequest {
    #[schema(example = "john_doe_updated")]
    pub username: Option<String>,
    
    #[schema(example = "john.doe.updated@example.com")]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    #[schema(example = "currentPassword123!")]
    pub current_password: String,
    
    #[schema(example = "newPassword123!")]
    pub new_password: String,
    
    #[schema(example = "newPassword123!")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserListResponse {
    pub users: Vec<UserDto>,
    
    #[schema(example = 100)]
    pub total: u64,
    
    #[schema(example = 1)]
    pub page: u32,
    
    #[schema(example = 20)]
    pub page_size: u32,
    
    #[schema(example = 5)]
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserSearchRequest {
    #[schema(example = "john")]
    pub query: String,
    
    #[schema(example = 1)]
    pub page: u32,
    
    #[schema(example = 20)]
    pub page_size: u32,
    
    #[schema(example = "User")]
    pub role: Option<String>,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,
}