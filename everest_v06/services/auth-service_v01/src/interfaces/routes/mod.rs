pub mod user_routes;
pub mod company_routes;
pub mod auth_routes;
pub mod audit_routes;

pub use user_routes::configure_user_routes;
pub use company_routes::configure_company_routes;
pub use auth_routes::configure_auth_routes;
pub use audit_routes::configure_audit_routes;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(configure_auth_routes)
            .configure(configure_user_routes)
            .configure(configure_company_routes)
            .configure(configure_audit_routes)
    );
}