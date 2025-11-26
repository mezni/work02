pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;

// Prelude for common imports
pub mod prelude {
//    pub use crate::domain::errors::DomainError;
//    pub use crate::application::errors::ApplicationError;
//    pub use crate::infrastructure::errors::InfrastructureError;
//    pub use crate::interfaces::errors::InterfaceError;
    
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}

// Server startup function
pub async fn start_server(config: infrastructure::config::Config) -> Result<(), Box<dyn std::error::Error>> {
    use actix_web::{web, App, HttpServer, HttpResponse};

    // Basic health check endpoint
    async fn health_check() -> HttpResponse {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "service": "auth-service"
        }))
    }

    tracing::info!("Starting HTTP server on {}:{}", config.server.host, config.server.port);
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(|| async { 
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Welcome to Auth Service!",
                    "version": env!("CARGO_PKG_VERSION")
                }))
            }))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
    .map_err(|e| e.into())
}
