use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterUserCommand {
    #[validate(length(min = 3, max = 50))]
    #[schema(example = "john_doe")]
    pub username: String,

    #[validate(email)]
    #[schema(example = "john@example.com")]
    pub email: String,

    #[validate(length(min = 8))]
    #[schema(example = "securePassword123!")]
    pub password: String,

    #[validate(must_match = "password")]
    #[schema(example = "securePassword123!")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginCommand {
    #[validate(email)]
    #[schema(example = "john@example.com")]
    pub email: String,

    #[schema(example = "securePassword123!")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenCommand {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ForgotPasswordCommand {
    #[validate(email)]
    #[schema(example = "john@example.com")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ResetPasswordCommand {
    #[schema(example = "reset_token_abc123")]
    pub token: String,

    #[validate(length(min = 8))]
    #[schema(example = "newSecurePassword123!")]
    pub new_password: String,

    #[validate(must_match = "new_password")]
    #[schema(example = "newSecurePassword123!")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LogoutCommand {
    #[schema(example = "user_123")]
    pub user_id: String,

    #[schema(example = "refresh_token_abc123")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidateTokenCommand {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordCommand {
    #[schema(example = "user_123")]
    pub user_id: String,
    
    #[schema(example = "currentPassword123!")]
    pub current_password: String,
    
    #[schema(example = "newPassword123!")]
    pub new_password: String,
}