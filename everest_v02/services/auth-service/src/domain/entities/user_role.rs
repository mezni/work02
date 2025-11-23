use crate::domain::value_objects::UserId;
use chrono::{DateTime, Utc};

/// User Role Entity - connects users to roles with context
#[derive(Debug, Clone)]
pub struct UserRole {
    user_id: UserId,
    role_name: String,
    assigned_at: DateTime<Utc>,
    assigned_by: UserId,
    scope: Option<String>, // For multi-tenant systems
}

impl UserRole {
    /// Create a new user role assignment
    pub fn new(
        user_id: UserId,
        role_name: String,
        assigned_by: UserId,
        scope: Option<String>,
    ) -> Self {
        Self {
            user_id,
            role_name,
            assigned_at: Utc::now(),
            assigned_by,
            scope,
        }
    }

    // Access methods
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn role_name(&self) -> &str {
        &self.role_name
    }

    pub fn assigned_at(&self) -> DateTime<Utc> {
        self.assigned_at
    }

    pub fn assigned_by(&self) -> UserId {
        self.assigned_by
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    // Business logic
    pub fn is_scoped(&self) -> bool {
        self.scope.is_some()
    }

    pub fn matches_scope(&self, scope: &str) -> bool {
        match &self.scope {
            Some(s) => s == scope,
            None => false,
        }
    }

    pub fn is_global(&self) -> bool {
        self.scope.is_none()
    }
}

// Identity is the combination of user_id, role_name, and scope
impl PartialEq for UserRole {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
            && self.role_name == other.role_name
            && self.scope == other.scope
    }
}

impl Eq for UserRole {}
