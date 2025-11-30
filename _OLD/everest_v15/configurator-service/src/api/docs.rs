use actix_web::{HttpResponse, Responder, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Import all components needed for the OpenAPI definition
use crate::api::organizations::{
    CreateOrganizationRequest, ErrorResponse, OrganizationListResponse, OrganizationResponse,
};
use crate::health::{HealthStatus, ReadinessStatus};

/// EV Charging Configurator API documentation
#[derive(OpenApi)]
#[openapi(
    info(
        title = "EV Charging Configurator API",
        version = "1.0.0",
        description = "API for managing EV charging stations, organizations, and users",
        contact(
            name = "EV Charging Platform",
            url = "https://ev-charging.example.com",
            email = "support@ev-charging.example.com"
        ),
        license(
            name = "Apache 2.0",
            url = "https://www.apache.org/licenses/LICENSE-2.0"
        )
    ),
    // Use module paths instead of function names
    paths(
        crate::health::health_check,
        crate::health::readiness_check,
        crate::api::organizations::create_organization,
        crate::api::organizations::get_organizations,
        crate::api::organizations::get_organization
    ),
    components(
        schemas(
            HealthStatus,
            ReadinessStatus,
            CreateOrganizationRequest,
            OrganizationResponse,
            OrganizationListResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Organizations", description = "Organization management endpoints")
    ),
    servers(
        (url = "/", description = "Local development server"),
        (url = "https://api.ev-charging.example.com", description = "Production server")
    )
)]
struct ApiDoc;

// Service handler for the raw JSON file
#[actix_web::get("/api-docs/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    // Generate and serve the specification JSON
    match ApiDoc::openapi().to_json() {
        Ok(json) => HttpResponse::Ok()
            .content_type("application/json")
            .body(json),
        Err(e) => {
            eprintln!("Failed to generate OpenAPI JSON: {}", e);
            HttpResponse::InternalServerError().body("Failed to generate OpenAPI specification")
        }
    }
}

// Configuration function for the raw JSON file handler
pub fn configure_json(cfg: &mut web::ServiceConfig) {
    cfg.service(openapi_json);
}

// Configuration function for the Swagger UI interface
pub fn configure_ui(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/docs/{_:.*}")
            // Specify the absolute path where the JSON file is served
            .url("/api/v1/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
