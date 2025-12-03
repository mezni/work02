pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
pub mod config;
pub mod db;
pub mod logger;
pub mod error;
pub mod shared;

// Re-exports for convenience
pub use config::AppConfig;
pub use error::{AppError, Result};
pub use shared::{constants, utils};
pub use domain::{
    User, Token, TokenClaims, Email, Password, UserRole,
    DomainError, DomainResult,
};
pub use application::{
    AuthService, RegistrationService, TokenService,
    ApplicationError, ApplicationResult,
};
pub use infrastructure::{
    KeycloakClient, KeycloakUserRepository, JwtTokenGenerator,
    InfrastructureError, InfrastructureResult,
};
pub use interfaces::{
    configure_routes,
    InterfaceError, InterfaceResult,
};