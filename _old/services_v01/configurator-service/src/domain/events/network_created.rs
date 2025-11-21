use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::value_objects::{Email, Phone};
use crate::domain::enums::NetworkType;

#[derive(Debug, Clone)]
pub struct NetworkCreated {
    pub network_id: Uuid,
    pub name: Option<String>,
    pub network_type: NetworkType,
    pub email: Option<Email>,
    pub phone: Option<Phone>,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
}

impl NetworkCreated {
    pub fn new(
        network_id: Uuid,
        name: Option<String>,
        network_type: NetworkType,
        email: Option<Email>,
        phone: Option<Phone>,
        created_by: Uuid,
    ) -> Self {
        Self {
            network_id,
            name,
            network_type,
            email,
            phone,
            created_by,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
