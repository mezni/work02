use actix_web::{get, web, HttpResponse, Responder};
use crate::application::health_service::HealthService;
use crate::application::dtos::health::HealthResponse;
use crate::AppState;

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
#[get("/health")]
pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let health = HealthService::check_health(&data.db_pool).await;
    HttpResponse::Ok().json(health)
}