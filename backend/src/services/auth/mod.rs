pub mod jwt;
pub mod refresh_tokens;
pub mod user_management;
pub mod slug_utils;

use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::api::{AuthResponse, CreateUserRequest, LoginRequest, UserResponse, SlugPreviewRequest, SlugPreviewResponse, RefreshTokenRequest, RefreshTokenResponse, RevokeTokenRequest, ProfileUpdateRequest, PasswordChangeRequest};

use self::jwt::JwtService;
use self::refresh_tokens::RefreshTokenService;
use self::user_management::UserManagementService;
use self::slug_utils::SlugUtils;

#[derive(Clone)]
pub struct AuthService {
    jwt_service: JwtService,
    refresh_token_service: RefreshTokenService,
    user_management_service: UserManagementService,
    slug_utils: SlugUtils,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        let jwt_service = JwtService::new(jwt_secret.clone());
        let refresh_token_service = RefreshTokenService::with_jwt_service(pool.clone(), jwt_service.clone());
        let user_management_service = UserManagementService::new(pool.clone());
        let slug_utils = SlugUtils::new(pool.clone());

        Self {
            jwt_service,
            refresh_token_service,
            user_management_service,
            slug_utils,
        }
    }

    // Main authentication methods that coordinate between services
    pub async fn register(&self, data: CreateUserRequest, device_info: Option<serde_json::Value>) -> Result<AuthResponse> {
        // Generate slug from display_name
        let (_, final_slug) = self.slug_utils.find_available_slug(SlugUtils::generate_slug(&data.display_name)).await?;
        
        // Create user
        let user = self.user_management_service.create_user(data, final_slug).await?;

        // Get user roles
        let roles = self.user_management_service.get_user_roles(user.id).await?;
        
        // Generate JWT token and refresh token
        let token = self.jwt_service.generate_token(&user)?;
        let refresh_token = self.refresh_token_service.create_refresh_token(user.id, device_info).await?;

        Ok(AuthResponse {
            token,
            refresh_token,
            user: UserResponse::from_user_with_roles(user, roles),
        })
    }

    pub async fn login(&self, data: LoginRequest, device_info: Option<serde_json::Value>) -> Result<Option<AuthResponse>> {
        // Get user by email and verify password
        let user = self.user_management_service.authenticate_user(data).await?;
        let user = match user {
            Some(user) => user,
            None => return Ok(None), // User not found or invalid password
        };

        // Get user roles
        let roles = self.user_management_service.get_user_roles(user.id).await?;
        
        // Generate JWT token and refresh token
        let token = self.jwt_service.generate_token(&user)?;
        let refresh_token = self.refresh_token_service.create_refresh_token(user.id, device_info).await?;

        Ok(Some(AuthResponse {
            token,
            refresh_token,
            user: UserResponse::from_user_with_roles(user, roles),
        }))
    }

    pub async fn verify_token(&self, token: &str) -> Result<Option<jwt::Claims>> {
        self.jwt_service.verify_token(token).await
    }

    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> Result<Option<RefreshTokenResponse>> {
        self.refresh_token_service.refresh_token(request).await
    }

    pub async fn revoke_refresh_token(&self, request: RevokeTokenRequest) -> Result<bool> {
        self.refresh_token_service.revoke_refresh_token(request).await
    }

    pub async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<u64> {
        self.refresh_token_service.revoke_all_user_tokens(user_id).await
    }

    pub async fn get_current_user(&self, user_id: Uuid) -> Result<Option<UserResponse>> {
        self.user_management_service.get_current_user(user_id).await
    }


    pub async fn preview_slug(&self, request: SlugPreviewRequest) -> Result<SlugPreviewResponse> {
        self.slug_utils.preview_slug(request).await
    }

    pub async fn update_profile(&self, user_id: Uuid, request: ProfileUpdateRequest) -> Result<UserResponse> {
        self.user_management_service.update_profile(user_id, request).await
    }

    pub async fn change_password(&self, user_id: Uuid, request: PasswordChangeRequest) -> Result<()> {
        self.user_management_service.change_password(user_id, request).await
    }
}
