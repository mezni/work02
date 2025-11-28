use crate::domain::models::User;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::Role;

pub struct UserDomainService;

impl UserDomainService {
    pub fn check_permission(user: &User, required_role: Role) -> Result<(), DomainError> {
        if user.role.to_i32() >= required_role.to_i32() {
            Ok(())
        } else {
            Err(DomainError::InternalError("User does not have required permissions.".to_string()))
        }
    }

    pub fn validate_user_creation(username: &str, email: &str, password: &str) -> Result<(), DomainError> {
        if username.len() < 3 || username.len() > 50 {
            return Err(DomainError::InvalidUsername(username.to_string()));
        }

        if password.len() < 8 {
            return Err(DomainError::InvalidPassword("Password must be at least 8 characters long".to_string()));
        }

        Ok(())
    }

    pub fn can_modify_user(current_user: &User, target_user_id: uuid::Uuid, target_organisation_id: Option<uuid::Uuid>) -> bool {
        match current_user.role {
            Role::Admin => true,
            Role::Partner => {
                if let Some(org_id) = target_organisation_id {
                    current_user.organisation_id == Some(org_id)
                } else {
                    false
                }
            }
            Role::Operator => current_user.id == target_user_id,
            _ => false,
        }
    }
}
