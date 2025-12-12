use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn health_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

pub async fn ready_handler() -> impl Responder {
    // Add actual readiness checks here (database, dependencies, etc.)
    HttpResponse::Ok().json(json!({
        "status": "ready",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}