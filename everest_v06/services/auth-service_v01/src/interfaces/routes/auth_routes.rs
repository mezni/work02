use actix_web::web;
use crate::interfaces::controllers::AuthController;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(AuthController::register::<
                crate::domain::repositories::UserRepositoryImpl,
                crate::infrastructure::audit::AuditorImpl,
            >))
            .route("/login", web::post().to(AuthController::login::<
                crate::domain::repositories::UserRepositoryImpl,
                crate::infrastructure::audit::AuditorImpl,
            >))
            .route("/refresh", web::post().to(AuthController::refresh_token::<
                crate::domain::repositories::UserRepositoryImpl,
                crate::infrastructure::audit::AuditorImpl,
            >))
            .route("/forgot-password", web::post().to(AuthController::forgot_password::<
                crate::domain::repositories::UserRepositoryImpl,
                crate::infrastructure::audit::AuditorImpl,
            >))
            .route("/change-password", web::post().to(AuthController::change_password::<
                crate::domain::repositories::UserRepositoryImpl,
                crate::infrastructure::audit::AuditorImpl,
            >))
    );
}