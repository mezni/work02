use auth_service::domain::entities::User;
use auth_service::domain::enums::UserRole;
use auth_service::domain::value_objects::{Email, Password};
use uuid::Uuid;

#[test]
fn test_email_validation() {
    // Valid email
    let email = Email::new("test@example.com".to_string());
    assert!(email.is_ok());
    
    // Invalid email
    let email = Email::new("invalid-email".to_string());
    assert!(email.is_err());
}

#[test]
fn test_password_validation() {
    // Valid password
    let password = Password::new("password123".to_string());
    assert!(password.is_ok());
    
    // Too short password
    let password = Password::new("short".to_string());
    assert!(password.is_err());
}

#[test]
fn test_user_creation() {
    let user = User::new(
        "keycloak-123".to_string(),
        "testuser".to_string(),
        "test@example.com".to_string(),
        UserRole::User,
        None,
    );
    
    assert!(user.is_ok());
    let user = user.unwrap();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    assert!(matches!(user.role, UserRole::User));
    assert!(user.company_id.is_none());
}

#[test]
fn test_user_permissions() {
    let admin_user = User::new(
        "keycloak-admin".to_string(),
        "admin".to_string(),
        "admin@example.com".to_string(),
        UserRole::Admin,
        None,
    ).unwrap();
    
    let partner_user = User::new(
        "keycloak-partner".to_string(),
        "partner".to_string(),
        "partner@example.com".to_string(),
        UserRole::Partner,
        Some(Uuid::new_v4()),
    ).unwrap();
    
    let regular_user = User::new(
        "keycloak-user".to_string(),
        "user".to_string(),
        "user@example.com".to_string(),
        UserRole::User,
        None,
    ).unwrap();
    
    assert!(admin_user.is_admin());
    assert!(partner_user.is_partner());
    assert!(regular_user.is_regular_user());
    
    let test_company_id = Uuid::new_v4();
    assert!(admin_user.can_manage_company(test_company_id));
    assert!(partner_user.can_manage_company(partner_user.company_id.unwrap()));
    assert!(!regular_user.can_manage_company(test_company_id));
}

#[test]
fn test_user_validation() {
    // Test invalid user creation
    let invalid_user = User::new(
        "".to_string(),
        "".to_string(),
        "invalid-email".to_string(),
        UserRole::User,
        None,
    );
    
    assert!(invalid_user.is_err());
}

#[test]
fn test_enum_conversions() {
    // Test UserRole from string
    assert_eq!("admin".parse::<UserRole>().unwrap(), UserRole::Admin);
    assert_eq!("partner".parse::<UserRole>().unwrap(), UserRole::Partner);
    assert_eq!("operator".parse::<UserRole>().unwrap(), UserRole::Operator);
    assert_eq!("user".parse::<UserRole>().unwrap(), UserRole::User);
    assert_eq!("guest".parse::<UserRole>().unwrap(), UserRole::Guest);
    
    // Test invalid role
    assert!("invalid".parse::<UserRole>().is_err());
    
    // Test display
    assert_eq!(UserRole::Admin.to_string(), "Admin");
    assert_eq!(UserRole::Partner.to_string(), "Partner");
}
