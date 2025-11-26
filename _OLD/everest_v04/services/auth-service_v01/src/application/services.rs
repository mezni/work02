use crate::domain::repository::UserRepository;
use anyhow::Result;
use crate::domain::models::User;
use crate::domain::value_objects::{Email, Password, CompanyName};
use crate::application::dtos::{CreateUserDTO, CreateUserResponseDTO};

#[derive(Clone)]
pub struct UserService<R: UserRepository + Clone> {
    pub repo: R,
}

impl<R: UserRepository + Clone> UserService<R> {
    pub fn new(repo: R) -> Self { Self { repo } }

    pub async fn create_user(&self, dto: CreateUserDTO) -> Result<CreateUserResponseDTO> {
        let email = Email::new(&dto.email)?;
        let company = CompanyName::new(&dto.company)?;
        let password = Password::new(&dto.password)?;

        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: dto.username,
            email,
            company,
            role: dto.role,
        };

        let user_id = self.repo.create_user(&user, &password.0).await?;
        self.repo.assign_role(&user_id, &user.role).await?;

        Ok(CreateUserResponseDTO { id: user_id })
    }
}
