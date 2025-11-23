// src/api/handlers/health_handler.rs
use actix_web::{HttpResponse, web};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone)]
pub struct HealthHandler;

impl HealthHandler {
    pub fn new() -> Self {
        Self
    }

    #[utoipa::path(
        get,
        path = "/health",
        responses(
            (status = 200, description = "Service is healthy", body = HealthResponse)
        ),
        tag = "health"
    )]
    pub async fn health_check(&self) -> HttpResponse {
        HttpResponse::Ok().json(HealthResponse {
            status: "ok".to_string(),
            timestamp: chrono::Utc::now(),
        })
    }

    #[utoipa::path(
        get,
        path = "/health/ready",
        responses(
            (status = 200, description = "Service is ready", body = HealthResponse),
            (status = 503, description = "Service is not ready")
        ),
        tag = "health"
    )]
    pub async fn readiness_check(&self) -> HttpResponse {
        // Add readiness checks here (database, Keycloak, etc.)
        HttpResponse::Ok().json(HealthResponse {
            status: "ready".to_string(),
            timestamp: chrono::Utc::now(),
        })
    }

    #[utoipa::path(
        get,
        path = "/health/live",
        responses(
            (status = 200, description = "Service is live", body = HealthResponse)
        ),
        tag = "health"
    )]
    pub async fn liveness_check(&self) -> HttpResponse {
        HttpResponse::Ok().json(HealthResponse {
            status: "live".to_string(),
            timestamp: chrono::Utc::now(),
        })
    }
}

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    status: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

// Helper function for 404 handler
pub async fn not_found_handler() -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({
        "error": "not_found",
        "message": "The requested resource was not found"
    }))
}