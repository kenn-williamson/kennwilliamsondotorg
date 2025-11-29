use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// Unsubscribe token for one-click email unsubscribe
/// Stores hashed tokens for security - raw token is sent in email, hash is stored
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UnsubscribeToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub email_type: String,
    pub created_at: DateTime<Utc>,
}

/// Information returned when looking up a token
#[derive(Debug, Clone)]
pub struct UnsubscribeTokenInfo {
    pub user_id: Uuid,
    pub email_type: String,
}

/// Email type constants for unsubscribe tokens
pub mod email_types {
    pub const BLOG_NOTIFICATIONS: &str = "blog_notifications";
    // Future email types can be added here:
    // pub const ANNOUNCEMENTS: &str = "announcements";
    // pub const WEEKLY_DIGEST: &str = "weekly_digest";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsubscribe_token_creation() {
        let token = UnsubscribeToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token_hash: "abc123hash".to_string(),
            email_type: email_types::BLOG_NOTIFICATIONS.to_string(),
            created_at: Utc::now(),
        };

        assert_eq!(token.email_type, "blog_notifications");
    }

    #[test]
    fn test_unsubscribe_token_serialization() {
        let token = UnsubscribeToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token_hash: "abc123hash".to_string(),
            email_type: email_types::BLOG_NOTIFICATIONS.to_string(),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&token).unwrap();
        assert!(json.contains("token_hash"));
        assert!(json.contains("email_type"));
        assert!(json.contains("blog_notifications"));
    }

    #[test]
    fn test_unsubscribe_token_info() {
        let info = UnsubscribeTokenInfo {
            user_id: Uuid::new_v4(),
            email_type: email_types::BLOG_NOTIFICATIONS.to_string(),
        };

        assert_eq!(info.email_type, "blog_notifications");
    }
}
