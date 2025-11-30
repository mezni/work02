fn main() {
    println!("Hello, world!");
}
use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use auth_service::{
    application::handlers::{UserCommandHandler, UserQueryHandler},
    infrastructure::{
        config::Config,
        keycloak::client::KeycloakClient,
        repositories::postgres_user_repository::PostgresUserRepository,
    },
    interfaces::{
        http::{handlers::UserHandlers, routes::configure_routes},
        api_doc::ApiDoc,
        middleware::logging::RequestLogging,
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "auth_service=info,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    info!("Starting Auth Service...");

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    info!("Configuration loaded successfully");

    // Setup database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await
        .expect("Failed to create database pool");

    info!("Database connection pool created");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    info!("Database migrations completed");

    // Initialize repositories
    let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));

    // Initialize Keycloak client
    let keycloak_client = Arc::new(KeycloakClient::new(config.keycloak.clone()));

    // Initialize handlers
    let command_handler = Arc::new(UserCommandHandler::new(user_repository.clone()));
    let query_handler = Arc::new(UserQueryHandler::new(user_repository.clone()));

    let user_handlers = web::Data::new(UserHandlers::new(
        command_handler,
        query_handler,
        keycloak_client,
    ));

    // Generate OpenAPI documentation
    let openapi = ApiDoc::openapi();

    let server_host = config.server.host.clone();
    let server_port = config.server.port;

    info!("Starting HTTP server at {}:{}", server_host, server_port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Middleware
            .wrap(middleware::Logger::default())
            .wrap(RequestLogging)
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            // Data
            .app_data(user_handlers.clone())
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            // Routes
            .configure(configure_routes::<PostgresUserRepository>)
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
