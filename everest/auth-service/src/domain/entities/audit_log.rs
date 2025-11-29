use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditLog {
    pub id: i32,
    pub user_id: String,
    pub organisation_id: Option<i32>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    #[schema(value_type = String)]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditLogQuery {
    pub user_id: Option<String>,
    pub organisation_id: Option<i32>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    #[schema(value_type = String)]
    pub start_date: Option<DateTime<Utc>>,
    #[schema(value_type = String)]
    pub end_date: Option<DateTime<Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}