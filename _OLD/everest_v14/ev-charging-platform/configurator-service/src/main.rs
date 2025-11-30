use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;

mod health;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    shared::telemetry::init_telemetry();
    
    // Load configuration
    let config = shared::AppConfig::load().expect("Failed to load configuration");
    
    // Create database connection pool with connection limits
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.connection_string())
        .await
        .expect("Failed to create database connection pool");
    
    // Run database migrations
    println!("🔄 Running database migrations...");
    match sqlx::migrate!("./migrations")
        .run(&pool)
        .await {
        Ok(_) => println!("✅ Database migrations completed successfully"),
        Err(e) => {
            eprintln!("❌ Database migrations failed: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("🚀 Starting Configurator Service");
    println!("📍 Server: {}:{}", config.server.host, config.server.port);
    println!("🗄️ Database: {}", config.database.connection_string());
    println!("📚 API Documentation: http://{}:{}/api/docs/", config.server.host, config.server.port);
    
    HttpServer::new(move || {
        App::new()
            // Application state
            .app_data(web::Data::new(pool.clone()))
            
            // Configure CORS
            .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
            )
            
            // Configure routes
            .configure(health::configure)
            .configure(api::configure)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}