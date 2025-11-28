use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organisation {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

impl Organisation {
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Organisation {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }
}
