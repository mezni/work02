use actix_web::{web, App, HttpServer};
use tracing::{info, error};
use tracing_subscriber;

use auth_service::{
    application::{
        command_handlers::{AuthCommandHandler, UserCommandHandler, CompanyCommandHandler},
        queries_handlers::{UserQueryHandler, CompanyQueryHandler, AuditQueryHandler},
        services::AuditService,
    },
    domain::repositories::{
        UserRepository, CompanyRepository, AuditRepository,
        UserRepositoryImpl, CompanyRepositoryImpl, AuditRepositoryImpl,
    },
    infrastructure::{
        auth::{KeycloakClient, JwtService},
        audit::AuditorImpl,
        config::Settings,
        database::DatabasePool,
    },
    interfaces::{
        routes::configure_routes,
        openapi::ApiDoc,
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting Auth Service...");

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");
    
    info!("Configuration loaded successfully");
    info!("Database URL: {}", settings.database_url());
    info!("Server address: {}", settings.server_address());

    // Initialize database pool
    let database_pool = DatabasePool::new(&settings.database_url())
        .await
        .expect("Failed to create database pool");

    // Initialize repositories
    let user_repository: UserRepositoryImpl = UserRepositoryImpl::new(database_pool.clone());
    let company_repository: CompanyRepositoryImpl = CompanyRepositoryImpl::new(database_pool.clone());
    let audit_repository: AuditRepositoryImpl = AuditRepositoryImpl::new(database_pool.clone());

    // Initialize infrastructure services
    let keycloak_client = KeycloakClient::new(&settings);
    let jwt_service = JwtService::new(&settings);
    let auditor = AuditorImpl::new(audit_repository.clone());

    // Initialize command handlers
    let auth_command_handler = AuthCommandHandler::new(
        user_repository.clone(),
        keycloak_client.clone(),
        jwt_service.clone(),
        auditor.clone(),
    );

    let user_command_handler = UserCommandHandler::new(
        user_repository.clone(),
        company_repository.clone(),
        keycloak_client.clone(),
        auditor.clone(),
    );

    let company_command_handler = CompanyCommandHandler::new(
        company_repository.clone(),
        user_repository.clone(),
    );

    // Initialize query handlers
    let user_query_handler = UserQueryHandler::new(user_repository.clone());
    let company_query_handler = CompanyQueryHandler::new(company_repository.clone(), user_repository.clone());
    let audit_query_handler = AuditQueryHandler::new(audit_repository.clone(), user_repository.clone());

    // Initialize services
    let audit_service = AuditService::new(audit_repository.clone(), settings.audit.retention_days);

    // Create OpenAPI spec
    let openapi_spec = ApiDoc::openapi();

    info!("Starting HTTP server on {}", settings.server_address());

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Register application data
            .app_data(web::Data::new(auth_command_handler.clone()))
            .app_data(web::Data::new(user_command_handler.clone()))
            .app_data(web::Data::new(company_command_handler.clone()))
            .app_data(web::Data::new(user_query_handler.clone()))
            .app_data(web::Data::new(company_query_handler.clone()))
            .app_data(web::Data::new(audit_query_handler.clone()))
            .app_data(web::Data::new(audit_service.clone()))
            .app_data(web::Data::new(jwt_service.clone()))
            .app_data(web::Data::new(keycloak_client.clone()))
            .app_data(web::Data::new(auditor.clone()))
            // Configure routes
            .configure(configure_routes)
            // Add Swagger UI
            .service(
                utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi_spec.clone()),
            )
            // Add health check endpoint
            .service(
                web::resource("/health")
                    .route(web::get().to(|| async { "Auth Service is healthy!" }))
            )
            // Add metrics endpoint (if you have metrics)
            .service(
                web::resource("/metrics")
                    .route(web::get().to(|| async { "Metrics endpoint" }))
            )
    })
    .bind(settings.server_address())?
    .workers(4) // Configure number of workers
    .run()
    .await
}