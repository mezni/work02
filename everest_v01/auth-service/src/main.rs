fn main() {
    println!("Hello, world!");
}
use auth_service::{
    config::AppConfig,
    logger,
    error::{AppError, Result},
    infrastructure::http_server::start_server,
    interfaces::http_routes::configure_routes,
};
use tracing::info;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize configuration
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
    
    // Initialize logging
    if let Err(e) = logger::init_logger(&config) {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }
    
    info!("Starting Auth Service v{}", env!("CARGO_PKG_VERSION"));
    info!("Environment: {}", config.environment);
    info!("Server: {}:{}", config.server.host, config.server.port);
    
    // Start the HTTP server
    match start_server(config, configure_routes).await {
        Ok(_) => {
            info!("Server shutdown gracefully");
            Ok(())
        }
        Err(e) => {
            error!("Server failed to start: {}", e);
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        }
    }
}