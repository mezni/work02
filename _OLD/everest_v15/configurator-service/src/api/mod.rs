// configurator-service/src/api/mod.rs
use actix_web::web;

pub mod docs;
pub mod organizations;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Configure Swagger UI at /docs
    cfg.configure(docs::configure_ui);

    // Configure organizations endpoints under /api/v1
    cfg.service(web::scope("/api/v1").configure(organizations::configure));
}
