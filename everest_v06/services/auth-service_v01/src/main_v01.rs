use actix_web::{web, App, HttpServer, middleware::Logger};
use tracing_subscriber;
use dotenvy::dotenv;

use auth_service::{
    infrastructure::{
        config::Settings,
        database::connection::Database,
        auth::{KeycloakClient, JwtService},
        audit::Auditor,
        cache::CacheManager,
    },
    interfaces::{
        configure_routes,
        openapi::spec::configure_swagger,
        controllers::{
            AuthController, UserController, CompanyController, AuditController
        },
    },
    application::handlers::{
        command_handlers::{AuthCommandHandler, UserCommandHandler, CompanyCommandHandler},
        query_handlers::{UserQueryHandler, CompanyQueryHandler, AuditQueryHandler},
    },
    domain::repositories::{
        UserRepository, CompanyRepository, AuditRepository
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let settings = Settings::new()
        .expect("Failed to load configuration");

    // Initialize database
    let database = Database::new(&settings.database_url())
        .await
        .expect("Failed to connect to database");

    // Run migrations
    database.run_migrations()
        .await
        .expect("Failed to run database migrations");

    // Initialize infrastructure components
    let keycloak_client = KeycloakClient::new(&settings);
    let jwt_service = JwtService::new(&settings);
    let cache_manager = CacheManager::new();

    // Initialize repositories
    let user_repository = auth_service::infrastructure::database::repositories::UserRepositoryImpl::new(database.pool().clone());
    let company_repository = auth_service::infrastructure::database::repositories::CompanyRepositoryImpl::new(database.pool().clone());
    let audit_repository = auth_service::infrastructure::database::repositories::AuditRepositoryImpl::new(database.pool().clone());

    // Initialize auditor
    let auditor = Auditor::new(audit_repository.clone());

    // Initialize command handlers
    let auth_handler = AuthCommandHandler::new(
        user_repository.clone(),
        keycloak_client.clone(),
        jwt_service.clone(),
        auditor.clone(),
    );

    let user_handler = UserCommandHandler::new(
        user_repository.clone(),
        company_repository.clone(),
        keycloak_client.clone(),
        auditor.clone(),
    );

    let company_handler = CompanyCommandHandler::new(
        company_repository.clone(),
        user_repository.clone(),
        auditor.clone(),
    );

    // Initialize query handlers
    let user_query_handler = UserQueryHandler::new(user_repository.clone());
    let company_query_handler = CompanyQueryHandler::new(company_repository.clone(), user_repository.clone());
    let audit_query_handler = AuditQueryHandler::new(audit_repository.clone(), user_repository.clone());

    // Initialize controllers
    let auth_controller = AuthController::new(auth_handler, user_query_handler.clone());
    let user_controller = UserController::new(user_handler, user_query_handler.clone());
    let company_controller = CompanyController::new(company_handler, company_query_handler, user_query_handler.clone());
    let audit_controller = AuditController::new(audit_query_handler);

    // Start HTTP server
    let server_address = settings.server_address();
    println!("Starting server on {}", server_address);

    HttpServer::new(move || {
        App::new()
            // Add middleware
            .wrap(Logger::default())
            // Add application data
            .app_data(web::Data::new(auth_controller.clone()))
            .app_data(web::Data::new(user_controller.clone()))
            .app_data(web::Data::new(company_controller.clone()))
            .app_data(web::Data::new(audit_controller.clone()))
            .app_data(web::Data::new(jwt_service.clone()))
            .app_data(web::Data::new(cache_manager.clone()))
            // Configure routes
            .configure(configure_routes)
            // Configure Swagger UI
            .service(configure_swagger())
            // Health check endpoint
            .service(
                web::resource("/health")
                    .route(web::get().to(|| async { "OK" }))
            )
    })
    .bind(&server_address)?
    .run()
    .await
}