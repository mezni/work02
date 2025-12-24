use utoipa::OpenApi;
use crate::application::dtos::health::HealthResponse;
use crate::presentation::controllers::health_controller;

#[derive(OpenApi)]
#[openapi(
    paths(health_controller::health_check),
    components(schemas(HealthResponse)),
    tags(
        (name = "Health", description = "Health check endpoints")
    )
)]
pub struct ApiDoc;