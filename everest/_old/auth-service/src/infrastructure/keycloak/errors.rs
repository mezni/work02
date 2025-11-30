use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeycloakError {
    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("User creation failed: {0}")]
    UserCreationFailed(String),

    #[error("Role assignment failed: {0}")]
    RoleAssignmentFailed(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}
