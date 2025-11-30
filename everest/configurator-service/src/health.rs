// configurator-service/src/health.rs
use actix_web::{HttpResponse, Responder, get, web};
use serde::Serialize;
use sqlx::{PgPool, Row};
use utoipa::ToSchema;

const API_PREFIX: &str = "/api/v1";

#[derive(Serialize, ToSchema)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: String,
}

#[derive(Serialize, ToSchema)]
pub struct ReadinessStatus {
    pub status: String,
    pub database: String,
    pub timestamp: String,
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthStatus)
    ),
    tag = "health"
)]
#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthStatus {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, description = "Service is ready", body = ReadinessStatus),
        (status = 503, description = "Service is not ready", body = ReadinessStatus)
    ),
    tag = "health"
)]
#[get("/ready")]
pub async fn readiness_check(pool: web::Data<PgPool>) -> impl Responder {
    let timestamp = chrono::Utc::now().to_rfc3339();

    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(ReadinessStatus {
            status: "ready".to_string(),
            database: "connected".to_string(),
            timestamp,
        }),
        Err(_) => HttpResponse::ServiceUnavailable().json(ReadinessStatus {
            status: "not_ready".to_string(),
            database: "disconnected".to_string(),
            timestamp,
        }),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(API_PREFIX)
            .service(health_check)
            .service(readiness_check),
    );
}
