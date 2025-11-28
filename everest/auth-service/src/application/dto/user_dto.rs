use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub organisation_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
    pub is_active: bool,
}

impl UserDTO {
    pub fn from_domain(user: &crate::domain::models::User) -> Self {
        Self {
            id: user.id,
            username: user.username.to_string(),
            email: user.email.to_string(),
            role: user.role.to_string(),
            organisation_id: user.organisation_id,
            station_id: user.station_id,
            is_active: user.is_active,
        }
    }
}
