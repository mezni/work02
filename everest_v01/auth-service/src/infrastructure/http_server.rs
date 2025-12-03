use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use std::sync::Arc;
use crate::{
    config::AppConfig,
    interfaces::http_routes::configure_routes,
    logger,
};
use super::error::InfrastructureError;

pub struct HttpServerBuilder {
    config: AppConfig,
}

impl HttpServerBuilder {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
    
    pub async fn build(
        self,
        routes_config: fn(&mut web::ServiceConfig),
    ) -> Result<actix_web::dev::Server, InfrastructureError> {
        let host = self.config.server.host.clone();
        let port = self.config.server.port;
        let workers = self.config.server.workers;
        
        logger::init_logger(&self.config.log_level)
            .map_err(|e| InfrastructureError::Configuration(e.to_string()))?;
        
        let server = HttpServer::new(move || {
            let cors = if self.config.server.enable_cors {
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            } else {
                Cors::default()
            };
            
            App::new()
                .wrap(cors)
                .wrap(logger::create_tracing_middleware())
                .wrap(actix_web::middleware::NormalizePath::trim())
                .configure(routes_config)
        })
        .workers(workers)
        .bind((host, port))
        .map_err(|e| InfrastructureError::Io(e.into()))?
        .run();
        
        Ok(server)
    }
}

pub async fn start_server(
    config: AppConfig,
    routes_config: fn(&mut web::ServiceConfig),
) -> Result<(), InfrastructureError> {
    log::info!("Starting HTTP server on {}:{}", config.server.host, config.server.port);
    
    let server_builder = HttpServerBuilder::new(config);
    let server = server_builder.build(routes_config).await?;
    
    log::info!("HTTP server started successfully");
    
    server.await
        .map_err(|e| InfrastructureError::Io(e.into()))
}