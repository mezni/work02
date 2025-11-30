use actix_web::web;

pub mod docs;
pub mod v1;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/v1").configure(v1::configure))
            .service(web::scope("/docs").configure(docs::configure)),
    );
}
