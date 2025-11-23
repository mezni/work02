// src/domain/events/user_updated.rs
use super::super::value_objects::{UserId, Email};

#[derive(Debug, Clone)]
pub struct UserUpdatedEvent {
    user_id: UserId,
    old_email: Email,
    occurred_at: chrono::DateTime<chrono::Utc>,
}

impl UserUpdatedEvent {
    pub fn new(user_id: UserId, old_email: Email) -> Self {
        Self {
            user_id,
            old_email,
            occurred_at: chrono::Utc::now(),
        }
    }
    
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    
    pub fn old_email(&self) -> &Email {
        &self.old_email
    }
}

impl super::DomainEvent for UserUpdatedEvent {
    fn event_type(&self) -> &str {
        "user.updated"
    }
    
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
}