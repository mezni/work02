use utoipa::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        ComponentsBuilder, OpenApiBuilder,
    },
    Modify, OpenApi,
};
use std::sync::Arc;

// OpenAPI documentation enhancements
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            // Add Bearer token security scheme
            components.add_security_scheme(
                "bearer_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
            
            // Add API key security scheme (if needed)
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(openapi::security::ApiKey::Header(
                    openapi::security::ApiKeyValue::new("X-API-Key"),
                )),
            );
        }
    }
}

pub struct TagsAddon;

impl Modify for TagsAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(paths) = openapi.paths.as_mut() {
            for (_, path_item) in paths.paths.iter_mut() {
                // You could add operation-specific tags here
                // based on the path pattern
            }
        }
    }
}

pub struct ExamplesAddon;

impl Modify for ExamplesAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            // Add example schemas
            if let Some(schemas) = components.schemas.as_mut() {
                // Add example for LoginRequest
                if let Some(login_schema) = schemas.get_mut("LoginRequest") {
                    if let openapi::schema::Schema::Object(ref mut obj) = login_schema {
                        obj.example = Some(
                            serde_json::json!({
                                "email": "user@example.com",
                                "password": "password123",
                                "remember_me": true
                            })
                            .into(),
                        );
                    }
                }
                
                // Add example for UserResponse
                if let Some(user_schema) = schemas.get_mut("UserResponse") {
                    if let openapi::schema::Schema::Object(ref mut obj) = user_schema {
                        obj.example = Some(
                            serde_json::json!({
                                "id": "550e8400-e29b-41d4-a716-446655440000",
                                "email": "user@example.com",
                                "role": "user",
                                "company_name": "Acme Inc",
                                "station_name": "Station 1",
                                "is_active": true,
                                "email_verified": true,
                                "created_at": "2024-01-01T00:00:00Z"
                            })
                            .into(),
                        );
                    }
                }
            }
        }
    }
}

// Custom OpenAPI builder with all enhancements
pub fn build_openapi() -> openapi::OpenApi {
    let openapi = crate::interfaces::swagger::ApiDoc::openapi();
    
    OpenApiBuilder::new()
        .openapi(openapi)
        .components(
            ComponentsBuilder::new()
                .security_scheme_from(
                    "bearer_token",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                )
                .build(),
        )
        .build()
}

// Helper to generate OpenAPI spec as JSON
pub fn generate_openapi_json() -> String {
    let openapi = build_openapi();
    serde_json::to_string_pretty(&openapi).unwrap()
}

// Health check schema for OpenAPI
#[derive(utoipa::ToSchema)]
pub struct HealthCheckSchema {
    pub status: String,
    pub service: String,
    pub timestamp: String,
}

#[derive(utoipa::ToSchema)]
pub struct DetailedHealthCheckSchema {
    pub status: String,
    pub services: Vec<HealthServiceSchema>,
    pub total_services: usize,
    pub healthy_services: usize,
    pub timestamp: String,
}

#[derive(utoipa::ToSchema)]
pub struct HealthServiceSchema {
    pub service: String,
    pub status: String,
    pub message: Option<String>,
    pub timestamp: String,
}