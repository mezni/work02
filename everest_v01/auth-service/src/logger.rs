use tracing::{info, error, warn, debug, trace};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use anyhow::{Result, Context};
use crate::config::AppConfig;

pub fn init_logger(config: &AppConfig) -> Result<()> {
    // Convert log crate records to tracing events
    LogTracer::init().context("Failed to initialize LogTracer")?;
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.log_level));
    
    match config.logging.format.as_str() {
        "json" => {
            // JSON logging for production
            let formatting_layer = BunyanFormattingLayer::new(
                "auth-service".into(),
                std::io::stdout,
            );
            
            let subscriber = Registry::default()
                .with(env_filter)
                .with(JsonStorageLayer)
                .with(formatting_layer);
            
            tracing::subscriber::set_global_default(subscriber)
                .context("Failed to set global default subscriber")?;
        }
        "text" | _ => {
            // Text logging for development
            let subscriber = fmt::Subscriber::builder()
                .with_env_filter(env_filter)
                .with_target(true)
                .with_level(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_ansi(!config.is_production())
                .finish();
            
            tracing::subscriber::set_global_default(subscriber)
                .context("Failed to set global default subscriber")?;
        }
    }
    
    info!(
        "Logger initialized with level: {} and format: {}",
        config.log_level,
        config.logging.format
    );
    
    Ok(())
}

pub fn create_tracing_middleware() -> tracing_actix_web::TracingLogger {
    tracing_actix_web::TracingLogger::default()
}

// Structured logging helpers
pub struct StructuredLogger;

impl StructuredLogger {
    pub fn log_request(
        method: &str,
        path: &str,
        status: u16,
        duration_ms: u128,
        user_id: Option<&uuid::Uuid>,
        client_ip: Option<&str>,
    ) {
        info!(
            method = method,
            path = path,
            status = status,
            duration_ms = duration_ms,
            user_id = user_id.map(|id| id.to_string()),
            client_ip = client_ip,
            "HTTP request"
        );
    }
    
    pub fn log_auth_success(
        user_id: &uuid::Uuid,
        email: &str,
        auth_method: &str,
        ip_address: Option<&str>,
    ) {
        info!(
            user_id = user_id.to_string(),
            email = email,
            auth_method = auth_method,
            ip_address = ip_address,
            "Authentication successful"
        );
    }
    
    pub fn log_auth_failure(
        email: Option<&str>,
        reason: &str,
        auth_method: &str,
        ip_address: Option<&str>,
    ) {
        warn!(
            email = email,
            reason = reason,
            auth_method = auth_method,
            ip_address = ip_address,
            "Authentication failed"
        );
    }
    
    pub fn log_registration(
        user_id: &uuid::Uuid,
        email: &str,
        source_ip: Option<&str>,
    ) {
        info!(
            user_id = user_id.to_string(),
            email = email,
            source_ip = source_ip,
            "User registered"
        );
    }
    
    pub fn log_token_issued(
        user_id: &uuid::Uuid,
        token_type: &str,
        expires_in: i64,
    ) {
        debug!(
            user_id = user_id.to_string(),
            token_type = token_type,
            expires_in = expires_in,
            "Token issued"
        );
    }
    
    pub fn log_token_revoked(
        user_id: &uuid::Uuid,
        token_type: &str,
        reason: &str,
    ) {
        info!(
            user_id = user_id.to_string(),
            token_type = token_type,
            reason = reason,
            "Token revoked"
        );
    }
    
    pub fn log_error(
        error: &anyhow::Error,
        context: &str,
        user_id: Option<&uuid::Uuid>,
        additional_fields: Option<serde_json::Value>,
    ) {
        error!(
            error = error.to_string(),
            context = context,
            user_id = user_id.map(|id| id.to_string()),
            additional_fields = additional_fields.map(|v| v.to_string()),
            "Application error"
        );
    }
    
    pub fn log_keycloak_error(
        operation: &str,
        error: &str,
        status_code: Option<u16>,
        user_id: Option<&uuid::Uuid>,
    ) {
        error!(
            operation = operation,
            error = error,
            status_code = status_code,
            user_id = user_id.map(|id| id.to_string()),
            "Keycloak operation failed"
        );
    }
    
    pub fn log_rate_limit(
        identifier: &str,
        limit: u32,
        window_seconds: u64,
        ip_address: Option<&str>,
    ) {
        warn!(
            identifier = identifier,
            limit = limit,
            window_seconds = window_seconds,
            ip_address = ip_address,
            "Rate limit exceeded"
        );
    }
    
    pub fn log_health_check(
        service: &str,
        status: &str,
        duration_ms: u128,
        error: Option<&str>,
    ) {
        if error.is_some() {
            warn!(
                service = service,
                status = status,
                duration_ms = duration_ms,
                error = error,
                "Health check failed"
            );
        } else {
            debug!(
                service = service,
                status = status,
                duration_ms = duration_ms,
                "Health check passed"
            );
        }
    }
}

// Logging macros for convenience
#[macro_export]
macro_rules! log_error {
    ($error:expr, $context:expr) => {
        $crate::logger::StructuredLogger::log_error(&$error, $context, None, None)
    };
    ($error:expr, $context:expr, $user_id:expr) => {
        $crate::logger::StructuredLogger::log_error(&$error, $context, Some($user_id), None)
    };
    ($error:expr, $context:expr, $user_id:expr, $fields:expr) => {
        $crate::logger::StructuredLogger::log_error(&$error, $context, Some($user_id), Some($fields))
    };
}

#[macro_export]
macro_rules! log_auth {
    ($user_id:expr, $email:expr, $method:expr, $ip:expr) => {
        $crate::logger::StructuredLogger::log_auth_success($user_id, $email, $method, $ip)
    };
}

#[macro_export]
macro_rules! log_auth_failure {
    ($email:expr, $reason:expr, $method:expr, $ip:expr) => {
        $crate::logger::StructuredLogger::log_auth_failure($email, $reason, $method, $ip)
    };
}