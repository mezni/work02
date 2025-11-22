#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use configurator_service::domain::enums::network_type::NetworkType;

    #[test]
    fn test_network_type_from_str() {
        assert_eq!(NetworkType::from_str("INDIVIDUAL").unwrap(), NetworkType::Individual);
        assert_eq!(NetworkType::from_str("COMPANY").unwrap(), NetworkType::Company);
        assert!(NetworkType::from_str("INVALID").is_err());
    }

    #[test]
    fn test_network_type_to_string() {
        assert_eq!(NetworkType::Individual.to_string(), "INDIVIDUAL");
        assert_eq!(NetworkType::Company.to_string(), "COMPANY");
    }
}