use utoipa::OpenApi;

use crate::interfaces::http::health::Response;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::http::health::health_checker_handler
    ),
    components(
        schemas(Response)
    ),
    tags(
        (name = "Configurator Service", description = "Manage resources")
    )
)]
pub struct ApiDoc;
