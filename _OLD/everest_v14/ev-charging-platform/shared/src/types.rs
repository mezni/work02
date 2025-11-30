// shared/src/types.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Use string types for OpenAPI compatibility
pub type UserId = String;
pub type OrganizationId = String;
pub type StationId = String;

// Helper functions to convert from UUID
pub fn to_user_id(uuid: uuid::Uuid) -> UserId {
    uuid.to_string()
}

pub fn to_organization_id(uuid: uuid::Uuid) -> OrganizationId {
    uuid.to_string()
}

pub fn to_station_id(uuid: uuid::Uuid) -> StationId {
    uuid.to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum UserRole {
    #[serde(rename = "super_admin")]
    SuperAdmin,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "operator")]
    Operator,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::SuperAdmin => write!(f, "super_admin"),
            UserRole::Partner => write!(f, "partner"),
            UserRole::Operator => write!(f, "operator"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum UserStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "deleted")]
    Deleted,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Pending => write!(f, "pending"),
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Deleted => write!(f, "deleted"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum OrganizationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum StationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "maintenance")]
    Maintenance,
}

// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserClaims {
    pub user_id: UserId,
    pub role: UserRole,
    pub organization_id: Option<OrganizationId>,
    pub station_id: Option<StationId>,
    pub user_status: UserStatus,
}

// Common query parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }