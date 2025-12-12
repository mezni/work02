use anyhow::Result;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init(log_level: &str) -> Result<()> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .json(),
        )
        .init();

    tracing::info!("Logging initialized at level: {}", log_level);

    Ok(())
}

#[macro_export]
macro_rules! log_error {
    ($err:expr) => {
        tracing::error!(
            error = %$err,
            backtrace = ?$err.backtrace(),
            "Error occurred"
        );
    };
}
