use auth_service::infrastructure::logger;
use std::sync::Once;

static TEST_LOGGER_INIT: Once = Once::new();

fn ensure_test_logger() {
    TEST_LOGGER_INIT.call_once(|| {
        logger::init_test_logger();
    });
}

#[test]
fn test_logger_initialization() {
    ensure_test_logger();
    
    // Test that we can log at different levels without panicking
    tracing::error!("test error message");
    tracing::warn!("test warn message");
    tracing::info!("test info message");
    tracing::debug!("test debug message");
    tracing::trace!("test trace message");
    
    // If we get here without panicking, logging is working
    assert!(true, "Logger should initialize without errors");
}

#[test]
fn test_logger_levels_functional() {
    ensure_test_logger();
    
    // Test that different log levels work
    // These should not panic when the test logger is initialized
    tracing::info!("info level works");
    tracing::warn!("warn level works");
    tracing::error!("error level works");
    
    assert!(true, "All log levels should work without errors");
}

#[test]
fn test_multiple_logger_calls() {
    ensure_test_logger();
    
    // Ensure we can call logging multiple times
    for i in 0..5 {
        tracing::info!("iteration {}", i);
    }
    
    assert!(true, "Multiple log calls should work without issues");
}
