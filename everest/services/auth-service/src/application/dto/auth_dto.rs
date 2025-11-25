use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "john_doe")]
    pub username: String,
    
    #[schema(example = "john@example.com")]
    pub email: String,
    
    #[schema(example = "securePassword123!")]
    pub password: String,
    
    #[schema(example = "securePassword123!")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "john@example.com")]
    pub email: String,
    
    #[schema(example = "securePassword123!")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ForgotPasswordRequest {
    #[schema(example = "john@example.com")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
    #[schema(example = "reset_token_abc123")]
    pub token: String,
    
    #[schema(example = "newSecurePassword123!")]
    pub new_password: String,
    
    #[schema(example = "newSecurePassword123!")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
    
    #[schema(example = "Bearer")]
    pub token_type: String,
    
    #[schema(example = 3600)]
    pub expires_in: u64,
    
    pub user: UserAuthDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAuthDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
    
    #[schema(example = "john_doe")]
    pub username: String,
    
    #[schema(example = "john@example.com")]
    pub email: String,
    
    #[schema(example = "User")]
    pub role: String,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<String>,
    
    #[schema(example = true)]
    pub email_verified: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenValidationResponse {
    #[schema(example = true)]
    pub valid: bool,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Option<String>,
    
    #[schema(example = "john_doe")]
    pub username: Option<String>,
    
    #[schema(example = "User")]
    pub role: Option<String>,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub company_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    #[schema(example = "currentPassword123!")]
    pub current_password: String,
    
    #[schema(example = "newPassword123!")]
    pub new_password: String,
    
    #[schema(example = "newPassword123!")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    #[schema(example = "200")]
    pub status_code: u16,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            status_code: 200,
        }
    }
    
    pub fn error(message: String, status_code: u16) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            status_code,
        }
    }
}