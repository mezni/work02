use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "johndoe", min_length = 3, max_length = 100)]
    pub username: String,
    #[schema(example = "John")]
    pub first_name: Option<String>,
    #[schema(example = "Doe")]
    pub last_name: Option<String>,
    #[schema(example = "+15551234567")]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterUserResponse {
    pub registration_id: String,
    pub status: String,
    pub expires_at: String,
}
