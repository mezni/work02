use auth_service::{
    infrastructure::config::Config,
    interfaces::routes::configure_routes,
    prelude::*,
};
use tracing_subscriber;

#[actix_web::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::init();
    
    // Load configuration
    let config = Config::load().expect("Failed to load configuration");
    
    // Create database pool
    let db_pool = auth_service::infrastructure::database::create_pool(&config.database)
        .await
        .expect("Failed to create database pool");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");
    
    // Configure and start server
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(configure_routes)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run();
    
    tracing::info!("Server running on {}:{}", config.server.host, config.server.port);
    server.await?;
    
    Ok(())
}
