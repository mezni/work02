use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Keycloak API request failed: {0}")]
    KeycloakRequestFailed(String),

    #[error("User not found in Keycloak: {0}")]
    UserNotFound(String),

    #[error("HTTP request error: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("Unexpected infrastructure error: {0}")]
    Unexpected(String),
}
