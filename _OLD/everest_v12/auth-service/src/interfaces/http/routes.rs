use actix_web::web;
use crate::interfaces::http::auth_controller::{register, login, admin_create_user};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/admin/create-user", web::post().to(admin_create_user))
    );
}
