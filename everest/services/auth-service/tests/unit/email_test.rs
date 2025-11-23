use auth_service::domain::value_objects::{Email, EmailError};

#[test]
fn test_valid_emails() {
    let valid_emails = [
        "user@example.com",
        "first.last@example.co.uk",
        "user+tag@example.org",
        "user.name@sub.domain.example.com",
        "123456@example.com",
        "user@example-domain.com",
        "_user@example.com",
        "user@example.museum",
        "user@example.io",
    ];

    for email_str in valid_emails {
        let email = Email::new(email_str).unwrap();
        assert_eq!(email.as_str(), email_str.to_lowercase());
    }
}

#[test]
fn test_invalid_emails() {
    let invalid_emails = [
        "",
        "invalid",
        "invalid@",
        "@example.com",
        "user@",
        "user@.com",
        "user@example..com",
        "user@example.c",
        "user@-example.com",
        "user@example-.com",
        "user name@example.com",
        &"a".repeat(255),
    ];

    for email_str in invalid_emails {
        let result = Email::new(email_str);
        assert!(result.is_err(), "Expected error for: '{}'", email_str);
    }
}

#[test]
fn test_email_normalization() {
    let test_cases = [
        ("USER@EXAMPLE.COM", "user@example.com"),
        ("User.Name@Example.Com", "user.name@example.com"),
        ("USER+TAG@EXAMPLE.COM", "user+tag@example.com"),
    ];

    for (input, expected) in test_cases {
        let email = Email::new(input).unwrap();
        assert_eq!(email.as_str(), expected);
    }
}

#[test]
fn test_email_parts() {
    let email = Email::new("john.doe+work@example.co.uk").unwrap();

    assert_eq!(email.local_part(), Some("john.doe+work"));
    assert_eq!(email.domain(), Some("example.co.uk"));
    assert_eq!(email.as_str(), "john.doe+work@example.co.uk");
}

#[test]
fn test_email_display() {
    let email = Email::new("test@example.com").unwrap();
    assert_eq!(format!("{}", email), "test@example.com");
}

#[test]
fn test_email_equality() {
    let email1 = Email::new("test@example.com").unwrap();
    let email2 = Email::new("TEST@EXAMPLE.COM").unwrap();
    let email3 = Email::new("other@example.com").unwrap();

    assert_eq!(email1, email2);
    assert_ne!(email1, email3);

    assert!(email1.eq_str("TEST@EXAMPLE.COM"));
    assert!(email1.eq_str("test@example.com"));
}

#[test]
fn test_email_clone() {
    let original = Email::new("original@example.com").unwrap();
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_email_serialization() {
    let email = Email::new("serialize@example.com").unwrap();
    let json = serde_json::to_string(&email).unwrap();
    let deserialized: Email = serde_json::from_str(&json).unwrap();
    assert_eq!(email, deserialized);
}

#[test]
fn test_disposable_email_detection() {
    let disposable_emails = [
        "user@tempmail.com",
        "test@mailinator.com",
        "temp@yopmail.com",
        "fake@10minutemail.com",
    ];

    let non_disposable_emails = [
        "user@gmail.com",
        "test@yahoo.com",
        "real@company.com",
        "valid@example.org",
    ];

    for email_str in disposable_emails {
        let email = Email::new(email_str).unwrap();
        assert!(
            email.is_disposable(),
            "Should detect as disposable: {}",
            email_str
        );
    }

    for email_str in non_disposable_emails {
        let email = Email::new(email_str).unwrap();
        assert!(
            !email.is_disposable(),
            "Should not detect as disposable: {}",
            email_str
        );
    }
}

#[test]
fn test_empty_email() {
    let result = Email::new("");
    assert!(matches!(result, Err(EmailError::Empty)));
}

#[test]
fn test_too_long_email() {
    let long_email = "a".repeat(255);
    let result = Email::new(&long_email);
    assert!(matches!(result, Err(EmailError::TooLong)));
}
