use actix_web::web;
use crate::config::Config;
use crate::middleware::JwtMiddleware;
use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Get config from app data
    let config = cfg.app_data::<actix_web::web::Data<Config>>()
        .expect("Config not found")
        .get_ref();

    // Create JWT middleware for protected routes
    let auth_middleware = JwtMiddleware::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
    );

    cfg.service(
        web::scope("")
            // Public routes
            .route("/health", web::get().to(handlers::health_check))
            .route("/stations/nearby", web::get().to(handlers::find_nearby_stations))
            .route("/stations/{station_id}/reviews", web::get().to(handlers::get_station_reviews))
            // Protected routes (require authentication)
            .service(
                web::scope("/reviews")
                    .wrap(auth_middleware)
                    .route("", web::post().to(handlers::create_review))
                    .route("/my", web::get().to(handlers::get_my_reviews))
                    .route("/{review_id}", web::put().to(handlers::update_review))
                    .route("/{review_id}", web::delete().to(handlers::delete_review))
            )
    );
}