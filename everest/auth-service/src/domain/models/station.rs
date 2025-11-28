use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub organisation_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

impl Station {
    pub fn new(name: String, description: Option<String>, organisation_id: Uuid) -> Self {
        let now = Utc::now();
        Station {
            id: Uuid::new_v4(),
            name,
            description,
            organisation_id,
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }
}
