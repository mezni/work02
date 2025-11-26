pub mod controllers;
pub mod routes;
pub mod openapi;
pub mod errors;

// Re-exports
pub use controllers::{AuthController, UserController, CompanyController};
pub use routes::configure_routes;
pub use errors::InterfaceError;
