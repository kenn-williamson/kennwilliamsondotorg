use anyhow::Result;
use rand::Rng;
use uuid::Uuid;

use super::slug::{generate_slug, is_valid_slug};
use super::AuthService;
use crate::events::types::ProfileUpdatedEvent;
use crate::models::api::{
    ProfileUpdateRequest, SlugPreviewRequest, SlugPreviewResponse, SlugValidationRequest,
    SlugValidationResponse, UserResponse,
};
use crate::repositories::traits::user_repository::UserUpdates;

impl AuthService {
    /// Get current user information
    pub async fn get_current_user(&self, user_id: Uuid) -> Result<Option<UserResponse>> {
        let user = self.user_repository.find_by_id(user_id).await?;
        match user {
            Some(user) => {
                let roles = self.user_repository.get_user_roles(user.id).await?;
                let user_response = self.build_user_response_with_details(user, roles).await?;
                Ok(Some(user_response))
            }
            None => Ok(None),
        }
    }

    /// Preview slug availability (for registration - generates slug from display name)
    pub async fn preview_slug(&self, request: SlugPreviewRequest) -> Result<SlugPreviewResponse> {
        let slug = generate_slug(&request.display_name, &*self.user_repository).await?;
        let available = !self.user_repository.slug_exists(&slug).await?;

        Ok(SlugPreviewResponse {
            slug: slug.clone(),
            available,
            final_slug: if available {
                slug
            } else {
                format!("{}-{}", slug, rand::rng().random_range(1..=999))
            },
        })
    }

    /// Validate slug format and availability (for profile updates)
    pub async fn validate_slug(
        &self,
        request: SlugValidationRequest,
    ) -> Result<SlugValidationResponse> {
        let slug = request.slug;

        // Check if slug format is valid
        let valid = is_valid_slug(&slug);

        if !valid {
            return Ok(SlugValidationResponse {
                slug: slug.clone(),
                valid: false,
                available: false,
            });
        }

        // Check availability
        let available = !self.user_repository.slug_exists(&slug).await?;

        Ok(SlugValidationResponse {
            slug: slug.clone(),
            valid: true,
            available,
        })
    }

    /// Update user profile
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        request: ProfileUpdateRequest,
    ) -> Result<UserResponse> {
        // Fetch current user to capture old values for event notification
        let old_user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Validate slug format - only check for excluded characters
        if !is_valid_slug(&request.slug) {
            return Err(anyhow::anyhow!("Invalid slug format"));
        }

        // Check if slug is available (excluding current user)
        if self
            .user_repository
            .slug_exists_excluding_user(&request.slug, user_id)
            .await?
        {
            return Err(anyhow::anyhow!("Slug already taken"));
        }

        // Update user profile
        let updates = UserUpdates {
            display_name: request.display_name.clone(),
            slug: request.slug.clone(),
        };

        let user = self.user_repository.update_user(user_id, &updates).await?;
        let roles = self.user_repository.get_user_roles(user.id).await?;

        // Publish ProfileUpdatedEvent if event publisher is configured
        if let Some(event_publisher) = &self.event_publisher {
            let event = ProfileUpdatedEvent::new(
                user_id,
                old_user.display_name,
                request.display_name,
                old_user.slug,
                request.slug,
            );
            if let Err(e) = event_publisher.publish(Box::new(event)).await {
                log::error!("Failed to publish ProfileUpdatedEvent: {}", e);
                // Don't fail the operation if event publishing fails
            }
        }

        let user_response = self.build_user_response_with_details(user, roles).await?;
        Ok(user_response)
    }

    /// Update timer privacy settings
    pub async fn update_timer_privacy(
        &self,
        user_id: Uuid,
        is_public: bool,
        show_in_list: bool,
    ) -> Result<UserResponse> {
        // Validate business rule: show_in_list requires is_public
        if show_in_list && !is_public {
            return Err(anyhow::anyhow!(
                "Cannot enable 'Show in List' when timer is not public"
            ));
        }

        // Update preferences in user_preferences table
        if let Some(prefs_repo) = &self.preferences_repository {
            prefs_repo
                .update_timer_settings(user_id, is_public, show_in_list)
                .await?;
        }

        // Get updated user and build response
        let user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;
        let roles = self.user_repository.get_user_roles(user.id).await?;
        let user_response = self.build_user_response_with_details(user, roles).await?;

        Ok(user_response)
    }

    /// Get list of users with public timers
    pub async fn get_users_with_public_timers(
        &self,
        limit: i64,
        offset: i64,
        search: Option<String>,
    ) -> Result<Vec<crate::models::db::user::UserWithTimer>> {
        self.user_repository
            .get_users_with_public_timers(limit, offset, search)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::mock_refresh_token_repository::MockRefreshTokenRepository;
    use crate::repositories::mocks::mock_user_preferences_repository::MockUserPreferencesRepository;
    use crate::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::models::db::User;
    use anyhow::Result;
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    fn create_test_user(user_id: Uuid) -> User {
        User {
            id: user_id,
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn get_current_user_successful() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let user = create_test_user(user_id);

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(vec!["user".to_string()]));

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.get_current_user(user_id).await?;

        assert!(result.is_some());
        let user_response = result.unwrap();
        assert_eq!(user_response.email, "test@example.com");
        assert_eq!(user_response.display_name, "Test User");
        assert_eq!(user_response.roles, vec!["user"]);

        Ok(())
    }

    #[tokio::test]
    async fn get_current_user_when_user_not_found() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(None));

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.get_current_user(user_id).await?;
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn get_current_user_handles_database_error() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.get_current_user(user_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));

        Ok(())
    }

    #[tokio::test]
    async fn preview_slug_with_available_slug() -> Result<()> {
        let mut user_repo = MockUserRepository::new();

        // Setup mock expectations - generate_slug calls slug_exists multiple times
        user_repo
            .expect_slug_exists()
            .times(1)
            .with(eq("john-doe"))
            .returning(|_| Ok(false)); // Slug is available

        // Second call for the preview_slug function itself
        user_repo
            .expect_slug_exists()
            .times(1)
            .with(eq("john-doe"))
            .returning(|_| Ok(false)); // Slug is available

        let request = SlugPreviewRequest {
            display_name: "John Doe".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.preview_slug(request).await?;

        assert_eq!(result.slug, "john-doe");
        assert!(result.available);
        assert_eq!(result.final_slug, "john-doe");

        Ok(())
    }

    #[tokio::test]
    async fn preview_slug_with_unavailable_slug() -> Result<()> {
        let mut user_repo = MockUserRepository::new();

        // Setup mock expectations - generate_slug calls slug_exists in a loop
        user_repo
            .expect_slug_exists()
            .times(1)
            .with(eq("john-doe"))
            .returning(|_| Ok(true)); // First slug is not available

        user_repo
            .expect_slug_exists()
            .times(1)
            .with(eq("john-doe-1"))
            .returning(|_| Ok(false)); // Fallback slug is available

        // Second call for the preview_slug function itself
        user_repo
            .expect_slug_exists()
            .times(1)
            .with(eq("john-doe-1"))
            .returning(|_| Ok(false)); // Final slug is available

        let request = SlugPreviewRequest {
            display_name: "John Doe".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.preview_slug(request).await?;

        assert_eq!(result.slug, "john-doe-1");
        assert!(result.available);
        assert_eq!(result.final_slug, "john-doe-1");

        Ok(())
    }

    #[tokio::test]
    async fn preview_slug_handles_database_error() -> Result<()> {
        let mut user_repo = MockUserRepository::new();

        // Setup mock expectations
        user_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let request = SlugPreviewRequest {
            display_name: "John Doe".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.preview_slug(request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));

        Ok(())
    }

    #[tokio::test]
    async fn update_profile_successful() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let updated_user = create_test_user(user_id);
        let old_user = create_test_user(user_id);

        // Setup mock expectations - now includes find_by_id to get old values
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(old_user.clone())));

        user_repo
            .expect_slug_exists_excluding_user()
            .times(1)
            .with(eq("new-slug"), eq(user_id))
            .returning(|_, _| Ok(false)); // Slug is available for this user

        user_repo
            .expect_update_user()
            .times(1)
            .with(
                eq(user_id),
                mockall::predicate::function(|updates: &UserUpdates| {
                    updates.display_name == "New Name"
                        && updates.slug == "new-slug"
                }),
            )
            .returning(move |_, _| Ok(updated_user.clone()));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let request = ProfileUpdateRequest {
            display_name: "New Name".to_string(),
            slug: "new-slug".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.update_profile(user_id, request).await?;

        assert_eq!(result.display_name, "Test User"); // From updated_user
        assert_eq!(result.slug, "test-user"); // From updated_user
        assert_eq!(result.roles, vec!["user"]);

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn update_profile_fails_with_invalid_slug_format() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let old_user = create_test_user(user_id);

        // Should fetch old user first
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(old_user.clone())));

        let request = ProfileUpdateRequest {
            display_name: "New Name".to_string(),
            slug: "invalid!!!slug".to_string(), // Contains invalid characters
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.update_profile(user_id, request).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid slug format"));

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn update_profile_fails_with_underscore_in_slug() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let old_user = create_test_user(user_id);

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(old_user.clone())));

        let request = ProfileUpdateRequest {
            display_name: "New Name".to_string(),
            slug: "invalid_slug".to_string(), // Contains underscore (discouraged)
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.update_profile(user_id, request).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid slug format"));

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn update_profile_fails_with_url_encoded_characters() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let old_user = create_test_user(user_id);

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(old_user.clone())));

        let request = ProfileUpdateRequest {
            display_name: "New Name".to_string(),
            slug: "invalid%20slug".to_string(), // Contains URL-encoded characters
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.update_profile(user_id, request).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid slug format"));

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn update_profile_fails_with_leading_hyphen() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let old_user = create_test_user(user_id);

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(old_user.clone())));

        let request = ProfileUpdateRequest {
            display_name: "New Name".to_string(),
            slug: "-invalid-slug".to_string(), // Starts with hyphen
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.update_profile(user_id, request).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid slug format"));

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn update_profile_fails_with_trailing_hyphen() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let old_user = create_test_user(user_id);

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(old_user.clone())));

        let request = ProfileUpdateRequest {
            display_name: "New Name".to_string(),
            slug: "invalid-slug-".to_string(), // Ends with hyphen
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.update_profile(user_id, request).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid slug format"));

        Ok(())
    }

    #[tokio::test]
    async fn update_profile_fails_when_slug_taken() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let old_user = create_test_user(user_id);

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(old_user.clone())));

        user_repo
            .expect_slug_exists_excluding_user()
            .times(1)
            .with(eq("taken-slug"), eq(user_id))
            .returning(|_, _| Ok(true)); // Slug is taken by another user

        let request = ProfileUpdateRequest {
            display_name: "New Name".to_string(),
            slug: "taken-slug".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.update_profile(user_id, request).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Slug already taken"));

        Ok(())
    }

    #[tokio::test]
    async fn update_profile_handles_database_error_during_slug_check() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let old_user = create_test_user(user_id);

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(old_user.clone())));

        user_repo
            .expect_slug_exists_excluding_user()
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Database error")));

        let request = ProfileUpdateRequest {
            display_name: "New Name".to_string(),
            slug: "new-slug".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.update_profile(user_id, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));

        Ok(())
    }

    #[tokio::test]
    async fn update_profile_handles_database_error_during_update() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();
        let old_user = create_test_user(user_id);

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(old_user.clone())));

        user_repo
            .expect_slug_exists_excluding_user()
            .times(1)
            .with(eq("new-slug"), eq(user_id))
            .returning(|_, _| Ok(false));

        user_repo
            .expect_update_user()
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Database error")));

        let request = ProfileUpdateRequest {
            display_name: "New Name".to_string(),
            slug: "new-slug".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.update_profile(user_id, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));

        Ok(())
    }

    // ========================================
    // Phase 4D: New Multi-Table Profile Tests
    // ========================================

    #[tokio::test]
    async fn test_update_timer_privacy_successful() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut prefs_repo = MockUserPreferencesRepository::new();
        let user_id = Uuid::new_v4();
        let user = create_test_user(user_id);

        prefs_repo
            .expect_update_timer_settings()
            .times(1)
            .with(eq(user_id), eq(true), eq(true))
            .returning(|_, _, _| Ok(()));

        // build_user_response_with_details will call find_by_user_id
        prefs_repo
            .expect_find_by_user_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(None)); // No preferences yet

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(vec!["user".to_string()]));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .preferences_repository(Box::new(prefs_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.update_timer_privacy(user_id, true, true).await;
        assert!(result.is_ok());
        let user_response = result.unwrap();
        assert_eq!(user_response.id, user_id);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_timer_privacy_validates_show_in_list_requires_public() -> Result<()> {
        let user_repo = MockUserRepository::new();
        let prefs_repo = MockUserPreferencesRepository::new();
        let user_id = Uuid::new_v4();

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .preferences_repository(Box::new(prefs_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        // Try to set show_in_list=true but is_public=false (should fail)
        let result = auth_service.update_timer_privacy(user_id, false, true).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not public"));

        Ok(())
    }

    #[tokio::test]
    async fn test_update_timer_privacy_allows_both_false() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut prefs_repo = MockUserPreferencesRepository::new();
        let user_id = Uuid::new_v4();
        let user = create_test_user(user_id);

        prefs_repo
            .expect_update_timer_settings()
            .times(1)
            .with(eq(user_id), eq(false), eq(false))
            .returning(|_, _, _| Ok(()));

        // build_user_response_with_details will call find_by_user_id
        prefs_repo
            .expect_find_by_user_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(None)); // No preferences yet

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(vec!["user".to_string()]));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .preferences_repository(Box::new(prefs_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.update_timer_privacy(user_id, false, false).await;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_update_timer_privacy_allows_public_without_list() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut prefs_repo = MockUserPreferencesRepository::new();
        let user_id = Uuid::new_v4();
        let user = create_test_user(user_id);

        prefs_repo
            .expect_update_timer_settings()
            .times(1)
            .with(eq(user_id), eq(true), eq(false))
            .returning(|_, _, _| Ok(()));

        // build_user_response_with_details will call find_by_user_id
        prefs_repo
            .expect_find_by_user_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(None)); // No preferences yet

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(vec!["user".to_string()]));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .preferences_repository(Box::new(prefs_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.update_timer_privacy(user_id, true, false).await;
        assert!(result.is_ok());

        Ok(())
    }
}
