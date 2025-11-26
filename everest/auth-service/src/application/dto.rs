use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::domain::enums::UserRole;

// Helper macro to implement ToSchema for Uuid fields
macro_rules! uuid_schema {
    () => {
        /// UUID in string format
        #[derive(ToSchema)]
        #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
        struct UuidSchema(String);
    };
}

uuid_schema!();

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserDto {
    #[schema(value_type = String, example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    #[schema(value_type = Option<String>)]
    pub company_id: Option<Uuid>,
    pub email_verified: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 100))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    pub role: UserRole,

    #[schema(value_type = Option<String>)]
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, max = 100))]
    pub username: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    pub role: Option<UserRole>,

    #[schema(value_type = Option<String>)]
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CompanyDto {
    #[schema(value_type = String, example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[schema(value_type = String, example = "550e8400-e29b-41d4-a716-446655440000")]
    pub created_by: Uuid,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCompanyDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCompanyDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BusinessClaims {
    pub sub: String,
    pub email: String,
    pub username: String,
    pub role: UserRole,
    #[schema(value_type = Option<String>)]
    pub company_id: Option<Uuid>,
    pub permissions: Vec<String>,
    pub exp: i64,
    pub iat: i64,
}
