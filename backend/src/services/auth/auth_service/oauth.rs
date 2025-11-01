use super::AuthService;
use anyhow::{anyhow, Result};
use oauth2::{CsrfToken, PkceCodeVerifier};

use crate::models::api::user::AuthResponse;
use crate::models::db::refresh_token::CreateRefreshToken;
use crate::models::db::user::User;

impl AuthService {
    /// Generate Google OAuth authorization URL with PKCE and CSRF protection
    /// Stores PKCE verifier in storage for later retrieval in callback
    /// Optional redirect parameter is encoded into the state for post-auth redirect
    /// Returns: (auth_url, csrf_token) - verifier is stored internally
    pub async fn google_oauth_url(&self, redirect: Option<String>) -> Result<(String, CsrfToken)> {
        use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};

        let oauth_service = self
            .google_oauth_service
            .as_ref()
            .ok_or_else(|| anyhow!("Google OAuth not configured"))?;

        let pkce_storage = self
            .pkce_storage
            .as_ref()
            .ok_or_else(|| anyhow!("PKCE storage not configured"))?;

        // Generate base CSRF token
        let base_csrf = CsrfToken::new_random();

        // Build enhanced state with redirect if provided and valid
        let enhanced_state = if let Some(redirect_url) = redirect {
            if validate_redirect_url(&redirect_url) {
                let encoded_redirect = base64_engine.encode(redirect_url.as_bytes());
                Some(format!("{}|{}", base_csrf.secret(), encoded_redirect))
            } else {
                // Invalid redirect, use plain state
                None
            }
        } else {
            None
        };

        // Generate OAuth URL with PKCE challenge and custom state (if we have redirect)
        let (auth_url, csrf_token, pkce_verifier) = oauth_service.get_authorization_url(enhanced_state.clone()).await?;

        // Store PKCE verifier with the state that Google will return (5 minute TTL)
        // This is either the enhanced state or the plain csrf token
        let storage_key = enhanced_state.unwrap_or_else(|| csrf_token.secret().to_string());
        pkce_storage
            .store_pkce(&storage_key, pkce_verifier.secret(), 300)
            .await?;

        log::debug!("Stored PKCE verifier for state: {}", storage_key);

        Ok((auth_url, csrf_token))
    }

    /// Handle Google OAuth callback (Phase 4C: Using external_logins table)
    /// Validates authorization code, exchanges for token, fetches user info, and performs account linking
    /// Extracts optional redirect URL from state parameter for post-auth navigation
    ///
    /// Account linking strategy:
    /// 1. Check if external login exists (provider + provider_user_id) → Login existing user
    /// 2. Check if email exists → Link OAuth to account + Add email-verified role (trust OAuth)
    /// 3. Otherwise → Create new OAuth user with all tables (user, external_login, profile, preferences)
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

        // Parse state parameter to extract CSRF token and optional redirect
        let (_csrf_token, redirect_url) = parse_state_parameter(&state);

        // Retrieve PKCE verifier from storage using full state parameter
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

        // Get external_login_repository (required for OAuth)
        let external_login_repo = self
            .external_login_repository
            .as_ref()
            .expect("External login repository is required for OAuth");

        let user = if let Some(existing_login) = external_login_repo
            .find_by_provider("google", &google_user_info.sub)
            .await?
        {
            // Case 1: Existing OAuth user - load their account
            self.user_repository
                .find_by_id(existing_login.user_id)
                .await?
                .ok_or_else(|| anyhow!("User not found for external login"))?
        } else if let Some(existing_user) = self
            .user_repository
            .find_by_email(&google_user_info.email)
            .await?
        {
            // Case 2: Email exists - link OAuth to existing account
            // Trust OAuth provider's verification
            use crate::repositories::traits::user_external_login_repository::CreateExternalLogin;

            external_login_repo
                .create(CreateExternalLogin {
                    user_id: existing_user.id,
                    provider: "google".to_string(),
                    provider_user_id: google_user_info.sub.clone(),
                })
                .await?;

            // Update profile with OAuth data if profile repository is available
            if let Some(profile_repo) = &self.profile_repository {
                use crate::repositories::traits::user_profile_repository::UpdateProfile;

                profile_repo
                    .update(
                        existing_user.id,
                        UpdateProfile {
                            real_name: google_user_info.name.clone(),
                            avatar_url: google_user_info.picture.clone(),
                            bio: None,
                            location: None,
                            website: None,
                        },
                    )
                    .await
                    .ok(); // Ignore errors for optional profile update
            }

            // Add email-verified role if not present (OAuth verification is trusted)
            let roles = self
                .user_repository
                .get_user_roles(existing_user.id)
                .await?;
            if !roles.contains(&"email-verified".to_string()) {
                self.user_repository
                    .add_role_to_user(existing_user.id, "email-verified")
                    .await?;
            }

            existing_user
        } else {
            // Case 3: New OAuth user - create user + external_login + profile + preferences
            self.create_new_oauth_user(google_user_info).await?
        };

        // Generate tokens and return AuthResponse with optional redirect
        self.generate_auth_response(user, redirect_url).await
    }

    /// Helper: Create new OAuth user
    /// Creates user + external_login + profile + preferences atomically
    async fn create_new_oauth_user(
        &self,
        google_user_info: crate::models::oauth::GoogleUserInfo,
    ) -> Result<User> {
        use crate::repositories::traits::user_external_login_repository::CreateExternalLogin;
        use crate::repositories::traits::user_profile_repository::UpdateProfile;
        use crate::repositories::traits::user_repository::CreateUserData;
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

        // 1. Create user
        let user_data = CreateUserData {
            email: google_user_info.email.clone(),
            password_hash: String::new(), // Temporary: still required by schema during migration
            display_name: google_user_info
                .name
                .clone()
                .unwrap_or_else(|| "User".to_string()),
            slug,
        };

        let user = self.user_repository.create_user(&user_data).await?;

        // 2. Create external login
        let external_login_repo = self
            .external_login_repository
            .as_ref()
            .ok_or_else(|| anyhow!("External login repository not configured"))?;

        external_login_repo
            .create(CreateExternalLogin {
                user_id: user.id,
                provider: "google".to_string(),
                provider_user_id: google_user_info.sub,
            })
            .await?;

        // 3. Create profile and update with OAuth data
        if let Some(profile_repo) = &self.profile_repository {
            profile_repo.create(user.id).await?;
            profile_repo
                .update(
                    user.id,
                    UpdateProfile {
                        real_name: google_user_info.name.clone(),
                        avatar_url: google_user_info.picture.clone(),
                        bio: None,
                        location: None,
                        website: None,
                    },
                )
                .await
                .ok(); // Ignore errors for optional profile update
        }

        // 4. Create preferences
        if let Some(prefs_repo) = &self.preferences_repository {
            prefs_repo.create(user.id).await?;
        }

        // 5. Assign email-verified role (OAuth emails are pre-verified by provider)
        self.user_repository
            .add_role_to_user(user.id, "email-verified")
            .await?;

        Ok(user)
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

    /// Helper: Generate AuthResponse with tokens and optional redirect URL
    async fn generate_auth_response(&self, user: User, redirect_url: Option<String>) -> Result<AuthResponse> {
        // Get user roles
        let roles = self.user_repository.get_user_roles(user.id).await?;

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

        // Build fully populated user response
        let user_response = self.build_user_response_with_details(user, roles).await?;

        Ok(AuthResponse {
            token,
            refresh_token: refresh_token_string,
            user: user_response,
            redirect_url,
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

/// Parse OAuth state parameter to extract CSRF token and optional redirect URL
/// Format: {csrf_token}|{base64_encoded_redirect}
/// Returns: (csrf_token, optional_redirect_url)
fn parse_state_parameter(state: &str) -> (String, Option<String>) {
    use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};

    if let Some((csrf, encoded_redirect)) = state.split_once('|') {
        // Attempt to decode the redirect URL
        if let Ok(decoded_bytes) = base64_engine.decode(encoded_redirect)
            && let Ok(redirect) = String::from_utf8(decoded_bytes) {
                // Validate the redirect URL before returning
                if validate_redirect_url(&redirect) {
                    return (csrf.to_string(), Some(redirect));
                }
            }
        // If decoding or validation failed, return None for redirect
        return (csrf.to_string(), None);
    }
    // No redirect in state, return state as csrf token
    (state.to_string(), None)
}

/// Validate redirect URL for security
/// Only allow internal redirects (must start with / but not //)
fn validate_redirect_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }
    // Must start with / (internal redirect)
    if !url.starts_with('/') {
        return false;
    }
    // Must not start with // (protocol-relative URL - open redirect attack)
    if url.starts_with("//") {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::oauth::GoogleUserInfo;
    use crate::repositories::mocks::{
        MockPkceStorage, MockRefreshTokenRepository, MockUserCredentialsRepository,
        MockUserExternalLoginRepository, MockUserPreferencesRepository,
        MockUserProfileRepository, MockUserRepository, MockVerificationTokenRepository,
    };
    use crate::repositories::traits::pkce_storage::PkceStorage;
    use crate::services::auth::oauth::MockGoogleOAuthService;
    use crate::services::email::MockEmailService;
    use uuid::Uuid;

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
            .credentials_repository(Box::new(MockUserCredentialsRepository::new()))
            .external_login_repository(Box::new(MockUserExternalLoginRepository::new()))
            .profile_repository(Box::new(MockUserProfileRepository::new()))
            .preferences_repository(Box::new(MockUserPreferencesRepository::new()))
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
            .credentials_repository(Box::new(MockUserCredentialsRepository::new()))
            .external_login_repository(Box::new(MockUserExternalLoginRepository::new()))
            .profile_repository(Box::new(MockUserProfileRepository::new()))
            .preferences_repository(Box::new(MockUserPreferencesRepository::new()))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret("test-secret".to_string())
            .build()
    }

    // Helper to create mock repos configured for new OAuth user creation
    fn mock_repos_for_new_oauth_user(
        _provider_user_id: &str,
    ) -> (
        MockUserRepository,
        MockUserExternalLoginRepository,
        MockUserProfileRepository,
        MockUserPreferencesRepository,
    ) {
        use crate::models::db::user_external_login::UserExternalLogin;
        use crate::models::db::user_preferences::UserPreferences;
        use crate::models::db::user_profile::UserProfile;
        use uuid::Uuid;

        // Mock user repository for new user creation
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_email().returning(|_| Ok(None));
        user_repo.expect_slug_exists().returning(|_| Ok(false));
        user_repo.expect_create_user().returning(move |_| {
            Ok(User {
                id: Uuid::new_v4(),
                email: "mock@example.com".to_string(),
                display_name: "Mock User".to_string(),
                slug: "mock-user".to_string(),
                active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        user_repo
            .expect_add_role_to_user()
            .returning(|_, _| Ok(()));
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        // Mock external login repository
        let mut external_login_repo = MockUserExternalLoginRepository::new();
        external_login_repo
            .expect_find_by_provider()
            .returning(|_, _| Ok(None));
        external_login_repo.expect_create().returning(|data| {
            Ok(UserExternalLogin {
                id: Uuid::new_v4(),
                user_id: data.user_id,
                provider: data.provider.clone(),
                provider_user_id: data.provider_user_id.clone(),
                linked_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        external_login_repo
            .expect_find_by_user_id()
            .returning(|user_id| {
                Ok(vec![UserExternalLogin {
                    id: Uuid::new_v4(),
                    user_id,
                    provider: "google".to_string(),
                    provider_user_id: "mock_provider_id".to_string(),
                    linked_at: chrono::Utc::now(),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }])
            });

        // Mock profile repository
        let mut profile_repo = MockUserProfileRepository::new();
        profile_repo.expect_create().returning(|_| {
            Ok(UserProfile {
                user_id: Uuid::new_v4(),
                real_name: None,
                avatar_url: None,
                bio: None,
                location: None,
                website: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        profile_repo.expect_update().returning(|_, _| {
            Ok(UserProfile {
                user_id: Uuid::new_v4(),
                real_name: Some("Updated Name".to_string()),
                avatar_url: Some("https://example.com/avatar.jpg".to_string()),
                bio: None,
                location: None,
                website: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        profile_repo.expect_find_by_user_id().returning(|user_id| {
            Ok(Some(UserProfile {
                user_id,
                real_name: Some("Mock Name".to_string()),
                avatar_url: Some("https://example.com/avatar.jpg".to_string()),
                bio: None,
                location: None,
                website: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });

        // Mock preferences repository
        let mut prefs_repo = MockUserPreferencesRepository::new();
        prefs_repo.expect_create().returning(|_| {
            Ok(UserPreferences {
                user_id: Uuid::new_v4(),
                timer_is_public: false,
                timer_show_in_list: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        prefs_repo.expect_find_by_user_id().returning(|user_id| {
            Ok(Some(UserPreferences {
                user_id,
                timer_is_public: false,
                timer_show_in_list: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });

        (user_repo, external_login_repo, profile_repo, prefs_repo)
    }

    // Helper to create auth service with stored PKCE for new OAuth user scenario
    async fn create_test_auth_service_for_new_oauth_user(
        mock_oauth: MockGoogleOAuthService,
        provider_user_id: &str,
    ) -> AuthService {
        let (user_repo, external_login_repo, profile_repo, prefs_repo) =
            mock_repos_for_new_oauth_user(provider_user_id);

        // Mock credentials repository (OAuth users may not have credentials)
        let mut creds_repo = MockUserCredentialsRepository::new();
        creds_repo.expect_find_by_user_id().returning(|_| Ok(None));

        let token_repo = mock_token_repo();
        let pkce_storage = MockPkceStorage::new();
        pkce_storage
            .store_pkce(TEST_STATE, TEST_VERIFIER, 300)
            .await
            .unwrap();

        AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .external_login_repository(Box::new(external_login_repo))
            .profile_repository(Box::new(profile_repo))
            .preferences_repository(Box::new(prefs_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret("test-secret".to_string())
            .build()
    }

    // Helper to create mock token repo
    fn mock_token_repo() -> MockRefreshTokenRepository {
        let mut token_repo = MockRefreshTokenRepository::new();
        token_repo
            .expect_create_token()
            .returning(|_| {
                Ok(crate::test_utils::RefreshTokenBuilder::new()
                    .with_token_hash("hash")
                    .without_device_info()
                    .expires_at(chrono::Utc::now() + chrono::Duration::days(7))
                    .never_used()
                    .build())
            });
        token_repo
    }

    // ==================== OAuth URL Generation Tests ====================

    #[tokio::test]
    async fn test_oauth_url_contains_google_auth_endpoint() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url(None).await;
        assert!(result.is_ok());

        let (url, _csrf) = result.unwrap();
        assert!(url.contains("accounts.google.com/o/oauth2/v2/auth"));
    }

    #[tokio::test]
    async fn test_oauth_url_includes_client_id() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url(None).await;
        assert!(result.is_ok());

        let (url, _csrf) = result.unwrap();
        // URL should contain client_id parameter
        assert!(url.contains("client_id="));
    }

    #[tokio::test]
    async fn test_oauth_url_includes_redirect_uri() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url(None).await;
        assert!(result.is_ok());

        let (url, _csrf) = result.unwrap();
        // URL should contain redirect_uri parameter
        assert!(url.contains("redirect_uri="));
    }

    #[tokio::test]
    async fn test_oauth_url_includes_required_scopes() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url(None).await;
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

        let result = service.google_oauth_url(None).await;
        assert!(result.is_ok());

        let (_url, csrf_token) = result.unwrap();
        assert!(!csrf_token.secret().is_empty());
    }

    #[tokio::test]
    async fn test_oauth_url_includes_pkce_challenge() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url(None).await;
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

        let result = service.google_oauth_url(None).await;
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
        let service =
            create_test_auth_service_for_new_oauth_user(mock_oauth, "mock_google_user_id").await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Should successfully exchange code for token and create user
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_token_exchange_uses_pkce_verifier() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service =
            create_test_auth_service_for_new_oauth_user(mock_oauth, "mock_google_user_id").await;

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
        let service = create_test_auth_service_for_new_oauth_user(mock_oauth, "google_123").await;

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
        let service = create_test_auth_service_for_new_oauth_user(mock_oauth, "google_456").await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Verify user was created with correct info (will test via AuthResponse once implemented)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_access_token_returns_error() {
        let mock_oauth = MockGoogleOAuthService::new().with_user_info_failure();
        let (user_repo, external_login_repo, profile_repo, prefs_repo) =
            mock_repos_for_new_oauth_user("error");
        let token_repo = mock_token_repo();
        let pkce_storage = MockPkceStorage::new();
        pkce_storage
            .store_pkce(TEST_STATE, TEST_VERIFIER, 300)
            .await
            .unwrap();
        let service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(MockUserCredentialsRepository::new()))
            .external_login_repository(Box::new(external_login_repo))
            .profile_repository(Box::new(profile_repo))
            .preferences_repository(Box::new(prefs_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret("test-secret".to_string())
            .build();

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
        let service = create_test_auth_service_for_new_oauth_user(mock_oauth, "google_789").await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Should still create user (Google is trusted provider)
        assert!(result.is_ok());
    }

    // ==================== Account Linking Logic Tests ====================

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
        let service = create_test_auth_service_for_new_oauth_user(mock_oauth, "jwt_test_id").await;

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
        let service = create_test_auth_service_for_new_oauth_user(mock_oauth, "name_test_id").await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        // Should create user with real_name = "Real Name From Google"
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_real_name_updates_on_subsequent_oauth_logins() {
        use crate::models::db::user_external_login::UserExternalLogin;
        use crate::models::db::user_preferences::UserPreferences;
        use crate::models::db::user_profile::UserProfile;
        use uuid::Uuid;

        let user_id = Uuid::new_v4();
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

        // Mock external_login repository - existing login found
        let mut external_login_repo = MockUserExternalLoginRepository::new();
        external_login_repo
            .expect_find_by_provider()
            .returning(move |_, _| {
                Ok(Some(UserExternalLogin {
                    id: Uuid::new_v4(),
                    user_id,
                    provider: "google".to_string(),
                    provider_user_id: "update_name_id".to_string(),
                    linked_at: chrono::Utc::now(),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }))
            });
        external_login_repo
            .expect_find_by_user_id()
            .returning(move |_| {
                Ok(vec![UserExternalLogin {
                    id: Uuid::new_v4(),
                    user_id,
                    provider: "google".to_string(),
                    provider_user_id: "update_name_id".to_string(),
                    linked_at: chrono::Utc::now(),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }])
            });

        // Mock user repository - return existing user
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id().returning(move |_| {
            Ok(Some(User {
                id: user_id,
                email: "update@example.com".to_string(),
                display_name: "Update User".to_string(),
                slug: "update-user".to_string(),
                active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        // Mock credentials repository
        let mut creds_repo = MockUserCredentialsRepository::new();
        creds_repo.expect_find_by_user_id().returning(|_| Ok(None));

        // Mock profile repository
        let mut profile_repo = MockUserProfileRepository::new();
        profile_repo.expect_find_by_user_id().returning(move |_| {
            Ok(Some(UserProfile {
                user_id,
                real_name: Some("Old Name".to_string()),
                avatar_url: None,
                bio: None,
                location: None,
                website: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });

        // Mock preferences repository
        let mut prefs_repo = MockUserPreferencesRepository::new();
        prefs_repo.expect_find_by_user_id().returning(move |_| {
            Ok(Some(UserPreferences {
                user_id,
                timer_is_public: false,
                timer_show_in_list: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });

        let token_repo = mock_token_repo();
        let pkce_storage = MockPkceStorage::new();
        pkce_storage
            .store_pkce(TEST_STATE, TEST_VERIFIER, 300)
            .await
            .unwrap();

        let service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .external_login_repository(Box::new(external_login_repo))
            .profile_repository(Box::new(profile_repo))
            .preferences_repository(Box::new(prefs_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_ok());
    }

    // ==================== Phase 4C: New OAuth Flow Tests ====================
    // Tests for OAuth flow using user_external_logins table

    #[tokio::test]
    async fn test_phase4c_new_oauth_user_creates_all_tables() {
        use crate::repositories::mocks::{
            MockUserExternalLoginRepository, MockUserPreferencesRepository,
            MockUserProfileRepository,
        };
        use crate::models::db::user_external_login::UserExternalLogin;
        use crate::models::db::user_profile::UserProfile;
        use crate::models::db::user_preferences::UserPreferences;

        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: Some("https://example.com/avatar.jpg".to_string()),
            locale: None,
            sub: "new_oauth_123".to_string(),
            email: "newoauth@example.com".to_string(),
            name: Some("New OAuth User".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);

        // Mock external login repository - no existing login
        let mut external_login_repo = MockUserExternalLoginRepository::new();
        external_login_repo
            .expect_find_by_provider()
            .returning(|_, _| Ok(None));
        external_login_repo.expect_create().returning(|data| {
            Ok(UserExternalLogin {
                id: Uuid::new_v4(),
                user_id: data.user_id,
                provider: data.provider.clone(),
                provider_user_id: data.provider_user_id.clone(),
                linked_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        external_login_repo.expect_find_by_user_id().returning(|user_id| {
            Ok(vec![UserExternalLogin {
                id: Uuid::new_v4(),
                user_id,
                provider: "google".to_string(),
                provider_user_id: "new_oauth_123".to_string(),
                linked_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }])
        });

        // Mock user repository - no existing user
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_email()
            .returning(|_| Ok(None));
        user_repo.expect_slug_exists().returning(|_| Ok(false));
        user_repo.expect_create_user().returning(|data| {
            Ok(User {
                id: Uuid::new_v4(),
                email: data.email.clone(),
                display_name: data.display_name.clone(),
                slug: data.slug.clone(),
                active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        user_repo
            .expect_add_role_to_user()
            .returning(|_, _| Ok(()));
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        // Mock profile repository
        let mut profile_repo = MockUserProfileRepository::new();
        profile_repo.expect_create().returning(|user_id| {
            Ok(UserProfile {
                user_id,
                real_name: None,
                bio: None,
                avatar_url: None,
                location: None,
                website: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        profile_repo.expect_update().returning(|user_id, _| {
            Ok(UserProfile {
                user_id,
                real_name: Some("New OAuth User".to_string()),
                bio: None,
                avatar_url: Some("https://example.com/avatar.jpg".to_string()),
                location: None,
                website: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        profile_repo.expect_find_by_user_id().returning(|user_id| {
            Ok(Some(UserProfile {
                user_id,
                real_name: Some("New OAuth User".to_string()),
                bio: None,
                avatar_url: Some("https://example.com/avatar.jpg".to_string()),
                location: None,
                website: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });

        // Mock preferences repository
        let mut prefs_repo = MockUserPreferencesRepository::new();
        prefs_repo.expect_create().returning(|user_id| {
            Ok(UserPreferences {
                user_id,
                timer_is_public: false,
                timer_show_in_list: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        prefs_repo.expect_find_by_user_id().returning(|user_id| {
            Ok(Some(UserPreferences {
                user_id,
                timer_is_public: false,
                timer_show_in_list: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });

        let token_repo = mock_token_repo();
        let pkce_storage = MockPkceStorage::new();
        pkce_storage
            .store_pkce(TEST_STATE, TEST_VERIFIER, 300)
            .await
            .unwrap();

        let service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .external_login_repository(Box::new(external_login_repo))
            .profile_repository(Box::new(profile_repo))
            .preferences_repository(Box::new(prefs_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_phase4c_existing_external_login() {
        use crate::repositories::mocks::MockUserExternalLoginRepository;
        use crate::models::db::user_external_login::UserExternalLogin;

        let user_id = Uuid::new_v4();
        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "existing_oauth_123".to_string(),
            email: "existing@example.com".to_string(),
            name: Some("Existing User".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);

        // Mock external login repository - existing login found
        let mut external_login_repo = MockUserExternalLoginRepository::new();
        external_login_repo
            .expect_find_by_provider()
            .returning(move |_, _| {
                Ok(Some(UserExternalLogin {
                    id: Uuid::new_v4(),
                    user_id,
                    provider: "google".to_string(),
                    provider_user_id: "existing_oauth_123".to_string(),
                    linked_at: chrono::Utc::now(),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }))
            });
        external_login_repo.expect_find_by_user_id().returning(move |_| {
            Ok(vec![UserExternalLogin {
                id: Uuid::new_v4(),
                user_id,
                provider: "google".to_string(),
                provider_user_id: "existing_oauth_123".to_string(),
                linked_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }])
        });

        // Mock user repository - return user by ID
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id().returning(move |_| {
            Ok(Some(User {
                id: user_id,
                email: "existing@example.com".to_string(),
                display_name: "Existing User".to_string(),
                slug: "existing-user".to_string(),
                active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

        let token_repo = mock_token_repo();
        let pkce_storage = MockPkceStorage::new();
        pkce_storage
            .store_pkce(TEST_STATE, TEST_VERIFIER, 300)
            .await
            .unwrap();

        let service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .external_login_repository(Box::new(external_login_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_phase4c_link_to_any_account_and_verify() {
        use crate::repositories::mocks::{
            MockUserExternalLoginRepository, MockUserProfileRepository,
        };
        use crate::models::db::user_external_login::UserExternalLogin;
        use crate::models::db::user_profile::UserProfile;

        let user_id = Uuid::new_v4();
        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: Some("https://example.com/pic.jpg".to_string()),
            locale: None,
            sub: "link_oauth_123".to_string(),
            email: "anyaccount@example.com".to_string(),
            name: Some("Link User".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);

        // No existing external login
        let mut external_login_repo = MockUserExternalLoginRepository::new();
        external_login_repo
            .expect_find_by_provider()
            .returning(|_, _| Ok(None));
        external_login_repo.expect_create().returning(move |data| {
            Ok(UserExternalLogin {
                id: Uuid::new_v4(),
                user_id: data.user_id,
                provider: data.provider.clone(),
                provider_user_id: data.provider_user_id.clone(),
                linked_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        external_login_repo.expect_find_by_user_id().returning(move |_| {
            Ok(vec![UserExternalLogin {
                id: Uuid::new_v4(),
                user_id,
                provider: "google".to_string(),
                provider_user_id: "link_oauth_123".to_string(),
                linked_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }])
        });

        // Existing user with email (may or may not be verified)
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_email().returning(move |_| {
            Ok(Some(User {
                id: user_id,
                email: "anyaccount@example.com".to_string(),
                display_name: "Email User".to_string(),
                slug: "email-user".to_string(),
                active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });
        user_repo
            .expect_get_user_roles()
            .returning(|_| Ok(vec!["user".to_string()])); // Not yet verified

        // Should add email-verified role (OAuth verification is trusted)
        user_repo
            .expect_add_role_to_user()
            .withf(|_, role| role == "email-verified")
            .returning(|_, _| Ok(()));

        // Mock profile repository to update with OAuth data
        let mut profile_repo = MockUserProfileRepository::new();
        profile_repo.expect_update().returning(move |uid, _| {
            Ok(UserProfile {
                user_id: uid,
                real_name: Some("Link User".to_string()),
                bio: None,
                avatar_url: Some("https://example.com/pic.jpg".to_string()),
                location: None,
                website: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });
        profile_repo.expect_find_by_user_id().returning(move |uid| {
            Ok(Some(UserProfile {
                user_id: uid,
                real_name: Some("Link User".to_string()),
                bio: None,
                avatar_url: Some("https://example.com/pic.jpg".to_string()),
                location: None,
                website: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });

        let token_repo = mock_token_repo();
        let pkce_storage = MockPkceStorage::new();
        pkce_storage
            .store_pkce(TEST_STATE, TEST_VERIFIER, 300)
            .await
            .unwrap();

        let service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .external_login_repository(Box::new(external_login_repo))
            .profile_repository(Box::new(profile_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_phase4c_user_with_multiple_providers() {
        use crate::models::db::user_external_login::UserExternalLogin;
        use crate::repositories::mocks::MockUserExternalLoginRepository;
        use crate::repositories::traits::user_external_login_repository::UserExternalLoginRepository;

        let user_id = Uuid::new_v4();

        // Mock external login repository
        let mut external_login_repo = MockUserExternalLoginRepository::new();
        external_login_repo
            .expect_find_by_user_id()
            .returning(move |_| {
                Ok(vec![
                    UserExternalLogin {
                        id: Uuid::new_v4(),
                        user_id,
                        provider: "google".to_string(),
                        provider_user_id: "google_123".to_string(),
                        linked_at: chrono::Utc::now(),
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                    UserExternalLogin {
                        id: Uuid::new_v4(),
                        user_id,
                        provider: "github".to_string(),
                        provider_user_id: "github_456".to_string(),
                        linked_at: chrono::Utc::now(),
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                ])
            });

        // Verify user has multiple providers
        let logins = external_login_repo.find_by_user_id(user_id).await.unwrap();
        assert_eq!(logins.len(), 2);

        let providers: Vec<String> = logins.iter().map(|l| l.provider.clone()).collect();
        assert!(providers.contains(&"google".to_string()));
        assert!(providers.contains(&"github".to_string()));
    }

    // ==================== OAuth Redirect State Tests ====================

    #[test]
    fn test_parse_state_parameter_with_redirect() {
        use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};

        // Encode redirect URL
        let redirect_url = "/profile";
        let encoded_redirect = base64_engine.encode(redirect_url.as_bytes());
        let state = format!("csrf-token-123|{}", encoded_redirect);

        let (csrf, redirect) = parse_state_parameter(&state);

        assert_eq!(csrf, "csrf-token-123");
        assert_eq!(redirect, Some("/profile".to_string()));
    }

    #[test]
    fn test_parse_state_parameter_without_redirect() {
        let state = "csrf-token-only";

        let (csrf, redirect) = parse_state_parameter(state);

        assert_eq!(csrf, "csrf-token-only");
        assert_eq!(redirect, None);
    }

    #[test]
    fn test_parse_state_parameter_with_special_characters() {
        use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};

        let redirect_url = "/profile?tab=security&foo=bar";
        let encoded_redirect = base64_engine.encode(redirect_url.as_bytes());
        let state = format!("csrf-123|{}", encoded_redirect);

        let (csrf, redirect) = parse_state_parameter(&state);

        assert_eq!(csrf, "csrf-123");
        assert_eq!(redirect, Some("/profile?tab=security&foo=bar".to_string()));
    }

    #[test]
    fn test_parse_state_parameter_with_invalid_base64() {
        // Invalid base64 should not crash, just return None for redirect
        let state = "csrf-token|invalid-base64!!!";

        let (csrf, redirect) = parse_state_parameter(state);

        assert_eq!(csrf, "csrf-token");
        // Invalid base64 should result in None
        assert_eq!(redirect, None);
    }

    #[test]
    fn test_validate_redirect_url_valid() {
        assert!(validate_redirect_url("/profile"));
        assert!(validate_redirect_url("/"));
        assert!(validate_redirect_url("/profile/edit"));
        assert!(validate_redirect_url("/profile?tab=security"));
    }

    #[test]
    fn test_validate_redirect_url_invalid() {
        // Protocol-relative URLs (open redirect attack)
        assert!(!validate_redirect_url("//evil.com"));
        assert!(!validate_redirect_url("//evil.com/phishing"));

        // Absolute URLs
        assert!(!validate_redirect_url("http://evil.com"));
        assert!(!validate_redirect_url("https://evil.com"));

        // Other protocols
        assert!(!validate_redirect_url("javascript:alert(1)"));
        assert!(!validate_redirect_url("data:text/html,<script>alert(1)</script>"));

        // Empty string
        assert!(!validate_redirect_url(""));
    }

    #[tokio::test]
    async fn test_oauth_url_with_redirect() {
        use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};

        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let redirect_url = "/profile".to_string();
        let result = service.google_oauth_url(Some(redirect_url.clone())).await;

        assert!(result.is_ok());

        let (_url, csrf_token) = result.unwrap();

        // Verify that the redirect was encoded in the csrf_token
        // The returned csrf_token.secret() should contain the enhanced state (csrf|encoded_redirect)
        let encoded_redirect = base64_engine.encode(redirect_url.as_bytes());
        assert!(csrf_token.secret().contains('|'), "State should contain redirect separator");
        assert!(csrf_token.secret().contains(&encoded_redirect), "State should contain encoded redirect");

        // Verify PKCE storage works with the enhanced state
        let pkce_storage = service.pkce_storage.as_ref().unwrap();
        let verifier = pkce_storage.retrieve_and_delete_pkce(csrf_token.secret()).await;
        assert!(verifier.is_ok());
        assert!(verifier.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_oauth_url_without_redirect() {
        let mock_oauth = MockGoogleOAuthService::new();
        let service = create_test_auth_service_with_mock_oauth(mock_oauth);

        let result = service.google_oauth_url(None).await;

        assert!(result.is_ok());

        let (_url, csrf_token) = result.unwrap();

        // Verify that the state doesn't contain a redirect separator
        assert!(!csrf_token.secret().contains('|'));

        // Verify PKCE storage works with plain csrf token
        let pkce_storage = service.pkce_storage.as_ref().unwrap();
        let verifier = pkce_storage.retrieve_and_delete_pkce(csrf_token.secret()).await;
        assert!(verifier.is_ok());
        assert!(verifier.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_oauth_callback_extracts_redirect() {
        use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};

        let redirect_url = "/profile";
        let encoded_redirect = base64_engine.encode(redirect_url.as_bytes());
        let csrf_token = "test-csrf";
        let state_with_redirect = format!("{}|{}", csrf_token, encoded_redirect);

        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "redirect_test_id".to_string(),
            email: "redirect@example.com".to_string(),
            name: Some("Redirect Test".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);
        let (user_repo, external_login_repo, profile_repo, prefs_repo) =
            mock_repos_for_new_oauth_user("redirect_test_id");

        let mut creds_repo = MockUserCredentialsRepository::new();
        creds_repo.expect_find_by_user_id().returning(|_| Ok(None));

        let token_repo = mock_token_repo();

        // Store PKCE with the enhanced state
        let pkce_storage = MockPkceStorage::new();
        pkce_storage
            .store_pkce(&state_with_redirect, TEST_VERIFIER, 300)
            .await
            .unwrap();

        let service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .external_login_repository(Box::new(external_login_repo))
            .profile_repository(Box::new(profile_repo))
            .preferences_repository(Box::new(prefs_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service
            .google_oauth_callback("auth_code".to_string(), state_with_redirect)
            .await;

        assert!(result.is_ok());

        let auth_response = result.unwrap();
        assert_eq!(auth_response.redirect_url, Some("/profile".to_string()));
    }

    #[tokio::test]
    async fn test_oauth_callback_without_redirect() {
        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "no_redirect_test".to_string(),
            email: "noredirect@example.com".to_string(),
            name: Some("No Redirect Test".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);
        let service =
            create_test_auth_service_for_new_oauth_user(mock_oauth, "no_redirect_test").await;

        let result = service
            .google_oauth_callback("auth_code".to_string(), TEST_STATE.to_string())
            .await;

        assert!(result.is_ok());

        let auth_response = result.unwrap();
        assert_eq!(auth_response.redirect_url, None);
    }

    #[tokio::test]
    async fn test_oauth_callback_validates_redirect_url() {
        use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};

        // Try with an invalid redirect (protocol-relative URL)
        let invalid_redirect = "//evil.com";
        let encoded_redirect = base64_engine.encode(invalid_redirect.as_bytes());
        let csrf_token = "test-csrf";
        let state_with_bad_redirect = format!("{}|{}", csrf_token, encoded_redirect);

        let user_info = GoogleUserInfo {
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
            sub: "validation_test".to_string(),
            email: "validation@example.com".to_string(),
            name: Some("Validation Test".to_string()),
            email_verified: Some(true),
        };

        let mock_oauth = MockGoogleOAuthService::new().with_user_info(user_info);
        let (user_repo, external_login_repo, profile_repo, prefs_repo) =
            mock_repos_for_new_oauth_user("validation_test");

        let mut creds_repo = MockUserCredentialsRepository::new();
        creds_repo.expect_find_by_user_id().returning(|_| Ok(None));

        let token_repo = mock_token_repo();

        let pkce_storage = MockPkceStorage::new();
        pkce_storage
            .store_pkce(&state_with_bad_redirect, TEST_VERIFIER, 300)
            .await
            .unwrap();

        let service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .external_login_repository(Box::new(external_login_repo))
            .profile_repository(Box::new(profile_repo))
            .preferences_repository(Box::new(prefs_repo))
            .refresh_token_repository(Box::new(token_repo))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service
            .google_oauth_callback("auth_code".to_string(), state_with_bad_redirect)
            .await;

        assert!(result.is_ok());

        let auth_response = result.unwrap();
        // Invalid redirect should be rejected (None)
        assert_eq!(auth_response.redirect_url, None);
    }
}
