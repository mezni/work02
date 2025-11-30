use actix_web::web;
use crate::domain::repositories::UserRepository;
use super::handlers;

pub fn configure_routes<R: UserRepository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/auth/login", web::post().to(handlers::login::<R>))
            .route("/users", web::post().to(handlers::create_user::<R>))
            .route("/users/{id}", web::get().to(handlers::get_user::<R>))
            .route(
                "/organisations/{org_name}/users",
                web::get().to(handlers::list_users_by_organisation::<R>),
            ),
    )
    .route("/health", web::get().to(handlers::health_check));
}
