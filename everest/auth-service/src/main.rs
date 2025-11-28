use auth_service::startup;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing before anything else
    auth_service::infrastructure::logging::setup_tracing(); 

    // Start the application
    startup::run().await
}
