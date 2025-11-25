use auth_service::domain::enums::{AuditAction, UserRole};

#[test]
fn test_user_role_display() {
    assert_eq!(UserRole::Admin.to_string(), "admin");
    assert_eq!(UserRole::Partner.to_string(), "partner");
    assert_eq!(UserRole::Operator.to_string(), "operator");
    assert_eq!(UserRole::User.to_string(), "user");
    assert_eq!(UserRole::Guest.to_string(), "guest");
}

#[test]
fn test_user_role_parsing() {
    // Remove the std::str::FromStr import since we're not using it
    assert_eq!(UserRole::from_str("admin"), Some(UserRole::Admin));
    assert_eq!(UserRole::from_str("partner"), Some(UserRole::Partner));
    assert_eq!(UserRole::from_str("operator"), Some(UserRole::Operator));
    assert_eq!(UserRole::from_str("user"), Some(UserRole::User));
    assert_eq!(UserRole::from_str("guest"), Some(UserRole::Guest));

    // Use is_none() for invalid role
    assert!(UserRole::from_str("invalid").is_none());
}

#[test]
fn test_audit_action_display() {
    assert_eq!(AuditAction::UserRegistered.to_string(), "USER_REGISTERED");
    assert_eq!(AuditAction::UserLoggedIn.to_string(), "USER_LOGGED_IN");
    assert_eq!(AuditAction::CompanyCreated.to_string(), "COMPANY_CREATED");
}

#[test]
fn test_user_role_permissions() {
    assert!(UserRole::Admin.can_manage_users());
    assert!(UserRole::Admin.can_manage_companies());

    assert!(UserRole::Partner.can_manage_users());
    assert!(!UserRole::Partner.can_manage_companies());

    assert!(UserRole::Operator.can_manage_users());
    assert!(!UserRole::Operator.can_manage_companies());

    assert!(!UserRole::User.can_manage_users());
    assert!(!UserRole::User.can_manage_companies());

    assert!(!UserRole::Guest.can_manage_users());
    assert!(!UserRole::Guest.can_manage_companies());
}

#[test]
fn test_user_role_all() {
    let all_roles = UserRole::all();
    assert_eq!(all_roles.len(), 5);
    assert!(all_roles.contains(&UserRole::Admin));
    assert!(all_roles.contains(&UserRole::Partner));
    assert!(all_roles.contains(&UserRole::Operator));
    assert!(all_roles.contains(&UserRole::User));
    assert!(all_roles.contains(&UserRole::Guest));
}
