use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::value_objects::{Email, Phone};

#[derive(Debug, Clone)]
pub struct NetworkUpdated {
    pub network_id: Uuid,
    pub name: Option<String>,
    pub email: Option<Email>,
    pub phone: Option<Phone>,
    pub updated_by: Uuid,
    pub updated_at: NaiveDateTime,
}

impl NetworkUpdated {
    pub fn new(
        network_id: Uuid,
        name: Option<String>,
        email: Option<Email>,
        phone: Option<Phone>,
        updated_by: Uuid,
    ) -> Self {
        Self {
            network_id,
            name,
            email,
            phone,
            updated_by,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
