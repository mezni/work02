// src/api/mod.rs
pub mod handlers;
pub mod routes;
pub mod openapi;
pub mod middleware;
pub mod error;

pub use error::ApiError;

pub use routes::configure_routes;
pub use openapi::configure_swagger;
pub use handlers::health_handler::not_found_handler;