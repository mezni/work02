pub mod controllers;
pub mod errors;
pub mod openapi;
pub mod routes;

// Re-exports
pub use controllers::{AuthController, CompanyController, UserController};
pub use errors::InterfaceError;
pub use routes::configure_routes;
