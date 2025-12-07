use actix_web::web;
use crate::interfaces::handlers;
use crate::middleware::JwtAuth;

pub fn configure_routes(cfg: &mut web::ServiceConfig, jwt_auth: JwtAuth) {
    cfg.service(
        web::scope("/api")
            .wrap(jwt_auth)
            .service(
                web::scope("/stations")
                    .route("/nearby", web::get().to(handlers::get_nearby_stations))
            )
            .service(
                web::scope("/reviews")
                    .route("", web::post().to(handlers::create_review))
                    .route("/station/{station_id}", web::get().to(handlers::get_station_reviews))
                    .route("/{review_id}", web::put().to(handlers::update_review))
                    .route("/{review_id}", web::delete().to(handlers::delete_review))
            )
            .service(
                web::scope("/user")
                    .route("/info", web::get().to(handlers::get_user_info))
            )
    )
    .route("/health", web::get().to(handlers::health_check));
}