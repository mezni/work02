use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct LoginRequest {
    #[schema(example = "testuser")]
    pub username: String,
    #[schema(example = "securepassword123")]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct RegisterRequest {
    #[schema(example = "newuser")]
    pub username: String,
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "securepassword123")]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct AdminCreateUserRequest {
    #[schema(example = "admin_created_user")]
    pub username: String,
    #[schema(example = "admin@example.com")]
    pub email: String,
    #[schema(example = "securepassword123")]
    pub password: String,
    #[schema(example = "partner")]
    pub role: String,
    #[schema(example = "12345678-0000-0000-0000-000000000001", nullable = true)]
    pub organisation_id: Option<Uuid>,
    #[schema(example = "12345678-0000-0000-0000-000000000001", nullable = true)]
    pub station_id: Option<Uuid>,
}
