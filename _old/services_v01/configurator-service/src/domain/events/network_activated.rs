use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NetworkActivated {
    pub network_id: Uuid,
    pub activated_by: Uuid,
    pub activated_at: NaiveDateTime,
}

impl NetworkActivated {
    pub fn new(network_id: Uuid, activated_by: Uuid) -> Self {
        Self {
            network_id,
            activated_by,
            activated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
