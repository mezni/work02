use std::sync::Arc;
use crate::domain::{
    entities::User,
    repositories::UserRepository,
    services::AuthorizationService,
    value_objects::{Email, OrganisationName, Role},
    errors::DomainError,
};
use super::{
    commands::CreateUserCommand,
    queries::{GetUserQuery, GetUserByKeycloakIdQuery, ListUsersByOrganisationQuery},
    dto::{UserResponse, CreateUserRequest},
    errors::ApplicationError,
};

pub struct UserCommandHandler<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserCommandHandler<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn handle_create_user(
        &self,
        command: CreateUserCommand,
    ) -> Result<User, ApplicationError> {
        // Authorization check
        if !AuthorizationService::can_create_user(&command.requester_role, &command.role) {
            return Err(ApplicationError::Unauthorized(
                "Insufficient permissions to create this user type".to_string(),
            ));
        }

        // Validate email
        let email = Email::new(command.email)
            .map_err(|e| ApplicationError::ValidationError(e))?;

        // Check if user already exists
        if let Some(_) = self.user_repository.find_by_email(email.value()).await? {
            return Err(ApplicationError::ValidationError(
                "User with this email already exists".to_string(),
            ));
        }

        // Validate organisation for operators
        let organisation_name = if command.role == Role::Operator {
            if command.organisation_name.is_none() {
                return Err(ApplicationError::ValidationError(
                    "Operator must belong to an organisation".to_string(),
                ));
            }
            Some(OrganisationName::new(command.organisation_name.unwrap())
                .map_err(|e| ApplicationError::ValidationError(e))?)
        } else {
            command.organisation_name.map(|n| {
                OrganisationName::new(n)
                    .map_err(|e| ApplicationError::ValidationError(e))
            }).transpose()?
        };

        // Note: Keycloak ID would be set after Keycloak user creation
        // This is a placeholder - actual implementation in infrastructure layer
        let keycloak_id = format!("keycloak_{}", uuid::Uuid::new_v4());

        let user = User::new(
            keycloak_id,
            email,
            command.username,
            command.role,
            organisation_name,
        );

        user.validate_operator_has_organisation()
            .map_err(|e| ApplicationError::ValidationError(e))?;

        let saved_user = self.user_repository.save(&user).await?;
        Ok(saved_user)
    }
}

pub struct UserQueryHandler<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserQueryHandler<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn handle_get_user(
        &self,
        query: GetUserQuery,
    ) -> Result<Option<User>, ApplicationError> {
        Ok(self.user_repository.find_by_id(query.user_id).await?)
    }

    pub async fn handle_get_user_by_keycloak_id(
        &self,
        query: GetUserByKeycloakIdQuery,
    ) -> Result<Option<User>, ApplicationError> {
        Ok(self.user_repository.find_by_keycloak_id(&query.keycloak_id).await?)
    }

    pub async fn handle_list_by_organisation(
        &self,
        query: ListUsersByOrganisationQuery,
    ) -> Result<Vec<User>, ApplicationError> {
        // Get requester to check permissions
        let requester = self.user_repository
            .find_by_id(query.requester_id)
            .await?
            .ok_or(ApplicationError::Unauthorized("Requester not found".to_string()))?;

        // Check if requester can access this organisation
        if !requester.can_access_organisation(&query.organisation_name) {
            return Err(ApplicationError::Unauthorized(
                "Cannot access this organisation".to_string(),
            ));
        }

        Ok(self.user_repository
            .list_by_organisation(&query.organisation_name)
            .await?)
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            keycloak_id: user.keycloak_id,
            email: user.email.value().to_string(),
            username: user.username,
            role: user.role,
            organisation_name: user.organisation_name.map(|o| o.value().to_string()),
            is_active: user.is_active,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}
