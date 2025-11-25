use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use crate::domain::enums::AuditAction;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuditLogDto {
    pub id: Uuid,
    pub action: AuditAction,
    pub user_id: Option<String>,
    pub user_email: Option<String>,
    pub user_role: Option<String>,
    pub company_id: Option<Uuid>,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub request_path: String,
    pub request_method: String,
    pub status_code: u16,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuditLogListResponse {
    pub logs: Vec<AuditLogDto>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuditSearchRequest {
    pub user_id: Option<String>,
    pub company_id: Option<Uuid>,
    pub action: Option<AuditAction>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}