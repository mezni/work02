pub mod entities;
pub mod enums;
pub mod errors;
pub mod repositories;
pub mod value_objects;

// Re-exports
pub use entities::{AuditLog, Company, User};
pub use enums::{AuditAction, UserRole};
pub use errors::DomainError;
pub use value_objects::{Email, Password};
