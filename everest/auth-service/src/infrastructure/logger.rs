pub fn init_logger() {
    // Initialize with default configuration
    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(true)
        .with_max_level(tracing::Level::INFO)
        .pretty()
        .init();
}

pub fn init_test_logger() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .try_init();
}
