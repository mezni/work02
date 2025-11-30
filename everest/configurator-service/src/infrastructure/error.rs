use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

// Example function that uses AppError
pub fn validate_input(input: &str) -> Result<(), AppError> {
    if input.is_empty() {
        return Err(AppError::Validation("Input cannot be empty".to_string()));
    }
    Ok(())
}

impl actix_web::ResponseError for AppError {}
