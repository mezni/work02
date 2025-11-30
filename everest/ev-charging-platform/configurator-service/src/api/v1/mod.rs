use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
struct ApiHealth {
    status: String,
    version: String,
    timestamp: String,
}

async fn v1_health() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(ApiHealth {
        status: "healthy".to_string(),
        version: "v1".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

async fn v1_ready() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(ApiHealth {
        status: "ready".to_string(),
        version: "v1".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(web::resource("/health").route(web::get().to(v1_health)))
            .service(web::resource("/ready").route(web::get().to(v1_ready)))
    );
}