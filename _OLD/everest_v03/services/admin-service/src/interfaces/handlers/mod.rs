pub mod network_handlers;
pub mod station_handlers;
pub mod charger_handlers;
pub mod connector_handlers;

pub use network_handlers::*;
pub use station_handlers::*;
pub use charger_handlers::*;
pub use connector_handlers::*;

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use crate::application::dto::HealthCheckResponse;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}