use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::domain::user_entity::User;
use crate::domain::value_objects::UserRole;

// Request DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    
    #[validate(length(max = 20))]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    
    #[validate(length(max = 20))]
    pub phone: Option<String>,
    
    pub photo: Option<String>,
    pub role: UserRole,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    
    #[validate(length(max = 20))]
    pub phone: Option<String>,
    
    pub photo: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    
    #[validate(length(min = 8))]
    pub new_password: String,
}

// Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub user_id: String,
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
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
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
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, limit: i64, offset: i64) -> Self {
        Self {
            data,
            total,
            limit,
            offset,
        }
    }
}

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
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            service: "auth-service".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: Utc::now(),
        }
    }
}

// Query parameters
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchParams {
    pub query: String,
    
    #[serde(default = "default_limit")]
    pub limit: i64,
    
    #[serde(default)]
    pub offset: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_request_validation() {
        let valid = RegisterRequest {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            phone: None,
        };
        assert!(valid.validate().is_ok());

        let invalid_email = RegisterRequest {
            email: "invalid-email".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
            first_name: None,
            last_name: None,
            phone: None,
        };
        assert!(invalid_email.validate().is_err());

        let short_password = RegisterRequest {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "pass".to_string(),
            first_name: None,
            last_name: None,
            phone: None,
        };
        assert!(short_password.validate().is_err());
    }

    #[test]
    fn test_pagination_defaults() {
        let params = PaginationParams::default();
        assert_eq!(params.limit, 20);
        assert_eq!(params.offset, 0);
    }
}