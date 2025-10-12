use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// User profile data (bio, avatar, etc.)
/// Optional data - users may not have profile information
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_profile_serialization() {
        let profile = UserProfile {
            user_id: Uuid::new_v4(),
            real_name: Some("John Doe".to_string()),
            bio: Some("Software developer".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            location: Some("San Francisco".to_string()),
            website: Some("https://johndoe.com".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("Software developer"));
    }

    #[test]
    fn test_user_profile_minimal() {
        // Profile with all optional fields as None
        let profile = UserProfile {
            user_id: Uuid::new_v4(),
            real_name: None,
            bio: None,
            avatar_url: None,
            location: None,
            website: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(profile.real_name.is_none());
        assert!(profile.bio.is_none());
        assert!(profile.avatar_url.is_none());
    }

    #[test]
    fn test_user_profile_partial() {
        // Profile with some fields filled
        let profile = UserProfile {
            user_id: Uuid::new_v4(),
            real_name: Some("Jane Smith".to_string()),
            bio: None,
            avatar_url: Some("https://example.com/jane.jpg".to_string()),
            location: None,
            website: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(profile.real_name, Some("Jane Smith".to_string()));
        assert!(profile.bio.is_none());
        assert!(profile.avatar_url.is_some());
    }

    #[test]
    fn test_user_profile_from_oauth() {
        // Profile created from OAuth data
        let profile = UserProfile {
            user_id: Uuid::new_v4(),
            real_name: Some("OAuth User".to_string()),
            bio: None,
            avatar_url: Some("https://lh3.googleusercontent.com/...".to_string()),
            location: None,
            website: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(profile.real_name.is_some());
        assert!(profile.avatar_url.is_some());
    }
}
