use serde::{Deserialize, Serialize};
use crate::domain::value_objects::{UserId, Email};
use chrono::{DateTime, Utc};

/// Authentication-related domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthEvent {
    UserLoggedIn(UserLoggedIn),
    UserLoggedOut(UserLoggedOut),
    LoginFailed(LoginFailed),
    PasswordResetRequested(PasswordResetRequested),
    PasswordResetCompleted(PasswordResetCompleted),
    MfaEnabled(MfaEnabled),
    MfaDisabled(MfaDisabled),
    MfaFailed(MfaFailed),
    SessionRevoked(SessionRevoked),
    SuspiciousActivityDetected(SuspiciousActivityDetected),
    AccountLocked(AccountLocked),
    AccountUnlocked(AccountUnlocked),
}

/// Event emitted when user successfully logs in
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedIn {
    pub user_id: UserId,
    pub email: Email,
    pub login_at: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: String,
    pub auth_method: AuthMethod,
    pub device_fingerprint: String,
}

/// Event emitted when user logs out
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedOut {
    pub user_id: UserId,
    pub logout_at: DateTime<Utc>,
    pub session_duration_seconds: u64,
    pub initiated_by: LogoutInitiator,
}

/// Event emitted when login fails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginFailed {
    pub email: Email,
    pub failed_at: DateTime<Utc>,
    pub ip_address: String,
    pub failure_reason: LoginFailureReason,
    pub consecutive_failures: u32,
}

/// Event emitted when password reset is requested
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequested {
    pub user_id: UserId,
    pub email: Email,
    pub requested_at: DateTime<Utc>,
    pub reset_token_hash: String,
    pub initiated_by: ResetInitiator,
}

/// Event emitted when password reset is completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetCompleted {
    pub user_id: UserId,
    pub reset_at: DateTime<Utc>,
    pub ip_address: String,
}

/// Event emitted when MFA is enabled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaEnabled {
    pub user_id: UserId,
    pub enabled_at: DateTime<Utc>,
    pub mfa_type: MfaType,
}

/// Event emitted when MFA is disabled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaDisabled {
    pub user_id: UserId,
    pub disabled_at: DateTime<Utc>,
    pub mfa_type: MfaType,
    pub initiated_by: ChangeInitiator,
}

/// Event emitted when MFA verification fails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaFailed {
    pub user_id: UserId,
    pub failed_at: DateTime<Utc>,
    pub mfa_type: MfaType,
    pub failure_reason: MfaFailureReason,
}

/// Event emitted when session is revoked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRevoked {
    pub user_id: UserId,
    pub session_id: String,
    pub revoked_at: DateTime<Utc>,
    pub reason: SessionRevocationReason,
    pub initiated_by: ChangeInitiator,
}

/// Event emitted when suspicious activity is detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousActivityDetected {
    pub user_id: UserId,
    pub detected_at: DateTime<Utc>,
    pub activity_type: SuspiciousActivityType,
    pub ip_address: String,
    pub user_agent: String,
    pub severity: SeverityLevel,
}

/// Event emitted when account is locked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLocked {
    pub user_id: UserId,
    pub locked_at: DateTime<Utc>,
    pub reason: LockReason,
    pub lock_duration_minutes: Option<u32>, // None for manual unlock
}

/// Event emitted when account is unlocked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUnlocked {
    pub user_id: UserId,
    pub unlocked_at: DateTime<Utc>,
    pub initiated_by: ChangeInitiator,
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Password,
    OAuthGoogle,
    OAuthGithub,
    OAuthMicrosoft,
    MagicLink,
    Biometric,
}

/// Who initiated logout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogoutInitiator {
    User,
    System, // Auto-logout, session expiry
    Admin,
}

/// Login failure reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoginFailureReason {
    InvalidCredentials,
    AccountLocked,
    AccountDeactivated,
    MfaRequired,
    MfaFailed,
    DeviceNotRecognized,
    LocationSuspicious,
}

/// Password reset initiator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResetInitiator {
    User,
    Admin,
}

/// MFA types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaType {
    Totp,    // Time-based OTP
    Sms,     // SMS codes
    Email,   // Email codes
    WebAuthn, // Hardware keys
    BackupCodes,
}

/// MFA failure reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaFailureReason {
    InvalidCode,
    ExpiredCode,
    MaxAttemptsExceeded,
    DeviceNotRegistered,
}

/// Session revocation reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionRevocationReason {
    UserRequest,
    SecurityPolicy,
    SuspiciousActivity,
    PasswordChange,
    AdminAction,
    DeviceCompromise,
}

/// Suspicious activity types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuspiciousActivityType {
    MultipleFailedLogins,
    LoginFromNewLocation,
    LoginFromNewDevice,
    UnusualLoginTime,
    HighVelocityRequests,
    KnownMaliciousIp,
}

/// Severity levels for security events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeverityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Account lock reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LockReason {
    TooManyFailedAttempts,
    SuspiciousActivity,
    AdminManualLock,
    SecurityBreach,
    PaymentIssues,
}