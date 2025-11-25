#[cfg(test)]
mod tests {
    use auth_service::{domain::enums::UserRole, infrastructure::auth::middleware::RoleGuard};

    #[test]
    fn test_role_guard_admin() {
        let guard = RoleGuard::admin();

        assert!(guard.check(&UserRole::Admin));
        assert!(!guard.check(&UserRole::Partner));
        assert!(!guard.check(&UserRole::Operator));
        assert!(!guard.check(&UserRole::User));
        assert!(!guard.check(&UserRole::Guest));
    }

    #[test]
    fn test_role_guard_company_management() {
        let guard = RoleGuard::company_management();

        assert!(guard.check(&UserRole::Admin));
        assert!(guard.check(&UserRole::Partner));
        assert!(guard.check(&UserRole::Operator));
        assert!(!guard.check(&UserRole::User));
        assert!(!guard.check(&UserRole::Guest));
    }

    #[test]
    fn test_role_guard_user_management() {
        let guard = RoleGuard::user_management();

        assert!(guard.check(&UserRole::Admin));
        assert!(guard.check(&UserRole::Partner));
        assert!(guard.check(&UserRole::Operator));
        assert!(!guard.check(&UserRole::User));
        assert!(!guard.check(&UserRole::Guest));
    }

    #[test]
    fn test_role_guard_authenticated() {
        let guard = RoleGuard::authenticated();

        assert!(guard.check(&UserRole::Admin));
        assert!(guard.check(&UserRole::Partner));
        assert!(guard.check(&UserRole::Operator));
        assert!(guard.check(&UserRole::User));
        assert!(guard.check(&UserRole::Guest));
    }

    #[test]
    fn test_role_guard_custom_roles() {
        let guard = RoleGuard::new(vec![UserRole::Admin, UserRole::User]);

        assert!(guard.check(&UserRole::Admin));
        assert!(!guard.check(&UserRole::Partner));
        assert!(!guard.check(&UserRole::Operator));
        assert!(guard.check(&UserRole::User));
        assert!(!guard.check(&UserRole::Guest));
    }

    #[test]
    fn test_role_guard_empty_roles() {
        let guard = RoleGuard::new(vec![]);

        assert!(!guard.check(&UserRole::Admin));
        assert!(!guard.check(&UserRole::Partner));
        assert!(!guard.check(&UserRole::Operator));
        assert!(!guard.check(&UserRole::User));
        assert!(!guard.check(&UserRole::Guest));
    }
}
