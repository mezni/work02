use crate::interfaces::handlers::user_handlers;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // User routes
            .route("/users", web::post().to(user_handlers::create_user))
            .route("/users", web::get().to(user_handlers::list_users))
            .route("/users/{user_id}", web::get().to(user_handlers::get_user))
            .route(
                "/users/username/{username}",
                web::get().to(user_handlers::get_user_by_username),
            )
            .route(
                "/users/{user_id}/enable",
                web::put().to(user_handlers::enable_user),
            )
            .route(
                "/users/{user_id}/disable",
                web::put().to(user_handlers::disable_user),
            )
            .route(
                "/users/{user_id}",
                web::delete().to(user_handlers::delete_user),
            )
            // Role routes
            .route(
                "/users/{user_id}/roles",
                web::post().to(user_handlers::assign_role),
            )
            .route(
                "/users/{user_id}/roles",
                web::get().to(user_handlers::get_user_roles),
            ),
    );
}