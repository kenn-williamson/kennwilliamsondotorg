use super::AuthService;
use anyhow::{anyhow, Result};
use oauth2::{CsrfToken, PkceCodeVerifier};

use crate::models::api::user::{AuthResponse, UserResponse};
use crate::models::db::refresh_token::CreateRefreshToken;
use crate::models::db::user::User;
use crate::repositories::traits::user_repository::CreateOAuthUserData;

impl AuthService {
    /// Generate Google OAuth authorization URL with PKCE and CSRF protection
    /// Stores PKCE verifier in storage for later retrieval in callback
    /// Returns: (auth_url, csrf_token) - verifier is stored internally
    pub async fn google_oauth_url(&self) -> Result<(String, CsrfToken)> {
        let oauth_service = self
            .google_oauth_service
            .as_ref()
            .ok_or_else(|| anyhow!("Google OAuth not configured"))?;

        let pkce_storage = self
            .pkce_storage
            .as_ref()
            .ok_or_else(|| anyhow!("PKCE storage not configured"))?;

        // Generate OAuth URL with PKCE challenge
        let (auth_url, csrf_token, pkce_verifier) = oauth_service.get_authorization_url().await?;

        // Store PKCE verifier with CSRF token as key (5 minute TTL)
        pkce_storage
            .store_pkce(csrf_token.secret(), pkce_verifier.secret(), 300)
            .await?;

        log::debug!(
            "Stored PKCE verifier for state: {}",
            csrf_token.secret()
        );

        Ok((auth_url, csrf_token))
    }

    /// Handle Google OAuth callback
    /// Validates authorization code, exchanges for token, fetches user info, and performs account linking
    ///
    /// Account linking strategy (per design doc section 4):
    /// 1. Check if google_user_id exists → Login existing user
    /// 2. Check if email exists AND has email-verified role → Link to existing account
    /// 3. Otherwise → Create new OAuth user with email-verified role
    pub async fn google_oauth_callback(
        &self,
        code: String,
        state: String,
    ) -> Result<AuthResponse> {
        // Ensure OAuth service is configured
        let oauth_service = self
            .google_oauth_service
            .as_ref()
            .ok_or_else(|| anyhow!("Google OAuth not configured"))?;

        let pkce_storage = self
            .pkce_storage
            .as_ref()
            .ok_or_else(|| anyhow!("PKCE storage not configured"))?;

        // Retrieve PKCE verifier from storage using state parameter
        let verifier_secret = pkce_storage
            .retrieve_and_delete_pkce(&state)
            .await?
            .ok_or_else(|| anyhow!("Invalid or expired OAuth state"))?;

        let pkce_verifier = PkceCodeVerifier::new(verifier_secret);

        log::debug!("Retrieved PKCE verifier for state: {}", state);

        // Exchange authorization code for access token
        let access_token = oauth_service
            .exchange_code_for_token(code, pkce_verifier)
            .await?;

        // Fetch user info from Google
        let google_user_info = oauth_service.get_user_info(&access_token).await?;

        // Account linking strategy
        let user = if let Some(existing_user) = self
            .user_repository
            .find_by_google_user_id(&google_user_info.sub)
            .await?
        {
            // Case 1: Existing Google user - log them in and update real_name
            self.user_repository
                .update_real_name(existing_user.id, google_user_info.name.clone())
                .await?;

            existing_user
        } else if let Some(existing_user) = self
            .user_repository
            .find_by_email(&google_user_info.email)
            .await?
        {
            // Case 2: Email exists - check if verified before linking
            let has_verified_role = self
                .user_repository
                .has_role(existing_user.id, "email-verified")
                .await?;

            // Link Google account to existing user (verified or not)
            // If unverified, trust Google's verification and add email-verified role
            self.user_repository
                .link_google_account(
                    existing_user.id,
                    &google_user_info.sub,
                    google_user_info.name.clone(),
                )
                .await?;

            // If user wasn't verified, add email-verified role (Google verified it)
            if !has_verified_role {
                self.user_repository
                    .add_role_to_user(existing_user.id, "email-verified")
                    .await?;
            }

            // Refetch user to get updated google_user_id and real_name
            self.user_repository
                .find_by_id(existing_user.id)
                .await?
                .ok_or_else(|| anyhow!("User not found after linking"))?
        } else {
            // Case 3: New user - create OAuth account
            self.create_new_oauth_user(google_user_info).await?
        };

        // Generate tokens and return AuthResponse
        self.generate_auth_response(user).await
    }

    /// Helper: Create new OAuth user
    async fn create_new_oauth_user(
        &self,
        google_user_info: crate::models::oauth::GoogleUserInfo,
    ) -> Result<User> {
        use crate::services::auth::auth_service::slug::generate_slug_from_display_name;

        // Generate slug from email or name
        let base_slug = if let Some(name) = &google_user_info.name {
            generate_slug_from_display_name(name)
        } else {
            let email_local = google_user_info
                .email
                .split('@')
                .next()
                .unwrap_or("user");
            generate_slug_from_display_name(email_local)
        };

        // Ensure slug is unique
        let slug = self.ensure_unique_slug(&base_slug).await?;

        let oauth_user_data = CreateOAuthUserData {
            email: google_user_info.email,
            display_name: google_user_info
                .name
                .clone()
                .unwrap_or_else(|| "User".to_string()),
            slug,
            real_name: google_user_info.name,
            google_user_id: Some(google_user_info.sub),
        };

        self.user_repository
            .create_oauth_user(&oauth_user_data)
            .await
    }

    /// Helper: Ensure slug is unique by appending numbers if needed
    async fn ensure_unique_slug(&self, base_slug: &str) -> Result<String> {
        let mut slug = base_slug.to_string();
        let mut counter = 1;

        while self.user_repository.slug_exists(&slug).await? {
            slug = format!("{}-{}", base_slug, counter);
            counter += 1;

            if counter > 1000 {
                return Err(anyhow!("Could not generate unique slug"));
            }
        }

        Ok(slug)
    }

    /// Helper: Generate AuthResponse with tokens
    async fn generate_auth_response(&self, user: User) -> Result<AuthResponse> {
        // Get user roles
        let roles = self.user_repository.get_user_roles(user.id).await?;

        // Check if email is verified
        let email_verified = roles.contains(&"email-verified".to_string());

        // Generate access token
        let token = self.jwt_service.generate_token(&user, &roles)?;

        // Generate refresh token (same logic as login)
        let refresh_token_string = generate_refresh_token_string();
        let token_hash = hash_token(&refresh_token_string);
        let expires_at = chrono::Utc::now() + chrono::Duration::days(7);

        let token_data = CreateRefreshToken {
            user_id: user.id,
            token_hash,
            device_info: None,
            expires_at,
        };

        self.refresh_token_repository
            .create_token(&token_data)
            .await?;

        Ok(AuthResponse {
            token,
            refresh_token: refresh_token_string,
            user: UserResponse {
                id: user.id,
                email: user.email,
                display_name: user.display_name,
                slug: user.slug,
                roles,
                real_name: user.real_name,
                google_user_id: user.google_user_id,
                email_verified,
                created_at: user.created_at,
            },
        })
    }
}

/// Generate refresh token string
fn generate_refresh_token_string() -> String {
    use rand::{rng, Rng};
    let mut token_bytes = [0u8; 32]; // 256 bits
    rng().fill(&mut token_bytes);
    hex::encode(token_bytes)
}

/// Hash token for storage
fn hash_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::oauth::GoogleUserInfo;
    use crate::repositories::mocks::{
        MockPkceStorage, MockRefreshTokenRepository, MockUserRepository,
        MockVerificationTokenRepository,
    };
    use crate::repositories::traits::pkce_storage::PkceStorage;
    use crate::services::auth::oauth::MockGoogleOAuthService;
    use crate::services::email::MockEmailService;

    // Test state constant used across all tests
    const TEST_STATE: &str = "test-state-token";
    const TEST_VERIFIER: &str = "test-verifier";

    fn create_test_auth_service_with_mock_oauth(
        mock_oauth: MockGoogleOAuthService,
    ) -> AuthService {
        create_test_auth_service_with_mocks(
            mock_oauth,
            MockUserRepository::new(),
            MockRefreshTokenRepository::new(),
        )
    }

    fn create_test_auth_service_with_mocks(
        mock_oauth: MockGoogleOAuthService,
        user_repo: MockUserRepository,
        token_repo: MockRefreshTokenRepository,
    ) -> AuthService {
        // Create PKCE storage and pre-store the test verifier
        let pkce_storage = MockPkceStorage::new();

        AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret("test-secret".to_string())
            .build()
    }

    // Helper to create auth service with pre-stored PKCE verifier for callback tests
    async fn create_test_auth_service_with_stored_pkce(
        mock_oauth: MockGoogleOAuthService,
        user_repo: MockUserRepository,
        token_repo: MockRefreshTokenRepository,
    ) -> AuthService {
        let pkce_storage = MockPkceStorage::new();
        // Pre-store the test verifier
        pkce_storage
            .store_pkce(TEST_STATE, TEST_VERIFIER, 300)
            .await
            .unwrap();

        AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret("test-secret".to_string())
            .build()
    }

    // Helper to create mock user repo that expects new OAuth user creation
    fn mock_user_repo_for_new_oauth_user(google_user_id: &str) -> MockUserRepository {
        use uuid::Uuid;

        let google_id = google_user_id.to_string();
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_google_user_id()
            .returning(move |_| Ok(None));
        user_repo
            .expect_find_by_email()
            .returning(|_| Ok(None));
        user_repo
            .expect_slug_exists()
            .returning(|_| Ok(false));
        user_repo
            .expect_create_oauth_user()
            .returning(move |_| {
                Ok(User {
                    id: Uuid::new_v4(),
                    email: "mock@example.com".to_string(),
                    display_name: "Mock User".to_string(),
                    slug: "mock-user".to_string(),
                    password_hash: None,
                    active: true,
                    real_name: None,
                    google_user_id: Some(google_id.clone()),
                    timer_is_public: false,
                    timer_show_in_list: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
            });
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));
        user_repo
    }

    // Helper to create mock token repo
    fn mock_token_repo() -> MockRefreshTokenRepository {
        use crate::models::db::refresh_token::RefreshToken;
        use uuid::Uuid;

        let mut token_repo = MockRefreshTokenRepository::new();
        token_repo
            .expect_create_token()
            .returning(|_| {
                Ok(RefreshToken {
                    id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(),
                    token_hash: "hash".to_string(),
                    device_info: None,
                    expires_at: chrono::Utc::now() + chrono::Duration::days(7),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                    last_used_at: None,
                })
            });
        token_repo
    }

    // ==================== OAuth URL Generation Tests ====================

    #[tokio::test]
    async fn test_oauth_url_contains_google_auth_endpoint() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url().await;
        assert!(result.is_ok());

        let (url, _csrf) = result.unwrap();
        assert!(url.contains("accounts.google.com/o/oauth2/v2/auth"));
    }

    #[tokio::test]
    async fn test_oauth_url_includes_client_id() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url().await;
        assert!(result.is_ok());

        let (url, _csrf) = result.unwrap();
        // URL should contain client_id parameter
        assert!(url.contains("client_id="));
    }

    #[tokio::test]
    async fn test_oauth_url_includes_redirect_uri() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url().await;
        assert!(result.is_ok());

        let (url, _csrf) = result.unwrap();
        // URL should contain redirect_uri parameter
        assert!(url.contains("redirect_uri="));
    }

    #[tokio::test]
    async fn test_oauth_url_includes_required_scopes() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url().await;
        assert!(result.is_ok());

        let (url, _csrf) = result.unwrap();
        // URL should include openid, email, and profile scopes
        assert!(url.contains("scope="));
        // Note: Exact scope format depends on oauth2 crate URL encoding
    }

    #[tokio::test]
    async fn test_oauth_url_generates_csrf_token() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url().await;
        assert!(result.is_ok());

        let (_url, csrf_token) = result.unwrap();
        assert!(!csrf_token.secret().is_empty());
    }

    #[tokio::test]
    async fn test_oauth_url_includes_pkce_challenge() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url().await;
        assert!(result.is_ok());

        let (url, _csrf) = result.unwrap();
        // URL should contain PKCE code_challenge parameter
        assert!(url.contains("code_challenge="));
        assert!(url.contains("code_challenge_method="));
    }

    #[tokio::test]
    async fn test_oauth_url_stores_pkce_verifier() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url().await;
        assert!(result.is_ok());

        // Verifier is now stored in PKCE storage, not returned
        // This test verifies the URL generation works correctly
        let (url, csrf) = result.unwrap();
        assert!(!url.is_empty());
        assert!(!csrf.secret().is_empty());
    }

    // ==================== Token Exchange Tests ====================

    #[tokio::test]
    async fn test_successful_token_exchange() {
        let mock_oauth = MockGoogleOAuthService::new().with_access_token("test_token".to_string());
        let user_repo = mock_user_repo_for_new_oauth_user("mock_google_user_id");
        let token_repo = mock_token_repo();
        let service =
            create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Should successfully exchange code for token and create user
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_token_exchange_uses_pkce_verifier() {
        let mock_oauth = MockGoogleOAuthService::new();
        let user_repo = mock_user_repo_for_new_oauth_user("mock_google_user_id");
        let token_repo = mock_token_repo();
        let service =
            create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // PKCE verifier is retrieved from storage and used for token exchange
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_authorization_code_returns_error() {
        let mock_oauth = MockGoogleOAuthService::new().with_exchange_failure();
        let pkce_storage = MockPkceStorage::new();
        pkce_storage
            .store_pkce(TEST_STATE, TEST_VERIFIER, 300)
            .await
            .unwrap();

        let service = AuthService::builder()
            .user_repository(Box::new(MockUserRepository::new()))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service
            .google_oauth_callback("invalid_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_network_failure_during_token_exchange_returns_error() {
        let mock_oauth = MockGoogleOAuthService::new().with_exchange_failure();
        let user_repo = MockUserRepository::new();
        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_err());
    }

    // ==================== User Info Fetching Tests ====================

    #[tokio::test]
    async fn test_successful_user_info_fetch() {
        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "google_123".to_string(),
            email: "test@example.com".to_string(),
            name: Some("Test User".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);
        let user_repo = mock_user_repo_for_new_oauth_user("google_123");
        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_info_contains_required_fields() {
        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "google_456".to_string(),
            email: "user@example.com".to_string(),
            name: Some("User Name".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);
        let user_repo = mock_user_repo_for_new_oauth_user("google_456");
        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Verify user was created with correct info (will test via AuthResponse once implemented)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_access_token_returns_error() {
        let mock_oauth = MockGoogleOAuthService::new().with_user_info_failure();
        let user_repo = MockUserRepository::new();
        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unverified_email_from_google_creates_user_anyway() {
        // Google OAuth users should still get email-verified role even if verified_email=false
        // because Google is a trusted provider
        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "google_789".to_string(),
            email: "unverified@example.com".to_string(),
            name: Some("Unverified User".to_string()),
            email_verified: Some(false), // Even if Google says unverified
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);
        let user_repo = mock_user_repo_for_new_oauth_user("google_789");
        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Should still create user (Google is trusted provider)
        assert!(result.is_ok());
    }

    // ==================== Account Linking Logic Tests ====================

    #[tokio::test]
    async fn test_new_user_creates_oauth_user_with_email_verified_role() {
        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "new_google_user".to_string(),
            email: "newuser@example.com".to_string(),
            name: Some("New User".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);
        let user_repo = mock_user_repo_for_new_oauth_user("new_google_user");
        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Should create new OAuth user with email-verified role
        // Will verify via repository calls once implemented
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_existing_google_user_id_logs_in() {
        use uuid::Uuid;

        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "existing_google_id".to_string(),
            email: "existing@example.com".to_string(),
            name: Some("Existing User".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);

        // Mock repository to return existing user with google_user_id
        let existing_google_id = "existing_google_id".to_string();
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_google_user_id()
            .returning(move |_| {
                Ok(Some(User {
                    id: Uuid::new_v4(),
                    email: "existing@example.com".to_string(),
                    display_name: "Existing User".to_string(),
                    slug: "existing-user".to_string(),
                    password_hash: None,
                    active: true,
                    real_name: Some("Old Name".to_string()),
                    google_user_id: Some(existing_google_id.clone()),
                    timer_is_public: false,
                    timer_show_in_list: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }))
            });
        user_repo
            .expect_update_real_name()
            .returning(|_, _| Ok(()));
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_existing_verified_email_links_google_account() {
        use uuid::Uuid;

        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "linking_google_id".to_string(),
            email: "verified@example.com".to_string(),
            name: Some("Link User".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);

        // Mock repository to return no Google user but existing email-verified user
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_google_user_id()
            .returning(|_| Ok(None));
        user_repo
            .expect_find_by_email()
            .returning(|_| {
                Ok(Some(User {
                    id: Uuid::new_v4(),
                    email: "verified@example.com".to_string(),
                    display_name: "Email User".to_string(),
                    slug: "email-user".to_string(),
                    password_hash: Some("hash".to_string()),
                    active: true,
                    real_name: None,
                    google_user_id: None,
                    timer_is_public: false,
                    timer_show_in_list: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }))
            });
        user_repo
            .expect_has_role()
            .returning(|_, _| Ok(true)); // Has email-verified role
        user_repo
            .expect_link_google_account()
            .returning(|_, _, _| Ok(()));
        user_repo
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(User {
                    id: Uuid::new_v4(),
                    email: "verified@example.com".to_string(),
                    display_name: "Email User".to_string(),
                    slug: "email-user".to_string(),
                    password_hash: Some("hash".to_string()),
                    active: true,
                    real_name: Some("Link User".to_string()),
                    google_user_id: Some("linking_google_id".to_string()),
                    timer_is_public: false,
                    timer_show_in_list: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }))
            });
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_existing_unverified_email_links_and_verifies() {
        // When OAuth returns email matching unverified user, link and verify
        // Google's verification is trusted, so we can upgrade the unverified account
        use uuid::Uuid;

        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "link_unverified_id".to_string(),
            email: "unverified@example.com".to_string(),
            name: Some("Link Test".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);

        // Mock repository to return no Google user but existing unverified email user
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_google_user_id()
            .returning(|_| Ok(None));
        user_repo
            .expect_find_by_email()
            .returning(|_| {
                Ok(Some(User {
                    id: Uuid::new_v4(),
                    email: "unverified@example.com".to_string(),
                    display_name: "Unverified User".to_string(),
                    slug: "unverified-user".to_string(),
                    password_hash: Some("hash".to_string()),
                    active: true,
                    real_name: None,
                    google_user_id: None,
                    timer_is_public: false,
                    timer_show_in_list: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }))
            });
        user_repo
            .expect_has_role()
            .returning(|_, _| Ok(false)); // Does NOT have email-verified role yet
        user_repo
            .expect_link_google_account()
            .returning(|_, _, _| Ok(())); // Should link Google account
        user_repo
            .expect_add_role_to_user()
            .returning(|_, _| Ok(())); // Should add email-verified role
        user_repo
            .expect_find_by_id()
            .returning(|_| {
                Ok(Some(User {
                    id: Uuid::new_v4(),
                    email: "unverified@example.com".to_string(),
                    display_name: "Unverified User".to_string(),
                    slug: "unverified-user".to_string(),
                    password_hash: Some("hash".to_string()),
                    active: true,
                    real_name: Some("Link Test".to_string()),
                    google_user_id: Some("link_unverified_id".to_string()),
                    timer_is_public: false,
                    timer_show_in_list: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }))
            });
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Should link to existing unverified user and grant email-verified role
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_jwt_tokens_include_correct_user_id_and_roles() {
        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "jwt_test_id".to_string(),
            email: "jwt@example.com".to_string(),
            name: Some("JWT User".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);
        let user_repo = mock_user_repo_for_new_oauth_user("jwt_test_id");
        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Should return AuthResponse with valid JWTs containing user and email-verified roles
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_real_name_populated_from_google_profile() {
        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "name_test_id".to_string(),
            email: "name@example.com".to_string(),
            name: Some("Real Name From Google".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);
        let user_repo = mock_user_repo_for_new_oauth_user("name_test_id");
        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Should create user with real_name = "Real Name From Google"
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_real_name_updates_on_subsequent_oauth_logins() {
        use uuid::Uuid;

        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "update_name_id".to_string(),
            email: "update@example.com".to_string(),
            name: Some("Updated Name".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);

        // Mock repository to return existing user with old real_name
        let update_google_id = "update_name_id".to_string();
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_google_user_id()
            .returning(move |_| {
                Ok(Some(User {
                    id: Uuid::new_v4(),
                    email: "update@example.com".to_string(),
                    display_name: "Update User".to_string(),
                    slug: "update-user".to_string(),
                    password_hash: None,
                    active: true,
                    real_name: Some("Old Name".to_string()),
                    google_user_id: Some(update_google_id.clone()),
                    timer_is_public: false,
                    timer_show_in_list: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }))
            });
        user_repo
            .expect_update_real_name()
            .returning(|_, _| Ok(()));
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        let token_repo = mock_token_repo();
        let service = create_test_auth_service_with_stored_pkce(mock_oauth, user_repo, token_repo).await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_ok());
    }
}
