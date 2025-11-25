use tracing::{info, error, warn};
use uuid::Uuid;

use crate::application::commands::{
    CreateUserCommand, UpdateUserCommand, ChangeUserRoleCommand, AssignUserToCompanyCommand,
    RemoveUserFromCompanyCommand, UpdateProfileCommand, ChangePasswordCommand,
    CreateCompanyCommand, UpdateCompanyCommand, DeleteCompanyCommand,
    RegisterUserCommand, LoginCommand, RefreshTokenCommand, ForgotPasswordCommand,
    ResetPasswordCommand, LogoutCommand,
};
use crate::application::errors::ApplicationError;
use crate::domain::entities::{User, Company};
use crate::domain::value_objects::{Email, Password};
use crate::domain::enums::UserRole;
use crate::domain::repositories::{UserRepository, CompanyRepository};
use crate::infrastructure::auth::{KeycloakClient, JwtService};
use crate::infrastructure::audit::Auditor;

pub struct UserCommandHandler<T: UserRepository, C: CompanyRepository, A> {
    user_repository: T,
    company_repository: C,
    keycloak_client: KeycloakClient,
    auditor: A,
}

impl<T: UserRepository, C: CompanyRepository, A> UserCommandHandler<T, C, A> {
    pub fn new(
        user_repository: T,
        company_repository: C,
        keycloak_client: KeycloakClient,
        auditor: A,
    ) -> Self {
        Self {
            user_repository,
            company_repository,
            keycloak_client,
            auditor,
        }
    }

    pub async fn handle_create_user(
        &self,
        command: CreateUserCommand,
        created_by: &User,
    ) -> Result<User, ApplicationError> {
        // Validate permissions
        if !created_by.can_manage_users() {
            return Err(ApplicationError::Authorization(
                "Insufficient permissions to create users".to_string(),
            ));
        }

        // Check if user can manage the target company
        if let Some(company_id) = command.company_id {
            if !created_by.has_company_access(&company_id) {
                return Err(ApplicationError::Authorization(
                    "Cannot create user in this company".to_string(),
                ));
            }
        }

        // Check if email already exists
        let email = Email::new(command.email.clone())?;
        if self.user_repository.exists_by_email(&email).await? {
            return Err(ApplicationError::Validation(
                "Email already exists".to_string(),
            ));
        }

        // Check if username already exists
        if self.user_repository.exists_by_username(&command.username).await? {
            return Err(ApplicationError::Validation(
                "Username already exists".to_string(),
            ));
        }

        // Create user in Keycloak first
        let keycloak_id = self
            .keycloak_client
            .create_user(
                &command.username,
                &command.email,
                "TempPassword123!", // Temporary password, user will need to reset
                None,
                None,
            )
            .await?;

        // Create domain user
        let user = User::new(keycloak_id, command.username, email, command.role)?;

        // Assign to company if specified
        if let Some(company_id) = command.company_id {
            // Verify company exists
            let company = self
                .company_repository
                .find_by_id(&company_id)
                .await?
                .ok_or_else(|| ApplicationError::CompanyNotFound(company_id.to_string()))?;

            // Assign user to company
            // Note: This will validate if the role can be assigned to a company
            // user.assign_to_company(company_id)?;
        }

        // Save user
        self.user_repository.create(&user).await?;

        info!("User created successfully: {}", user.id);
        Ok(user)
    }

    pub async fn handle_change_user_role(
        &self,
        command: ChangeUserRoleCommand,
        changed_by: &User,
    ) -> Result<(), ApplicationError> {
        // Validate permissions
        if !changed_by.can_manage_users() {
            return Err(ApplicationError::Authorization(
                "Insufficient permissions to change user roles".to_string(),
            ));
        }

        // Get target user
        let mut target_user = self
            .user_repository
            .find_by_id(&command.user_id)
            .await?
            .ok_or_else(|| ApplicationError::UserNotFound(command.user_id.to_string()))?;

        // Check if changer can manage this user
        if !changed_by.can_manage_user(&target_user) {
            return Err(ApplicationError::Authorization(
                "Cannot manage this user".to_string(),
            ));
        }

        // Change role
        target_user.change_role(command.new_role)?;

        // Update user
        self.user_repository.update(&target_user).await?;

        // Log audit event
        self.auditor
            .log_user_role_change(
                target_user.id.to_string(),
                command.user_id.to_string(),
                target_user.role.to_string(),
                command.new_role.to_string(),
                changed_by.id.to_string(),
                changed_by.company_id,
            )
            .await?;

        info!("User role changed: {}", command.user_id);
        Ok(())
    }

    pub async fn handle_assign_user_to_company(
        &self,
        command: AssignUserToCompanyCommand,
        assigned_by: &User,
    ) -> Result<(), ApplicationError> {
        // Validate permissions
        if !assigned_by.can_manage_users() {
            return Err(ApplicationError::Authorization(
                "Insufficient permissions to assign users to companies".to_string(),
            ));
        }

        // Get target user
        let mut target_user = self
            .user_repository
            .find_by_id(&command.user_id)
            .await?
            .ok_or_else(|| ApplicationError::UserNotFound(command.user_id.to_string()))?;

        // Verify company exists
        let company = self
            .company_repository
            .find_by_id(&command.company_id)
            .await?
            .ok_or_else(|| ApplicationError::CompanyNotFound(command.company_id.to_string()))?;

        // Check if assigner can manage this company
        if !assigned_by.has_company_access(&command.company_id) {
            return Err(ApplicationError::Authorization(
                "Cannot assign users to this company".to_string(),
            ));
        }

        // Assign user to company
        target_user.assign_to_company(command.company_id)?;

        // Update user
        self.user_repository.update(&target_user).await?;

        info!("User assigned to company: {} -> {}", command.user_id, command.company_id);
        Ok(())
    }

    pub async fn handle_update_profile(
        &self,
        command: UpdateProfileCommand,
        user_id: Uuid,
    ) -> Result<User, ApplicationError> {
        let mut user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| ApplicationError::UserNotFound(user_id.to_string()))?;

        // Update fields if provided
        if let Some(username) = command.username {
            if self.user_repository.exists_by_username(&username).await? {
                return Err(ApplicationError::Validation(
                    "Username already exists".to_string(),
                ));
            }
            user.username = username;
        }

        if let Some(email) = command.email {
            let email_vo = Email::new(email)?;
            if self.user_repository.exists_by_email(&email_vo).await? {
                return Err(ApplicationError::Validation(
                    "Email already exists".to_string(),
                ));
            }
            user.email = email_vo;
        }

        self.user_repository.update(&user).await?;

        info!("User profile updated: {}", user_id);
        Ok(user)
    }
}

pub struct AuthCommandHandler<T: UserRepository, A> {
    user_repository: T,
    keycloak_client: KeycloakClient,
    jwt_service: JwtService,
    auditor: A,
}

impl<T: UserRepository, A> AuthCommandHandler<T, A> {
    pub fn new(
        user_repository: T,
        keycloak_client: KeycloakClient,
        jwt_service: JwtService,
        auditor: A,
    ) -> Self {
        Self {
            user_repository,
            keycloak_client,
            jwt_service,
            auditor,
        }
    }

    pub async fn handle_register_user(
        &self,
        command: RegisterUserCommand,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<User, ApplicationError> {
        // Validate passwords match
        if command.password != command.confirm_password {
            return Err(ApplicationError::Validation(
                "Passwords do not match".to_string(),
            ));
        }

        // Check if email already exists
        let email = Email::new(command.email.clone())?;
        if self.user_repository.exists_by_email(&email).await? {
            return Err(ApplicationError::Validation(
                "Email already exists".to_string(),
            ));
        }

        // Check if username already exists
        if self.user_repository.exists_by_username(&command.username).await? {
            return Err(ApplicationError::Validation(
                "Username already exists".to_string(),
            ));
        }

        // Create user in Keycloak
        let keycloak_id = self
            .keycloak_client
            .create_user(
                &command.username,
                &command.email,
                &command.password,
                None,
                None,
            )
            .await?;

        // Create domain user with User role (default for self-registration)
        let user = User::new(keycloak_id, command.username, email, UserRole::User)?;

        // Save user
        self.user_repository.create(&user).await?;

        // Log audit event
        self.auditor
            .log_user_registration(
                user.id.to_string(),
                user.email.value().to_string(),
                user.role.to_string(),
                ip_address,
                user_agent,
            )
            .await?;

        info!("User registered successfully: {}", user.id);
        Ok(user)
    }

    pub async fn handle_login(
        &self,
        command: LoginCommand,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(String, String, User), ApplicationError> {
        // Authenticate with Keycloak
        let keycloak_response = self
            .keycloak_client
            .login(&command.email, &command.password)
            .await?;

        // Get user info from Keycloak
        let user_info = self
            .keycloak_client
            .user_info(&keycloak_response.access_token)
            .await?;

        // Find user in our database
        let user = self
            .user_repository
            .find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or_else(|| {
                ApplicationError::UserNotFound(format!("Keycloak user: {}", user_info.sub))
            })?;

        // Generate JWT token
        let jwt_token = self.jwt_service.generate_token(
            &user.id.to_string(),
            &user.username,
            &user.email.value(),
            &user.role.to_string(),
            user.company_id.as_ref().map(|id| id.to_string().as_str()),
        )?;

        // Log audit event
        self.auditor
            .log_user_login(
                user.id.to_string(),
                user.email.value().to_string(),
                user.role.to_string(),
                user.company_id,
                ip_address,
                user_agent,
            )
            .await?;

        info!("User logged in successfully: {}", user.id);
        Ok((jwt_token, keycloak_response.refresh_token, user))
    }

    pub async fn handle_refresh_token(
        &self,
        command: RefreshTokenCommand,
    ) -> Result<(String, String), ApplicationError> {
        // Refresh token with Keycloak
        let keycloak_response = self
            .keycloak_client
            .refresh_token(&command.refresh_token)
            .await?;

        // Get user info to generate new JWT
        let user_info = self
            .keycloak_client
            .user_info(&keycloak_response.access_token)
            .await?;

        // Find user
        let user = self
            .user_repository
            .find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or_else(|| {
                ApplicationError::UserNotFound(format!("Keycloak user: {}", user_info.sub))
            })?;

        // Generate new JWT token
        let jwt_token = self.jwt_service.generate_token(
            &user.id.to_string(),
            &user.username,
            &user.email.value(),
            &user.role.to_string(),
            user.company_id.as_ref().map(|id| id.to_string().as_str()),
        )?;

        Ok((jwt_token, keycloak_response.refresh_token))
    }
}

// Similar implementations for CompanyCommandHandler and other handlers...