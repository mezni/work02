use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::http::health::health_checker_handler,
        crate::interfaces::http::handlers::get_users_handler
    ),
    components(
        schemas(
            crate::interfaces::http::health::Response,
            crate::interfaces::http::handlers::User
        )
    ),
    tags(
        (name = "Configurator Service", description = "Manage resources")
    )
)]
pub struct ApiDoc;
