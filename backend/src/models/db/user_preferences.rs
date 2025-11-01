use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// User application preferences
/// Every user has preferences (created with defaults)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserPreferences {
    pub user_id: Uuid,
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserPreferences {
    /// Create default preferences for a new user (primarily for testing)
    #[cfg(test)]
    pub fn default_for_user(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            timer_is_public: false,
            timer_show_in_list: false,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_preferences_serialization() {
        let prefs = UserPreferences {
            user_id: Uuid::new_v4(),
            timer_is_public: true,
            timer_show_in_list: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&prefs).unwrap();
        assert!(json.contains("timer_is_public"));
        assert!(json.contains("timer_show_in_list"));
    }

    #[test]
    fn test_user_preferences_defaults() {
        let user_id = Uuid::new_v4();
        let prefs = UserPreferences::default_for_user(user_id);

        assert_eq!(prefs.user_id, user_id);
        assert!(!prefs.timer_is_public);
        assert!(!prefs.timer_show_in_list);
    }

    #[test]
    fn test_user_preferences_public_timer() {
        let prefs = UserPreferences {
            user_id: Uuid::new_v4(),
            timer_is_public: true,
            timer_show_in_list: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(prefs.timer_is_public);
        assert!(prefs.timer_show_in_list);
    }

    #[test]
    fn test_user_preferences_validation_ready() {
        // Future: Business rule - show_in_list requires is_public
        // This test documents the expected constraint
        let prefs = UserPreferences {
            user_id: Uuid::new_v4(),
            timer_is_public: false,
            timer_show_in_list: true, // This should be validated at service layer
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Model allows it (validation happens in service layer)
        assert!(!prefs.timer_is_public);
        assert!(prefs.timer_show_in_list);
        // Note: This invalid state is allowed at model level
        // but should be caught by service layer validation
    }
}
