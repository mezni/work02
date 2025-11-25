use auth_service::domain::errors::DomainError;
use auth_service::domain::value_objects::Password;

#[test]
fn test_password_creation_success() {
    let password = Password::new("ValidPass123!").unwrap();
    assert!(!password.hash().is_empty());
}

// Replace parameterized tests with individual tests
#[test]
fn test_password_validation_failure_too_short() {
    let result = Password::new("short");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::InvalidPassword(_)
    ));
}

#[test]
fn test_password_validation_failure_no_uppercase() {
    let result = Password::new("nouppercase123!");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::InvalidPassword(_)
    ));
}

#[test]
fn test_password_validation_failure_no_lowercase() {
    let result = Password::new("NOLOWERCASE123!");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::InvalidPassword(_)
    ));
}

#[test]
fn test_password_validation_failure_no_digits() {
    let result = Password::new("NoDigits!");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::InvalidPassword(_)
    ));
}

#[test]
fn test_password_validation_failure_empty() {
    let result = Password::new("");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::InvalidPassword(_)
    ));
}

#[test]
fn test_password_validation_success_valid1() {
    let result = Password::new("ValidPass123!");
    assert!(result.is_ok());
}

#[test]
fn test_password_validation_success_valid2() {
    let result = Password::new("AnotherValid1!");
    assert!(result.is_ok());
}

#[test]
fn test_password_validation_success_valid3() {
    let result = Password::new("TestPassword456!");
    assert!(result.is_ok());
}

#[test]
fn test_password_verification() {
    let plain_password = "ValidPass123!";
    let password = Password::new(plain_password).unwrap();

    // Correct password should verify
    assert!(password.verify(plain_password));

    // Wrong password should not verify
    assert!(!password.verify("WrongPass123!"));

    // Empty password should not verify
    assert!(!password.verify(""));
}

#[test]
fn test_password_from_hash() {
    let original_password = Password::new("ValidPass123!").unwrap();
    let hash = original_password.hash().to_string();

    let password_from_hash = Password::from_hash(hash.clone());
    assert_eq!(password_from_hash.hash(), hash);

    // Should verify with original password
    assert!(password_from_hash.verify("ValidPass123!"));
}

#[test]
fn test_password_try_from_string() {
    let password: Password = "ValidPass123!".to_string().try_into().unwrap();
    assert!(password.verify("ValidPass123!"));

    let result: Result<Password, DomainError> = "short".to_string().try_into();
    assert!(result.is_err());
}

#[test]
fn test_password_hash_uniqueness() {
    let password1 = Password::new("ValidPass123!").unwrap();
    let password2 = Password::new("ValidPass123!").unwrap();

    // Same password should generate different hashes (due to salt)
    assert_ne!(password1.hash(), password2.hash());

    // But both should verify correctly
    assert!(password1.verify("ValidPass123!"));
    assert!(password2.verify("ValidPass123!"));
}

#[test]
fn test_password_complexity_requirements() {
    // Test missing uppercase
    let result = Password::new("lowercase123!");
    assert!(result.is_err());

    // Test missing lowercase
    let result = Password::new("UPPERCASE123!");
    assert!(result.is_err());

    // Test missing digits
    let result = Password::new("NoDigitsHere!");
    assert!(result.is_err());

    // Test valid with all requirements
    let result = Password::new("Valid123Password!");
    assert!(result.is_ok());
}

#[test]
fn test_password_length_limits() {
    // Test minimum length
    let result = Password::new("Ab1!defg"); // 8 characters
    assert!(result.is_ok());

    // Test too short
    let result = Password::new("Ab1!def"); // 7 characters
    assert!(result.is_err());

    // Test very long password (should work)
    let long_password = "A".repeat(50) + "b1!";
    let result = Password::new(&long_password);
    assert!(result.is_ok());
}

#[test]
fn test_password_whitespace_handling() {
    // Passwords with leading/trailing whitespace should be invalid
    let result = Password::new("  ValidPass123!  ");
    assert!(result.is_err());

    let result = Password::new("ValidPass123! ");
    assert!(result.is_err());

    let result = Password::new(" ValidPass123!");
    assert!(result.is_err());

    // Passwords with internal whitespace should also be invalid
    let result = Password::new("Valid Pass123!");
    assert!(result.is_err());

    let result = Password::new("Valid\tPass123!");
    assert!(result.is_err());

    let result = Password::new("Valid\nPass123!");
    assert!(result.is_err());
}
