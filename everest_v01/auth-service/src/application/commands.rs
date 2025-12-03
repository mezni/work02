use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use crate::application::error::ApplicationResult;

// Login Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginCommand {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 1))]
    pub password: String,
    
    pub remember_me: Option<bool>,
}

impl LoginCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}

// Register Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterCommand {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    #[validate(must_match = "password")]
    pub confirm_password: String,
}

impl RegisterCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        // Additional validation
        if self.password != self.confirm_password {
            return Err(crate::application::error::ApplicationError::PasswordMismatch);
        }
        
        Ok(())
    }
}

// Refresh Token Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RefreshTokenCommand {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

impl RefreshTokenCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}

// Logout Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LogoutCommand {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

impl LogoutCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}

// Change Password Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangePasswordCommand {
    pub current_password: String,
    
    #[validate(length(min = 8))]
    pub new_password: String,
    
    #[validate(must_match = "new_password")]
    pub confirm_password: String,
}

impl ChangePasswordCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        if self.new_password != self.confirm_password {
            return Err(crate::application::error::ApplicationError::PasswordMismatch);
        }
        
        Ok(())
    }
}

// Update Profile Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProfileCommand {
    #[validate(length(max = 100))]
    pub company_name: Option<String>,
    
    #[validate(length(max = 50))]
    pub station_name: Option<String>,
}

impl UpdateProfileCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}

// Reset Password Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResetPasswordCommand {
    #[validate(email)]
    pub email: String,
}

impl ResetPasswordCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}

// Confirm Password Reset Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ConfirmPasswordResetCommand {
    #[validate(length(min = 1))]
    pub token: String,
    
    #[validate(length(min = 8))]
    pub new_password: String,
    
    #[validate(must_match = "new_password")]
    pub confirm_password: String,
}

impl ConfirmPasswordResetCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        if self.new_password != self.confirm_password {
            return Err(crate::application::error::ApplicationError::PasswordMismatch);
        }
        
        Ok(())
    }
}

// Verify Email Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VerifyEmailCommand {
    #[validate(length(min = 1))]
    pub token: String,
}

impl VerifyEmailCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}

// Resend Verification Command
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResendVerificationCommand {
    #[validate(email)]
    pub email: String,
}

impl ResendVerificationCommand {
    pub fn validate_command(&self) -> ApplicationResult<()> {
        self.validate()
            .map_err(|e| crate::application::error::ApplicationError::Validation(e.to_string()))?;
        
        Ok(())
    }
}