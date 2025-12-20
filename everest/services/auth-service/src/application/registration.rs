use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResendRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationStatusResponse {
    pub id: String,
    pub status: String,
    pub verified: bool,
    pub created_at: String,
}
