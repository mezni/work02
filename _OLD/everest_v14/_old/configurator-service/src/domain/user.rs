use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub enum Role {
    Admin,
    Operator,
    Guest,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub org_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
}

impl User {
    pub fn new(
        name: String,
        email: String,
        role: Role,
        org_id: Option<Uuid>,
        station_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            role,
            org_id,
            station_id,
        }
    }
}
