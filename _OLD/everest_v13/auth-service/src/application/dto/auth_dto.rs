// src/application/dto/auth_dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenClaims {
    pub sub: String, // user_id
    pub username: String,
    pub exp: usize, // expiration time
    pub iat: usize, // issued at
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ValidateTokenResponse {
    pub valid: bool,
    pub user_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Token cannot be empty"))]
    pub token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub message: String,
}
