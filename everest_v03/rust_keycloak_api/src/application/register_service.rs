use crate::domain::user::User;
use crate::domain::user_repository::UserRepository;
use crate::domain::errors::DomainError;

pub struct RegisterService {
    user_repo: Box<dyn UserRepository>,
}

impl RegisterService {
    pub fn new(user_repo: Box<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn register_user(&self, user: User) -> Result<(), DomainError> {
        self.user_repo.create(&user).await
    }
}
