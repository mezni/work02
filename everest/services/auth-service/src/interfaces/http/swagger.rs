use crate::application::health_service::HealthReport;
use crate::core::constants::APP_VERSION;
use crate::interfaces::http::health_handler;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(health_handler::get_health),
    components(schemas(HealthReport)),
    info(
        title = "Modern Rust API",
        version = APP_VERSION, // Dynamic version in Swagger
        description = "DDD structured Actix-web API"
    ),
    tags(
        (name = "System", description = "Diagnostic endpoints")
    )
)]
pub struct ApiDoc;
