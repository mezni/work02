use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoleRequest {
    pub id: i32,
    pub user_id: String,
    pub requested_role: String,
    pub reason: Option<String>,
    pub status: RoleRequestStatus,
    pub reviewed_by: Option<String>,
    pub review_notes: Option<String>,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub reviewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum RoleRequestStatus {
    Pending,
    Approved,
    Denied,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRoleRequest {
    pub user_id: String,
    pub requested_role: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReviewRoleRequest {
    pub status: RoleRequestStatus,
    pub review_notes: Option<String>,
}