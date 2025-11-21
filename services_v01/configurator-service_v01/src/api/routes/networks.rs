// src/api/routes/networks.rs
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/networks"), // Routes will be added here
    );
}
