#[cfg(test)]
mod tests {
    use configurator_service::domain::value_objects::email::Email;

    #[test]
    fn test_valid_email() {
        let email = Email::new("test@example.com").unwrap();
        assert_eq!(email.value(), "test@example.com");
    }

    #[test]
    fn test_invalid_email() {
        assert!(Email::new("invalid").is_err());
        assert!(Email::new("test@").is_err());
        assert!(Email::new("@example.com").is_err());
    }
}
