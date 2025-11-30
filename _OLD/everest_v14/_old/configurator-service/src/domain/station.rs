use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Station {
    pub id: Uuid,
    pub name: String,
    pub org_id: Option<Uuid>,
}

impl Station {
    pub fn new(name: String, org_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            org_id,
        }
    }
}
