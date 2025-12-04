use crate::application::errors::ApplicationError;
use crate::domain::errors::DomainError;
use crate::domain::user::User;
use crate::domain::user_repository::UserRepository;
use crate::application::user_dto::RegisterRequestDto;

pub struct RegisterService<R: UserRepository> {
    pub user_repo: R,
}

impl<R: UserRepository> RegisterService<R> {
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub fn register_user(
        &self,
        req: RegisterRequestDto,
    ) -> Result<User, ApplicationError> {
        // Check username/email existence
        if self.user_repo.exists_by_username(&req.username)
            .map_err(|e| ApplicationError::Unexpected(format!("{:?}", e)))?
        {
            return Err(ApplicationError::UserAlreadyExists(req.username));
        }

        if self.user_repo.exists_by_email(&req.email)
            .map_err(|e| ApplicationError::Unexpected(format!("{:?}", e)))?
        {
            return Err(ApplicationError::UserAlreadyExists(req.email));
        }

        // Generate user ID and create domain user
        let user = User::new(
            uuid::Uuid::new_v4().to_string(),
            req.username,
            req.email,
            req.first_name,
            req.last_name,
            "".to_string(), // password handled by Keycloak
        ).map_err(|e| ApplicationError::ValidationError(format!("{:?}", e)))?;

        self.user_repo.save(&user)
            .map_err(|e| ApplicationError::Unexpected(format!("{:?}", e)))?;

        Ok(user)
    }
}
