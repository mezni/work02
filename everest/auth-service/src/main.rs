mod application;
mod domain;
mod infrastructure;
mod interfaces;

use crate::application::dto::auth_dto::{
    LoginRequest, LoginResponse, RefreshTokenRequest, ValidateTokenResponse,
};
use crate::application::dto::user_dto::{
    AssignRoleDto, CreateUserDto, CreateUserResponse, ErrorResponse, SuccessResponse, UserDto,
    UserRolesDto,
};
use crate::application::services::auth_service::AuthService;
use crate::application::services::user_service::UserService;
use crate::infrastructure::config::auth_config::AuthConfig;
use crate::infrastructure::config::keycloak_config::KeycloakConfig;
use crate::infrastructure::config::server_config::ServerConfig;
use crate::infrastructure::keycloak::client::KeycloakClient;
use crate::infrastructure::persistence::keycloak_user_repository::KeycloakUserRepository;
use crate::interfaces::routes::configure_routes;
use crate::interfaces::AppState;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use anyhow::Result;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        interfaces::handlers::auth_handlers::login,
        interfaces::handlers::auth_handlers::validate_token,
        interfaces::handlers::auth_handlers::refresh_token,
        interfaces::handlers::auth_handlers::logout,
        interfaces::handlers::user_handlers::create_user,
        interfaces::handlers::user_handlers::get_user,
        interfaces::handlers::user_handlers::get_user_by_username,
        interfaces::handlers::user_handlers::list_users,
        interfaces::handlers::user_handlers::enable_user,
        interfaces::handlers::user_handlers::disable_user,
        interfaces::handlers::user_handlers::delete_user,
        interfaces::handlers::user_handlers::assign_role,
        interfaces::handlers::user_handlers::get_user_roles,
    ),
    components(
        schemas(
            LoginRequest,
            LoginResponse,
            RefreshTokenRequest,
            ValidateTokenResponse,
            CreateUserDto,
            UserDto,
            AssignRoleDto,
            UserRolesDto,
            ErrorResponse,
            SuccessResponse,
            CreateUserResponse
        )
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Roles", description = "Role management endpoints")
    ),
    info(
        title = "Auth Service API",
        version = "1.0.0",
        description = "REST API for authentication and user management using Keycloak",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Auth Service");

    // Load configurations
    let keycloak_config = KeycloakConfig::from_env().map_err(|e| {
        error!("Failed to load Keycloak configuration: {}", e);
        e
    })?;

    let server_config = ServerConfig::from_env().map_err(|e| {
        error!("Failed to load server configuration: {}", e);
        e
    })?;

    let auth_config = AuthConfig::from_env().map_err(|e| {
        error!("Failed to load auth configuration: {}", e);
        e
    })?;

    info!("Configuration loaded successfully");
    info!("Keycloak URL: {}", keycloak_config.url);
    info!("Keycloak Realm: {}", keycloak_config.realm);
    info!("Server address: {}", server_config.address());

    // Initialize Keycloak client
    let keycloak_client = KeycloakClient::new(keycloak_config).await.map_err(|e| {
        error!("Failed to initialize Keycloak client: {}", e);
        e
    })?;

    // Initialize repository
    let user_repository = Arc::new(KeycloakUserRepository::new(Arc::new(
        keycloak_client.clone(),
    )));

    // Initialize application services
    let user_service = Arc::new(UserService::new(user_repository.clone()));
    let auth_service = Arc::new(AuthService::new(
        user_repository,
        Arc::new(keycloak_client),
        auth_config,
    ));

    // Create application state
    let app_state = web::Data::new(AppState::new(user_service, auth_service));

    let server_address = server_config.address();

    info!("Auth Service initialized successfully");
    info!(
        "Swagger UI available at: http://{}/swagger-ui/",
        server_address
    );
    info!(
        "API documentation at: http://{}/api-docs/openapi.json",
        server_address
    );

    // Start server
    info!("🚀 Server starting on http://{}", server_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(configure_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(&server_address)
    .map_err(|e| {
        error!("Failed to bind to address {}: {}", server_address, e);
        e
    })?
    .run()
    .await
    .map_err(|e| {
        error!("Server error: {}", e);
        e
    })?;

    Ok(())
}
