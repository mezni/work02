use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::handlers::health_check,
    ),
    components(
        schemas(
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User authentication and registration"),
        (name = "Users", description = "User management")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT token from Keycloak authentication"))
                        .build(),
                ),
            );
        }

        openapi.info.title = "BorneMap Auth Service API".to_string();
        openapi.info.version = "1.0.0".to_string();
        openapi.info.description = Some(
            "REST API for user authentication and management using Keycloak\n\n\
            "
            .to_string(),
        );
    }
}
