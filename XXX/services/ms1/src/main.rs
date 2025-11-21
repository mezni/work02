use ms1::infrastructure::config::Settings;
use ms1::interfaces::controllers::health_controller;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");
    
    // Start the server
    health_controller::start_server(settings.server.port).await?;
    
    Ok(())
}
