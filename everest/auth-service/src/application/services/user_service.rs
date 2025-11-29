use crate::application::dto::user_dto::{CreateUserDto, UserDto};
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::{RepositoryError, UserRepository};
use crate::domain::value_objects::{Email, UserId};
use std::sync::Arc;
use tracing::info;

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, dto: CreateUserDto) -> Result<String, RepositoryError> {
        info!("Creating user from DTO: {}", dto.username);

        let email =
            Email::new(dto.email).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let user = User::new(dto.username, email, dto.first_name, dto.last_name);

        let user_id = self.user_repository.create(&user, &dto.password).await?;
        Ok(user_id.as_str().to_string())
    }

    pub async fn get_user(&self, user_id: &str) -> Result<Option<UserDto>, RepositoryError> {
        let id = UserId::new(user_id.to_string());
        let user = self.user_repository.find_by_id(&id).await?;
        Ok(user.map(|u| self.to_dto(&u)))
    }

    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<UserDto>, RepositoryError> {
        let user = self.user_repository.find_by_username(username).await?;
        Ok(user.map(|u| self.to_dto(&u)))
    }

    pub async fn list_users(&self) -> Result<Vec<UserDto>, RepositoryError> {
        let users = self.user_repository.find_all().await?;
        Ok(users.iter().map(|u| self.to_dto(u)).collect())
    }

    pub async fn enable_user(&self, user_id: &str) -> Result<(), RepositoryError> {
        let id = UserId::new(user_id.to_string());
        let mut user = self
            .user_repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(user_id.to_string()))?;

        user.enable();
        self.user_repository.update(&user).await
    }

    pub async fn disable_user(&self, user_id: &str) -> Result<(), RepositoryError> {
        let id = UserId::new(user_id.to_string());
        let mut user = self
            .user_repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(user_id.to_string()))?;

        user.disable();
        self.user_repository.update(&user).await
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<(), RepositoryError> {
        let id = UserId::new(user_id.to_string());
        self.user_repository.delete(&id).await
    }

    pub async fn assign_role(&self, user_id: &str, role_name: &str) -> Result<(), RepositoryError> {
        let id = UserId::new(user_id.to_string());
        self.user_repository.assign_role(&id, role_name).await
    }

    pub async fn get_user_roles(&self, user_id: &str) -> Result<Vec<String>, RepositoryError> {
        let id = UserId::new(user_id.to_string());
        self.user_repository.get_roles(&id).await
    }

    fn to_dto(&self, user: &User) -> UserDto {
        UserDto {
            id: user
                .id
                .as_ref()
                .map(|id| id.as_str().to_string())
                .unwrap_or_default(),
            username: user.username.clone(),
            email: user.email.as_str().to_string(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            enabled: user.enabled,
        }
    }
}
