use actix_web::{HttpResponse, Responder, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Explicitly import all components needed for the OpenAPI definition
use crate::health::{HealthStatus, ReadinessStatus, health_check, readiness_check};

/// EV Charging Configurator API documentation
#[derive(OpenApi)]
#[openapi(
    info(
        title = "EV Charging Configurator API",
        version = "1.0.0",
        description = "API for managing EV charging stations, organizations, and users"
    ),
    // ✅ Include the path functions here to generate the specification metadata
    paths(
        crate::health::health_check,
        crate::health::readiness_check,
        // crate::api::organizations::create_organization,
    ),
    components(
        schemas(
            HealthStatus,
            ReadinessStatus,
            // CreateOrganizationRequest,
            // OrganizationResponse
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        // (name = "Organizations", description = "Organization management endpoints")
    )
)]
struct ApiDoc;

// 1. Service handler for the raw JSON file
// This function will be registered under the /api/v1 scope in main.rs
#[actix_web::get("/api-docs/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    // Generate and serve the specification JSON
    HttpResponse::Ok().json(ApiDoc::openapi())
}

// 2. Configuration function for the raw JSON file handler
// This is called by `main.rs` and placed in the /api/v1 scope.
pub fn configure_json(cfg: &mut web::ServiceConfig) {
    cfg.service(openapi_json);
}

// 3. Configuration function for the Swagger UI interface
// This is called by `api/mod.rs` and placed in the root scope.
pub fn configure_ui(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/docs/{_:.*}")
            // Specify the absolute path where the JSON file is served
            .url("/api/v1/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
