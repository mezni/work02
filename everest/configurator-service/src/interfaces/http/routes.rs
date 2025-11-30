use super::handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/hello", web::get().to(handlers::hello))
            .route("/health", web::get().to(handlers::health_check))
            .route("/user/{user_id}", web::get().to(handlers::get_user))
            .route("/user", web::post().to(handlers::create_user))
            .route("/update-name", web::post().to(handlers::update_app_name)),
    );
}
