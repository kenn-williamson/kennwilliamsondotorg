use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::models::api::data_export::{
    IncidentTimerExportData, PhraseExclusionExportData, PhraseSuggestionExportData,
    SessionExportData, UserDataExport, UserExportData, VerificationTokenExportData,
};

// Add the export_user_data method to AuthService
use super::AuthService;

impl AuthService {
    /// Export all user data in JSON format for GDPR/CCPA compliance
    pub async fn export_user_data(&self, user_id: Uuid) -> Result<UserDataExport> {
        // Get user data
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
            email: user.email,
            display_name: user.display_name,
            slug: user.slug,
            real_name: user.real_name,
            google_user_id: user.google_user_id,
            active: user.active,
            email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
            roles,
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

        Ok(UserDataExport {
            export_date: Utc::now(),
            export_version: "1.0".to_string(),
            user: user_export,
            incident_timers,
            phrase_suggestions,
            phrase_exclusions,
            active_sessions,
            verification_history,
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
            password_hash: Some("hashed_password".to_string()),
            display_name: "Test User".to_string(),
            slug: "testuser".to_string(),
            real_name: Some("Test User Real Name".to_string()),
            google_user_id: Some("google_123".to_string()),
            active: true,
            timer_is_public: false,
            timer_show_in_list: false,
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
        assert_eq!(export_data.user.real_name, Some("Test User Real Name".to_string()));
        assert_eq!(export_data.user.google_user_id, Some("google_123".to_string()));
        assert!(export_data.user.active);
        assert!(export_data.user.email_verified);
        assert_eq!(export_data.user.roles, vec!["user", "email-verified"]);
        
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
        assert_eq!(export_data.user.real_name, Some("Test User Real Name".to_string()));
        assert_eq!(export_data.user.google_user_id, Some("google_123".to_string()));
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
        let mut user = create_test_user_with_id(user_id);
        user.password_hash = None; // OAuth-only user
        
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
        assert_eq!(export_data.user.google_user_id, Some("google_123".to_string()));
        assert!(export_data.user.email_verified); // Has email-verified role
        assert_eq!(export_data.user.roles, vec!["user", "email-verified"]);
        
        // Verify no password_hash in export (implicitly tested by struct)
    }
}
