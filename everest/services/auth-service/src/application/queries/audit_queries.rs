use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::enums::AuditAction;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAuditLogsQuery {
    pub user_id: Option<String>,
    pub company_id: Option<Uuid>,
    pub action: Option<AuditAction>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub page: u32,
    pub page_size: u32,
    pub requested_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserAuditLogsQuery {
    pub user_id: String,
    pub page: u32,
    pub page_size: u32,
    pub requested_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCompanyAuditLogsQuery {
    pub company_id: Uuid,
    pub page: u32,
    pub page_size: u32,
    pub requested_by: Uuid,
}