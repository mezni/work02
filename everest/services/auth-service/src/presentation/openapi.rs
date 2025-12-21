use crate::application::health_dto::HealthResponseDto;
use crate::presentation::controllers::health_controller;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(health_controller::get_health),
    components(schemas(HealthResponseDto))
)]
pub struct ApiDoc;

// ENSURE THIS IS "pub fn"
pub fn configure_openapi(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
