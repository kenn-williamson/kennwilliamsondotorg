use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// Local password authentication credentials
/// Optional table - OAuth-only users won't have a row here
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserCredentials {
    pub user_id: Uuid,
    pub password_hash: String,
    pub password_updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_credentials_serialization() {
        let creds = UserCredentials {
            user_id: Uuid::new_v4(),
            password_hash: "$2b$12$test_hash".to_string(),
            password_updated_at: Utc::now(),
            created_at: Utc::now(),
        };

        // Verify serialization works
        let json = serde_json::to_string(&creds).unwrap();
        assert!(json.contains("password_hash"));
        assert!(json.contains("$2b$12$test_hash"));
    }

    #[test]
    fn test_user_credentials_structure() {
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        let creds = UserCredentials {
            user_id,
            password_hash: "hashed_password".to_string(),
            password_updated_at: now,
            created_at: now,
        };

        assert_eq!(creds.user_id, user_id);
        assert_eq!(creds.password_hash, "hashed_password");
        assert_eq!(creds.password_updated_at, now);
    }

    #[test]
    fn test_user_credentials_debug_trait() {
        let creds = UserCredentials {
            user_id: Uuid::new_v4(),
            password_hash: "secret".to_string(),
            password_updated_at: Utc::now(),
            created_at: Utc::now(),
        };

        // Verify Debug trait works
        let debug_str = format!("{:?}", creds);
        assert!(debug_str.contains("UserCredentials"));
    }
}
