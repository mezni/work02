use crate::domain::audit::Audit;
use crate::domain::user::User;
use crate::domain::value_objects::UserRole;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::Validate;

// ============================================================================
// Authentication DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 100))]
    pub username: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,

    #[validate(length(max = 100))]
    pub first_name: Option<String>,

    #[validate(length(max = 100))]
    pub last_name: Option<String>,

    #[validate(length(min = 10, max = 20))]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub user: UserResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1))]
    pub current_password: String,

    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ResetPasswordRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct VerifyEmailRequest {
    pub token: String,
}

// ============================================================================
// User Management DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 100))]
    pub username: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,

    #[validate(length(max = 100))]
    pub first_name: Option<String>,

    #[validate(length(max = 100))]
    pub last_name: Option<String>,

    #[validate(length(min = 10, max = 20))]
    pub phone: Option<String>,

    pub photo: Option<String>,

    pub role: UserRole,

    #[validate(length(max = 32))]
    pub network_id: Option<String>,

    #[validate(length(max = 32))]
    pub station_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(length(max = 100))]
    pub first_name: Option<String>,

    #[validate(length(max = 100))]
    pub last_name: Option<String>,

    #[validate(length(min = 10, max = 20))]
    pub phone: Option<String>,

    pub photo: Option<String>,

    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRoleRequest {
    pub role: UserRole,

    #[validate(length(max = 32))]
    pub network_id: Option<String>,

    #[validate(length(max = 32))]
    pub station_id: Option<String>,
}

// ============================================================================
// Response DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub user_id: String,
    pub keycloak_id: Option<String>,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub is_verified: bool,
    pub role: String,
    pub network_id: String,
    pub station_id: String,
    pub source: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            keycloak_id: Some(user.keycloak_id),
            email: user.email,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            photo: user.photo,
            is_verified: user.is_verified,
            role: user.role,
            network_id: user.network_id,
            station_id: user.station_id,
            source: user.source,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserDetailResponse {
    #[serde(flatten)]
    pub user: UserResponse,
    pub full_name: String,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl From<User> for UserDetailResponse {
    fn from(user: User) -> Self {
        let full_name = user.full_name();
        let created_by = user.created_by.clone();
        let updated_by = user.updated_by.clone();

        Self {
            user: user.into(),
            full_name,
            created_by,
            updated_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditLogResponse {
    pub audit_id: String,
    pub user_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub ip_address: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub location: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Audit> for AuditLogResponse {
    fn from(audit: Audit) -> Self {
        let location = audit.location_string();

        Self {
            audit_id: audit.audit_id,
            user_id: audit.user_id,
            action: audit.action,
            resource_type: audit.resource_type,
            resource_id: audit.resource_id,
            ip_address: audit.ip_address,
            country: audit.country,
            city: audit.city,
            location,
            user_agent: audit.user_agent,
            created_at: audit.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserStatistics {
    pub total: i64,
    pub active: i64,
    pub inactive: i64,
    pub verified: i64,
    pub unverified: i64,
    pub by_role: HashMap<String, i64>,
    pub by_source: HashMap<String, i64>,
}

// ============================================================================
// Pagination DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: default_limit(),
            offset: 0,
        }
    }
}

impl PaginationParams {
    pub fn new(limit: i64, offset: i64) -> Self {
        Self { limit, offset }
    }

    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchParams {
    pub query: String,

    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default)]
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub page: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, limit: i64, offset: i64) -> Self {
        let page = (offset / limit) + 1;
        let total_pages = (total as f64 / limit as f64).ceil() as i64;
        let has_next = offset + limit < total;
        let has_prev = offset > 0;

        Self {
            data,
            pagination: PaginationMeta {
                total,
                limit,
                offset,
                page,
                total_pages,
                has_next,
                has_prev,
            },
        }
    }
}

// ============================================================================
// Generic Response DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message)
    }

    pub fn created(resource: &str) -> Self {
        Self::new(format!("{} created successfully", resource))
    }

    pub fn updated(resource: &str) -> Self {
        Self::new(format!("{} updated successfully", resource))
    }

    pub fn deleted(resource: &str) -> Self {
        Self::new(format!("{} deleted successfully", resource))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub uptime_seconds: u64,
}

impl HealthResponse {
    pub fn healthy(uptime_seconds: u64) -> Self {
        Self {
            status: "healthy".to_string(),
            service: "auth-service".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: Utc::now(),
            uptime_seconds,
        }
    }

    pub fn unhealthy(reason: String) -> Self {
        Self {
            status: format!("unhealthy: {}", reason),
            service: "auth-service".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: Utc::now(),
            uptime_seconds: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

impl ErrorResponse {
    pub fn new(error: String, message: String) -> Self {
        Self {
            error,
            message,
            details: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

// ============================================================================
// Filter DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserFilterParams {
    pub role: Option<String>,
    pub source: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub network_id: Option<String>,
    pub station_id: Option<String>,

    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditFilterParams {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,

    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// ============================================================================
// Bulk Operation DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BulkDeactivateRequest {
    #[validate(length(min = 1))]
    pub user_ids: Vec<String>,

    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BulkOperationResponse {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub errors: Vec<BulkOperationError>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BulkOperationError {
    pub user_id: String,
    pub error: String,
}

// ============================================================================
// Export DTOs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExportRequest {
    pub format: ExportFormat,
    pub filters: Option<UserFilterParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Json,
    Excel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params_default() {
        let params = PaginationParams::default();
        assert_eq!(params.limit, 20);
        assert_eq!(params.offset, 0);
    }

    #[test]
    fn test_pagination_meta() {
        let response = PaginatedResponse::new(vec![1, 2, 3], 100, 20, 0);
        assert_eq!(response.pagination.total, 100);
        assert_eq!(response.pagination.page, 1);
        assert_eq!(response.pagination.total_pages, 5);
        assert!(response.pagination.has_next);
        assert!(!response.pagination.has_prev);
    }

    #[test]
    fn test_message_response_builders() {
        let msg = MessageResponse::created("User");
        assert_eq!(msg.message, "User created successfully");

        let msg = MessageResponse::updated("Profile");
        assert_eq!(msg.message, "Profile updated successfully");

        let msg = MessageResponse::deleted("Account");
        assert_eq!(msg.message, "Account deleted successfully");
    }

    #[test]
    fn test_register_request_validation() {
        let valid = RegisterRequest {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            phone: Some("+1234567890".to_string()),
        };
        assert!(valid.validate().is_ok());

        let invalid = RegisterRequest {
            email: "invalid-email".to_string(),
            username: "ab".to_string(), // Too short
            password: "pass".to_string(), // Too short
            first_name: None,
            last_name: None,
            phone: None,
        };
        assert!(invalid.validate().is_err());
    }
}