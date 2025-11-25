pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;

// Prelude for common imports
pub mod prelude {
    pub use crate::domain::errors::DomainError;
    pub use crate::application::errors::ApplicationError;
    pub use crate::infrastructure::errors::InfrastructureError;
    pub use crate::interfaces::errors::InterfaceError;
    
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}

// Re-exports for common types
pub use domain::entities::{User, Company};
pub use application::dto::{UserDto, CompanyDto};
