// tests/unit/value_objects/email_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::Email;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_email() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_is_normalized_to_lowercase() {
        let email = Email::new("USER@EXAMPLE.COM").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_trim_whitespace() {
        let email = Email::new("  user@example.com  ").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_rejects_empty_string() {
        let result = Email::new("");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_missing_at_symbol() {
        let result = Email::new("userexample.com");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_missing_username() {
        let result = Email::new("@example.com");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_missing_domain() {
        let result = Email::new("user@");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_invalid_domain_format() {
        let result = Email::new("user@example");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_equality() {
        let email1 = Email::new("user@example.com").unwrap();
        let email2 = Email::new("USER@EXAMPLE.COM").unwrap();
        assert_eq!(email1, email2);
    }

    #[test]
    fn test_email_display() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(format!("{}", email), "user@example.com");
    }

    #[test]
    fn test_email_into_string() {
        let email = Email::new("user@example.com").unwrap();
        let string: String = email.into();
        assert_eq!(string, "user@example.com");
    }
}