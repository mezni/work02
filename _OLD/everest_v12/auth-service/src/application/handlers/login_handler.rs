use crate::{
    domain::{
        repositories::UserRepository,
        value_objects::Username,
        services::AuthService,
    },
    application::{
        commands::login_user::LoginUserCommand,
        errors::ApplicationError,
        dto::{auth_response::AuthResponse, user_dto::UserDTO},
    },
    infrastructure::jwt::token_enricher::JwtTokenEnricher,
};

pub struct LoginHandler<R> {
    user_repo: R,
    jwt_enricher: JwtTokenEnricher,
}

impl<R: UserRepository> LoginHandler<R> {
    pub fn new(user_repo: R, jwt_enricher: JwtTokenEnricher) -> Self {
        LoginHandler { user_repo, jwt_enricher }
    }

    pub async fn execute(&self, cmd: LoginUserCommand) -> Result<AuthResponse, ApplicationError> {
        let username_vo = Username::parse(cmd.username.clone())
            .map_err(ApplicationError::Domain)?;

        let user = self.user_repo.get_by_username(&username_vo).await?
            .ok_or(ApplicationError::AuthenticationFailed)?;

        // Validate credentials
        let is_valid = AuthService::validate_credentials(&user, &cmd.password)
            .map_err(ApplicationError::Domain)?;

        if !is_valid {
            return Err(ApplicationError::AuthenticationFailed);
        }

        // Generate JWT token
        let token = self.jwt_enricher
            .enrich_and_encode(&user)
            .await
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;

        let user_dto = UserDTO::from_domain(&user);

        Ok(AuthResponse::new(token, user_dto))
    }
}
