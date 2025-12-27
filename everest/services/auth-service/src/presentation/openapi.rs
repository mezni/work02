use utoipa::OpenApi;

use crate::application::dtos::health::HealthResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::presentation::controllers::health_controller::health_check
    ),
    components(
        schemas(HealthResponse)
    ),
    tags(
        (name = "System", description = "System and health endpoints")
    )
)]
pub struct ApiDoc;
