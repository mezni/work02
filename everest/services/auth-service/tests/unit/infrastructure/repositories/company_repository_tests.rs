#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    // Import from your library name
    use auth_service::{Company, CompanyRepository, CompanyRepositoryImpl, DomainError};

    // Test helper to create a test company
    fn create_test_company(id: Uuid, created_by: Uuid) -> Company {
        Company {
            id,
            name: format!("Company {}", id),
            description: Some(format!("Description for company {}", id)),
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[sqlx::test]
    async fn test_create_company_success(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let company_id = Uuid::new_v4();
        let creator_id = Uuid::new_v4();
        let company = create_test_company(company_id, creator_id);

        // Act
        let result = repo.create(&company).await;

        // Assert
        assert!(result.is_ok());

        // Verify company was created
        let saved_company = repo.find_by_id(&company_id).await.unwrap();
        assert!(saved_company.is_some());
        let saved_company = saved_company.unwrap();
        assert_eq!(saved_company.id, company_id);
        assert_eq!(saved_company.name, company.name);
        assert_eq!(saved_company.created_by, creator_id);
    }

    #[sqlx::test]
    async fn test_create_company_duplicate_name(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let company1 = create_test_company(Uuid::new_v4(), Uuid::new_v4());
        let company2 = create_test_company(Uuid::new_v4(), Uuid::new_v4());

        // Set same name for both companies
        let company2 = Company {
            name: company1.name.clone(),
            ..company2
        };

        // Act - Create first company
        let result1 = repo.create(&company1).await;
        assert!(result1.is_ok());

        // Act - Try to create second company with same name
        let result2 = repo.create(&company2).await;

        // Assert
        assert!(result2.is_err());
        if let Err(DomainError::Validation(msg)) = result2 {
            assert!(msg.contains("Failed to create company"));
        } else {
            panic!("Expected Validation error");
        }
    }

    #[sqlx::test]
    async fn test_find_company_by_id_success(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let company_id = Uuid::new_v4();
        let creator_id = Uuid::new_v4();
        let company = create_test_company(company_id, creator_id);

        // Setup
        repo.create(&company).await.unwrap();

        // Act
        let result = repo.find_by_id(&company_id).await;

        // Assert
        assert!(result.is_ok());
        let found_company = result.unwrap();
        assert!(found_company.is_some());
        let found_company = found_company.unwrap();
        assert_eq!(found_company.id, company_id);
        assert_eq!(found_company.name, company.name);
        assert_eq!(found_company.created_by, creator_id);
    }

    #[sqlx::test]
    async fn test_find_company_by_id_not_found(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let non_existent_id = Uuid::new_v4();

        // Act
        let result = repo.find_by_id(&non_existent_id).await;

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[sqlx::test]
    async fn test_update_company_success(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let company_id = Uuid::new_v4();
        let creator_id = Uuid::new_v4();
        let mut company = create_test_company(company_id, creator_id);

        // Setup - Create company
        repo.create(&company).await.unwrap();

        // Act - Update company
        company.name = "Updated Company Name".to_string();
        company.description = Some("Updated description".to_string());
        company.updated_at = Utc::now();

        let result = repo.update(&company).await;

        // Assert
        assert!(result.is_ok());

        // Verify update
        let updated_company = repo.find_by_id(&company_id).await.unwrap().unwrap();
        assert_eq!(updated_company.name, "Updated Company Name");
        assert_eq!(
            updated_company.description,
            Some("Updated description".to_string())
        );
    }

    #[sqlx::test]
    async fn test_update_company_not_found(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let non_existent_company = create_test_company(Uuid::new_v4(), Uuid::new_v4());

        // Act
        let result = repo.update(&non_existent_company).await;

        // Assert
        assert!(result.is_err());
        if let Err(DomainError::CompanyNotFound(id)) = result {
            assert_eq!(id, non_existent_company.id.to_string());
        } else {
            panic!("Expected CompanyNotFound error");
        }
    }

    #[sqlx::test]
    async fn test_delete_company_success(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let company_id = Uuid::new_v4();
        let creator_id = Uuid::new_v4();
        let company = create_test_company(company_id, creator_id);

        // Setup
        repo.create(&company).await.unwrap();

        // Act
        let result = repo.delete(&company_id).await;

        // Assert
        assert!(result.is_ok());

        // Verify deletion
        let deleted_company = repo.find_by_id(&company_id).await.unwrap();
        assert!(deleted_company.is_none());
    }

    #[sqlx::test]
    async fn test_delete_company_not_found(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let non_existent_id = Uuid::new_v4();

        // Act
        let result = repo.delete(&non_existent_id).await;

        // Assert
        assert!(result.is_err());
        if let Err(DomainError::CompanyNotFound(id)) = result {
            assert_eq!(id, non_existent_id.to_string());
        } else {
            panic!("Expected CompanyNotFound error");
        }
    }

    #[sqlx::test]
    async fn test_find_all_companies_pagination(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let creator_id = Uuid::new_v4();

        // Create multiple companies
        for i in 0..5 {
            let company = create_test_company(Uuid::new_v4(), creator_id);
            repo.create(&company).await.unwrap();
        }

        // Act - Get first page
        let result_page1 = repo.find_all(1, 3).await;

        // Assert
        assert!(result_page1.is_ok());
        let companies_page1 = result_page1.unwrap();
        assert_eq!(companies_page1.len(), 3);

        // Act - Get second page
        let result_page2 = repo.find_all(2, 3).await;

        // Assert
        assert!(result_page2.is_ok());
        let companies_page2 = result_page2.unwrap();
        assert_eq!(companies_page2.len(), 2);
    }

    #[sqlx::test]
    async fn test_find_companies_by_creator(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let creator1_id = Uuid::new_v4();
        let creator2_id = Uuid::new_v4();

        // Create companies for creator 1
        let company1 = create_test_company(Uuid::new_v4(), creator1_id);
        let company2 = create_test_company(Uuid::new_v4(), creator1_id);

        // Create company for creator 2
        let company3 = create_test_company(Uuid::new_v4(), creator2_id);

        // Setup
        repo.create(&company1).await.unwrap();
        repo.create(&company2).await.unwrap();
        repo.create(&company3).await.unwrap();

        // Act - Find companies by creator 1
        let result = repo.find_by_creator(&creator1_id).await;

        // Assert
        assert!(result.is_ok());
        let companies = result.unwrap();
        assert_eq!(companies.len(), 2);

        let company_ids: Vec<Uuid> = companies.iter().map(|c| c.id).collect();
        assert!(company_ids.contains(&company1.id));
        assert!(company_ids.contains(&company2.id));
        assert!(!company_ids.contains(&company3.id));
    }

    #[sqlx::test]
    async fn test_exists_by_name(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let company = create_test_company(Uuid::new_v4(), Uuid::new_v4());
        let test_name = company.name.clone();

        // Test before creation
        let exists_before = repo.exists_by_name(&test_name).await.unwrap();
        assert!(!exists_before);

        // Setup
        repo.create(&company).await.unwrap();

        // Test after creation
        let exists_after = repo.exists_by_name(&test_name).await.unwrap();
        assert!(exists_after);
    }

    #[sqlx::test]
    async fn test_find_by_name_success(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let company = create_test_company(Uuid::new_v4(), Uuid::new_v4());
        let company_name = company.name.clone();

        // Setup
        repo.create(&company).await.unwrap();

        // Act
        let result = repo.find_by_name(&company_name).await;

        // Assert
        assert!(result.is_ok());
        let found_company = result.unwrap();
        assert!(found_company.is_some());
        assert_eq!(found_company.unwrap().name, company_name);
    }

    #[sqlx::test]
    async fn test_find_by_name_not_found(pool: PgPool) {
        let repo = CompanyRepositoryImpl::new(pool);
        let non_existent_name = "Non Existent Company";

        // Act
        let result = repo.find_by_name(non_existent_name).await;

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
