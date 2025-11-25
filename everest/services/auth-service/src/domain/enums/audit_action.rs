use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, ToSchema)]
pub enum AuditAction {
    // Authentication actions
    #[strum(serialize = "USER_REGISTERED")]
    UserRegistered,

    #[strum(serialize = "USER_LOGGED_IN")]
    UserLoggedIn,

    #[strum(serialize = "USER_LOGGED_OUT")]
    UserLoggedOut,

    #[strum(serialize = "USER_PASSWORD_CHANGED")]
    UserPasswordChanged,

    #[strum(serialize = "USER_PASSWORD_RESET_REQUESTED")]
    UserPasswordResetRequested,

    // User management actions
    #[strum(serialize = "USER_CREATED")]
    UserCreated,

    #[strum(serialize = "USER_UPDATED")]
    UserUpdated,

    #[strum(serialize = "USER_DELETED")]
    UserDeleted,

    #[strum(serialize = "USER_ROLE_CHANGED")]
    UserRoleChanged,

    #[strum(serialize = "USER_COMPANY_ASSIGNED")]
    UserCompanyAssigned,

    #[strum(serialize = "USER_EMAIL_VERIFIED")]
    UserEmailVerified,

    // Company management actions
    #[strum(serialize = "COMPANY_CREATED")]
    CompanyCreated,

    #[strum(serialize = "COMPANY_UPDATED")]
    CompanyUpdated,

    #[strum(serialize = "COMPANY_DELETED")]
    CompanyDeleted,

    // Security actions
    #[strum(serialize = "UNAUTHORIZED_ACCESS_ATTEMPT")]
    UnauthorizedAccessAttempt,

    #[strum(serialize = "SUSPICIOUS_ACTIVITY")]
    SuspiciousActivity,
}
