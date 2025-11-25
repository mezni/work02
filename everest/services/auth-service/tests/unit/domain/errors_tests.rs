use auth_service::domain::errors::DomainError;

#[test]
fn test_domain_error_display() {
    let validation_error = DomainError::Validation("Test validation".to_string());
    assert_eq!(
        validation_error.to_string(),
        "Validation error: Test validation"
    );

    let not_found_error = DomainError::UserNotFound("user-123".to_string());
    assert_eq!(not_found_error.to_string(), "User not found: user-123");
}

#[test]
fn test_domain_error_codes() {
    assert_eq!(
        DomainError::Validation("".to_string()).code(),
        "DOMAIN_VALIDATION_ERROR"
    );
    assert_eq!(
        DomainError::UserNotFound("".to_string()).code(),
        "USER_NOT_FOUND"
    );
    assert_eq!(
        DomainError::CompanyNotFound("".to_string()).code(),
        "COMPANY_NOT_FOUND"
    );
    assert_eq!(
        DomainError::EmailAlreadyExists("".to_string()).code(),
        "EMAIL_ALREADY_EXISTS"
    );
    assert_eq!(
        DomainError::UsernameAlreadyExists("".to_string()).code(),
        "USERNAME_ALREADY_EXISTS"
    );
    assert_eq!(
        DomainError::InvalidEmail("".to_string()).code(),
        "INVALID_EMAIL"
    );
    assert_eq!(
        DomainError::InvalidPassword("".to_string()).code(),
        "INVALID_PASSWORD"
    );
    assert_eq!(
        DomainError::InvalidRole("".to_string()).code(),
        "INVALID_ROLE"
    );
    assert_eq!(
        DomainError::UserAlreadyInCompany("".to_string()).code(),
        "USER_ALREADY_IN_COMPANY"
    );
    assert_eq!(
        DomainError::Unauthorized("".to_string()).code(),
        "UNAUTHORIZED"
    );
    assert_eq!(
        DomainError::InvalidOperation("".to_string()).code(),
        "INVALID_OPERATION"
    );
    assert_eq!(
        DomainError::BusinessRuleViolation("".to_string()).code(),
        "BUSINESS_RULE_VIOLATION"
    );
}
