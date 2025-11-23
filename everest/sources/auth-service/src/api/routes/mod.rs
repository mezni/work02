// src/api/routes/mod.rs
pub mod user_routes;
pub mod admin_routes;
pub mod health_routes;

use actix_web::web;
use crate::api::handlers::{UserHandler, AdminHandler, HealthHandler};
use crate::application::UserApplicationService;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    user_routes::configure_user_routes(cfg);
    admin_routes::configure_admin_routes(cfg);
    health_routes::configure_health_routes(cfg);
}

pub fn create_handlers(
    user_service: web::Data<UserApplicationService>
) -> (
    UserHandler, 
    AdminHandler, 
    HealthHandler
) {
    let user_handler = UserHandler::new(user_service.clone());
    let admin_handler = AdminHandler::new(user_service);
    let health_handler = HealthHandler::new();
    
    (user_handler, admin_handler, health_handler)
}