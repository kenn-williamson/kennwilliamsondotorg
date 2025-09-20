use anyhow::Result;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use rand::{thread_rng, Rng};

use crate::models::api::{AuthResponse, CreateUserRequest, LoginRequest, UserResponse, SlugPreviewRequest, SlugPreviewResponse, RefreshTokenRequest, RefreshTokenResponse, RevokeTokenRequest, ProfileUpdateRequest, PasswordChangeRequest};
use crate::repositories::traits::user_repository::{UserRepository, CreateUserData, UserUpdates};
use crate::repositories::traits::refresh_token_repository::{RefreshTokenRepository, CreateRefreshTokenData};

use super::jwt::JwtService;

pub struct AuthService {
    jwt_service: JwtService,
    user_repository: Box<dyn UserRepository>,
    refresh_token_repository: Box<dyn RefreshTokenRepository>,
}

impl AuthService {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        refresh_token_repository: Box<dyn RefreshTokenRepository>,
        jwt_secret: String,
    ) -> Self {
        let jwt_service = JwtService::new(jwt_secret);
        
        Self {
            jwt_service,
            user_repository,
            refresh_token_repository,
        }
    }

    // Main authentication methods that coordinate between services
    pub async fn register(&self, data: CreateUserRequest, device_info: Option<serde_json::Value>) -> Result<AuthResponse> {
        // Generate slug from display_name
        let slug = self.generate_slug(&data.display_name).await?;
        
        // Hash password
        let password_hash = hash(&data.password, DEFAULT_COST)?;
        
        // Create user data
        let user_data = CreateUserData {
            email: data.email,
            password_hash,
            display_name: data.display_name,
            slug,
        };
        
        // Create user via repository
        let user = self.user_repository.create_user(&user_data).await?;

        // Get user roles
        let roles = self.user_repository.get_user_roles(user.id).await?;
        
        // Generate JWT token and refresh token
        let token = self.jwt_service.generate_token(&user)?;
        let refresh_token = self.create_refresh_token(user.id, device_info).await?;

        Ok(AuthResponse {
            token,
            refresh_token,
            user: UserResponse::from_user_with_roles(user, roles),
        })
    }

    pub async fn login(&self, data: LoginRequest, device_info: Option<serde_json::Value>) -> Result<Option<AuthResponse>> {
        // Get user by email
        let user = self.user_repository.find_by_email(&data.email).await?;
        let user = match user {
            Some(user) => user,
            None => return Ok(None), // User not found
        };

        // Verify password
        if !verify(&data.password, &user.password_hash)? {
            return Ok(None); // Invalid password
        }

        // Get user roles
        let roles = self.user_repository.get_user_roles(user.id).await?;
        
        // Generate JWT token and refresh token
        let token = self.jwt_service.generate_token(&user)?;
        let refresh_token = self.create_refresh_token(user.id, device_info).await?;

        Ok(Some(AuthResponse {
            token,
            refresh_token,
            user: UserResponse::from_user_with_roles(user, roles),
        }))
    }

    pub async fn verify_token(&self, token: &str) -> Result<Option<super::jwt::Claims>> {
        self.jwt_service.verify_token(token).await
    }

    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> Result<Option<RefreshTokenResponse>> {
        // Hash the provided token to lookup in database
        let token_hash = self.hash_token(&request.refresh_token);
        
        // Find refresh token
        let token_record = self.refresh_token_repository.find_by_token(&token_hash).await?;
        let token_record = match token_record {
            Some(token) => token,
            None => return Ok(None), // Token not found or expired
        };

        // Check 6-month hard limit
        let six_months_ago = Utc::now() - Duration::days(180);
        if token_record.created_at < six_months_ago {
            // Delete the expired token
            self.refresh_token_repository.revoke_token(&token_hash).await?;
            return Ok(None);
        }

        // Get user
        let user = self.user_repository.find_by_id(token_record.user_id).await?;
        let user = match user {
            Some(user) => user,
            None => return Ok(None), // User no longer exists
        };

        // Generate new JWT and refresh token
        let new_jwt = self.jwt_service.generate_token(&user)?;
        let new_refresh_token = self.generate_refresh_token_string();
        let new_token_hash = self.hash_token(&new_refresh_token);

        // Delete old token and create new token
        self.refresh_token_repository.revoke_token(&token_hash).await?;
        
        // Create new refresh token
        let expires_at = Utc::now() + Duration::days(7);
        let token_data = CreateRefreshTokenData {
            user_id: token_record.user_id,
            token: new_token_hash,
            device_info: token_record.device_info,
            expires_at,
        };
        self.refresh_token_repository.create_token(&token_data).await?;

        Ok(Some(RefreshTokenResponse {
            token: new_jwt,
            refresh_token: new_refresh_token,
        }))
    }

    pub async fn revoke_refresh_token(&self, request: RevokeTokenRequest) -> Result<bool> {
        let token_hash = self.hash_token(&request.refresh_token);
        let result = self.refresh_token_repository.revoke_token(&token_hash).await;
        Ok(result.is_ok())
    }

    pub async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<u64> {
        self.refresh_token_repository.revoke_all_user_tokens(user_id).await?;
        Ok(1) // Return count of affected tokens
    }

    pub async fn get_current_user(&self, user_id: Uuid) -> Result<Option<UserResponse>> {
        let user = self.user_repository.find_by_id(user_id).await?;
        match user {
            Some(user) => {
                let roles = self.user_repository.get_user_roles(user.id).await?;
                Ok(Some(UserResponse::from_user_with_roles(user, roles)))
            },
            None => Ok(None),
        }
    }

    pub async fn preview_slug(&self, request: SlugPreviewRequest) -> Result<SlugPreviewResponse> {
        let slug = self.generate_slug(&request.display_name).await?;
        let available = !self.user_repository.slug_exists(&slug).await?;
        
        Ok(SlugPreviewResponse {
            slug: slug.clone(),
            available,
            final_slug: if available { slug } else { format!("{}-{}", slug, thread_rng().gen_range(1..=999)) },
        })
    }

    pub async fn update_profile(&self, user_id: Uuid, request: ProfileUpdateRequest) -> Result<UserResponse> {
        // Validate slug format
        let generated_slug = self.generate_slug(&request.slug).await?;
        if generated_slug != request.slug {
            return Err(anyhow::anyhow!("Invalid slug format"));
        }

        // Check if slug is available (excluding current user)
        if self.user_repository.slug_exists_excluding_user(&request.slug, user_id).await? {
            return Err(anyhow::anyhow!("Slug already taken"));
        }

        // Update user profile
        let updates = UserUpdates {
            display_name: Some(request.display_name),
            slug: Some(request.slug),
            active: None,
        };
        
        let user = self.user_repository.update_user(user_id, &updates).await?;
        let roles = self.user_repository.get_user_roles(user.id).await?;

        Ok(UserResponse::from_user_with_roles(user, roles))
    }

    pub async fn change_password(&self, user_id: Uuid, request: PasswordChangeRequest) -> Result<()> {
        // Get current user
        let user = self.user_repository.find_by_id(user_id).await?;
        let user = match user {
            Some(user) => user,
            None => return Err(anyhow::anyhow!("User not found")),
        };

        // Verify current password
        if !verify(&request.current_password, &user.password_hash)? {
            return Err(anyhow::anyhow!("Current password is incorrect"));
        }

        // Hash new password
        let new_password_hash = hash(&request.new_password, DEFAULT_COST)?;

        // Update password
        self.user_repository.update_password(user_id, &new_password_hash).await?;

        Ok(())
    }

    // Helper methods

    /// Generate slug from display name
    async fn generate_slug(&self, display_name: &str) -> Result<String> {
        let base_slug = display_name
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-");

        let mut slug = base_slug.clone();
        let mut counter = 1;

        while self.user_repository.slug_exists(&slug).await? {
            slug = format!("{}-{}", base_slug, counter);
            counter += 1;
        }

        Ok(slug)
    }

    /// Create refresh token
    async fn create_refresh_token(&self, user_id: Uuid, device_info: Option<serde_json::Value>) -> Result<String> {
        // Generate random token
        let token = self.generate_refresh_token_string();
        let token_hash = self.hash_token(&token);
        
        // Set expiration (7 days)
        let expires_at = Utc::now() + Duration::days(7);
        
        // Create token data
        let token_data = CreateRefreshTokenData {
            user_id,
            token: token_hash,
            device_info,
            expires_at,
        };
        
        // Store in database
        self.refresh_token_repository.create_token(&token_data).await?;
        
        // Return plain token (not hash)
        Ok(token)
    }

    /// Generate refresh token string
    fn generate_refresh_token_string(&self) -> String {
        let mut token_bytes = [0u8; 32]; // 256 bits
        thread_rng().fill(&mut token_bytes);
        hex::encode(token_bytes)
    }

    /// Hash token for storage
    fn hash_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }
}
