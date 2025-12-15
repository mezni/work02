pub mod audit_repository;
pub mod outbox_repository;
pub mod user_repository;

// Re-export traits and implementations for convenience
pub use audit_repository::{AuditRepository, PostgresAuditRepository};
pub use outbox_repository::{OutboxRepository, PostgresOutboxRepository};
pub use user_repository::{PostgresUserRepository, UserRepository};