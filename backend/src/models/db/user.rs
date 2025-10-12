use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// Current monolithic user model
/// TODO(Phase 7): Remove password_hash, google_user_id, real_name, timer_* fields
/// after data migration is complete
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,        // TODO: Move to user_credentials
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub real_name: Option<String>,            // TODO: Move to user_profiles
    pub google_user_id: Option<String>,       // TODO: Move to user_external_logins
    pub timer_is_public: bool,                // TODO: Move to user_preferences
    pub timer_show_in_list: bool,             // TODO: Move to user_preferences
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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
    pub real_name: Option<String>,
    pub google_user_id: Option<String>,
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
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
/// This is the target structure after refactoring
/// Will be renamed to `User` in Phase 9 after old `User` is removed
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserCore {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserCore {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            slug: user.slug,
            active: user.active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// External login information for user details
#[derive(Debug, Clone, Serialize)]
pub struct ExternalLoginInfo {
    pub provider: String,
    pub linked_at: DateTime<Utc>,
}

/// Profile information summary
#[derive(Debug, Clone, Serialize)]
pub struct ProfileInfo {
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

/// Preferences summary
#[derive(Debug, Clone, Serialize)]
pub struct PreferencesInfo {
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
}

/// Complete user data with all related information
/// Used when we need full user context (e.g., profile pages, admin views)
#[derive(Debug, Clone, Serialize)]
pub struct UserWithDetails {
    pub user: UserCore,
    pub has_password: bool,
    pub external_logins: Vec<ExternalLoginInfo>,
    pub profile: Option<ProfileInfo>,
    pub preferences: Option<PreferencesInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_user_core_serialization() {
        // Test that UserCore only contains core identity fields
        let user = UserCore {
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
    fn test_user_core_has_no_auth_fields() {
        // Verify UserCore doesn't expose sensitive auth data
        let user = UserCore {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_value(&user).unwrap();

        // These fields should NOT exist in UserCore
        assert!(json.get("password_hash").is_none());
        assert!(json.get("google_user_id").is_none());
        assert!(json.get("real_name").is_none());
        assert!(json.get("timer_is_public").is_none());
    }

    #[test]
    fn test_user_with_details_structure() {
        // Test composite structure for full user data
        let user = UserCore {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let details = UserWithDetails {
            user: user.clone(),
            has_password: true,
            external_logins: vec![],
            profile: None,
            preferences: None,
        };

        // Verify structure
        assert_eq!(details.user.id, user.id);
        assert_eq!(details.has_password, true);
        assert_eq!(details.external_logins.len(), 0);
        assert!(details.profile.is_none());
    }

    #[test]
    fn test_user_with_details_oauth_user() {
        // Test OAuth-only user (no password)
        let user = UserCore {
            id: Uuid::new_v4(),
            email: "oauth@example.com".to_string(),
            display_name: "OAuth User".to_string(),
            slug: "oauth-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let external_login = ExternalLoginInfo {
            provider: "google".to_string(),
            linked_at: Utc::now(),
        };

        let details = UserWithDetails {
            user: user.clone(),
            has_password: false,
            external_logins: vec![external_login],
            profile: None,
            preferences: None,
        };

        // Verify OAuth-only structure
        assert_eq!(details.has_password, false);
        assert_eq!(details.external_logins.len(), 1);
        assert_eq!(details.external_logins[0].provider, "google");
    }
}
