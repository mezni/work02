#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    use auth_service::{
        Company, CompanyRepository, CompanyRepositoryImpl, DomainError, Email, User,
        UserRepository, UserRepositoryImpl, UserRole,
    };

    // Test helper to create a test user with a valid company
    async fn create_test_user(pool: &PgPool, id: Uuid) -> User {
        // First create a company for the user
        let company_repo = CompanyRepositoryImpl::new(pool.clone());
        let company_id = Uuid::new_v4();
        let creator_id = Uuid::new_v4();

        let company = Company {
            id: company_id,
            name: format!("Test Company {}", company_id),
            description: Some("Test company description".to_string()),
            created_by: creator_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Create the company in the database
        company_repo.create(&company).await.unwrap();

        User {
            id,
            keycloak_id: format!("keycloak_{}", id),
            username: format!("user_{}", id),
            email: Email::new(format!("user{}@test.com", id)).unwrap(),
            role: UserRole::User,
            company_id: Some(company_id), // Use the valid company ID
            email_verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // Test helper to create a user without a company (for non-company users)
    fn create_test_user_without_company(id: Uuid) -> User {
        User {
            id,
            keycloak_id: format!("keycloak_{}", id),
            username: format!("user_{}", id),
            email: Email::new(format!("user{}@test.com", id)).unwrap(),
            role: UserRole::User,
            company_id: None, // No company for this user
            email_verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[sqlx::test]
    async fn test_create_user_success(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user_id = Uuid::new_v4();
        let user = create_test_user(&pool, user_id).await;

        // Act
        let result = repo.create(&user).await;

        // Assert
        assert!(result.is_ok());

        // Verify user was created
        let saved_user = repo.find_by_id(&user_id).await.unwrap();
        assert!(saved_user.is_some());
        let saved_user = saved_user.unwrap();
        assert_eq!(saved_user.id, user_id);
        assert_eq!(saved_user.email.value(), user.email.value());
        assert_eq!(saved_user.role, user.role);
    }

    #[sqlx::test]
    async fn test_create_user_duplicate_email(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user1 = create_test_user(&pool, Uuid::new_v4()).await;
        let user2 = create_test_user_without_company(Uuid::new_v4());

        // Set same email for both users
        let user2 = User {
            email: user1.email.clone(),
            ..user2
        };

        // Act - Create first user
        let result1 = repo.create(&user1).await;
        assert!(result1.is_ok());

        // Act - Try to create second user with same email
        let result2 = repo.create(&user2).await;

        // Assert
        assert!(result2.is_err());
        if let Err(DomainError::Validation(msg)) = result2 {
            assert!(msg.contains("Failed to create user"));
        } else {
            panic!("Expected Validation error");
        }
    }

    #[sqlx::test]
    async fn test_find_user_by_id_success(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user_id = Uuid::new_v4();
        let user = create_test_user(&pool, user_id).await;

        // Setup
        repo.create(&user).await.unwrap();

        // Act
        let result = repo.find_by_id(&user_id).await;

        // Assert
        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert!(found_user.is_some());
        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, user_id);
        assert_eq!(found_user.username, user.username);
        assert_eq!(found_user.email.value(), user.email.value());
    }

    #[sqlx::test]
    async fn test_find_user_by_id_not_found(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool);
        let non_existent_id = Uuid::new_v4();

        // Act
        let result = repo.find_by_id(&non_existent_id).await;

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[sqlx::test]
    async fn test_find_user_by_email_success(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user_id = Uuid::new_v4();
        let user = create_test_user(&pool, user_id).await;
        let user_email = user.email.clone();

        // Setup
        repo.create(&user).await.unwrap();

        // Act
        let result = repo.find_by_email(&user_email).await;

        // Assert
        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert!(found_user.is_some());
        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, user_id);
        assert_eq!(found_user.email.value(), user_email.value());
    }

    #[sqlx::test]
    async fn test_find_user_by_keycloak_id_success(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user_id = Uuid::new_v4();
        let user = create_test_user(&pool, user_id).await;
        let keycloak_id = user.keycloak_id.clone();

        // Setup
        repo.create(&user).await.unwrap();

        // Act
        let result = repo.find_by_keycloak_id(&keycloak_id).await;

        // Assert
        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert!(found_user.is_some());
        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, user_id);
        assert_eq!(found_user.keycloak_id, keycloak_id);
    }

    #[sqlx::test]
    async fn test_update_user_success(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user_id = Uuid::new_v4();
        let mut user = create_test_user(&pool, user_id).await;

        // Setup - Create user
        repo.create(&user).await.unwrap();

        // Act - Update user
        user.username = "updated_username".to_string();
        user.role = UserRole::Admin;
        user.updated_at = Utc::now();

        let result = repo.update(&user).await;

        // Assert
        assert!(result.is_ok());

        // Verify update
        let updated_user = repo.find_by_id(&user_id).await.unwrap().unwrap();
        assert_eq!(updated_user.username, "updated_username");
        assert_eq!(updated_user.role, UserRole::Admin);
    }

    #[sqlx::test]
    async fn test_update_user_not_found(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool);
        let non_existent_user = create_test_user_without_company(Uuid::new_v4());

        // Act
        let result = repo.update(&non_existent_user).await;

        // Assert
        assert!(result.is_err());
        if let Err(DomainError::UserNotFound(id)) = result {
            assert_eq!(id, non_existent_user.id.to_string());
        } else {
            panic!("Expected UserNotFound error");
        }
    }

    #[sqlx::test]
    async fn test_delete_user_success(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user_id = Uuid::new_v4();
        let user = create_test_user(&pool, user_id).await;

        // Setup
        repo.create(&user).await.unwrap();

        // Act
        let result = repo.delete(&user_id).await;

        // Assert
        assert!(result.is_ok());

        // Verify deletion
        let deleted_user = repo.find_by_id(&user_id).await.unwrap();
        assert!(deleted_user.is_none());
    }

    #[sqlx::test]
    async fn test_delete_user_not_found(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool);
        let non_existent_id = Uuid::new_v4();

        // Act
        let result = repo.delete(&non_existent_id).await;

        // Assert
        assert!(result.is_err());
        if let Err(DomainError::UserNotFound(id)) = result {
            assert_eq!(id, non_existent_id.to_string());
        } else {
            panic!("Expected UserNotFound error");
        }
    }

    #[sqlx::test]
    async fn test_find_users_by_company(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());

        // Create a specific company for this test
        let company_repo = CompanyRepositoryImpl::new(pool.clone());
        let company_id = Uuid::new_v4();
        let creator_id = Uuid::new_v4();

        let company = Company {
            id: company_id,
            name: format!("Test Company {}", company_id),
            description: Some("Test company description".to_string()),
            created_by: creator_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        company_repo.create(&company).await.unwrap();

        // Create users for the same company
        let user1_id = Uuid::new_v4();
        let user1 = User {
            id: user1_id,
            keycloak_id: format!("keycloak_{}", user1_id),
            username: format!("user_{}", user1_id),
            email: Email::new(format!("user1_{}@test.com", user1_id)).unwrap(),
            role: UserRole::User,
            company_id: Some(company_id),
            email_verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let user2_id = Uuid::new_v4();
        let user2 = User {
            id: user2_id,
            keycloak_id: format!("keycloak_{}", user2_id),
            username: format!("user_{}", user2_id),
            email: Email::new(format!("user2_{}@test.com", user2_id)).unwrap(),
            role: UserRole::User,
            company_id: Some(company_id),
            email_verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Create user for different company
        let other_company_id = Uuid::new_v4();
        let other_company = Company {
            id: other_company_id,
            name: format!("Other Company {}", other_company_id),
            description: Some("Other company description".to_string()),
            created_by: creator_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        company_repo.create(&other_company).await.unwrap();

        let user3_id = Uuid::new_v4();
        let user3 = User {
            id: user3_id,
            keycloak_id: format!("keycloak_{}", user3_id),
            username: format!("user_{}", user3_id),
            email: Email::new(format!("user3_{}@test.com", user3_id)).unwrap(),
            role: UserRole::User,
            company_id: Some(other_company_id),
            email_verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Setup
        repo.create(&user1).await.unwrap();
        repo.create(&user2).await.unwrap();
        repo.create(&user3).await.unwrap();

        // Act
        let result = repo.find_by_company(&company_id).await;

        // Assert
        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 2);

        let user_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();
        assert!(user_ids.contains(&user1.id));
        assert!(user_ids.contains(&user2.id));
        assert!(!user_ids.contains(&user3.id));
    }

    #[sqlx::test]
    async fn test_exists_by_email(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user = create_test_user(&pool, Uuid::new_v4()).await;
        let test_email = user.email.clone();

        // Test before creation
        let exists_before = repo.exists_by_email(&test_email).await.unwrap();
        assert!(!exists_before);

        // Setup
        repo.create(&user).await.unwrap();

        // Test after creation
        let exists_after = repo.exists_by_email(&test_email).await.unwrap();
        assert!(exists_after);
    }

    #[sqlx::test]
    async fn test_exists_by_username(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user = create_test_user(&pool, Uuid::new_v4()).await;
        let test_username = user.username.clone();

        // Test before creation
        let exists_before = repo.exists_by_username(&test_username).await.unwrap();
        assert!(!exists_before);

        // Setup
        repo.create(&user).await.unwrap();

        // Test after creation
        let exists_after = repo.exists_by_username(&test_username).await.unwrap();
        assert!(exists_after);
    }

    #[sqlx::test]
    async fn test_find_by_username_success(pool: PgPool) {
        let repo = UserRepositoryImpl::new(pool.clone());
        let user = create_test_user(&pool, Uuid::new_v4()).await;
        let username = user.username.clone();

        // Setup
        repo.create(&user).await.unwrap();

        // Act
        let result = repo.find_by_username(&username).await;

        // Assert
        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().username, username);
    }
}
