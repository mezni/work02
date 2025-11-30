// configurator-service/src/main.rs
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod api;
mod health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    shared::telemetry::init_telemetry();

    // Load configuration
    let config = shared::AppConfig::load().expect("Failed to load configuration");

    // Create database connection pool
    let pool = sqlx::postgres::PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");

    println!("🚀 Starting Configurator Service");
    println!("📍 Server: {}:{}", config.server.host, config.server.port);
    println!(
        "📚 API Documentation: http://{}:{}/api/docs/",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            // Application state
            .app_data(web::Data::new(pool.clone()))
            // Configure CORS
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            // Configure routes
            .configure(health::configure) // Root health endpoints: /health, /ready
            .configure(api::configure) // API endpoints: /api/v1/* and /api/docs/*
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
