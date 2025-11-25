use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUserCommand {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(must_match = "password")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginCommand {
    #[validate(email)]
    pub email: String,

    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenCommand {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ForgotPasswordCommand {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResetPasswordCommand {
    pub token: String,

    #[validate(length(min = 8))]
    pub new_password: String,

    #[validate(must_match = "new_password")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutCommand {
    pub user_id: String,
    pub refresh_token: String,
}