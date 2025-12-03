pub mod auth_service;
pub mod registration_service;
pub mod token_service;
pub mod commands;
pub mod queries;
pub mod error;
pub mod service_traits;

// Re-exports
pub use auth_service::AuthService;
pub use registration_service::RegistrationService;
pub use token_service::TokenService;
pub use commands::{LoginCommand, RegisterCommand, RefreshTokenCommand, LogoutCommand};
pub use queries::{GetUserQuery, ValidateTokenQuery};
pub use error::{ApplicationError, ApplicationResult};
pub use service_traits::{AuthServiceTrait, RegistrationServiceTrait, TokenServiceTrait};