use super::config::Config;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init(config: &Config) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_timer(fmt::time::UtcTime::rfc_3339())
        .init();

    tracing::info!("Logger initialized with level: {}", config.log_level);
}
