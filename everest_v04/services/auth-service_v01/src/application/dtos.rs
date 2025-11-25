use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub company: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserResponseDTO {
    pub id: String,
}
