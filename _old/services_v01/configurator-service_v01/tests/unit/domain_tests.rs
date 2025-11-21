use configurator_service::domain::models::person::{Person, PersonError};
use configurator_service::domain::enums::RoleType;
use configurator_service::domain::value_objects::{Email, Phone};
use uuid::Uuid;

fn create_test_email() -> Email {
    Email::new("test@example.com").unwrap()
}

fn create_test_phone() -> Phone {
    Phone::new("1234567890").unwrap()
}

fn create_test_person() -> Person {
    Person::new(
        "John Doe".into(),
        Some(create_test_email()),
        Some(create_test_phone()),
        Some("Manager".into()),
        Some("Operations".into()),
        RoleType::Operations,
        Uuid::new_v4(),
    ).unwrap()
}

#[test]
fn test_person_creation() {
    let creator_id = Uuid::new_v4();
    let person = Person::new(
        "John Doe".into(),
        Some(create_test_email()),
        Some(create_test_phone()),
        Some("Manager".into()),
        Some("Operations".into()),
        RoleType::Operations,
        creator_id,
    ).unwrap();

    assert_eq!(person.full_name(), "John Doe");
    assert_eq!(person.job_title(), Some("Manager"));
    assert_eq!(person.department(), Some("Operations"));
    assert_eq!(person.role_type(), RoleType::Operations);
    assert_eq!(person.created_by(), creator_id);
    assert!(person.has_contact_method());
    assert!(person.is_active());
    assert!(person.is_live());
    assert!(!person.is_verified());
    assert!(person.is_orphaned());
}

#[test]
fn test_person_validation() {
    let creator_id = Uuid::new_v4();

    // Empty name
    let result = Person::new(
        "".into(),
        Some(create_test_email()),
        None,
        None,
        None,
        RoleType::General,
        creator_id,
    );
    assert!(matches!(result, Err(PersonError::EmptyName)));

    // No contact method
    let result = Person::new(
        "John Doe".into(),
        None,
        None,
        None,
        None,
        RoleType::General,
        creator_id,
    );
    assert!(matches!(result, Err(PersonError::NoContactMethod)));
}

#[test]
fn test_person_assignment() {
    let mut person = create_test_person();
    let individual_id = Uuid::new_v4();
    let company_id = Uuid::new_v4();
    let updater_id = Uuid::new_v4();

    // Assign to individual
    person.assign_to_individual(individual_id).unwrap();
    assert!(person.is_individual_person());
    assert!(!person.is_company_person());
    assert!(!person.is_orphaned());

    // Try to assign to company (should fail)
    let result = person.assign_to_company(company_id, updater_id);
    assert!(result.is_err());

    // Unassign and assign to company
    person.unassign(updater_id);
    person.assign_to_company(company_id, updater_id).unwrap();
    assert!(!person.is_individual_person());
    assert!(person.is_company_person());
    assert!(!person.is_orphaned());
}

#[test]
fn test_person_status_management() {
    let mut person = create_test_person();
    let updater_id = Uuid::new_v4();

    // Verify
    person.verify(updater_id);
    assert!(person.is_verified());

    // Deactivate
    person.deactivate(updater_id);
    assert!(!person.is_active());

    // Archive
    person.archive(updater_id);
    assert!(!person.is_live());

    // Restore and activate
    person.restore(updater_id);
    person.activate(updater_id);
    assert!(person.is_live());
    assert!(person.is_active());
}

#[test]
fn test_permission_checks() {
    let admin_person = Person::new(
        "Admin".into(),
        Some(create_test_email()),
        None,
        None,
        None,
        RoleType::Admin,
        Uuid::new_v4(),
    ).unwrap();

    let billing_person = Person::new(
        "Billing".into(),
        Some(create_test_email()),
        None,
        None,
        None,
        RoleType::Billing,
        Uuid::new_v4(),
    ).unwrap();

    let general_person = Person::new(
        "General".into(),
        Some(create_test_email()),
        None,
        None,
        None,
        RoleType::General,
        Uuid::new_v4(),
    ).unwrap();

    assert!(admin_person.can_manage_billing());
    assert!(admin_person.can_manage_operations());
    assert!(admin_person.can_manage_technical());

    assert!(billing_person.can_manage_billing());
    assert!(!billing_person.can_manage_operations());
    assert!(!billing_person.can_manage_technical());

    assert!(!general_person.can_manage_billing());
    assert!(!general_person.can_manage_operations());
    assert!(!general_person.can_manage_technical());
}

#[test]
fn test_job_info_updates() {
    let mut person = create_test_person();
    let updater_id = Uuid::new_v4();

    person.update_job_info(
        Some("Senior Manager".into()),
        Some("Technology".into()),
        updater_id,
    );

    assert_eq!(person.job_title(), Some("Senior Manager"));
    assert_eq!(person.department(), Some("Technology"));
    assert_eq!(person.updated_by(), Some(updater_id));
}

// You can also add tests for other domain models here
#[test]
fn test_network_model() {
    // Add network tests here when you create them
}

#[test] 
fn test_value_objects() {
    // Add value object tests here
}
