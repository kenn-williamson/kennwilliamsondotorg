use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// OAuth external login (supports multiple providers per user)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserExternalLogin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub linked_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_external_login_serialization() {
        let login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            provider: "google".to_string(),
            provider_user_id: "google_12345".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&login).unwrap();
        assert!(json.contains("google"));
        assert!(json.contains("google_12345"));
    }

    #[test]
    fn test_user_external_login_google_provider() {
        let login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(login.provider, "google");
        assert_eq!(login.provider_user_id, "google_123");
    }

    #[test]
    fn test_user_external_login_github_provider() {
        let login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            provider: "github".to_string(),
            provider_user_id: "github_456".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(login.provider, "github");
        assert_eq!(login.provider_user_id, "github_456");
    }

    #[test]
    fn test_user_external_login_multiple_providers() {
        // User can have multiple external logins
        let user_id = Uuid::new_v4();

        let google_login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id,
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let github_login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id,
            provider: "github".to_string(),
            provider_user_id: "github_456".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Both logins for same user but different providers
        assert_eq!(google_login.user_id, github_login.user_id);
        assert_ne!(google_login.provider, github_login.provider);
        assert_ne!(google_login.id, github_login.id);
    }
}
