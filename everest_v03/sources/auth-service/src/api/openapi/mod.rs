// src/api/openapi/mod.rs
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use actix_web::web;

use crate::application::{
    CreateUserCommand, 
    UpdateUserCommand, 
    DeactivateUserCommand,
    UserDto,
};
use crate::api::handlers::{
    user_handler::GetUsersQueryParams,
    health_handler::HealthResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::user_handler::UserHandler::create_user,
        crate::api::handlers::user_handler::UserHandler::get_users,
        crate::api::handlers::user_handler::UserHandler::get_user_by_id,
        crate::api::handlers::user_handler::UserHandler::update_user,
        crate::api::handlers::admin_handler::AdminHandler::deactivate_user,
        crate::api::handlers::admin_handler::AdminHandler::get_users_admin,
        crate::api::handlers::health_handler::HealthHandler::health_check,
        crate::api::handlers::health_handler::HealthHandler::readiness_check,
        crate::api::handlers::health_handler::HealthHandler::liveness_check,
    ),
    components(
        schemas(
            CreateUserCommand,
            UpdateUserCommand,
            DeactivateUserCommand,
            UserDto,
            crate::application::UserListDto,
            GetUsersQueryParams,
            HealthResponse,
        )
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "admin", description = "Administrative endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_token",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new("bearer")
                )
            )
        }
    }
}

pub fn configure_swagger(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", openapi.clone()),
    );
}