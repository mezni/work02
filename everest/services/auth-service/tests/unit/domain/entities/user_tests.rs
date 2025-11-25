use auth_service::domain::entities::User;
use auth_service::domain::enums::UserRole;
use auth_service::domain::errors::DomainError;
use auth_service::domain::value_objects::Email;
use uuid::Uuid;

#[test]
fn test_user_creation_success() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::User,
    )
    .unwrap();

    assert_eq!(user.username, "testuser");
    assert_eq!(user.role, UserRole::User);
    assert_eq!(user.email_verified, false);
    assert!(user.company_id.is_none());
}

#[test]
fn test_user_role_checks() {
    let email = Email::new("admin@example.com".to_string()).unwrap();
    let admin_user = User::new(
        "keycloak-admin".to_string(),
        "admin".to_string(),
        email,
        UserRole::Admin,
    )
    .unwrap();

    assert!(admin_user.is_admin());
    assert!(!admin_user.is_user());
    // Use role methods instead of user methods
    assert!(admin_user.role.can_manage_users());
    assert!(admin_user.role.can_manage_companies());
}

// Individual tests to replace parameterized test
#[test]
fn test_user_can_manage_users_admin() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::Admin,
    )
    .unwrap();
    assert!(user.role.can_manage_users());
}

#[test]
fn test_user_can_manage_users_partner() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::Partner,
    )
    .unwrap();
    assert!(user.role.can_manage_users());
}

#[test]
fn test_user_can_manage_users_operator() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::Operator,
    )
    .unwrap();
    assert!(user.role.can_manage_users());
}

#[test]
fn test_user_can_manage_users_user() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::User,
    )
    .unwrap();
    assert!(!user.role.can_manage_users());
}

#[test]
fn test_user_can_manage_users_guest() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::Guest,
    )
    .unwrap();
    assert!(!user.role.can_manage_users());
}

#[test]
fn test_user_company_access() {
    let company_id = Uuid::new_v4();
    let email = Email::new("partner@example.com".to_string()).unwrap();
    let mut partner_user = User::new(
        "keycloak-partner".to_string(),
        "partner".to_string(),
        email,
        UserRole::Partner,
    )
    .unwrap();

    // Partner without company cannot access any company
    assert!(!partner_user.has_company_access(&company_id));

    // Assign company to partner
    partner_user.assign_to_company(company_id).unwrap();
    assert!(partner_user.has_company_access(&company_id));

    // Admin can access any company even without assignment
    let admin_email = Email::new("admin@example.com".to_string()).unwrap();
    let admin_user = User::new(
        "keycloak-admin".to_string(),
        "admin".to_string(),
        admin_email,
        UserRole::Admin,
    )
    .unwrap();

    assert!(admin_user.has_company_access(&company_id));
    assert!(admin_user.has_company_access(&Uuid::new_v4()));
}

#[test]
fn test_user_role_change() {
    let company_id = Uuid::new_v4();
    let email = Email::new("test@example.com".to_string()).unwrap();
    let mut user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::Partner,
    )
    .unwrap();

    // Assign to company first
    user.assign_to_company(company_id).unwrap();
    assert!(user.company_id.is_some());

    // Change to user role (should remove company assignment)
    user.change_role(UserRole::User).unwrap();
    assert!(user.company_id.is_none());
    assert!(user.is_user());

    // Try to change to company role without company (should fail)
    let result = user.change_role(UserRole::Partner);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::InvalidOperation(_)
    ));
}

#[test]
fn test_user_email_verification() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let mut user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::User,
    )
    .unwrap();

    assert!(!user.email_verified);

    user.mark_email_verified();
    assert!(user.email_verified);
}

#[test]
fn test_user_company_assignment_validation() {
    let email = Email::new("user@example.com".to_string()).unwrap();
    let mut user = User::new(
        "keycloak-123".to_string(),
        "user".to_string(),
        email,
        UserRole::User,
    )
    .unwrap();

    // User role cannot be assigned to company
    let result = user.assign_to_company(Uuid::new_v4());
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::InvalidOperation(_)
    ));
}

#[test]
fn test_user_from_str() {
    assert_eq!(UserRole::from_str("admin"), Some(UserRole::Admin));
    assert_eq!(UserRole::from_str("partner"), Some(UserRole::Partner));
    assert_eq!(UserRole::from_str("operator"), Some(UserRole::Operator));
    assert_eq!(UserRole::from_str("user"), Some(UserRole::User));
    assert_eq!(UserRole::from_str("guest"), Some(UserRole::Guest));
    assert!(UserRole::from_str("invalid").is_none()); // Use is_none() for Option

    // Case insensitive
    assert_eq!(UserRole::from_str("ADMIN"), Some(UserRole::Admin));
    assert_eq!(UserRole::from_str("Admin"), Some(UserRole::Admin));
}

#[test]
fn test_user_role_company_scoped() {
    assert!(UserRole::Admin.is_company_scoped());
    assert!(UserRole::Partner.is_company_scoped());
    assert!(UserRole::Operator.is_company_scoped());
    assert!(!UserRole::User.is_company_scoped());
    assert!(!UserRole::Guest.is_company_scoped());
}

#[test]
fn test_user_role_display() {
    assert_eq!(UserRole::Admin.to_string(), "admin");
    assert_eq!(UserRole::Partner.to_string(), "partner");
    assert_eq!(UserRole::Operator.to_string(), "operator");
    assert_eq!(UserRole::User.to_string(), "user");
    assert_eq!(UserRole::Guest.to_string(), "guest");
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

#[test]
fn test_user_creation_with_different_roles() {
    // Test Admin role
    let admin_email = Email::new("admin@example.com".to_string()).unwrap();
    let admin_user = User::new(
        "keycloak-admin".to_string(),
        "admin".to_string(),
        admin_email,
        UserRole::Admin,
    )
    .unwrap();
    assert!(admin_user.is_admin());
    assert!(admin_user.role.can_manage_companies()); // Use role method

    // Test Partner role
    let partner_email = Email::new("partner@example.com".to_string()).unwrap();
    let partner_user = User::new(
        "keycloak-partner".to_string(),
        "partner".to_string(),
        partner_email,
        UserRole::Partner,
    )
    .unwrap();
    assert!(partner_user.is_partner());
    assert!(!partner_user.role.can_manage_companies()); // Use role method

    // Test Operator role
    let operator_email = Email::new("operator@example.com".to_string()).unwrap();
    let operator_user = User::new(
        "keycloak-operator".to_string(),
        "operator".to_string(),
        operator_email,
        UserRole::Operator,
    )
    .unwrap();
    assert!(operator_user.is_operator());
    assert!(!operator_user.role.can_manage_companies()); // Use role method

    // Test User role
    let user_email = Email::new("user@example.com".to_string()).unwrap();
    let user_user = User::new(
        "keycloak-user".to_string(),
        "user".to_string(),
        user_email,
        UserRole::User,
    )
    .unwrap();
    assert!(user_user.is_user());
    assert!(!user_user.role.can_manage_users()); // Use role method

    // Test Guest role
    let guest_email = Email::new("guest@example.com".to_string()).unwrap();
    let guest_user = User::new(
        "keycloak-guest".to_string(),
        "guest".to_string(),
        guest_email,
        UserRole::Guest,
    )
    .unwrap();
    assert!(guest_user.is_guest());
    assert!(!guest_user.role.can_manage_users()); // Use role method
}

#[test]
fn test_user_company_removal() {
    let company_id = Uuid::new_v4();
    let email = Email::new("test@example.com".to_string()).unwrap();
    let mut user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::Partner,
    )
    .unwrap();

    // Assign to company
    user.assign_to_company(company_id).unwrap();
    assert!(user.company_id.is_some());

    // Remove from company
    user.remove_from_company();
    assert!(user.company_id.is_none());
}

#[test]
fn test_user_timestamps() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::User,
    )
    .unwrap();

    // Check that created_at and updated_at are set and equal initially
    assert!(user.created_at <= chrono::Utc::now());
    assert!(user.updated_at <= chrono::Utc::now());
    assert_eq!(user.created_at, user.updated_at);
}

#[test]
fn test_user_role_change_with_company() {
    let company_id = Uuid::new_v4();
    let email = Email::new("test@example.com".to_string()).unwrap();
    let mut user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        email,
        UserRole::Partner,
    )
    .unwrap();

    // Assign to company
    user.assign_to_company(company_id).unwrap();

    // Change to another company-scoped role (should keep company)
    user.change_role(UserRole::Operator).unwrap();
    assert!(user.company_id.is_some());
    assert!(user.is_operator());

    // Change to Admin (should keep company)
    user.change_role(UserRole::Admin).unwrap();
    assert!(user.company_id.is_some());
    assert!(user.is_admin());
}

#[test]
fn test_user_self_management() {
    let email = Email::new("user@example.com".to_string()).unwrap();
    let user = User::new(
        "keycloak-123".to_string(),
        "user".to_string(),
        email,
        UserRole::User,
    )
    .unwrap();

    // User can always manage themselves
    assert!(user.can_manage_user(&user));
}

#[test]
fn test_user_without_company_cannot_manage_others() {
    let email = Email::new("partner@example.com".to_string()).unwrap();
    let partner_without_company = User::new(
        "keycloak-partner".to_string(),
        "partner".to_string(),
        email,
        UserRole::Partner,
    )
    .unwrap();

    let target_email = Email::new("target@example.com".to_string()).unwrap();
    let target_user = User::new(
        "keycloak-target".to_string(),
        "target".to_string(),
        target_email,
        UserRole::User,
    )
    .unwrap();

    // Partner without company cannot manage any user
    assert!(!partner_without_company.can_manage_user(&target_user));
}

#[test]
fn test_user_can_manage_other_users() {
    let company_id = Uuid::new_v4();

    // Create admin user
    let admin_email = Email::new("admin@example.com".to_string()).unwrap();
    let admin_user = User::new(
        "keycloak-admin".to_string(),
        "admin".to_string(),
        admin_email,
        UserRole::Admin,
    )
    .unwrap();

    // Create partner user in same company
    let partner_email = Email::new("partner@example.com".to_string()).unwrap();
    let mut partner_user = User::new(
        "keycloak-partner".to_string(),
        "partner".to_string(),
        partner_email,
        UserRole::Partner,
    )
    .unwrap();
    partner_user.assign_to_company(company_id).unwrap();

    // Create target user - DON'T assign to company since User role can't be in company
    let target_email = Email::new("target@example.com".to_string()).unwrap();
    let target_user = User::new(
        "keycloak-target".to_string(),
        "target".to_string(),
        target_email,
        UserRole::User,
    )
    .unwrap();
    // Remove this line: target_user.assign_to_company(company_id).unwrap();

    // Admin can manage any user
    assert!(admin_user.can_manage_user(&target_user));
    assert!(admin_user.can_manage_user(&partner_user));

    // Partner can manage users in same company? No, because target user is not in company
    // Partner can only manage users that are in their company
    assert!(!partner_user.can_manage_user(&target_user));

    // User can only manage themselves
    assert!(target_user.can_manage_user(&target_user));
    assert!(!target_user.can_manage_user(&partner_user));
}

#[test]
fn test_user_cannot_manage_different_company_users() {
    let company1_id = Uuid::new_v4();
    let company2_id = Uuid::new_v4();

    // Create partner in company 1
    let partner_email = Email::new("partner1@example.com".to_string()).unwrap();
    let mut partner1 = User::new(
        "keycloak-partner1".to_string(),
        "partner1".to_string(),
        partner_email,
        UserRole::Partner,
    )
    .unwrap();
    partner1.assign_to_company(company1_id).unwrap();

    // Create partner in company 2
    let partner2_email = Email::new("partner2@example.com".to_string()).unwrap();
    let mut partner2 = User::new(
        "keycloak-partner2".to_string(),
        "partner2".to_string(),
        partner2_email,
        UserRole::Partner,
    )
    .unwrap();
    partner2.assign_to_company(company2_id).unwrap();

    // Create operator in company 1 (use Operator instead of User role for company assignment)
    let operator_email = Email::new("operator1@example.com".to_string()).unwrap();
    let mut operator1 = User::new(
        "keycloak-operator1".to_string(),
        "operator1".to_string(),
        operator_email,
        UserRole::Operator,
    )
    .unwrap();
    operator1.assign_to_company(company1_id).unwrap();

    // Partner 1 can manage operator in same company
    assert!(partner1.can_manage_user(&operator1));

    // Partner 1 cannot manage partner in different company
    assert!(!partner1.can_manage_user(&partner2));

    // Partner 2 cannot manage operator in different company
    assert!(!partner2.can_manage_user(&operator1));
}
