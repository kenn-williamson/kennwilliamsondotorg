use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// User with timer data for public timer list
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserWithTimer {
    pub id: Uuid,
    pub display_name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

/// User with roles from database (raw SQLx result)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserWithRoles {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub roles: Option<Vec<String>>,
}

/// Verification token for email verification
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct VerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Password reset token for password recovery flow
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Core user model with only identity fields
/// This is the primary User model after the auth schema refactor (Phase 9)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// TEST HELPERS - Available throughout crate for test code
// ============================================================================

#[cfg(test)]
/// Test helper functions for creating User instances in unit tests
/// These avoid brittle manual struct construction throughout test code
pub mod test_helpers {
    use super::*;

    /// Build a test User with random ID and sensible defaults
    pub fn build_test_user() -> User {
        let id = Uuid::new_v4();
        User {
            id,
            email: format!("test-{}@example.com", id),
            display_name: "Test User".to_string(),
            slug: format!("test-user-{}", id),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Build a test User with a specific ID
    pub fn build_test_user_with_id(id: Uuid) -> User {
        User {
            id,
            email: format!("test-{}@example.com", id),
            display_name: "Test User".to_string(),
            slug: format!("test-user-{}", id),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Build a UserWithRoles with optional roles
    pub fn build_test_user_with_roles(roles: Option<Vec<String>>) -> UserWithRoles {
        let id = Uuid::new_v4();
        UserWithRoles {
            id,
            email: format!("test-{}@example.com", id),
            display_name: "Test User".to_string(),
            slug: format!("test-user-{}", id),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            roles,
        }
    }

    /// Build an admin user with admin and user roles
    pub fn build_test_admin_user() -> UserWithRoles {
        let id = Uuid::new_v4();
        UserWithRoles {
            id,
            email: format!("admin-{}@example.com", id),
            display_name: "Admin User".to_string(),
            slug: format!("admin-user-{}", id),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            roles: Some(vec!["user".to_string(), "admin".to_string()]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_user_serialization() {
        // Test that User only contains core identity fields
        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Verify serialization works
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("test@example.com"));
        assert!(json.contains("test-user"));

        // Verify it doesn't contain removed fields
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("google_user_id"));
        assert!(!json.contains("timer_is_public"));
    }

    #[test]
    fn test_user_has_no_auth_fields() {
        // Verify User doesn't expose sensitive auth data
        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_value(&user).unwrap();

        // These fields should NOT exist in User
        assert!(json.get("password_hash").is_none());
        assert!(json.get("google_user_id").is_none());
        assert!(json.get("real_name").is_none());
        assert!(json.get("timer_is_public").is_none());
    }

}
