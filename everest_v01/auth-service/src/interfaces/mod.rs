pub mod http_routes;
pub mod handlers;
pub mod middleware;
pub mod dtos;
pub mod swagger;
pub mod openapi;
pub mod error;

// Re-exports
pub use http_routes::configure_routes;
pub use handlers::{
    AuthHandler, UserHandler, HealthHandler, AdminHandler,
    AuthHandlerImpl, UserHandlerImpl, HealthHandlerImpl, AdminHandlerImpl
};
pub use middleware::{AuthMiddleware, ErrorHandler, RateLimiter};
pub use dtos::*;
pub use swagger::configure_swagger;
pub use error::{InterfaceError, InterfaceResult};