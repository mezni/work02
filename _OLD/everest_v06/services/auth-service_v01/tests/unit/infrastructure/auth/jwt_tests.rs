// tests/unit/infrastructure/auth/jwt_tests.rs
#[cfg(test)]
mod tests {
    use auth_service::{
        infrastructure::auth::jwt::{Claims, JwtService},
        infrastructure::config::Settings,
    };
    use chrono::{Duration, Utc};

    fn create_test_settings() -> Settings {
        Settings::default()
    }

    #[test]
    fn test_generate_and_validate_token() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);

        let token = jwt_service
            .generate_token(
                "user-123",
                "testuser",
                "test@example.com",
                "admin",
                Some("company-456"),
            )
            .unwrap();

        let claims = jwt_service.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, "admin");
        assert_eq!(claims.company_id, Some("company-456".to_string()));
    }

    #[test]
    fn test_generate_token_without_company() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);

        let token = jwt_service
            .generate_token("user-123", "testuser", "test@example.com", "user", None)
            .unwrap();

        let claims = jwt_service.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.role, "user");
        assert!(claims.company_id.is_none());
    }

    #[test]
    fn test_validate_invalid_token() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);

        let result = jwt_service.validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_refresh_token() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);

        let original_token = jwt_service
            .generate_token(
                "user-123",
                "testuser",
                "test@example.com",
                "user",
                Some("company-456"),
            )
            .unwrap();

        let refreshed_token = jwt_service.refresh_token(&original_token).unwrap();

        let original_claims = jwt_service.validate_token(&original_token).unwrap();
        let refreshed_claims = jwt_service.validate_token(&refreshed_token).unwrap();

        // Claims should be the same except for expiration
        assert_eq!(original_claims.sub, refreshed_claims.sub);
        assert_eq!(original_claims.username, refreshed_claims.username);
        assert_eq!(original_claims.email, refreshed_claims.email);
        assert_eq!(original_claims.role, refreshed_claims.role);
        assert_eq!(original_claims.company_id, refreshed_claims.company_id);

        // JTI should be different
        assert_ne!(original_claims.jti, refreshed_claims.jti);
    }

    #[test]
    fn test_get_remaining_time() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);

        let token = jwt_service
            .generate_token("user-123", "testuser", "test@example.com", "user", None)
            .unwrap();

        let remaining_time = jwt_service.get_remaining_time(&token).unwrap();
        assert!(remaining_time.num_seconds() > 0);
        assert!(remaining_time.num_seconds() <= 3600);
    }

    #[test]
    fn test_token_expiration() {
        let mut settings = create_test_settings();
        settings.auth.jwt_expiration_seconds = 1; // 1 second expiration

        let jwt_service = JwtService::new(&settings);

        let token = jwt_service
            .generate_token("user-123", "testuser", "test@example.com", "user", None)
            .unwrap();

        // Wait for token to expire
        std::thread::sleep(std::time::Duration::from_secs(2));

        let remaining_time = jwt_service.get_remaining_time(&token).unwrap();
        assert_eq!(remaining_time.num_seconds(), 0);
    }

    #[test]
    fn test_claims_serialization() {
        let claims = Claims {
            sub: "user-123".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            jti: "test-jti".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            role: "admin".to_string(),
            company_id: Some("company-456".to_string()),
        };

        // Test that claims can be serialized and deserialized
        let serialized = serde_json::to_string(&claims).unwrap();
        let deserialized: Claims = serde_json::from_str(&serialized).unwrap();

        assert_eq!(claims.sub, deserialized.sub);
        assert_eq!(claims.username, deserialized.username);
        assert_eq!(claims.email, deserialized.email);
        assert_eq!(claims.role, deserialized.role);
        assert_eq!(claims.company_id, deserialized.company_id);
    }
}
