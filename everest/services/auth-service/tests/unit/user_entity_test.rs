use auth_service::domain::entities::User;
use auth_service::domain::value_objects::{Email, Password};
use chrono::{Duration, Utc};

#[test]
fn test_user_creation() {
    let email = Email::new("test@example.com").unwrap();
    let password = Password::new("SecurePass123!").unwrap();
    let user = User::new(
        email.clone(),
        password,
        "John".to_string(),
        "Doe".to_string(),
        Some("johndoe".to_string()),
    );

    assert_eq!(user.email(), &email);
    assert_eq!(user.first_name(), "John");
    assert_eq!(user.last_name(), "Doe");
    assert_eq!(user.username(), Some("johndoe"));
    assert!(!user.email_verified());
    assert!(user.active());
    assert!(user.roles().is_empty());
}

#[test]
fn test_user_identity_based_equality() {
    let email = Email::new("test@example.com").unwrap();
    let password = Password::new("SecurePass123!").unwrap();

    let user1 = User::new(
        email.clone(),
        password.clone(),
        "John".to_string(),
        "Doe".to_string(),
        None,
    );

    let user2 = User::new(
        email.clone(),
        password,
        "Jane".to_string(),  // Different name
        "Smith".to_string(), // Different last name
        None,
    );

    // Different users should not be equal (different IDs)
    assert_ne!(user1, user2);

    // Same identity check should be false
    assert!(!user1.same_identity(&user2));

    // Same user should be equal to itself
    assert_eq!(user1, user1);
    assert!(user1.same_identity(&user1));
}

#[test]
fn test_user_email_verification() {
    let email = Email::new("test@example.com").unwrap();
    let password = Password::new("SecurePass123!").unwrap();
    let mut user = User::new(email, password, "John".to_string(), "Doe".to_string(), None);

    assert!(!user.email_verified());

    user.verify_email();

    assert!(user.email_verified());
    // Updated at should be after creation
    assert!(user.updated_at() > user.created_at());
}

#[test]
fn test_user_profile_update() {
    let email = Email::new("test@example.com").unwrap();
    let password = Password::new("SecurePass123!").unwrap();
    let mut user = User::new(
        email,
        password,
        "John".to_string(),
        "Doe".to_string(),
        Some("johndoe".to_string()),
    );

    let original_updated_at = user.updated_at();

    user.update_profile(
        "Jane".to_string(),
        "Smith".to_string(),
        Some("janesmith".to_string()),
    );

    assert_eq!(user.first_name(), "Jane");
    assert_eq!(user.last_name(), "Smith");
    assert_eq!(user.username(), Some("janesmith"));
    assert!(user.updated_at() > original_updated_at);
}

#[test]
fn test_user_password_change() {
    let email = Email::new("test@example.com").unwrap();
    let old_password = Password::new("OldPass123!").unwrap();
    let new_password = Password::new("NewPass456!").unwrap();

    let mut user = User::new(
        email,
        old_password.clone(),
        "John".to_string(),
        "Doe".to_string(),
        None,
    );

    // Store the verification state before password change
    let verifies_with_old = user.verify_password(&old_password);

    user.change_password(new_password.clone());

    // Should be able to verify with new password
    assert!(user.verify_password(&new_password));
    // Should not verify with old password
    assert!(!user.verify_password(&old_password));
    // Should have changed from original state
    assert_ne!(user.verify_password(&old_password), verifies_with_old);
}

#[test]
fn test_user_login_recording() {
    let email = Email::new("test@example.com").unwrap();
    let password = Password::new("SecurePass123!").unwrap();
    let mut user = User::new(email, password, "John".to_string(), "Doe".to_string(), None);

    assert!(user.last_login_at().is_none());

    user.record_login();

    assert!(user.last_login_at().is_some());
    assert!(user.updated_at() > user.created_at());
}

#[test]
fn test_user_activation_deactivation() {
    let email = Email::new("test@example.com").unwrap();
    let password = Password::new("SecurePass123!").unwrap();
    let mut user = User::new(email, password, "John".to_string(), "Doe".to_string(), None);

    assert!(user.active());

    user.deactivate();
    assert!(!user.active());

    user.activate();
    assert!(user.active());
}

#[test]
fn test_user_role_management() {
    let email = Email::new("test@example.com").unwrap();
    let password = Password::new("SecurePass123!").unwrap();
    let mut user = User::new(email, password, "John".to_string(), "Doe".to_string(), None);

    assert!(user.roles().is_empty());
    assert!(!user.has_role("admin"));

    // Assign role
    user.assign_role("admin".to_string());
    assert!(user.has_role("admin"));
    assert_eq!(user.roles().len(), 1);

    // Assign same role again (should be idempotent)
    user.assign_role("admin".to_string());
    assert_eq!(user.roles().len(), 1);

    // Assign another role
    user.assign_role("user".to_string());
    assert!(user.has_role("admin"));
    assert!(user.has_role("user"));
    assert_eq!(user.roles().len(), 2);

    // Revoke role
    assert!(user.revoke_role("admin"));
    assert!(!user.has_role("admin"));
    assert!(user.has_role("user"));
    assert_eq!(user.roles().len(), 1);

    // Revoke non-existent role
    assert!(!user.revoke_role("nonexistent"));
    assert_eq!(user.roles().len(), 1);
}

#[test]
fn test_user_can_login_validation() {
    let email = Email::new("test@example.com").unwrap();
    let password = Password::new("SecurePass123!").unwrap();

    // New user should not be able to login (email not verified)
    let mut user = User::new(
        email.clone(),
        password.clone(),
        "John".to_string(),
        "Doe".to_string(),
        None,
    );
    assert!(!user.can_login());

    // After email verification, should be able to login
    user.verify_email();
    assert!(user.can_login());

    // If deactivated, should not be able to login
    user.deactivate();
    assert!(!user.can_login());

    // Reactivated and verified should be able to login
    user.activate();
    assert!(user.can_login());
}

#[test]
fn test_user_password_verification() {
    let email = Email::new("test@example.com").unwrap();
    let correct_password = Password::new("CorrectPass123!").unwrap();
    let wrong_password = Password::new("WrongPass456!").unwrap();

    let user = User::new(
        email,
        correct_password.clone(),
        "John".to_string(),
        "Doe".to_string(),
        None,
    );

    assert!(user.verify_password(&correct_password));
    assert!(!user.verify_password(&wrong_password));
}

#[test]
fn test_user_immutable_identity() {
    let email = Email::new("test@example.com").unwrap();
    let password = Password::new("SecurePass123!").unwrap();
    let user = User::new(email, password, "John".to_string(), "Doe".to_string(), None);

    let original_id = user.id();

    // The ID should remain constant throughout the entity's lifetime
    assert_eq!(user.id(), original_id);
}
