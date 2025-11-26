use auth_service::domain::value_objects::{Password, PasswordError};

#[test]
fn test_valid_password() {
    let password = Password::new("SecurePass123!").unwrap();
    assert_eq!(password.as_str(), "SecurePass123!");
}

#[test]
fn test_password_too_short() {
    let result = Password::new("Short1!");
    assert!(matches!(result, Err(PasswordError::TooShort)));
}

#[test]
fn test_password_no_uppercase() {
    let result = Password::new("lowercase123!");
    assert!(matches!(result, Err(PasswordError::NoUppercase)));
}

#[test]
fn test_password_no_lowercase() {
    let result = Password::new("UPPERCASE123!");
    assert!(matches!(result, Err(PasswordError::NoLowercase)));
}

#[test]
fn test_password_no_digit() {
    let result = Password::new("NoDigitsHere!");
    assert!(matches!(result, Err(PasswordError::NoDigit)));
}

#[test]
fn test_password_no_special_char() {
    let result = Password::new("NoSpecial123");
    assert!(matches!(result, Err(PasswordError::NoSpecialChar)));
}

#[test]
fn test_password_display_masks_content() {
    let password = Password::new("MyPass123!").unwrap();
    let displayed = format!("{}", password);

    assert!(displayed.chars().all(|c| c == '*'));
    assert_eq!(displayed.len(), password.as_str().len());
    assert!(!displayed.contains("MyPass123!"));
}

#[test]
fn test_password_equality() {
    let pass1 = Password::new("SamePass123!").unwrap();
    let pass2 = Password::new("SamePass123!").unwrap();
    let pass3 = Password::new("Different123!").unwrap();

    assert_eq!(pass1, pass2);
    assert_ne!(pass1, pass3);
}

#[test]
fn test_password_clone() {
    let original = Password::new("ClonePass123!").unwrap();
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_password_serialization() {
    let password = Password::new("SerialPass123!").unwrap();
    let json = serde_json::to_string(&password).unwrap();
    let deserialized: Password = serde_json::from_str(&json).unwrap();
    assert_eq!(password, deserialized);
}

#[test]
fn test_password_hash_and_verify() {
    let password = Password::new("TestPass123!").unwrap();
    let hash = password.hash();

    assert!(password.verify(&hash));

    let wrong_password = Password::new("WrongPass123!").unwrap();
    assert!(!wrong_password.verify(&hash));
}

#[test]
fn test_edge_case_passwords() {
    assert!(Password::new("Ab1!defg").is_ok());
    assert!(Password::new("Abc1!🚀").is_ok());
}
