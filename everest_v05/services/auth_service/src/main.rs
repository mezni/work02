mod config;
mod domain;
mod application;
mod infrastructure;
mod interfaces;

use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber;

use infrastructure::repositories::PostgresUserRepository;
use infrastructure::keycloak::KeycloakClient;
use application::services::UserService;
use interfaces::http::routes::configure_routes;
use interfaces::http::swagger::swagger_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::ServiceConfig::from_env();

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.postgres_url())
        .await
        .expect("Failed to create pool");

    // Initialize Keycloak client with your config
    let keycloak_config = infrastructure::keycloak::KeycloakConfig::from_service_config(&config);
    let keycloak_client = KeycloakClient::new(keycloak_config);

    // Initialize repositories and services
    let user_repository = PostgresUserRepository::new(pool);
    let user_service = UserService::new(user_repository, keycloak_client);

    // Start HTTP server
    println!("Starting server on {}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .configure(configure_routes)
            .service(swagger_config())
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}