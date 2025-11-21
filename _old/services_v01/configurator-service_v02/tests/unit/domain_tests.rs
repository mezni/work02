use configurator_service::domain::enums::{
    company_type::CompanyType, network_type::NetworkType, role_type::RoleType,
};
use configurator_service::domain::models::{
    company::Company, individual::Individual, network::Network, person::Person,
};
use uuid::Uuid;

#[test]
fn test_person_creation_and_contact_update() {
    let updater = Some(Uuid::new_v4());

    let mut person = Person::new(
        "John".to_string(),
        "Doe".to_string(),
        RoleType::ADMIN,
        Some("user@test.com".to_string()),
        Some("+15551234567".to_string()),
        Some("Engineer".to_string()),
        Some("R&D".to_string()),
        updater,
    );

    // Verify initial values
    assert_eq!(person.first_name, "John");
    assert_eq!(person.email, Some("user@test.com".to_string()));

    // Update contact
    person.update_contact(
        Some("bob@test.com".to_string()),
        Some("+15550001111".to_string()),
        updater,
    );
    assert_eq!(person.email, Some("bob@test.com".to_string()));
    assert_eq!(person.phone, Some("+15550001111".to_string()));

    // Update position
    person.update_position(
        Some("Manager".to_string()),
        Some("Product".to_string()),
        updater,
    );
    assert_eq!(person.job_title, Some("Manager".to_string()));
    assert_eq!(person.department, Some("Product".to_string()));
}

#[test]
fn test_individual_lifecycle() {
    let updater = Some(Uuid::new_v4());

    let mut individual = Individual::new(
        "Jane".to_string(),
        "Smith".to_string(),
        Some("jane@test.com".to_string()),
        Some("+15550002222".to_string()),
        updater,
    );

    assert_eq!(individual.first_name, "Jane");
    assert!(individual.is_live);
    assert!(individual.is_active);

    individual.verify(updater);
    assert!(individual.is_verified);

    individual.deactivate(updater);
    assert!(!individual.is_active);

    individual.activate(updater);
    assert!(individual.is_active);

    let person = Person::new(
        "Alice".to_string(),
        "Brown".to_string(),
        RoleType::ADMIN,
        None,
        None,
        None,
        None,
        updater,
    );
    individual.add_person(person);
    assert_eq!(individual.persons.len(), 1);
}

#[test]
fn test_company_lifecycle() {
    let updater = Some(Uuid::new_v4());

    let mut company = Company::new(
        "ACME Inc".to_string(),
        CompanyType::COMPANY,
        Some("50-100".to_string()),
        Some("https://acme.com".to_string()),
        Some("support@test.com".to_string()),
        Some("+15550003333".to_string()),
        updater,
    );

    assert_eq!(company.name, "ACME Inc");
    assert_eq!(company.company_type, CompanyType::COMPANY);

    company.verify(updater);
    assert!(company.is_verified);

    company.deactivate(updater);
    assert!(!company.is_active);

    company.activate(updater);
    assert!(company.is_active);

    let person = Person::new(
        "Bob".to_string(),
        "Green".to_string(),
        RoleType::ADMIN,
        None,
        None,
        None,
        None,
        updater,
    );
    company.add_person(person);
    assert_eq!(company.persons.len(), 1);
}

#[test]
fn test_network_creation() {
    let updater = Some(Uuid::new_v4());

    let individual = Individual::new(
        "Jane".to_string(),
        "Smith".to_string(),
        Some("jane@test.com".to_string()),
        Some("+15550002222".to_string()),
        updater,
    );

    let network_individual = Network::new_individual(
        individual.clone(),
        Some("support@test.com".to_string()),
        Some("+15550001111".to_string()),
        updater,
    );

    assert_eq!(network_individual.network_type, NetworkType::INDIVIDUAL);
    assert!(network_individual.owner_individual.is_some());
    assert!(network_individual.owner_company.is_none());

    let company = Company::new(
        "ACME Inc".to_string(),
        CompanyType::COMPANY,
        None,
        None,
        Some("support@test.com".to_string()),
        Some("+15550003333".to_string()),
        updater,
    );

    let network_company = Network::new_company(
        company.clone(),
        Some("support@test.com".to_string()),
        Some("+15550004444".to_string()),
        updater,
    );

    assert_eq!(network_company.network_type, NetworkType::COMPANY);
    assert!(network_company.owner_company.is_some());
    assert!(network_company.owner_individual.is_none());
}
