use utoipa::OpenApi;

// Import the handler functions and their request/response schemas
use crate::interfaces::http::health_handler::{HealthResponse, health_handler};
use crate::interfaces::http::register_handler::{RegisterRequest, RegisterResponse, register_handler};
use crate::interfaces::http::login_handler::{LoginRequest, LoginResponse, login_handler};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::http::health_handler::health_handler,
        crate::interfaces::http::register_handler::register_handler,
        crate::interfaces::http::login_handler::login_handler
    ),
    components(
        schemas(
            HealthResponse,
            RegisterRequest,
            RegisterResponse,
            LoginRequest,
            LoginResponse
        )
    ),
    tags(
        (name = "System", description = "System endpoints like health check"),
        (name = "Authentication", description = "User authentication endpoints")
    )
)]
pub struct ApiDoc;