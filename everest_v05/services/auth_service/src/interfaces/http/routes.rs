use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::interfaces::http::handlers::{create_user, login, get_user};
use crate::interfaces::http::auth_middleware::jwt_validator;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(jwt_validator);
    
    cfg
        .service(
            web::scope("/api/v1")
                .route("/login", web::post().to(login))
                .service(
                    web::scope("")
                        .wrap(auth_middleware)
                        .route("/users", web::post().to(create_user))
                        .route("/users/{id}", web::get().to(get_user))
                )
        );
}