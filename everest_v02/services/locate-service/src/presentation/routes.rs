use actix_web::web;
use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/health", web::get().to(handlers::health_check))
            .service(
                web::scope("/stations")
                    .route("", web::get().to(handlers::get_stations))
                    .route("/{station_id}", web::get().to(handlers::get_station_by_id))
                    .route("/{station_id}/details", web::get().to(handlers::get_station_with_reviews))
                    .route("/{station_id}/reviews", web::get().to(handlers::get_station_reviews))
                    .route("/{station_id}/reviews", web::post().to(handlers::create_review))
            )
    );
}