use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    User,
    registration::{Registration, RegistrationRequest},
    repository::{UserRepository, RegistrationRepository},
    value_objects::{UserRole, Email, Password},
};
use crate::application::error::{ApplicationError, ApplicationResult};
use super::service_traits::RegistrationServiceTrait;

pub struct RegistrationService {
    user_repository: Arc<dyn UserRepository>,
    registration_repository: Arc<dyn RegistrationRepository>,
}

impl RegistrationService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        registration_repository: Arc<dyn RegistrationRepository>,
    ) -> Self {
        Self {
            user_repository,
            registration_repository,
        }
    }
    
    fn validate_registration_request(&self, request: &RegistrationRequest) -> ApplicationResult<()> {
        // Validate using domain validation
        request.validate_domain()
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        // Additional business rules
        if request.password.len() < 8 {
            return Err(ApplicationError::WeakPassword);
        }
        
        if !request.password.chars().any(|c| c.is_ascii_digit()) {
            return Err(ApplicationError::WeakPassword);
        }
        
        Ok(())
    }
}

#[async_trait]
impl RegistrationServiceTrait for RegistrationService {
    async fn register(&self, email: &str, password: &str) -> ApplicationResult<Uuid> {
        // Create registration request
        let request = RegistrationRequest {
            email: email.to_string(),
            password: password.to_string(),
            confirm_password: password.to_string(),
        };
        
        // Validate request
        self.validate_registration_request(&request)?;
        
        // Check if user already exists
        let user_exists = self.user_repository.exists_by_email(email).await
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        if user_exists {
            return Err(ApplicationError::EmailAlreadyExists);
        }
        
        // Create domain user for reference
        let domain_email = Email::new(email.to_string())
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        let domain_password = Password::new(password)
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        // Create registration
        let registration = Registration::new(email.to_string())
            .with_role("user") // Default role
            .with_company_name(String::new()) // Empty company name
            .with_station_name(String::new()); // Empty station name
        
        // Register user in Keycloak
        let user_id = self.registration_repository.register_user(&registration, password).await
            .map_err(|e| ApplicationError::RegistrationFailed(e.to_string()))?;
        
        // Emit user registered event if you have event system
        
        Ok(user_id)
    }
    
    async fn verify_email(&self, token: &str) -> ApplicationResult<()> {
        if token.is_empty() {
            return Err(ApplicationError::Validation("Token cannot be empty".to_string()));
        }
        
        self.registration_repository.verify_email(token).await
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
    
    async fn resend_verification(&self, email: &str) -> ApplicationResult<()> {
        if email.is_empty() {
            return Err(ApplicationError::Validation("Email cannot be empty".to_string()));
        }
        
        // Check if user exists
        let user_exists = self.user_repository.exists_by_email(email).await
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        if !user_exists {
            // Don't reveal that user doesn't exist
            return Ok(());
        }
        
        self.registration_repository.resend_verification(email).await
            .map_err(|e| ApplicationError::ServiceUnavailable(e.to_string()))?;
        
        Ok(())
    }
}