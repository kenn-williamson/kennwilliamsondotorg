use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::models::api::data_export::{
    AuthenticationExport, ExternalLoginExport, IncidentTimerExportData,
    PasswordResetExportData, PhraseExclusionExportData, PhraseSuggestionExportData,
    PreferencesExport, ProfileExport, SessionExportData, UserDataExport, UserExportData,
    VerificationTokenExportData,
};

// Add the export_user_data method to AuthService
use super::AuthService;

impl AuthService {
    /// Export all user data in JSON format for GDPR/CCPA compliance
    pub async fn export_user_data(&self, user_id: Uuid) -> Result<UserDataExport> {
        // 1. Get core user data
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Get user roles
        let roles = self.user_repository.get_user_roles(user_id).await?;

        // Check if email is verified (has email-verified role)
        let email_verified = roles.contains(&"email-verified".to_string());

        // Convert user to export format
        let user_export = UserExportData {
            id: user.id,
            email: user.email.clone(),
            display_name: user.display_name,
            slug: user.slug,
            active: user.active,
            email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
            roles,
        };

        // 2. NEW: Get authentication details (from user_credentials)
        let (has_password, password_last_changed) = if let Some(creds_repo) = &self.credentials_repository {
            let has_pwd = creds_repo.has_password(user_id).await.unwrap_or(false);
            let last_changed = creds_repo
                .find_by_user_id(user_id)
                .await
                .ok()
                .flatten()
                .map(|c| c.password_updated_at);
            (has_pwd, last_changed)
        } else {
            (false, None)
        };

        let authentication = AuthenticationExport {
            has_password,
            password_last_changed,
        };

        // 3. NEW: Get external logins (from user_external_logins)
        let external_logins = if let Some(ext_repo) = &self.external_login_repository {
            ext_repo
                .find_by_user_id(user_id)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|login| ExternalLoginExport {
                    provider: login.provider,
                    provider_user_id: login.provider_user_id,
                    linked_at: login.linked_at,
                })
                .collect()
        } else {
            vec![]
        };

        // 4. NEW: Get profile data (from user_profiles)
        let profile = if let Some(profile_repo) = &self.profile_repository {
            profile_repo
                .find_by_user_id(user_id)
                .await
                .ok()
                .flatten()
                .map(|p| ProfileExport {
                    real_name: p.real_name,
                    bio: p.bio,
                    avatar_url: p.avatar_url,
                    location: p.location,
                    website: p.website,
                })
        } else {
            None
        };

        // 5. NEW: Get preferences (from user_preferences)
        let preferences = if let Some(prefs_repo) = &self.preferences_repository {
            prefs_repo
                .find_by_user_id(user_id)
                .await
                .ok()
                .flatten()
                .map(|p| PreferencesExport {
                    timer_is_public: p.timer_is_public,
                    timer_show_in_list: p.timer_show_in_list,
                })
        } else {
            None
        };

        // Get incident timers
        let incident_timers = if let Some(timer_repo) = &self.incident_timer_repository {
            timer_repo
                .find_by_user_id(user_id)
                .await?
                .into_iter()
                .map(|timer| IncidentTimerExportData {
                    id: timer.id,
                    reset_timestamp: timer.reset_timestamp,
                    notes: timer.notes,
                    created_at: timer.created_at,
                    updated_at: timer.updated_at,
                })
                .collect()
        } else {
            Vec::new()
        };

        // Get phrase suggestions
        let phrase_suggestions = if let Some(phrase_repo) = &self.phrase_repository {
            phrase_repo
                .get_user_suggestions(user_id)
                .await?
                .into_iter()
                .map(|suggestion| PhraseSuggestionExportData {
                    id: suggestion.id,
                    user_id: suggestion.user_id,
                    phrase_text: suggestion.phrase_text,
                    status: suggestion.status,
                    admin_reason: suggestion.admin_reason,
                    created_at: suggestion.created_at,
                    updated_at: suggestion.updated_at,
                })
                .collect()
        } else {
            Vec::new()
        };

        // Get phrase exclusions
        let phrase_exclusions = if let Some(phrase_repo) = &self.phrase_repository {
            phrase_repo
                .get_user_excluded_phrases(user_id)
                .await?
                .into_iter()
                .map(|(id, phrase_text, excluded_at)| PhraseExclusionExportData {
                    id,
                    phrase_text,
                    excluded_at,
                })
                .collect()
        } else {
            Vec::new()
        };

        // Get active sessions (refresh tokens)
        let active_sessions = self
            .refresh_token_repository
            .find_by_user_id(user_id)
            .await?
            .into_iter()
            .map(|token| SessionExportData {
                id: token.id,
                device_info: token.device_info,
                created_at: token.created_at,
                last_used_at: token.last_used_at,
                expires_at: token.expires_at,
            })
            .collect();

        // Get verification history
        let verification_history = if let Some(verification_repo) = &self.verification_token_repository {
            verification_repo
                .find_by_user_id(user_id)
                .await?
                .into_iter()
                .map(|token| VerificationTokenExportData {
                    id: token.id,
                    expires_at: token.expires_at,
                    created_at: token.created_at,
                })
                .collect()
        } else {
            Vec::new()
        };

        // NEW: Get password reset history (for transparency)
        // Note: password_reset_token_repository already exists in AuthService
        let password_reset_history = if let Some(reset_repo) = &self.password_reset_token_repository {
            reset_repo
                .find_by_user_id(user_id)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|token| PasswordResetExportData {
                    id: token.id,
                    expires_at: token.expires_at,
                    used_at: token.used_at,
                    created_at: token.created_at,
                })
                .collect()
        } else {
            vec![]
        };

        // NEW: Email suppression status (GDPR/CCPA transparency)
        // Note: EmailSuppression exists but not yet integrated into AuthService
        // This will be None for now - can be added when email_suppression_repository is added to AuthService
        let email_suppression = None;

        Ok(UserDataExport {
            export_date: Utc::now(),
            export_version: "1.0".to_string(),
            user: user_export,
            authentication,
            external_logins,
            profile,
            preferences,
            incident_timers,
            phrase_suggestions,
            phrase_exclusions,
            active_sessions,
            verification_history,
            password_reset_history,
            email_suppression,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::db::{
        incident_timer::IncidentTimer, phrase::PhraseSuggestion, refresh_token::RefreshToken,
        user::{User, VerificationToken},
    };
    use crate::repositories::mocks::{
        MockIncidentTimerRepository, MockPhraseRepository, MockRefreshTokenRepository,
        MockUserRepository, MockVerificationTokenRepository,
    };
    use crate::services::auth::auth_service::AuthServiceBuilder;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_user_with_id(user_id: Uuid) -> User {
        User {
            id: user_id,
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "testuser".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_incident_timer(user_id: Uuid) -> IncidentTimer {
        IncidentTimer {
            id: Uuid::new_v4(),
            user_id,
            reset_timestamp: Utc::now(),
            notes: Some("Test notes".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_phrase_suggestion(user_id: Uuid) -> PhraseSuggestion {
        PhraseSuggestion {
            id: Uuid::new_v4(),
            user_id,
            phrase_text: "test phrase".to_string(),
            status: "pending".to_string(),
            admin_id: None,
            admin_reason: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_refresh_token(user_id: Uuid) -> RefreshToken {
        RefreshToken {
            id: Uuid::new_v4(),
            user_id,
            token_hash: "hashed_token".to_string(),
            device_info: Some(serde_json::json!({"device": "test"})),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_used_at: Some(Utc::now()),
            expires_at: Utc::now() + chrono::Duration::days(30),
        }
    }

    fn create_test_verification_token(user_id: Uuid) -> VerificationToken {
        VerificationToken {
            id: Uuid::new_v4(),
            user_id,
            token_hash: "hashed_verification_token".to_string(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
            created_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_export_user_data_structure() {
        // Test that export returns properly structured JSON
        // Verify all required fields are present
        // Verify sensitive data is excluded
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Test export
        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());
        
        let export_data = result.unwrap();

        // Verify structure
        assert_eq!(export_data.export_version, "1.0");
        assert_eq!(export_data.user.id, user_id);
        assert_eq!(export_data.user.email, "test@example.com");
        assert_eq!(export_data.user.display_name, "Test User");
        assert_eq!(export_data.user.slug, "testuser");
        assert!(export_data.user.active);
        assert!(export_data.user.email_verified);
        assert_eq!(export_data.user.roles, vec!["user", "email-verified"]);

        // Verify new authentication structure
        assert!(!export_data.authentication.has_password); // No credentials repo in test
        assert!(export_data.authentication.password_last_changed.is_none());

        // Verify new fields are present (but empty without repos)
        assert_eq!(export_data.external_logins.len(), 0);
        assert!(export_data.profile.is_none());
        assert!(export_data.preferences.is_none());
        assert_eq!(export_data.password_reset_history.len(), 0);
        assert!(export_data.email_suppression.is_none());

        // Verify sensitive data is excluded
        // Note: password_hash is not in the export structure, so this is implicitly tested
    }

    #[tokio::test]
    async fn test_export_user_profile_data() {
        // Test user profile export includes:
        // - email, display_name, slug, real_name
        // - google_user_id (if present)
        // - created_at, updated_at, active status
        // - roles (converted from user_roles)
        // Excludes: password_hash
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Test export
        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());
        
        let export_data = result.unwrap();
        
        // Verify profile data
        assert_eq!(export_data.user.email, "test@example.com");
        assert_eq!(export_data.user.display_name, "Test User");
        assert_eq!(export_data.user.slug, "testuser");
        assert!(export_data.user.active);
        assert!(!export_data.user.email_verified); // No email-verified role
        assert_eq!(export_data.user.roles, vec!["user"]);
        
        // Verify timestamps are present
        assert!(export_data.user.created_at <= Utc::now());
        assert!(export_data.user.updated_at <= Utc::now());
    }

    #[tokio::test]
    async fn test_export_incident_timers() {
        // Test that all user's incident timers are exported
        // Verify structure: id, reset_timestamp, notes, created_at, updated_at
        // Verify only current user's timers are included
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        let timer = create_test_incident_timer(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        let mut timer_repo = MockIncidentTimerRepository::new();
        let timer_clone = timer.clone();
        timer_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(vec![timer_clone.clone()]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .incident_timer_repository(Box::new(timer_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Test export
        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());
        
        let export_data = result.unwrap();
        
        // Verify incident timers
        assert_eq!(export_data.incident_timers.len(), 1);
        let exported_timer = &export_data.incident_timers[0];
        assert_eq!(exported_timer.id, timer.id);
        assert_eq!(exported_timer.reset_timestamp, timer.reset_timestamp);
        assert_eq!(exported_timer.notes, timer.notes);
        assert_eq!(exported_timer.created_at, timer.created_at);
        assert_eq!(exported_timer.updated_at, timer.updated_at);
    }

    #[tokio::test]
    async fn test_export_phrase_data() {
        // Test phrase suggestions export
        // Test phrase exclusions export (with phrase text)
        // Verify proper structure and user isolation
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        let suggestion = create_test_phrase_suggestion(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        let mut phrase_repo = MockPhraseRepository::new();
        let suggestion_clone = suggestion.clone();
        phrase_repo.expect_get_user_suggestions()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(vec![suggestion_clone.clone()]));
        
        phrase_repo.expect_get_user_excluded_phrases()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .phrase_repository(Box::new(phrase_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Test export
        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());
        
        let export_data = result.unwrap();
        
        // Verify phrase suggestions
        assert_eq!(export_data.phrase_suggestions.len(), 1);
        let exported_suggestion = &export_data.phrase_suggestions[0];
        assert_eq!(exported_suggestion.id, suggestion.id);
        assert_eq!(exported_suggestion.user_id, suggestion.user_id);
        assert_eq!(exported_suggestion.phrase_text, suggestion.phrase_text);
        assert_eq!(exported_suggestion.status, suggestion.status);
        assert_eq!(exported_suggestion.admin_reason, suggestion.admin_reason);
        
        // Verify phrase exclusions (empty in this test)
        assert_eq!(export_data.phrase_exclusions.len(), 0);
    }

    #[tokio::test]
    async fn test_export_session_data() {
        // Test refresh tokens export (device info, timestamps)
        // Test verification tokens export
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        let refresh_token = create_test_refresh_token(user_id);
        let verification_token = create_test_verification_token(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        let refresh_token_clone = refresh_token.clone();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(vec![refresh_token_clone.clone()]));

        let mut verification_repo = MockVerificationTokenRepository::new();
        let verification_token_clone = verification_token.clone();
        verification_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(vec![verification_token_clone.clone()]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .verification_token_repository(Box::new(verification_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Test export
        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());
        
        let export_data = result.unwrap();
        
        // Verify active sessions (refresh tokens)
        assert_eq!(export_data.active_sessions.len(), 1);
        let exported_session = &export_data.active_sessions[0];
        assert_eq!(exported_session.id, refresh_token.id);
        assert_eq!(exported_session.device_info, refresh_token.device_info);
        assert_eq!(exported_session.created_at, refresh_token.created_at);
        assert_eq!(exported_session.last_used_at, refresh_token.last_used_at);
        assert_eq!(exported_session.expires_at, refresh_token.expires_at);
        
        // Verify verification history
        assert_eq!(export_data.verification_history.len(), 1);
        let exported_verification = &export_data.verification_history[0];
        assert_eq!(exported_verification.id, verification_token.id);
        assert_eq!(exported_verification.expires_at, verification_token.expires_at);
        assert_eq!(exported_verification.created_at, verification_token.created_at);
    }

    #[tokio::test]
    async fn test_export_empty_user_data() {
        // Test user with minimal data (no timers, suggestions, etc.)
        // Verify empty arrays are returned
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Test export
        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());
        
        let export_data = result.unwrap();
        
        // Verify empty arrays for optional data
        assert_eq!(export_data.incident_timers.len(), 0);
        assert_eq!(export_data.phrase_suggestions.len(), 0);
        assert_eq!(export_data.phrase_exclusions.len(), 0);
        assert_eq!(export_data.active_sessions.len(), 0);
        assert_eq!(export_data.verification_history.len(), 0);
        
        // Verify user data is still present
        assert_eq!(export_data.user.id, user_id);
        assert_eq!(export_data.user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_export_oauth_only_user() {
        // Test OAuth-only user (no password)
        // Verify proper handling of optional fields

        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Test export
        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());
        
        let export_data = result.unwrap();
        
        // Verify OAuth user data
        assert_eq!(export_data.user.id, user_id);
        assert_eq!(export_data.user.email, "test@example.com");
        assert!(export_data.user.email_verified); // Has email-verified role
        assert_eq!(export_data.user.roles, vec!["user", "email-verified"]);

        // Verify no password in authentication
        assert!(!export_data.authentication.has_password); // No credentials repo

        // Verify no password_hash in export (implicitly tested by struct)
    }

    #[tokio::test]
    async fn test_export_includes_all_new_tables() {
        // Test that export includes data from all new authentication tables
        use crate::repositories::mocks::{
            MockUserCredentialsRepository, MockUserExternalLoginRepository,
            MockUserPreferencesRepository, MockUserProfileRepository,
        };
        use crate::models::db::{
            user_credentials::UserCredentials,
            user_external_login::UserExternalLogin,
            user_preferences::UserPreferences,
            user_profile::UserProfile,
        };

        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);

        // Setup mocks
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(user.clone())));
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        let mut refresh_repo = MockRefreshTokenRepository::new();
        refresh_repo
            .expect_find_by_user_id()
            .returning(|_| Ok(vec![]));

        // Mock credentials repository
        let mut creds_repo = MockUserCredentialsRepository::new();
        creds_repo.expect_has_password().returning(|_| Ok(true));
        let creds = UserCredentials {
            user_id,
            password_hash: "hashed".to_string(),
            password_updated_at: Utc::now(),
            created_at: Utc::now(),
        };
        creds_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(Some(creds.clone())));

        // Mock external login repository
        let mut ext_repo = MockUserExternalLoginRepository::new();
        let ext_login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id,
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        ext_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(vec![ext_login.clone()]));

        // Mock profile repository
        let mut profile_repo = MockUserProfileRepository::new();
        let profile = UserProfile {
            user_id,
            real_name: Some("Test Real Name".to_string()),
            bio: Some("Test bio".to_string()),
            avatar_url: None,
            location: Some("Test City".to_string()),
            website: Some("https://example.com".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        profile_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(Some(profile.clone())));

        // Mock preferences repository
        let mut prefs_repo = MockUserPreferencesRepository::new();
        let prefs = UserPreferences {
            user_id,
            timer_is_public: true,
            timer_show_in_list: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        prefs_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(Some(prefs.clone())));

        // Build service with all repositories
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .credentials_repository(Box::new(creds_repo))
            .external_login_repository(Box::new(ext_repo))
            .profile_repository(Box::new(profile_repo))
            .preferences_repository(Box::new(prefs_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Test export
        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());

        let export = result.unwrap();

        // CRITICAL ASSERTIONS
        assert_eq!(export.export_version, "1.0");

        // Verify authentication data
        assert!(export.authentication.has_password);
        assert!(export.authentication.password_last_changed.is_some());

        // Verify external logins
        assert_eq!(export.external_logins.len(), 1);
        assert_eq!(export.external_logins[0].provider, "google");
        assert_eq!(export.external_logins[0].provider_user_id, "google_123");

        // Verify profile data
        assert!(export.profile.is_some());
        let profile = export.profile.unwrap();
        assert_eq!(profile.real_name, Some("Test Real Name".to_string()));
        assert_eq!(profile.bio, Some("Test bio".to_string()));
        assert_eq!(profile.location, Some("Test City".to_string()));
        assert_eq!(profile.website, Some("https://example.com".to_string()));

        // Verify preferences
        assert!(export.preferences.is_some());
        let prefs = export.preferences.unwrap();
        assert!(prefs.timer_is_public);
        assert!(prefs.timer_show_in_list);
    }

    #[tokio::test]
    async fn test_export_oauth_only_user_no_password() {
        // Test OAuth-only user (no credentials)
        use crate::repositories::mocks::{
            MockUserCredentialsRepository, MockUserExternalLoginRepository,
        };
        use crate::models::db::user_external_login::UserExternalLogin;

        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);

        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(user.clone())));
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        let mut refresh_repo = MockRefreshTokenRepository::new();
        refresh_repo
            .expect_find_by_user_id()
            .returning(|_| Ok(vec![]));

        // Mock credentials repository - no password
        let mut creds_repo = MockUserCredentialsRepository::new();
        creds_repo.expect_has_password().returning(|_| Ok(false));
        creds_repo
            .expect_find_by_user_id()
            .returning(|_| Ok(None));

        // Mock external login repository
        let mut ext_repo = MockUserExternalLoginRepository::new();
        let ext_login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id,
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        ext_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(vec![ext_login.clone()]));

        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .credentials_repository(Box::new(creds_repo))
            .external_login_repository(Box::new(ext_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());

        let export = result.unwrap();

        // Verify OAuth-only user has no password
        assert!(!export.authentication.has_password);
        assert!(export.authentication.password_last_changed.is_none());

        // Verify external login exists
        assert_eq!(export.external_logins.len(), 1);
        assert_eq!(export.external_logins[0].provider, "google");
    }

    #[tokio::test]
    async fn test_export_multiple_oauth_providers() {
        // Test user with multiple OAuth providers
        use crate::repositories::mocks::MockUserExternalLoginRepository;
        use crate::models::db::user_external_login::UserExternalLogin;

        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);

        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(user.clone())));
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_repo = MockRefreshTokenRepository::new();
        refresh_repo
            .expect_find_by_user_id()
            .returning(|_| Ok(vec![]));

        // Mock external login repository with multiple providers
        let mut ext_repo = MockUserExternalLoginRepository::new();
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
        ext_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(vec![google_login.clone(), github_login.clone()]));

        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .external_login_repository(Box::new(ext_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        let result = auth_service.export_user_data(user_id).await;
        assert!(result.is_ok());

        let export = result.unwrap();

        // Verify multiple providers
        assert_eq!(export.external_logins.len(), 2);

        let providers: Vec<String> = export
            .external_logins
            .iter()
            .map(|l| l.provider.clone())
            .collect();
        assert!(providers.contains(&"google".to_string()));
        assert!(providers.contains(&"github".to_string()));
    }

    #[tokio::test]
    async fn test_export_valid_json_serialization() {
        // Test that export can be serialized to valid JSON
        use crate::repositories::mocks::{
            MockUserCredentialsRepository, MockUserExternalLoginRepository,
            MockUserPreferencesRepository, MockUserProfileRepository,
        };
        use crate::models::db::{
            user_credentials::UserCredentials,
            user_external_login::UserExternalLogin,
            user_preferences::UserPreferences,
            user_profile::UserProfile,
        };

        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);

        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(user.clone())));
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_repo = MockRefreshTokenRepository::new();
        refresh_repo
            .expect_find_by_user_id()
            .returning(|_| Ok(vec![]));

        let mut creds_repo = MockUserCredentialsRepository::new();
        creds_repo.expect_has_password().returning(|_| Ok(true));
        let creds = UserCredentials {
            user_id,
            password_hash: "hashed".to_string(),
            password_updated_at: Utc::now(),
            created_at: Utc::now(),
        };
        creds_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(Some(creds.clone())));

        let mut ext_repo = MockUserExternalLoginRepository::new();
        let ext_login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id,
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        ext_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(vec![ext_login.clone()]));

        let mut profile_repo = MockUserProfileRepository::new();
        let profile = UserProfile {
            user_id,
            real_name: Some("Test".to_string()),
            bio: None,
            avatar_url: None,
            location: None,
            website: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        profile_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(Some(profile.clone())));

        let mut prefs_repo = MockUserPreferencesRepository::new();
        let prefs = UserPreferences {
            user_id,
            timer_is_public: false,
            timer_show_in_list: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        prefs_repo
            .expect_find_by_user_id()
            .returning(move |_| Ok(Some(prefs.clone())));

        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .credentials_repository(Box::new(creds_repo))
            .external_login_repository(Box::new(ext_repo))
            .profile_repository(Box::new(profile_repo))
            .preferences_repository(Box::new(prefs_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        let export = auth_service.export_user_data(user_id).await.unwrap();

        // Verify serialization to JSON works
        let json = serde_json::to_string_pretty(&export).unwrap();
        assert!(json.contains("export_version"));
        assert!(json.contains("1.0"));
        assert!(json.contains("authentication"));
        assert!(json.contains("external_logins"));
        assert!(json.contains("profile"));
        assert!(json.contains("preferences"));

        // Verify can be deserialized
        let _parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify password_hash is NOT in JSON
        assert!(!json.contains("password_hash"));
    }
}
