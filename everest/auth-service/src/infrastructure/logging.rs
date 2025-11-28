use tracing_subscriber::{fmt, EnvFilter};

pub fn setup_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("auth_service=info,actix_web=info,sqlx=warn"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();
}
