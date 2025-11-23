// src/domain/events/mod.rs
pub mod user_created;
pub mod user_deactivated;
pub mod user_updated;

pub use user_created::UserCreatedEvent;
pub use user_deactivated::UserDeactivatedEvent;
pub use user_updated::UserUpdatedEvent;

pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &str;
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn version(&self) -> &str {
        "1.0"
    }
}
