#[cfg(test)]
mod tests {
    use configurator_service::domain::value_objects::phone::Phone;

    #[test]
    fn test_valid_phone() {
        let phone = Phone::new("+1234567890").unwrap();
        assert_eq!(phone.value(), "+1234567890");
    }

    #[test]
    fn test_invalid_phone() {
        assert!(Phone::new("abc").is_err());
        assert!(Phone::new("123").is_err()); // Too short
    }
}