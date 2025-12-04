use crate::user_dto::{CreateUserDto, RoleMapping};
use crate::user_repository::UserRepository;
use std::error::Error;

pub struct RegisterService {
    user_repo: UserRepository,
}

impl RegisterService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn register_user(
        &self,
        token: &str,
        user: &CreateUserDto,
        role: &RoleMapping,
    ) -> Result<(), Box<dyn Error>> {
        self.user_repo.register_user(token, user, role).await
    }
}
