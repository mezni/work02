use actix_web::{web, HttpResponse};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::{Config, SwaggerUi};

// Import the entire modules
use crate::api::v1::organizations;
use crate::health;

#[derive(Serialize, ToSchema)]
struct ApiInfo {
    name: String,
    version: String,
    description: String,
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "EV Charging Configurator API",
        version = "1.0.0",
        description = "API for managing EV charging stations, organizations, and users"
    ),
    paths(
        health::health_check,
        health::readiness_check,
        organizations::create_organization,
    ),
    components(
        schemas(
            health::HealthStatus,
            health::ReadinessStatus,
            organizations::CreateOrganizationRequest,
            organizations::OrganizationResponse,
            ApiInfo
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Organizations", description = "Organization management endpoints"),
        (name = "API", description = "API information")
    )
)]
pub struct ApiDoc;

// Serve OpenAPI JSON at the correct path
async fn serve_openapi_json() -> HttpResponse {
    match ApiDoc::openapi().to_json() {
        Ok(json) => HttpResponse::Ok()
            .content_type("application/json")
            .body(json),
        Err(e) => {
            eprintln!("❌ Failed to generate OpenAPI JSON: {}", e);
            HttpResponse::InternalServerError()
                .body(format!("Failed to generate OpenAPI JSON: {}", e))
        }
    }
}

// API info endpoint
async fn api_info() -> HttpResponse {
    HttpResponse::Ok().json(ApiInfo {
        name: "EV Charging Configurator API".to_string(),
        version: "1.0.0".to_string(),
        description: "API for managing EV charging stations, organizations, and users".to_string(),
    })
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Create Swagger UI configuration with the correct URL
    let config = Config::new(["/api/docs/openapi.json"]);

    cfg.service(
        web::scope("")
            // Serve OpenAPI JSON at /api/docs/openapi.json
            .service(web::resource("/openapi.json").route(web::get().to(serve_openapi_json)))
            .service(web::resource("/info").route(web::get().to(api_info)))
            // Configure Swagger UI to look for OpenAPI JSON at the correct absolute path
            .service(
                SwaggerUi::new("/{_:.*}")
                    .config(config)
                    .url("/api/docs/openapi.json", ApiDoc::openapi()),
            ),
    );
}
