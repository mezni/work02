use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NetworkDeactivated {
    pub network_id: Uuid,
    pub deactivated_by: Uuid,
    pub deactivated_at: NaiveDateTime,
}

impl NetworkDeactivated {
    pub fn new(network_id: Uuid, deactivated_by: Uuid) -> Self {
        Self {
            network_id,
            deactivated_by,
            deactivated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
