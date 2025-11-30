use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::domain::value_objects::Role;
use crate::domain::entities::User;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub role: Role,
    pub organisation_name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub keycloak_id: String,
    pub email: String,
    pub username: String,
    pub role: Role,
    pub organisation_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Implement conversion from User to UserResponse
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            keycloak_id: user.keycloak_id,
            email: user.email.value().to_string(), // Convert &str to String
            username: user.username,
            role: user.role,
            organisation_name: user.organisation_name.map(|org| org.value().to_string()), // Convert Option<&str> to Option<String>
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EnrichedTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub token_type: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PublicTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub token_type: String,
}
