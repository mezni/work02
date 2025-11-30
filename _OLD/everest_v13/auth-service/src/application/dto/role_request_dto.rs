use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateRoleRequestDto {
    #[validate(length(min = 1))]
    pub requested_role: String,
    
    #[validate(length(max = 500))]
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct ReviewRoleRequestDto {
    #[validate(length(min = 1))]
    pub status: String,
    
    #[validate(length(max = 500))]
    pub review_notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoleRequestDto {
    pub id: i32,
    pub user_id: String,
    pub requested_role: String,
    pub reason: Option<String>,
    pub status: String,
    pub reviewed_by: Option<String>,
    pub review_notes: Option<String>,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub reviewed_at: Option<DateTime<Utc>>,
}

// Re-export common types
pub use super::user_dto::{ErrorResponse, SuccessResponse};