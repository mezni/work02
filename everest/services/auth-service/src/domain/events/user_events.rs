use serde::{Deserialize, Serialize};
use crate::domain::value_objects::{UserId, Email};
use chrono::{DateTime, Utc};

/// User-related domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserEvent {
    UserRegistered(UserRegistered),
    UserEmailVerified(UserEmailVerified),
    UserProfileUpdated(UserProfileUpdated),
    UserPasswordChanged(UserPasswordChanged),
    UserDeactivated(UserDeactivated),
    UserDeleted(UserDeleted),
    UserReactivated(UserReactivated),
}

/// Event emitted when a new user registers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistered {
    pub user_id: UserId,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub registered_at: DateTime<Utc>,
    pub source: RegistrationSource,
}

/// Event emitted when user verifies their email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEmailVerified {
    pub user_id: UserId,
    pub email: Email,
    pub verified_at: DateTime<Utc>,
}

/// Event emitted when user updates their profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUpdated {
    pub user_id: UserId,
    pub old_first_name: String,
    pub new_first_name: String,
    pub old_last_name: String,
    pub new_last_name: String,
    pub old_username: Option<String>,
    pub new_username: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// Event emitted when user changes password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPasswordChanged {
    pub user_id: UserId,
    pub changed_at: DateTime<Utc>,
    pub initiated_by: ChangeInitiator,
}

/// Event emitted when user is deactivated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeactivated {
    pub user_id: UserId,
    pub deactivated_at: DateTime<Utc>,
    pub reason: DeactivationReason,
    pub initiated_by: ChangeInitiator,
}

/// Event emitted when user is deleted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeleted {
    pub user_id: UserId,
    pub email: Email,
    pub deleted_at: DateTime<Utc>,
    pub reason: DeletionReason,
    pub initiated_by: ChangeInitiator,
}

/// Event emitted when user is reactivated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReactivated {
    pub user_id: UserId,
    pub reactivated_at: DateTime<Utc>,
    pub initiated_by: ChangeInitiator,
}

/// Source of user registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationSource {
    WebForm,
    OAuthGoogle,
    OAuthGithub,
    OAuthMicrosoft,
    AdminCreation,
    Api,
    MobileApp,
}

/// Who initiated the change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeInitiator {
    User,
    Admin,
    System,
}

/// Reason for user deactivation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeactivationReason {
    UserRequest,
    Inactivity,
    PolicyViolation,
    SuspiciousActivity,
    AdminDecision,
    PaymentIssue,
}

/// Reason for user deletion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeletionReason {
    UserRequest,
    AdminDecision,
    LegalRequirement,
    DataPurgePolicy,
}