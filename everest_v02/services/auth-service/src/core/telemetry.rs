use anyhow::Result;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{Registry, layer::SubscriberExt};

pub fn init(service_name: &str) -> Result<()> {
    let telemetry = get_subscriber(service_name.to_string());
    init_subscriber(telemetry)?;

    tracing::info!("Telemetry initialized for service: {}", service_name);

    Ok(())
}

pub fn get_subscriber(name: String) -> impl tracing::Subscriber + Send + Sync {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    Registry::default()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_target(true))
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) -> Result<()> {
    set_global_default(subscriber)?;
    Ok(())
}

// Instrument functions for tracing
#[macro_export]
macro_rules! trace_fn {
    () => {
        tracing::trace!(
            target: module_path!(),
            "Entering function: {}",
            stdext::function_name!()
        );
    };
}

// Custom span for database operations
pub fn create_db_span(operation: &str, table: &str) -> tracing::Span {
    tracing::info_span!(
        "database_operation",
        operation = operation,
        table = table,
        otel.kind = "client",
        db.system = "postgresql"
    )
}

// Custom span for HTTP requests
pub fn create_http_span(method: &str, path: &str) -> tracing::Span {
    tracing::info_span!(
        "http_request",
        http.method = method,
        http.route = path,
        otel.kind = "server"
    )
}
