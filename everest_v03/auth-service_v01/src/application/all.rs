// src/application/register_service.rs
use crate::domain::user::User;
use crate::domain::user_repository::UserRepository;
use crate::infrastructure::keycloak_user_repository::KeycloakUserRepository;

pub struct RegisterService {
    user_repository: KeycloakUserRepository,
}

impl RegisterService {
    pub fn new(user_repository: KeycloakUserRepository) -> Self {
        RegisterService { user_repository }
    }

    pub async fn register(&self, username: String, email: String, password: String) -> Result<User, Error> {
        let user = User::new(username, email);
        self.user_repository.save(user).await?;
        Ok(user)
    }
}

// src/application/authenticate_service.rs
use crate::domain::user::User;
use crate::infrastructure::keycloak_auth::KeycloakAuth;

pub struct AuthenticateService {
    keycloak_auth: KeycloakAuth,
}

impl AuthenticateService {
    pub fn new(keycloak_auth: KeycloakAuth) -> Self {
        AuthenticateService { keycloak_auth }
    }

    pub async fn authenticate(&self, username: String, password: String) -> Result<User, Error> {
        self.keycloak_auth.authenticate(&username, &password).await
    }
}

// src/application/user_dto.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}