use utoipa::OpenApi;

// ===== HEALTH CHECK =====
use crate::interfaces::http::health_handler::{
    __path_health_handler,
    HealthResponse,
};

// ===== REGISTER =====
use crate::interfaces::http::register_handler::{
    __path_register_handler,
    RegisterRequest,
    RegisterResponse,
};

// ===== LOGIN =====
use crate::interfaces::http::login_handler::{
    __path_login_handler,
    LoginRequest,
    LoginResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health_handler,
        register_handler,
        login_handler
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
