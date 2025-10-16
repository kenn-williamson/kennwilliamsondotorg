use super::jwt::JwtService;
use crate::repositories::traits::incident_timer_repository::IncidentTimerRepository;
use crate::repositories::traits::password_reset_token_repository::PasswordResetTokenRepository;
use crate::repositories::traits::phrase_repository::PhraseRepository;
use crate::repositories::traits::pkce_storage::PkceStorage;
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;
use crate::repositories::traits::user_credentials_repository::UserCredentialsRepository;
use crate::repositories::traits::user_external_login_repository::UserExternalLoginRepository;
use crate::repositories::traits::user_preferences_repository::UserPreferencesRepository;
use crate::repositories::traits::user_profile_repository::UserProfileRepository;
use crate::repositories::traits::user_repository::UserRepository;
use crate::repositories::traits::verification_token_repository::VerificationTokenRepository;
use crate::services::auth::oauth::GoogleOAuthServiceTrait;
use crate::services::email::EmailService;
use anyhow::Result;

pub mod builder;
pub mod account_deletion;
pub mod data_export;
pub mod email_verification;
pub mod login;
pub mod oauth;
pub mod password;
pub mod password_reset;
pub mod profile;
pub mod refresh_token;
pub mod register;
pub mod slug;

pub use builder::AuthServiceBuilder;

pub struct AuthService {
    jwt_service: JwtService,
    user_repository: Box<dyn UserRepository>,
    refresh_token_repository: Box<dyn RefreshTokenRepository>,
    verification_token_repository: Option<Box<dyn VerificationTokenRepository>>,
    password_reset_token_repository: Option<Box<dyn PasswordResetTokenRepository>>,
    email_service: Option<Box<dyn EmailService>>,
    google_oauth_service: Option<Box<dyn GoogleOAuthServiceTrait>>,
    pkce_storage: Option<Box<dyn PkceStorage>>,
    incident_timer_repository: Option<Box<dyn IncidentTimerRepository>>,
    phrase_repository: Option<Box<dyn PhraseRepository>>,
    credentials_repository: Option<Box<dyn UserCredentialsRepository>>,
    external_login_repository: Option<Box<dyn UserExternalLoginRepository>>,
    profile_repository: Option<Box<dyn UserProfileRepository>>,
    preferences_repository: Option<Box<dyn UserPreferencesRepository>>,
}

impl AuthService {
    /// Create a new AuthService builder
    pub fn builder() -> AuthServiceBuilder {
        AuthServiceBuilder::new()
    }

    /// Legacy constructor for existing tests (will be phased out)
    /// Prefer using AuthService::builder() for new code
    #[allow(dead_code)] // Legacy API for existing tests
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        refresh_token_repository: Box<dyn RefreshTokenRepository>,
        jwt_secret: String,
    ) -> Self {
        Self::builder()
            .user_repository(user_repository)
            .refresh_token_repository(refresh_token_repository)
            .jwt_secret(jwt_secret)
            .build()
    }

    pub async fn verify_token(&self, token: &str) -> Result<Option<super::jwt::Claims>> {
        self.jwt_service.verify_token(token).await
    }

    /// Build a fully populated UserResponse with nested data from all related tables
    /// This queries each repository individually (credentials, external_logins, profile, preferences)
    /// and constructs a complete UserResponse with all nested fields populated.
    pub async fn build_user_response_with_details(
        &self,
        user: crate::models::db::User,
        roles: Vec<String>,
    ) -> Result<crate::models::api::UserResponse> {
        use crate::models::api::{UserResponse, ProfileData, ExternalAccount, PreferencesData};

        let email_verified = roles.contains(&"email-verified".to_string());

        // Query profile data
        let profile = if let Some(profile_repo) = &self.profile_repository {
            profile_repo
                .find_by_user_id(user.id)
                .await?
                .map(|p| ProfileData {
                    real_name: p.real_name,
                    bio: p.bio,
                    avatar_url: p.avatar_url,
                    location: p.location,
                    website: p.website,
                })
        } else {
            None
        };

        // Query external logins
        let external_accounts = if let Some(ext_repo) = &self.external_login_repository {
            ext_repo
                .find_by_user_id(user.id)
                .await?
                .into_iter()
                .map(|login| ExternalAccount {
                    provider: login.provider,
                    linked_at: login.linked_at,
                })
                .collect()
        } else {
            vec![]
        };

        // Query preferences
        let preferences = if let Some(prefs_repo) = &self.preferences_repository {
            prefs_repo
                .find_by_user_id(user.id)
                .await?
                .map(|p| PreferencesData {
                    timer_is_public: p.timer_is_public,
                    timer_show_in_list: p.timer_show_in_list,
                })
        } else {
            None
        };

        // Check if user has password credentials
        let has_credentials = if let Some(creds_repo) = &self.credentials_repository {
            creds_repo.find_by_user_id(user.id).await?.is_some()
        } else {
            false
        };

        Ok(UserResponse {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            slug: user.slug,
            roles,
            email_verified,
            has_credentials,
            created_at: user.created_at,
            profile,
            external_accounts,
            preferences,
        })
    }
}
