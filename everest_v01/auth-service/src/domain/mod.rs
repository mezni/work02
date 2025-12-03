pub mod user;
pub mod token;
pub mod credentials;
pub mod registration;
pub mod events;
pub mod repository;
pub mod error;
pub mod value_objects;
pub mod aggregates;

// Re-exports
pub use user::User;
pub use token::{Token, TokenClaims};
pub use credentials::{Email, Password};
pub use registration::Registration;
pub use events::{DomainEvent, UserRegistered, UserLoggedIn};
pub use repository::{UserRepository, TokenRepository};
pub use error::{DomainError, DomainResult};
pub use value_objects::{UserRole, CompanyName, StationName};