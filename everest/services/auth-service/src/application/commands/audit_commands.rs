use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use crate::domain::enums::AuditAction;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAuditLogCommand {
    #[schema(example = "UserLogin")]
    pub action: AuditAction,
    
    #[schema(example = "user_123")]
    pub user_id: Option<String>,
    
    #[schema(example = "user@example.com")]
    pub user_email: Option<String>,
    
    #[schema(example = "Admin")]
    pub user_role: Option<String>,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,
    
    #[schema(example = "User")]
    pub resource_type: String,
    
    #[schema(example = "user_123")]
    pub resource_id: Option<String>,
    
    pub old_values: Option<serde_json::Value>,
    
    pub new_values: Option<serde_json::Value>,
    
    #[schema(example = "192.168.1.1")]
    pub ip_address: Option<String>,
    
    #[schema(example = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")]
    pub user_agent: Option<String>,
    
    #[schema(example = "/api/v1/users")]
    pub request_path: String,
    
    #[schema(example = "POST")]
    pub request_method: String,
    
    #[schema(example = 200)]
    pub status_code: u16,
    
    #[schema(example = "Invalid input data")]
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CleanupAuditLogsCommand {
    #[schema(example = 365)]
    pub retention_days: u32,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub executed_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ExportAuditLogsCommand {
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub start_date: DateTime<Utc>,
    
    #[schema(example = "2023-12-31T23:59:59Z")]
    pub end_date: DateTime<Utc>,
    
    pub format: ExportFormat,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub requested_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ExportFormat {
    #[schema(example = "Json")]
    Json,
    #[schema(example = "Csv")]
    Csv,
    #[schema(example = "Xml")]
    Xml,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LogUserLoginCommand {
    #[schema(example = "user_123")]
    pub user_id: String,
    
    #[schema(example = "user@example.com")]
    pub user_email: String,
    
    #[schema(example = "Admin")]
    pub user_role: String,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,
    
    #[schema(example = "192.168.1.1")]
    pub ip_address: Option<String>,
    
    #[schema(example = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LogUserRegistrationCommand {
    #[schema(example = "user_123")]
    pub user_id: String,
    
    #[schema(example = "user@example.com")]
    pub user_email: String,
    
    #[schema(example = "User")]
    pub user_role: String,
    
    #[schema(example = "192.168.1.1")]
    pub ip_address: Option<String>,
    
    #[schema(example = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LogUserRoleChangeCommand {
    #[schema(example = "user_123")]
    pub target_user_id: String,
    
    #[schema(example = "admin_456")]
    pub changed_by_user_id: String,
    
    #[schema(example = "User")]
    pub old_role: String,
    
    #[schema(example = "Admin")]
    pub new_role: String,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,
}