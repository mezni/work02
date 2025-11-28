use crate::domain::{
    repositories::UserRepository,
    models::User,
    value_objects::{Username, Email},
    services::{UserDomainService, AuthService},
};
use crate::application::{
    commands::register_user::RegisterUserCommand,
    errors::ApplicationError,
    dto::user_dto::UserDTO,
};

pub struct RegisterUserHandler<R> {
    user_repo: R,
}

impl<R: UserRepository> RegisterUserHandler<R> {
    pub fn new(user_repo: R) -> Self {
        RegisterUserHandler { user_repo }
    }

    pub async fn execute(&self, cmd: RegisterUserCommand) -> Result<UserDTO, ApplicationError> {
        // Validate input
        UserDomainService::validate_user_creation(&cmd.username, &cmd.email, &cmd.password)
            .map_err(ApplicationError::Domain)?;

        let username_vo = Username::parse(cmd.username.clone())
            .map_err(ApplicationError::Domain)?;
        let email_vo = Email::parse(cmd.email.clone())
            .map_err(ApplicationError::Domain)?;

        // Check if user already exists
        if self.user_repo.get_by_username(&username_vo).await?.is_some() {
            return Err(ApplicationError::Domain(
                crate::domain::errors::DomainError::UsernameAlreadyExists(cmd.username)
            ));
        }

        if self.user_repo.get_by_email(&email_vo).await?.is_some() {
            return Err(ApplicationError::Domain(
                crate::domain::errors::DomainError::EmailAlreadyExists(cmd.email)
            ));
        }

        // Hash password
        let password_hash = AuthService::hash_password(&cmd.password)
            .map_err(|e| ApplicationError::PasswordHashing(e.to_string()))?;

        // Create user
        let new_user = User::register(username_vo, email_vo, password_hash);
        
        let saved_user = self.user_repo.save(&new_user).await?;

        Ok(UserDTO::from_domain(&saved_user))
    }
}
