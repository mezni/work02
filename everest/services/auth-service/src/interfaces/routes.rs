use crate::interfaces::{handlers, openapi::ApiDoc};
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(handlers::register)),
            )
            .service(
                web::scope("/users")
                    .route("/me", web::get().to(handlers::get_me))
                    .route("/me", web::put().to(handlers::update_me)),
            )
            .service(
                web::scope("/admin")
                    .service(
                        web::scope("/users")
                            .route("", web::post().to(handlers::create_internal_user))
                            .route("", web::get().to(handlers::list_users))
                            .route("/{id}", web::put().to(handlers::update_user))
                            .route("/{id}", web::delete().to(handlers::delete_user)),
                    ),
            )
            .route("/health", web::get().to(handlers::health)),
    )
    .service(
        SwaggerUi::new("/api/v1/swagger-ui/{_:.*}")
            .url("/api/v1/openapi.json", ApiDoc::openapi()),
    );
}