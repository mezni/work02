use crate::api::controllers::*;
use actix_web::web;

pub fn config_network_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/networks")
            .route("", web::post().to(create_network))
            .route("", web::get().to(list_networks))
            .route("/{network_id}", web::get().to(get_network))
            .route("/{network_id}", web::delete().to(delete_network))
            .route("/{network_id}/verify", web::post().to(verify_network)),
    );
}
