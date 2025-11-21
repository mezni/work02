use configurator_service::domain::value_objects::{
    email::{Email, EmailError},
    phone::{Phone, PhoneError},
    contact_info::ContactInfo,
    location::{Location, LocationError},
    tags::{Tags, TagsError},
};

// =====================
// Email tests
// =====================
#[test]
fn test_valid_email() {
    let email = Email::new("test@example.com".into());
    assert!(email.is_ok());
}

#[test]
fn test_invalid_email_missing_at() {
    let email = Email::new("invalid-email".into());
    assert_eq!(email.unwrap_err(), EmailError::InvalidFormat);
}

#[test]
fn test_email_serialize_deserialize() {
    let email = Email::new("user@test.com".into()).unwrap();
    let json = serde_json::to_string(&email).unwrap();
    let restored: Email = serde_json::from_str(&json).unwrap();
    assert_eq!(email, restored);
}

// =====================
// Phone tests
// =====================
#[test]
fn test_valid_phone() {
    let phone = Phone::new("+12345678901".into());
    assert!(phone.is_ok());
}

#[test]
fn test_invalid_phone() {
    let phone = Phone::new("abc123".into());
    assert_eq!(phone.unwrap_err(), PhoneError::InvalidFormat);
}

#[test]
fn test_phone_serde() {
    let phone = Phone::new("+15550001111".into()).unwrap();
    let json = serde_json::to_string(&phone).unwrap();
    let restored: Phone = serde_json::from_str(&json).unwrap();
    assert_eq!(phone, restored);
}

// =====================
// ContactInfo tests
// =====================
#[test]
fn test_contact_info_creation() {
    let email = Email::new("info@test.com".into()).unwrap();
    let phone = Phone::new("+15551230000".into()).unwrap();

    let info = ContactInfo::new(Some(email.clone()), Some(phone.clone()));

    assert_eq!(info.email, Some(email));
    assert_eq!(info.phone, Some(phone));
}

#[test]
fn test_contact_info_serde() {
    let info = ContactInfo::new(
        Some(Email::new("a@b.com".unwrap())),
        Some(Phone::new("+123456789".unwrap())),
    );

    let json = serde_json::to_string(&info).unwrap();
    let restored: ContactInfo = serde_json::from_str(&json).unwrap();

    assert_eq!(info, restored);
}

// =====================
// Location tests
// =====================
#[test]
fn test_valid_location() {
    let loc = Location::new(
        45.0,
        -73.0,
        Some("123 Main St".into()),
        None,
        Some("Montreal".into()),
        None,
        Some("H2X".into()),
        Some("Canada".into()),
    );

    assert!(loc.is_ok());
}

#[test]
fn test_invalid_latitude() {
    let loc = Location::new(120.0, 0.0, None, None, None, None, None, None);
    assert_eq!(loc.unwrap_err(), LocationError::InvalidLatitude);
}

#[test]
fn test_invalid_longitude() {
    let loc = Location::new(45.0, 250.0, None, None, None, None, None, None);
    assert_eq!(loc.unwrap_err(), LocationError::InvalidLongitude);
}

#[test]
fn test_location_update_latitude() {
    let loc = Location::new(10.0, 10.0, None, None, None, None, None, None).unwrap();
    let updated = loc.with_latitude(20.0).unwrap();

    assert_eq!(updated.latitude, 20.0);
}

#[test]
fn test_location_serde() {
    let loc = Location::new(48.0, 2.0, None, None, None, None, None, None).unwrap();
    let json = serde_json::to_string(&loc).unwrap();
    let restored: Location = serde_json::from_str(&json).unwrap();
    assert_eq!(loc, restored);
}

// =====================
// Tags tests
// =====================
#[test]
fn test_valid_tags() {
    let tags = Tags::new(vec!["Fast".into(), "EV".into()]);
    assert!(tags.is_ok());
}

#[test]
fn test_trim_and_deduplicate() {
    let tags = Tags::new(vec![
        " Fast ".into(),
        "fast".into(),
        "EV".into(),
        "ev ".into(),
    ])
    .unwrap();

    assert_eq!(tags.inner().len(), 2); // "Fast", "EV"
}

#[test]
fn test_empty_tag_error() {
    let tags = Tags::new(vec!["   ".into()]);
    assert_eq!(tags.unwrap_err(), TagsError::EmptyTag);
}

#[test]
fn test_tag_too_long_error() {
    let long = "a".repeat(51);
    let tags = Tags::new(vec![long]);
    assert_eq!(tags.unwrap_err(), TagsError::TagTooLong);
}

#[test]
fn test_with_tag() {
    let tags = Tags::new(vec!["A".into()]).unwrap();
    let updated = tags.with_tag("B".into()).unwrap();
    assert_eq!(updated.inner().len(), 2);
}

#[test]
fn test_without_tag() {
    let tags = Tags::new(vec!["A".into(), "B".into()]).unwrap();
    let updated = tags.without_tag("A");

    assert_eq!(updated.inner().len(), 1);
    assert_eq!(updated.inner()[0], "B");
}

#[test]
fn test_tags_serde() {
    let tags = Tags::new(vec!["A".into(), "B".into()]).unwrap();
    let json = serde_json::to_string(&tags).unwrap();
    let restored: Tags = serde_json::from_str(&json).unwrap();
    assert_eq!(tags, restored);
}
