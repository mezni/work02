use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

mod health;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    shared::telemetry::init_telemetry();
    
    // Load configuration
    let config = match shared::AppConfig::load() {
        Ok(config) => {
            println!("✅ Configuration loaded from file");
            config
        }
        Err(e) => {
            println!("⚠️ Using default configuration: {}", e);
            shared::AppConfig::default()
        }
    };
    
    // Create database connection pool
    let pool = match sqlx::postgres::PgPool::connect(&config.database.connection_string()).await {
        Ok(pool) => {
            println!("✅ Database connected successfully");
            pool
        }
        Err(e) => {
            println!("❌ Failed to connect to database: {}", e);
            println!("💡 Make sure PostgreSQL is running: docker-compose up -d postgres");
            std::process::exit(1);
        }
    };
    
    println!("🚀 Starting Configurator Service");
    println!("📍 Server: {}:{}", config.server.host, config.server.port);
    println!("📊 Health: http://{}:{}/health", config.server.host, config.server.port);
    println!("🔧 Ready: http://{}:{}/ready", config.server.host, config.server.port);
    println!("📚 API Docs: http://{}:{}/api/docs/", config.server.host, config.server.port);
    println!("🔌 Database: {}", config.database.connection_string());
    
    HttpServer::new(move || {
        App::new()
            // Application state
            .app_data(web::Data::new(pool.clone()))
            
            // Configure CORS
            .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
            )
            
            // Configure routes
            .configure(health::configure)  // Root health endpoints
            .configure(api::configure)     // API endpoints
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}