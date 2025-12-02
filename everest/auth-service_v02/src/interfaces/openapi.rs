// src/interfaces/openapi.rs
use utoipa::OpenApi;
use crate::interfaces::dto::RegisterRequest;

// Create inline documentation functions
mod api_docs {
    use crate::interfaces::dto::RegisterRequest;
    
    #[utoipa::path(
        get,
        path = "/health",
        responses(
            (status = 200, description = "Health check endpoint")
        )
    )]
    pub fn health_doc() {}
    
    #[utoipa::path(
        post,
        path = "/users/register",
        request_body = RegisterRequest,
        responses(
            (status = 200, description = "User registered successfully")
        )
    )]
    pub fn register_user_doc(_body: RegisterRequest) {}
}

#[derive(OpenApi)]
#[openapi(
    paths(
        api_docs::health_doc,
        api_docs::register_user_doc
    ),
    components(
        schemas(RegisterRequest)
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;