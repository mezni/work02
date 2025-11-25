use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::interfaces::http::dto::{
    CreateUserRequest, LoginRequest, UserResponse, LoginResponse, ErrorResponse
};
use crate::domain::enums::UserRole;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::http::handlers::create_user,
        crate::interfaces::http::handlers::login,
        crate::interfaces::http::handlers::get_user,
    ),
    components(
        schemas(
            CreateUserRequest,
            LoginRequest,
            UserResponse,
            LoginResponse,
            ErrorResponse,
            UserRole
        )
    ),
    tags(
        (name = "auth", description = "Authentication and user management API")
    )
)]
pub struct ApiDoc;

pub fn swagger_config() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}