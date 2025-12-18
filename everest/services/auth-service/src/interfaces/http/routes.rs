use crate::core::constants::API_PREFIX;
use crate::interfaces::http::health_handler;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(API_PREFIX).service(health_handler::get_health));
}
