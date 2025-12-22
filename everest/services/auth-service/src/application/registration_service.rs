use crate::AppState;
use crate::application::registration_dto::{
    RegisterRequest, RegisterResponse, ResendVerificationResponse,
};
use crate::core::{
    constants::{MAX_RESEND_COUNT, REGISTRATION_PREFIX, RESEND_COOLDOWN_MINUTES},
    errors::AppError,
    id_generator::{generate_id, generate_verification_token},
};
use crate::domain::{
    entities::UserRegistration,
    enums::{RegistrationStatus, Source},
    repositories::{RegistrationRepository, UserRepository},
    value_objects::{Email, Password, Username},
};
use crate::infrastructure::keycloak_client::KeycloakClient; // CRITICAL: Import trait
use crate::infrastructure::persistence::{RegistrationRepositoryImpl, UserRepositoryImpl};
use actix_web::web;
use chrono::{TimeDelta, Utc};
use std::collections::HashMap;

pub struct RegistrationService;

impl RegistrationService {
    pub async fn register(
        state: web::Data<AppState>,
        request: RegisterRequest,
    ) -> Result<RegisterResponse, AppError> {
        // 1. Validation
        if request.password != request.confirm_password {
            return Err(AppError::ValidationError(
                "Passwords do not match".to_string(),
            ));
        }

        let email = Email::new(request.email)?;
        let username = Username::new(request.username)?;
        let password = Password::new(request.password)?;

        // 2. Repository Setup
        let user_repo = UserRepositoryImpl::new(state.db_pool.clone());
        let reg_repo = RegistrationRepositoryImpl::new(state.db_pool.clone());

        // 3. Duplicate Checks
        if user_repo.find_by_email(email.value()).await?.is_some() {
            return Err(AppError::AlreadyExists(
                "Email already registered".to_string(),
            ));
        }

        if user_repo
            .find_by_username(username.value())
            .await?
            .is_some()
        {
            return Err(AppError::AlreadyExists(
                "Username already taken".to_string(),
            ));
        }

        // 4. Logic & Keycloak Prep
        let registration_id = generate_id(REGISTRATION_PREFIX);
        let verification_token = generate_verification_token();
        let expires_at = Utc::now()
            + TimeDelta::try_hours(state.config.verification_expiry_hours as i64)
                .unwrap_or_default();

        // Package attributes for Keycloak
        let mut attributes = HashMap::new();
        if let Some(first) = &request.first_name {
            attributes.insert("firstName".to_string(), vec![first.clone()]);
        }
        if let Some(last) = &request.last_name {
            attributes.insert("lastName".to_string(), vec![last.clone()]);
        }
        attributes.insert(
            "verification_token".to_string(),
            vec![verification_token.clone()],
        );

        // 5. Keycloak Call
        let keycloak_id = state
            .keycloak_client
            .create_user(
                email.value(),
                username.value(),
                password.value(),
                Some(attributes),
            )
            .await?;

        // 6. Persistence
        let registration = UserRegistration {
            registration_id: registration_id.clone(),
            email: email.value().to_string(),
            username: username.value().to_string(),
            first_name: request.first_name,
            last_name: request.last_name,
            phone: request.phone,
            verification_token,
            status: RegistrationStatus::Pending,
            keycloak_id,
            user_id: None,
            resend_count: 0,
            expires_at,
            verified_at: None,
            created_at: Utc::now(),
            ip_address: None,
            user_agent: None,
            source: Source::Web,
        };

        reg_repo.create(&registration).await?;

        Ok(RegisterResponse {
            registration_id,
            email: email.value().to_string(),
            expires_at,
            message: "Registration created. Please verify your email.".to_string(),
        })
    }

    pub async fn resend_verification(
        state: web::Data<AppState>,
        email: String,
    ) -> Result<ResendVerificationResponse, AppError> {
        let email_obj = Email::new(email)?;
        let reg_repo = RegistrationRepositoryImpl::new(state.db_pool.clone());

        let registration = reg_repo
            .find_by_email(email_obj.value())
            .await?
            .ok_or_else(|| AppError::NotFound("Registration not found".to_string()))?;

        if registration.status == RegistrationStatus::Verified {
            return Err(AppError::BadRequest("Email already verified".to_string()));
        }

        if registration.resend_count >= MAX_RESEND_COUNT {
            return Err(AppError::MaxResendAttemptsReached);
        }

        // Cooldown Logic
        if registration.resend_count > 0 {
            let cooldown_mins = RESEND_COOLDOWN_MINUTES * registration.resend_count as i64;
            let cooldown_end =
                registration.created_at + TimeDelta::try_minutes(cooldown_mins).unwrap_or_default();
            if Utc::now() < cooldown_end {
                return Err(AppError::ResendCooldownActive);
            }
        }

        let new_token = generate_verification_token();
        let new_expires_at = Utc::now()
            + TimeDelta::try_hours(state.config.verification_expiry_hours as i64)
                .unwrap_or_default();

        reg_repo
            .update_verification_token(&registration.registration_id, &new_token, new_expires_at)
            .await?;
        reg_repo
            .increment_resend_count(&registration.registration_id)
            .await?;

        let _ = state
            .keycloak_client
            .send_verification_email(&registration.keycloak_id)
            .await;

        Ok(ResendVerificationResponse {
            message: "Verification email resent".to_string(),
            expires_at: new_expires_at,
        })
    }
}
