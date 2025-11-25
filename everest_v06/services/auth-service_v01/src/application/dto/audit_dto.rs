use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use crate::domain::enums::AuditAction;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuditLogDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
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
    
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuditLogListResponse {
    pub logs: Vec<AuditLogDto>,
    
    #[schema(example = 1000)]
    pub total: u64,
    
    #[schema(example = 1)]
    pub page: u32,
    
    #[schema(example = 50)]
    pub page_size: u32,
    
    #[schema(example = 20)]
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuditSearchRequest {
    #[schema(example = "user_123")]
    pub user_id: Option<String>,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<Uuid>,
    
    #[schema(example = "UserLogin")]
    pub action: Option<AuditAction>,
    
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub start_date: Option<DateTime<Utc>>,
    
    #[schema(example = "2023-12-31T23:59:59Z")]
    pub end_date: Option<DateTime<Utc>>,
    
    #[schema(example = 1)]
    pub page: Option<u32>,
    
    #[schema(example = 50)]
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuditStatisticsResponse {
    #[schema(example = 1500)]
    pub total_events: u64,
    
    pub events_by_action: std::collections::HashMap<String, u64>,
    
    pub events_by_user: std::collections::HashMap<String, u64>,
    
    pub events_by_company: std::collections::HashMap<String, u64>,
    
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub period_start: DateTime<Utc>,
    
    #[schema(example = "2023-12-31T23:59:59Z")]
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ExportAuditRequest {
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub start_date: DateTime<Utc>,
    
    #[schema(example = "2023-12-31T23:59:59Z")]
    pub end_date: DateTime<Utc>,
    
    #[schema(example = "Json")]
    pub format: ExportFormat,
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
pub struct AuditExportResponse {
    #[schema(example = "data:application/json;base64,eyJhdWRpdF9sb2dzIjpbXX0=")]
    pub data: String,
    
    #[schema(example = "audit_logs_2023.json")]
    pub filename: String,
    
    #[schema(example = "application/json")]
    pub content_type: String,
}