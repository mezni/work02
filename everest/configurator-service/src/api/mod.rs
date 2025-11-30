// api/mod.rs
use actix_web::web;

pub mod docs;

// This configure function will only handle the UI, which sits at the root (e.g., /docs)
pub fn configure(cfg: &mut web::ServiceConfig) {
    // Registers the Swagger UI interface
    cfg.configure(docs::configure_ui);
}
