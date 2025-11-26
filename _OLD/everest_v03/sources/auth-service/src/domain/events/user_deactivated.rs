// src/domain/events/user_deactivated.rs
use super::super::value_objects::UserId;

#[derive(Debug, Clone)]
pub struct UserDeactivatedEvent {
    user_id: UserId,
    occurred_at: chrono::DateTime<chrono::Utc>,
}

impl UserDeactivatedEvent {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            occurred_at: chrono::Utc::now(),
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
}

impl super::DomainEvent for UserDeactivatedEvent {
    fn event_type(&self) -> &str {
        "user.deactivated"
    }

    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
}
