use anyhow::Result;
use rand::Rng;
use uuid::Uuid;

use super::slug::{generate_slug, is_valid_slug};
use super::AuthService;
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
                Ok(Some(UserResponse::from_user_with_roles(user, roles)))
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
            display_name: request.display_name,
            slug: request.slug,
        };

        let user = self.user_repository.update_user(user_id, &updates).await?;
        let roles = self.user_repository.get_user_roles(user.id).await?;

        Ok(UserResponse::from_user_with_roles(user, roles))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::mock_refresh_token_repository::MockRefreshTokenRepository;
    use crate::repositories::mocks::mock_user_repository::MockUserRepository;
    use anyhow::Result;
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    fn create_test_user() -> crate::models::db::User {
        crate::models::db::User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: Some("hashed".to_string()),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            real_name: None,
            google_user_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn get_current_user_successful() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user = create_test_user();
        let user_id = user.id;

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
        let updated_user = create_test_user();

        // Setup mock expectations - only need slug_exists_excluding_user now
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
                    updates.display_name == "New Name".to_string()
                        && updates.slug == "new-slug".to_string()
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

        // Setup mock expectations - only need slug_exists_excluding_user now
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

        // Setup mock expectations - only need slug_exists_excluding_user now
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

        // Setup mock expectations - only need slug_exists_excluding_user now
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
}
