use utoipa::OpenApi;
use crate::application::dto::{CreateUserRequest, UserResponse, LoginRequest, EnrichedTokenResponse, LoginResponse};
use crate::domain::value_objects::Role;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::http::handlers::create_user::<crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository>,
        crate::interfaces::http::handlers::login::<crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository>,
        crate::interfaces::http::handlers::get_user::<crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository>,
        crate::interfaces::http::handlers::list_users_by_organisation::<crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository>,
        crate::interfaces::http::handlers::health_check,
    ),
    components(
        schemas(CreateUserRequest, UserResponse, LoginRequest, EnrichedTokenResponse, LoginResponse, Role)
    ),
    tags(
        (name = "auth-service", description = "Authentication and user management API")
    ),
    info(
        title = "Auth Service API",
        version = "1.0.0",
        description = "DDD-based authentication microservice with Keycloak integration"
    )
)]
pub struct ApiDoc;