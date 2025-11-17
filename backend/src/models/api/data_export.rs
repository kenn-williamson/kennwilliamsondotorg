use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserDataExport {
    pub export_date: DateTime<Utc>,
    pub export_version: String,
    pub user: UserExportData,
    pub authentication: AuthenticationExport,
    pub external_logins: Vec<ExternalLoginExport>,
    pub profile: Option<ProfileExport>,
    pub preferences: Option<PreferencesExport>,
    pub incident_timers: Vec<IncidentTimerExportData>,
    pub phrase_suggestions: Vec<PhraseSuggestionExportData>,
    pub phrase_exclusions: Vec<PhraseExclusionExportData>,
    pub active_sessions: Vec<SessionExportData>,
    pub verification_history: Vec<VerificationTokenExportData>,
    pub password_reset_history: Vec<PasswordResetExportData>,
    pub email_suppression: Option<EmailSuppressionExport>,
}

#[derive(Debug, Serialize)]
pub struct UserExportData {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthenticationExport {
    pub has_password: bool,
    pub password_last_changed: Option<DateTime<Utc>>,
    // NOTE: password_hash is NOT included for security reasons
}

#[derive(Debug, Serialize)]
pub struct ExternalLoginExport {
    pub provider: String,
    pub provider_user_id: String,
    pub linked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProfileExport {
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PreferencesExport {
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
}

#[derive(Debug, Serialize)]
pub struct IncidentTimerExportData {
    pub id: Uuid,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PhraseSuggestionExportData {
    pub id: Uuid,
    pub user_id: Uuid,
    pub phrase_text: String,
    pub status: String,
    pub admin_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PhraseExclusionExportData {
    pub id: Uuid,
    pub phrase_text: String,
    pub excluded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SessionExportData {
    pub id: Uuid,
    pub device_info: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VerificationTokenExportData {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PasswordResetExportData {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct EmailSuppressionExport {
    pub email: String,
    pub suppression_reason: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_user_data_export_serialization() {
        // Test JSON serialization of UserDataExport struct
        // Verify proper field names and types

        let export_data = UserDataExport {
            export_date: Utc::now(),
            export_version: "2.0".to_string(),
            user: UserExportData {
                id: Uuid::new_v4(),
                email: "test@example.com".to_string(),
                display_name: "Test User".to_string(),
                slug: "testuser".to_string(),
                active: true,
                email_verified: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                roles: vec!["user".to_string(), "email-verified".to_string()],
            },
            authentication: AuthenticationExport {
                has_password: true,
                password_last_changed: Some(Utc::now()),
            },
            external_logins: vec![],
            profile: Some(ProfileExport {
                real_name: Some("Test User Real Name".to_string()),
                bio: None,
                avatar_url: None,
                location: None,
                website: None,
            }),
            preferences: Some(PreferencesExport {
                timer_is_public: false,
                timer_show_in_list: false,
            }),
            incident_timers: vec![],
            phrase_suggestions: vec![],
            phrase_exclusions: vec![],
            active_sessions: vec![],
            verification_history: vec![],
            password_reset_history: vec![],
            email_suppression: None,
        };

        // Test that serialization works
        let json_result = serde_json::to_string(&export_data);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        // Verify required fields are present
        assert!(parsed.get("export_date").is_some());
        assert_eq!(
            parsed.get("export_version").unwrap().as_str().unwrap(),
            "2.0"
        );
        assert!(parsed.get("user").is_some());
        assert!(parsed.get("authentication").is_some());
        assert!(parsed.get("external_logins").is_some());
        assert!(parsed.get("profile").is_some());
        assert!(parsed.get("preferences").is_some());
        assert!(parsed.get("incident_timers").is_some());
        assert!(parsed.get("phrase_suggestions").is_some());
        assert!(parsed.get("phrase_exclusions").is_some());
        assert!(parsed.get("active_sessions").is_some());
        assert!(parsed.get("verification_history").is_some());
        assert!(parsed.get("password_reset_history").is_some());
        assert!(parsed.get("email_suppression").is_some());

        // Verify user data structure
        let user = parsed.get("user").unwrap();
        assert!(user.get("id").is_some());
        assert!(user.get("email").is_some());
        assert!(user.get("display_name").is_some());
        assert!(user.get("slug").is_some());
        assert!(user.get("active").is_some());
        assert!(user.get("email_verified").is_some());
        assert!(user.get("created_at").is_some());
        assert!(user.get("updated_at").is_some());
        assert!(user.get("roles").is_some());

        // Verify authentication structure
        let auth = parsed.get("authentication").unwrap();
        assert!(auth.get("has_password").is_some());
        assert!(auth.get("password_last_changed").is_some());
        assert!(auth.get("password_hash").is_none()); // Security: must NOT be present

        // Verify profile structure
        let profile = parsed.get("profile").unwrap();
        assert!(profile.get("real_name").is_some());

        // Verify sensitive data is NOT present
        assert!(user.get("password_hash").is_none());
        assert!(parsed.get("password_hash").is_none());
    }

    #[test]
    fn test_export_structure_validation() {
        // Test that all required fields are present
        // Test that optional fields are handled correctly

        let export_data = UserDataExport {
            export_date: Utc::now(),
            export_version: "2.0".to_string(),
            user: UserExportData {
                id: Uuid::new_v4(),
                email: "test@example.com".to_string(),
                display_name: "Test User".to_string(),
                slug: "testuser".to_string(),
                active: true,
                email_verified: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                roles: vec!["user".to_string()],
            },
            authentication: AuthenticationExport {
                has_password: false,
                password_last_changed: None,
            },
            external_logins: vec![],
            profile: None,     // Test optional field
            preferences: None, // Test optional field
            incident_timers: vec![],
            phrase_suggestions: vec![],
            phrase_exclusions: vec![],
            active_sessions: vec![],
            verification_history: vec![],
            password_reset_history: vec![],
            email_suppression: None,
        };

        // Test serialization with optional fields as None
        let json_result = serde_json::to_string(&export_data);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        // Verify optional fields are handled correctly
        assert!(parsed.get("profile").unwrap().is_null()); // Should be null in JSON
        assert!(parsed.get("preferences").unwrap().is_null()); // Should be null in JSON
        assert!(parsed.get("email_suppression").unwrap().is_null());

        // Verify arrays are present (even if empty)
        assert!(parsed.get("external_logins").unwrap().is_array());
        assert!(parsed.get("incident_timers").unwrap().is_array());
        assert!(parsed.get("phrase_suggestions").unwrap().is_array());
        assert!(parsed.get("phrase_exclusions").unwrap().is_array());
        assert!(parsed.get("active_sessions").unwrap().is_array());
        assert!(parsed.get("verification_history").unwrap().is_array());
        assert!(parsed.get("password_reset_history").unwrap().is_array());
    }

    #[test]
    fn test_incident_timer_export_data_serialization() {
        // Test IncidentTimerExportData serialization
        let timer_data = IncidentTimerExportData {
            id: Uuid::new_v4(),
            reset_timestamp: Utc::now(),
            notes: Some("Test notes".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json_result = serde_json::to_string(&timer_data);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert!(parsed.get("id").is_some());
        assert!(parsed.get("reset_timestamp").is_some());
        assert!(parsed.get("notes").is_some());
        assert!(parsed.get("created_at").is_some());
        assert!(parsed.get("updated_at").is_some());
    }

    #[test]
    fn test_phrase_suggestion_export_data_serialization() {
        // Test PhraseSuggestionExportData serialization
        let suggestion_data = PhraseSuggestionExportData {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            phrase_text: "Test phrase".to_string(),
            status: "pending".to_string(),
            admin_reason: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json_result = serde_json::to_string(&suggestion_data);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert!(parsed.get("id").is_some());
        assert!(parsed.get("user_id").is_some());
        assert!(parsed.get("phrase_text").is_some());
        assert!(parsed.get("status").is_some());
        assert!(parsed.get("admin_reason").is_some()); // Should be null
        assert!(parsed.get("created_at").is_some());
        assert!(parsed.get("updated_at").is_some());
    }

    #[test]
    fn test_session_export_data_serialization() {
        // Test SessionExportData serialization
        let session_data = SessionExportData {
            id: Uuid::new_v4(),
            device_info: Some(serde_json::json!({
                "user_agent": "Mozilla/5.0...",
                "ip_address": "192.168.1.1",
                "timestamp": "2025-10-04T10:00:00Z"
            })),
            created_at: Utc::now(),
            last_used_at: Some(Utc::now()),
            expires_at: Utc::now() + chrono::Duration::days(7),
        };

        let json_result = serde_json::to_string(&session_data);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert!(parsed.get("id").is_some());
        assert!(parsed.get("device_info").is_some());
        assert!(parsed.get("created_at").is_some());
        assert!(parsed.get("last_used_at").is_some());
        assert!(parsed.get("expires_at").is_some());

        // Verify device_info is properly serialized
        let device_info = parsed.get("device_info").unwrap();
        assert!(device_info.get("user_agent").is_some());
        assert!(device_info.get("ip_address").is_some());
        assert!(device_info.get("timestamp").is_some());
    }
}
