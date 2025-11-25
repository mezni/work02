use auth_service::domain::entities::Company;
use auth_service::domain::errors::DomainError;
use uuid::Uuid;

#[test]
fn test_company_creation_success() {
    let creator_id = Uuid::new_v4();
    let company = Company::new(
        "Test Company".to_string(),
        Some("Test Description".to_string()),
        creator_id,
    )
    .unwrap();

    assert_eq!(company.name, "Test Company");
    assert_eq!(company.description, Some("Test Description".to_string()));
    assert_eq!(company.created_by, creator_id);
}

#[test]
fn test_company_creation_validation() {
    let creator_id = Uuid::new_v4();

    // Empty name should fail
    let result = Company::new("".to_string(), None, creator_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));

    // Whitespace-only name should fail
    let result = Company::new("   ".to_string(), None, creator_id);
    assert!(result.is_err());

    // Very long name should fail
    let long_name = "a".repeat(256);
    let result = Company::new(long_name, None, creator_id);
    assert!(result.is_err());
}

#[test]
fn test_company_update() {
    let creator_id = Uuid::new_v4();
    let mut company = Company::new(
        "Old Name".to_string(),
        Some("Old Description".to_string()),
        creator_id,
    )
    .unwrap();

    let original_updated_at = company.updated_at;

    // Update name and description
    company
        .update(
            Some("New Name".to_string()),
            Some("New Description".to_string()),
        )
        .unwrap();

    assert_eq!(company.name, "New Name");
    assert_eq!(company.description, Some("New Description".to_string()));
    assert!(company.updated_at > original_updated_at);
}

#[test]
fn test_company_partial_update() {
    let creator_id = Uuid::new_v4();
    let mut company = Company::new(
        "Original Name".to_string(),
        Some("Original Description".to_string()),
        creator_id,
    )
    .unwrap();

    let original_updated_at = company.updated_at;

    // Update only name
    company
        .update(Some("Updated Name".to_string()), None)
        .unwrap();
    assert_eq!(company.name, "Updated Name");
    assert_eq!(
        company.description,
        Some("Original Description".to_string())
    );
    assert!(company.updated_at > original_updated_at);

    // Update only description
    let second_updated_at = company.updated_at;
    company
        .update(None, Some("Updated Description".to_string()))
        .unwrap();
    assert_eq!(company.name, "Updated Name");
    assert_eq!(company.description, Some("Updated Description".to_string()));
    assert!(company.updated_at > second_updated_at);
}

#[test]
fn test_company_update_validation() {
    let creator_id = Uuid::new_v4();
    let mut company = Company::new("Valid Name".to_string(), None, creator_id).unwrap();

    // Empty name should fail
    let result = company.update(Some("".to_string()), None);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));

    // Long name should fail
    let long_name = "a".repeat(256);
    let result = company.update(Some(long_name), None);
    assert!(result.is_err());
}
