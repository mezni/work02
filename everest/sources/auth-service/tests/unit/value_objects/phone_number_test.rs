// tests/domain/value_objects/phone_number_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::PhoneNumber;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_phone_number() {
        let phone = PhoneNumber::new("+1234567890").unwrap();
        assert_eq!(phone.as_str(), "+1234567890");
    }

    #[test]
    fn test_phone_number_removes_spaces_and_dashes() {
        let phone = PhoneNumber::new("+1 234-567-890").unwrap();
        assert_eq!(phone.as_str(), "+1234567890");
    }

    #[test]
    fn test_phone_number_allows_empty_string() {
        let phone = PhoneNumber::new("").unwrap();
        assert!(phone.is_empty());
    }

    #[test]
    fn test_phone_number_rejects_invalid_characters() {
        let invalid_numbers = vec!["+1-800-ABC-DEFG", "+1 (800) 123-4567", "800!123!4567"];
        
        for number in invalid_numbers {
            let result = PhoneNumber::new(number);
            assert!(matches!(result, Err(DomainError::Validation(_))), "Should have failed for: {}", number);
        }
    }

    #[test]
    fn test_phone_number_rejects_too_short() {
        let result = PhoneNumber::new("+123456789");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_phone_number_rejects_too_long() {
        let long_number = "+1".to_string() + &"2".repeat(15);
        let result = PhoneNumber::new(&long_number);
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_phone_number_is_empty() {
        let empty_phone = PhoneNumber::new("").unwrap();
        assert!(empty_phone.is_empty());
        
        let non_empty_phone = PhoneNumber::new("+1234567890").unwrap();
        assert!(!non_empty_phone.is_empty());
    }

    #[test]
    fn test_phone_number_display() {
        let phone = PhoneNumber::new("+1234567890").unwrap();
        assert_eq!(format!("{}", phone), "+1234567890");
    }
}