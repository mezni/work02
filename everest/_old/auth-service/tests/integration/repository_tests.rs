#[cfg(test)]
mod repository_tests {
    use sqlx::PgPool;
    use auth_service::{
        domain::{
            entities::User,
            repositories::UserRepository,
            value_objects::{Email, OrganisationName, Role},
        },
        infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
    };

    #[sqlx::test]
    async fn test_save_and_find_user(pool: PgPool) {
        let repository = PostgresUserRepository::new(pool);

        let email = Email::new("test@example.com".to_string()).unwrap();
        let org = OrganisationName::new("AcmeCorp".to_string()).unwrap();

        let user = User::new(
            "keycloak_123".to_string(),
            email,
            "testuser".to_string(),
            Role::Operator,
            Some(org),
        );

        // Save user
        let saved_user = repository.save(&user).await.unwrap();
        assert_eq!(saved_user.username, "testuser");

        // Find user by ID
        let found_user = repository.find_by_id(saved_user.id).await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().username, "testuser");

        // Find user by email
        let found_by_email = repository
            .find_by_email("test@example.com")
            .await
            .unwrap();
        assert!(found_by_email.is_some());
    }

    #[sqlx::test]
    async fn test_list_by_organisation(pool: PgPool) {
        let repository = PostgresUserRepository::new(pool);

        let org = OrganisationName::new("AcmeCorp".to_string()).unwrap();

        // Create multiple users in same organisation
        for i in 0..3 {
            let email = Email::new(format!("user{}@example.com", i)).unwrap();
            let user = User::new(
                format!("keycloak_{}", i),
                email,
                format!("user{}", i),
                Role::Operator,
                Some(org.clone()),
            );
            repository.save(&user).await.unwrap();
        }

        // List users by organisation
        let users = repository
            .list_by_organisation("AcmeCorp")
            .await
            .unwrap();

        assert_eq!(users.len(), 3);
    }
}
