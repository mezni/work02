use crate::domain::enums::AuditAction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
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
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

impl AuditLog {
    pub fn new(
        action: AuditAction,
        user_id: Option<String>,
        user_email: Option<String>,
        user_role: Option<String>,
        company_id: Option<Uuid>,
        resource_type: String,
        resource_id: Option<String>,
        old_values: Option<serde_json::Value>,
        new_values: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        request_path: String,
        request_method: String,
        status_code: u16,
        error_message: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            action,
            user_id,
            user_email,
            user_role,
            company_id,
            resource_type,
            resource_id,
            old_values,
            new_values,
            ip_address,
            user_agent,
            request_path,
            request_method,
            status_code,
            error_message,
            metadata,
            created_at: Utc::now(),
        }
    }
}
