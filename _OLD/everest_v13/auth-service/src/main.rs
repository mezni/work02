mod application;
mod domain;
mod infrastructure;
mod interfaces;

use crate::application::dto::auth_dto::{
    LoginRequest, LoginResponse, RefreshTokenRequest, ValidateTokenResponse,
};
use crate::application::dto::user_dto::{
    AssignRoleDto, CreateUserDto, CreateUserResponse, UserDto, UserRolesDto,
};
use crate::application::services::{
    AuthService, UserService, OrganisationService, AuditService, RoleRequestService
};
use crate::domain::entities::{
    organisation::Organisation,
    audit_log::{AuditLog, AuditLogQuery},
    role_request::{RoleRequest, CreateRoleRequest, ReviewRoleRequest},
};
use crate::domain::repositories::{
    organisation_repository::OrganisationRepository,
    audit_log_repository::AuditLogRepository,
    role_request_repository::RoleRequestRepository,
    user_repository::RepositoryError,
};
use crate::domain::value_objects::OrganisationId;
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
use async_trait::async_trait;
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
        interfaces::handlers::organisation_handlers::create_organisation,
        interfaces::handlers::organisation_handlers::list_organisations,
        interfaces::handlers::organisation_handlers::get_organisation,
        interfaces::handlers::organisation_handlers::update_organisation,
        interfaces::handlers::organisation_handlers::delete_organisation,
        interfaces::handlers::organisation_handlers::assign_user_to_organisation,
        interfaces::handlers::audit_handlers::get_audit_logs,
        interfaces::handlers::audit_handlers::export_audit_logs,
        interfaces::handlers::role_request_handlers::create_role_request,
        interfaces::handlers::role_request_handlers::list_role_requests,
        interfaces::handlers::role_request_handlers::review_role_request,
        interfaces::handlers::role_request_handlers::get_user_role_requests,
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
            application::dto::organisation_dto::CreateOrganisationDto,
            application::dto::organisation_dto::UpdateOrganisationDto,
            application::dto::organisation_dto::OrganisationDto,
            application::dto::organisation_dto::OrganisationResponse,
            application::dto::organisation_dto::AssignUserToOrganisationDto,
            application::dto::audit_dto::AuditLogDto,
            application::dto::audit_dto::AuditQueryDto,
            application::dto::role_request_dto::CreateRoleRequestDto,
            application::dto::role_request_dto::ReviewRoleRequestDto,
            application::dto::role_request_dto::RoleRequestDto,
            application::dto::user_dto::ErrorResponse,
            application::dto::user_dto::SuccessResponse,
            CreateUserResponse
        )
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Organisations", description = "Organisation management endpoints"),
        (name = "Audit", description = "Audit logging endpoints"),
        (name = "Role Requests", description = "Role request management endpoints")
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

// Placeholder repository implementations for compilation
struct MockOrganisationRepository;
impl MockOrganisationRepository {
    fn new() -> Self { Self }
}
#[async_trait]
impl OrganisationRepository for MockOrganisationRepository {
    async fn create(&self, _organisation: &Organisation) -> Result<OrganisationId, RepositoryError> {
        Ok(OrganisationId::new(1))
    }
    async fn find_by_id(&self, _id: &OrganisationId) -> Result<Option<Organisation>, RepositoryError> {
        Ok(Some(Organisation::new("Test".to_string(), "Test".to_string())))
    }
    async fn find_all(&self) -> Result<Vec<Organisation>, RepositoryError> {
        Ok(vec![])
    }
    async fn update(&self, _organisation: &Organisation) -> Result<(), RepositoryError> {
        Ok(())
    }
    async fn delete(&self, _id: &OrganisationId) -> Result<(), RepositoryError> {
        Ok(())
    }
    async fn find_live_organisations(&self) -> Result<Vec<Organisation>, RepositoryError> {
        Ok(vec![])
    }
}

struct MockAuditLogRepository;
impl MockAuditLogRepository {
    fn new() -> Self { Self }
}
#[async_trait]
impl AuditLogRepository for MockAuditLogRepository {
    async fn create(&self, _audit_log: &AuditLog) -> Result<i32, RepositoryError> {
        Ok(1)
    }
    async fn query(&self, _query: &AuditLogQuery) -> Result<Vec<AuditLog>, RepositoryError> {
        Ok(vec![])
    }
    async fn export(&self, _query: &AuditLogQuery) -> Result<Vec<u8>, RepositoryError> {
        Ok(vec![])
    }
}

struct MockRoleRequestRepository;
impl MockRoleRequestRepository {
    fn new() -> Self { Self }
}
#[async_trait]
impl RoleRequestRepository for MockRoleRequestRepository {
    async fn create(&self, _request: &CreateRoleRequest) -> Result<i32, RepositoryError> {
        Ok(1)
    }
    async fn find_by_id(&self, _id: i32) -> Result<Option<RoleRequest>, RepositoryError> {
        Ok(None)
    }
    async fn find_by_user_id(&self, _user_id: &str) -> Result<Vec<RoleRequest>, RepositoryError> {
        Ok(vec![])
    }
    async fn find_pending_requests(&self) -> Result<Vec<RoleRequest>, RepositoryError> {
        Ok(vec![])
    }
    async fn update(&self, _id: i32, _review: &ReviewRoleRequest, _reviewed_by: &str) -> Result<(), RepositoryError> {
        Ok(())
    }
}

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
        user_repository.clone(),
        Arc::new(keycloak_client.clone()),
        auth_config,
    ));

    // Create placeholder services for now
    let organisation_service = Arc::new(OrganisationService::new(
        Arc::new(MockOrganisationRepository::new()),
        user_repository.clone(),
    ));

    let audit_service = Arc::new(AuditService::new(
        Arc::new(MockAuditLogRepository::new()),
    ));

    let role_request_service = Arc::new(RoleRequestService::new(
        Arc::new(MockRoleRequestRepository::new()),
        user_repository.clone(),
    ));

    // Create application state with all services
    let app_state = web::Data::new(AppState::new(
        user_service,
        auth_service,
        organisation_service,
        audit_service,
        role_request_service,
    ));

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