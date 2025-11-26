// src/api/routes/user_routes.rs
use actix_web::web;
use crate::api::handlers::user_handler::UserHandler;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/users")
            .route("", web::post().to(UserHandler::create_user))
            .route("", web::get().to(UserHandler::get_users))
            .route("/{id}", web::get().to(UserHandler::get_user_by_id))
            .route("/{id}", web::put().to(UserHandler::update_user)),
    );
}