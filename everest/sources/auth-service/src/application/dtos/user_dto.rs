// src/application/dtos/user_dto.rs
use serde::Serialize;
use crate::domain::entities::User;

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: String,
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: String,
    pub is_active: bool,
    pub is_email_verified: bool,
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            keycloak_id: user.keycloak_id().to_string(),
            username: user.username().to_string(),
            email: user.email().to_string(),
            first_name: user.first_name().map(|s| s.to_string()),
            last_name: user.last_name().map(|s| s.to_string()),
            phone_number: user.phone_number().to_string(),
            is_active: user.is_active(),
            is_email_verified: user.is_email_verified(),
            last_login_at: user.last_login_at(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().to_string(),
            keycloak_id: user.keycloak_id().to_string(),
            username: user.username().to_string(),
            email: user.email().to_string(),
            first_name: user.first_name().map(|s| s.to_string()),
            last_name: user.last_name().map(|s| s.to_string()),
            phone_number: user.phone_number().to_string(),
            is_active: user.is_active(),
            is_email_verified: user.is_email_verified(),
            last_login_at: user.last_login_at(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}