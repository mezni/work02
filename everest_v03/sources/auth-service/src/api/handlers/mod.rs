// src/api/handlers/mod.rs
pub mod user_handler;
pub mod admin_handler;
pub mod health_handler;

pub use user_handler::UserHandler;
pub use admin_handler::AdminHandler;
pub use health_handler::HealthHandler;