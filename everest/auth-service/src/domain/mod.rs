pub mod entities;
pub mod value_objects;
pub mod enums;
pub mod repositories;
pub mod errors;

// Re-exports
pub use entities::{User, Company, AuditLog};
pub use value_objects::{Email, Password};
pub use enums::{UserRole, AuditAction};
pub use errors::DomainError;
