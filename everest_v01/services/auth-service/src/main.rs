use actix_web::{web, App, HttpRequest, HttpResponse, Responder};
use common_lib::*;

// Your service-specific handlers
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from microservice!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = config::AppConfig::from_env()
        .expect("Failed to load configuration");
    
    // Initialize logging
    logging::init_logging(&config.service_name, &config.environment);
    
    // Create database pool (if using database feature)
    #[cfg(feature = "database")]
    let pool = database::db::create_pool(
        &config.database.url,
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to create database pool");
    
    let server_config = config.server.clone();
    
    // Start server
    server::start_server(&server_config, move || {
        let mut app = App::new()
            // Add common middleware
            .wrap(middleware::RequestId)
            .wrap(middleware::RequestLogger)
            .wrap(actix_web::middleware::Compress::default())
            // Health endpoints
            .route("/health", web::get().to(telemetry::health_handler))
            .route("/ready", web::get().to(telemetry::ready_handler))
            // Your service-specific routes
            .route("/", web::get().to(hello));
        
        // Add database pool to app data if using database
        #[cfg(feature = "database")]
        {
            app = app.app_data(web::Data::new(pool.clone()));
        }
        
        app
    })
    .await
}