use actix_web::{middleware, App, HttpServer};
use std::time::Duration;

pub async fn start_server<F, T>(
    config: &crate::config::ServerConfig,
    app_factory: F,
) -> std::io::Result<()>
where
    F: Fn() -> App<T> + Send + Clone + 'static,
    T: actix_web::dev::ServiceFactory<
        ServiceRequest,
        Config = (),
        Error = Error,
        InitError = (),
    >,
{
    let addr = format!("{}:{}", config.host, config.port);
    
    tracing::info!("Starting server on {}", addr);
    
    HttpServer::new(app_factory)
        .workers(config.workers)
        .keep_alive(Duration::from_secs(config.keep_alive))
        .client_request_timeout(Duration::from_secs(config.client_timeout))
        .bind(&addr)?
        .run()
        .await
}
