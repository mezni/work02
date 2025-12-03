use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::{User, Token};

// Request DTOs
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateProfileRequest {
    pub company_name: Option<String>,
    pub station_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConfirmPasswordResetRequest {
    pub token: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResendVerificationRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRoleRequest {
    pub role: String,
}

// Response DTOs
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

impl AuthResponse {
    pub fn from_token(token: Token) -> Self {
        // In real implementation, you would extract user from token
        // or fetch user details
        Self {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            token_type: token.token_type,
            expires_in: token.expires_in,
            user: UserResponse {
                id: Uuid::new_v4(), // Would be actual user ID
                email: "user@example.com".to_string(), // Would be actual email
                role: "user".to_string(),
                company_name: "".to_string(),
                station_name: "".to_string(),
                is_active: true,
                email_verified: false,
                created_at: Utc::now(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationResponse {
    pub message: String,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub company_name: String,
    pub station_name: String,
    pub is_active: bool,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl UserResponse {
    pub fn from_user(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role.as_str().to_string(),
            company_name: user.company_name,
            station_name: user.station_name,
            is_active: user.is_active,
            email_verified: user.email_verified,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DetailedHealthResponse {
    pub status: String,
    pub services: Vec<crate::infrastructure::health_check::HealthStatus>,
    pub total_services: usize,
    pub healthy_services: usize,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SuccessResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenInfoResponse {
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
    pub company_name: String,
    pub station_name: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub email_verified: bool,
}

// Query Parameter DTOs
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 { 1 }
fn default_limit() -> u32 { 20 }

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserFilterQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

// WebSocket DTOs (if needed)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WebSocketAuthRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WebSocketMessage {
    pub r#type: String,
    pub data: serde_json::Value,
}

// Statistics DTOs
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserStatistics {
    pub total_users: u64,
    pub active_users: u64,
    pub verified_users: u64,
    pub users_by_role: std::collections::HashMap<String, u64>,
    pub new_users_today: u64,
    pub new_users_this_week: u64,
    pub new_users_this_month: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthStatistics {
    pub total_logins: u64,
    pub failed_logins: u64,
    pub successful_logins: u64,
    pub active_sessions: u64,
    pub average_session_duration: f64,
}