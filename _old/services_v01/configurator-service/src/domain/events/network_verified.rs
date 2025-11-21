use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NetworkVerified {
    pub network_id: Uuid,
    pub verified_by: Uuid,
    pub verified_at: NaiveDateTime,
}

impl NetworkVerified {
    pub fn new(network_id: Uuid, verified_by: Uuid) -> Self {
        Self {
            network_id,
            verified_by,
            verified_at: chrono::Utc::now().naive_utc(),
        }
    }
}
