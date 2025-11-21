// src/api/routes/mod.rs
pub mod networks;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(networks::config)
    );
}
