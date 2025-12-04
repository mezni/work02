use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "success")]
    pub status: &'static str,

    #[schema(example = "API is healthy")]
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "API health status", body = HealthResponse)
    ),
    tag = "System"
)]
#[get("/api/v1/health")]
pub async fn health_handler() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "success",
        message: "API is healthy".to_string(),
    })
}
