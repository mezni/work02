use actix_web::web;
use crate::interfaces::http::handlers::create_user;

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
}
