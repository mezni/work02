use auth_service::domain::errors::DomainError;
use auth_service::domain::value_objects::Email;

#[test]
fn test_email_creation_success() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    assert_eq!(email.value(), "test@example.com");
}

#[test]
fn test_email_normalization() {
    let email = Email::new("  TEST@Example.COM  ".to_string()).unwrap();
    assert_eq!(email.value(), "test@example.com");
}

// Replace parameterized tests with individual tests
#[test]
fn test_email_validation_failure_invalid() {
    let result = Email::new("invalid-email".to_string());
    assert!(result.is_err());
}

#[test]
fn test_email_validation_failure_no_at_symbol() {
    let result = Email::new("missingdomain.com".to_string());
    assert!(result.is_err());
}

#[test]
fn test_email_validation_failure_no_domain() {
    let result = Email::new("user@".to_string());
    assert!(result.is_err());
}

#[test]
fn test_email_validation_failure_spaces() {
    let result = Email::new("spaces in@email.com".to_string());
    assert!(result.is_err());
}

#[test]
fn test_email_validation_failure_empty() {
    let result = Email::new("".to_string());
    assert!(result.is_err());
}

#[test]
fn test_email_validation_failure_whitespace() {
    let result = Email::new("   ".to_string());
    assert!(result.is_err());
}

#[test]
fn test_email_validation_failure_multiple_at() {
    let result = Email::new("user@@example.com".to_string());
    assert!(result.is_err());
}

#[test]
fn test_email_validation_success_valid1() {
    let result = Email::new("valid@example.com".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_email_validation_success_valid2() {
    let result = Email::new("user.name@domain.co.uk".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_email_validation_success_valid3() {
    let result = Email::new("user+tag@example.org".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_email_validation_success_valid4() {
    let result = Email::new("user.name+tag@sub.domain.com".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_email_try_from_string() {
    let email: Email = "test@example.com".to_string().try_into().unwrap();
    assert_eq!(email.value(), "test@example.com");

    let result: Result<Email, DomainError> = "invalid".to_string().try_into();
    assert!(result.is_err());
}

#[test]
fn test_email_into_string() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    let string: String = email.into();
    assert_eq!(string, "test@example.com");
}

#[test]
fn test_email_as_ref() {
    let email = Email::new("test@example.com".to_string()).unwrap();
    assert_eq!(email.as_ref(), "test@example.com");
}

#[test]
fn test_email_equality() {
    let email1 = Email::new("test@example.com".to_string()).unwrap();
    let email2 = Email::new("TEST@EXAMPLE.COM".to_string()).unwrap();
    let email3 = Email::new("other@example.com".to_string()).unwrap();

    assert_eq!(email1, email2); // Case insensitive equality
    assert_ne!(email1, email3);
}

#[test]
fn test_email_validation_edge_cases() {
    // These should be valid according to email standards
    assert!(Email::new("user@localhost".to_string()).is_ok());
    assert!(Email::new("user@127.0.0.1".to_string()).is_ok());
    assert!(Email::new("user@example".to_string()).is_ok()); // Single word domain

    // These should be invalid
    assert!(Email::new("user@.com".to_string()).is_err());
    assert!(Email::new("@example.com".to_string()).is_err());
    assert!(Email::new("user@example..com".to_string()).is_err());
}
