#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("Unknown error")] 
    Unknown,
}
