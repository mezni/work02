use configurator_service::domain::enums::{
    company_type::CompanyType, network_type::NetworkType, operational_status::OperationalStatus,
    role_type::RoleType, verification_status::VerificationStatus,
};
use serde_json;
use std::str::FromStr;

//
// NETWORK TYPE TESTS
//
#[test]
fn test_network_type_parse() {
    assert_eq!(
        NetworkType::from_str("INDIVIDUAL").unwrap(),
        NetworkType::INDIVIDUAL
    );
    assert_eq!(
        NetworkType::from_str("individual").unwrap(),
        NetworkType::INDIVIDUAL
    );
    assert_eq!(
        NetworkType::from_str("COMPANY").unwrap(),
        NetworkType::COMPANY
    );
    assert!(NetworkType::from_str("INVALID").is_err());
}

#[test]
fn test_network_type_display() {
    assert_eq!(NetworkType::INDIVIDUAL.to_string(), "INDIVIDUAL");
    assert_eq!(NetworkType::COMPANY.to_string(), "COMPANY");
}

#[test]
fn test_network_type_serde() {
    let json = serde_json::to_string(&NetworkType::INDIVIDUAL).unwrap();
    assert_eq!(json, "\"INDIVIDUAL\"");

    let parsed: NetworkType = serde_json::from_str("\"COMPANY\"").unwrap();
    assert_eq!(parsed, NetworkType::COMPANY);
}

//
// COMPANY TYPE TESTS
//
#[test]
fn test_company_type_parse() {
    assert_eq!(
        CompanyType::from_str("COMPANY").unwrap(),
        CompanyType::COMPANY
    );
    assert_eq!(
        CompanyType::from_str("COOPERATIVE").unwrap(),
        CompanyType::COOPERATIVE
    );
    assert_eq!(
        CompanyType::from_str("GOVERNMENT").unwrap(),
        CompanyType::GOVERNMENT
    );
    assert!(CompanyType::from_str("BAD").is_err());
}

#[test]
fn test_company_type_display() {
    assert_eq!(CompanyType::COMPANY.to_string(), "COMPANY");
    assert_eq!(CompanyType::COOPERATIVE.to_string(), "COOPERATIVE");
}

#[test]
fn test_company_type_serde() {
    let json = serde_json::to_string(&CompanyType::GOVERNMENT).unwrap();
    assert_eq!(json, "\"GOVERNMENT\"");

    let parsed: CompanyType = serde_json::from_str("\"COMPANY\"").unwrap();
    assert_eq!(parsed, CompanyType::COMPANY);
}

//
// ROLE TYPE TESTS
//
#[test]
fn test_role_type_parse() {
    assert_eq!(RoleType::from_str("ADMIN").unwrap(), RoleType::ADMIN);
    assert_eq!(RoleType::from_str("billing").unwrap(), RoleType::BILLING);
    assert_eq!(
        RoleType::from_str("TECHNICAL").unwrap(),
        RoleType::TECHNICAL
    );
    assert!(RoleType::from_str("UNKNOWN").is_err());
}

#[test]
fn test_role_type_display() {
    assert_eq!(RoleType::OPERATIONS.to_string(), "OPERATIONS");
}

#[test]
fn test_role_type_serde() {
    let json = serde_json::to_string(&RoleType::GENERAL).unwrap();
    assert_eq!(json, "\"GENERAL\"");

    let parsed: RoleType = serde_json::from_str("\"ADMIN\"").unwrap();
    assert_eq!(parsed, RoleType::ADMIN);
}

//
// OPERATIONAL STATUS TESTS
//
#[test]
fn test_operational_status_parse() {
    assert_eq!(
        OperationalStatus::from_str("ACTIVE").unwrap(),
        OperationalStatus::ACTIVE
    );
    assert_eq!(
        OperationalStatus::from_str("maintenance").unwrap(),
        OperationalStatus::MAINTENANCE
    );
    assert!(OperationalStatus::from_str("BROKEN").is_err());
}

#[test]
fn test_operational_status_display() {
    assert_eq!(
        OperationalStatus::OUT_OF_SERVICE.to_string(),
        "OUT_OF_SERVICE"
    );
}

#[test]
fn test_operational_status_serde() {
    let json = serde_json::to_string(&OperationalStatus::COMMISSIONING).unwrap();
    assert_eq!(json, "\"COMMISSIONING\"");

    let parsed: OperationalStatus = serde_json::from_str("\"ACTIVE\"").unwrap();
    assert_eq!(parsed, OperationalStatus::ACTIVE);
}

//
// VERIFICATION STATUS TESTS
//
#[test]
fn test_verification_status_parse() {
    assert_eq!(
        VerificationStatus::from_str("PENDING").unwrap(),
        VerificationStatus::PENDING
    );
    assert_eq!(
        VerificationStatus::from_str("verified").unwrap(),
        VerificationStatus::VERIFIED
    );
    assert!(VerificationStatus::from_str("NOPE").is_err());
}

#[test]
fn test_verification_status_display() {
    assert_eq!(VerificationStatus::REJECTED.to_string(), "REJECTED");
}

#[test]
fn test_verification_status_serde() {
    let json = serde_json::to_string(&VerificationStatus::VERIFIED).unwrap();
    assert_eq!(json, "\"VERIFIED\"");

    let parsed: VerificationStatus = serde_json::from_str("\"PENDING\"").unwrap();
    assert_eq!(parsed, VerificationStatus::PENDING);
}
