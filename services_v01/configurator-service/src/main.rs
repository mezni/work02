// src/main.rs
use configurator_service::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting {}...", env!("CARGO_PKG_NAME"));
    
    // Initialize logging
    env_logger::init();
    
    // Start HTTP server
    api::start_server().await
}
