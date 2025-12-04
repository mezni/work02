use crate::application::errors::ApplicationError;
use crate::application::user_dto::LoginRequestDto;

/// LoginService is mainly a placeholder since Keycloak handles authentication and token generation
pub struct LoginService;

impl LoginService {
    pub fn new() -> Self {
        Self {}
    }

    /// Login request is validated by Keycloak, returns placeholder result
    pub fn login_user(&self, _req: LoginRequestDto) -> Result<String, ApplicationError> {
        // In practice, call Keycloak token endpoint here
        Err(ApplicationError::Unauthorized("Login must be performed via Keycloak".into()))
    }
}
