// src/api/routes/health_routes.rs
use actix_web::web;
use crate::api::handlers::HealthHandler;

pub fn configure_health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/health", web::get().to(HealthHandler::health_check))
            .route("/health/ready", web::get().to(HealthHandler::readiness_check))
            .route("/health/live", web::get().to(HealthHandler::liveness_check)),
    );
}