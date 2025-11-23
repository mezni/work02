// src/domain/events/user_created.rs
use super::super::value_objects::{Email, UserId, Username};

#[derive(Debug, Clone)]
pub struct UserCreatedEvent {
    user_id: UserId,
    email: Email,
    username: Username,
    occurred_at: chrono::DateTime<chrono::Utc>,
}

impl UserCreatedEvent {
    pub fn new(user_id: UserId, email: Email, username: Username) -> Self {
        Self {
            user_id,
            email,
            username,
            occurred_at: chrono::Utc::now(),
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn username(&self) -> &Username {
        &self.username
    }
}

impl super::DomainEvent for UserCreatedEvent {
    fn event_type(&self) -> &str {
        "user.created"
    }

    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
}
