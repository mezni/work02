#[cfg(test)]
mod domain_tests {
    use auth_service::domain::{
        entities::User,
        value_objects::{Email, OrganisationName, Role},
        services::AuthorizationService,
    };

    #[test]
    fn test_email_validation() {
        let valid_email = Email::new("test@example.com".to_string());
        assert!(valid_email.is_ok());

        let invalid_email = Email::new("invalid".to_string());
        assert!(invalid_email.is_err());
    }

    #[test]
    fn test_organisation_name_validation() {
        let valid_org = OrganisationName::new("AcmeCorp".to_string());
        assert!(valid_org.is_ok());

        let invalid_org = OrganisationName::new("".to_string());
        assert!(invalid_org.is_err());
    }

    #[test]
    fn test_user_creation() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let org = OrganisationName::new("AcmeCorp".to_string()).unwrap();

        let user = User::new(
            "keycloak_123".to_string(),
            email,
            "testuser".to_string(),
            Role::Operator,
            Some(org),
        );

        assert_eq!(user.role, Role::Operator);
        assert!(user.is_active);
    }

    #[test]
    fn test_operator_requires_organisation() {
        let email = Email::new("test@example.com".to_string()).unwrap();

        let user = User::new(
            "keycloak_123".to_string(),
            email,
            "testuser".to_string(),
            Role::Operator,
            None,
        );

        let result = user.validate_operator_has_organisation();
        assert!(result.is_err());
    }

    #[test]
    fn test_authorization_service_can_create_user() {
        // Admin can create all roles
        assert!(AuthorizationService::can_create_user(&Role::Admin, &Role::Admin));
        assert!(AuthorizationService::can_create_user(&Role::Admin, &Role::Partner));
        assert!(AuthorizationService::can_create_user(&Role::Admin, &Role::Operator));

        // Partner can only create operators
        assert!(!AuthorizationService::can_create_user(&Role::Partner, &Role::Admin));
        assert!(!AuthorizationService::can_create_user(&Role::Partner, &Role::Partner));
        assert!(AuthorizationService::can_create_user(&Role::Partner, &Role::Operator));

        // Operator cannot create users
        assert!(!AuthorizationService::can_create_user(&Role::Operator, &Role::Admin));
        assert!(!AuthorizationService::can_create_user(&Role::Operator, &Role::Partner));
        assert!(!AuthorizationService::can_create_user(&Role::Operator, &Role::Operator));
    }

    #[test]
    fn test_can_access_organisation() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let org = OrganisationName::new("AcmeCorp".to_string()).unwrap();

        // Admin can access any organisation
        let admin = User::new(
            "keycloak_admin".to_string(),
            email.clone(),
            "admin".to_string(),
            Role::Admin,
            None,
        );
        assert!(admin.can_access_organisation("AcmeCorp"));
        assert!(admin.can_access_organisation("OtherCorp"));

        // Operator can only access their organisation
        let operator = User::new(
            "keycloak_operator".to_string(),
            Email::new("operator@example.com".to_string()).unwrap(),
            "operator".to_string(),
            Role::Operator,
            Some(org),
        );
        assert!(operator.can_access_organisation("AcmeCorp"));
        assert!(!operator.can_access_organisation("OtherCorp"));
    }

    #[test]
    fn test_authorization_service_can_manage_user() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let org = OrganisationName::new("AcmeCorp".to_string()).unwrap();

        let admin = User::new(
            "keycloak_admin".to_string(),
            email.clone(),
            "admin".to_string(),
            Role::Admin,
            None,
        );

        let partner = User::new(
            "keycloak_partner".to_string(),
            Email::new("partner@example.com".to_string()).unwrap(),
            "partner".to_string(),
            Role::Partner,
            Some(org.clone()),
        );

        let operator = User::new(
            "keycloak_operator".to_string(),
            Email::new("operator@example.com".to_string()).unwrap(),
            "operator".to_string(),
            Role::Operator,
            Some(org),
        );

        // Admin can manage anyone
        assert!(AuthorizationService::can_manage_user(&admin, &partner));
        assert!(AuthorizationService::can_manage_user(&admin, &operator));

        // Partner can manage operators in same organisation
        assert!(AuthorizationService::can_manage_user(&partner, &operator));

        // Operator cannot manage anyone
        assert!(!AuthorizationService::can_manage_user(&operator, &partner));
        assert!(!AuthorizationService::can_manage_user(&operator, &admin));
    }
}
