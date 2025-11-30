use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
}

impl Organization {
    pub fn new(name: String, address: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            address,
        }
    }
}
