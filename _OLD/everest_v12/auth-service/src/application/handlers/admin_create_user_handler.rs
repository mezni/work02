use crate::domain::{
    repositories::{UserRepository, OrganisationRepository, StationRepository},
    models::User,
    value_objects::{Username, Email, Role},
    services::{UserDomainService, AuthService},
};
use crate::application::{
    commands::admin_create_user::AdminCreateUserCommand,
    errors::ApplicationError,
    dto::user_dto::UserDTO,
};

pub struct AdminCreateUserHandler<R, O, S> {
    user_repo: R,
    organisation_repo: O,
    station_repo: S,
}

impl<R: UserRepository, O: OrganisationRepository, S: StationRepository> AdminCreateUserHandler<R, O, S> {
    pub fn new(user_repo: R, organisation_repo: O, station_repo: S) -> Self {
        AdminCreateUserHandler { user_repo, organisation_repo, station_repo }
    }

    pub async fn execute(&self, cmd: AdminCreateUserCommand) -> Result<UserDTO, ApplicationError> {
        // Validate input
        UserDomainService::validate_user_creation(&cmd.username, &cmd.email, &cmd.password)
            .map_err(ApplicationError::Domain)?;

        let username_vo = Username::parse(cmd.username.clone())
            .map_err(ApplicationError::Domain)?;
        let email_vo = Email::parse(cmd.email.clone())
            .map_err(ApplicationError::Domain)?;
        let role = Role::from_str(&cmd.role)
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

        // Validate organisation and station if provided
        if let Some(org_id) = cmd.organisation_id {
            if self.organisation_repo.get_by_id(org_id).await?.is_none() {
                return Err(ApplicationError::OrganisationNotFound);
            }
        }

        if let Some(station_id) = cmd.station_id {
            if self.station_repo.get_by_id(station_id).await?.is_none() {
                return Err(ApplicationError::StationNotFound);
            }
        }

        // Hash password
        let password_hash = AuthService::hash_password(&cmd.password)
            .map_err(|e| ApplicationError::PasswordHashing(e.to_string()))?;

        // Create user
        let mut new_user = User::new(username_vo, email_vo, password_hash, role);
        new_user.organisation_id = cmd.organisation_id;
        new_user.station_id = cmd.station_id;
        
        let saved_user = self.user_repo.save(&new_user).await?;

        Ok(UserDTO::from_domain(&saved_user))
    }
}
