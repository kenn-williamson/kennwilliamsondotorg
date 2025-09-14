use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use rand::{thread_rng, Rng};

use crate::models::db::User;
use crate::models::api::{AuthResponse, CreateUserRequest, LoginRequest, UserResponse, SlugPreviewRequest, SlugPreviewResponse, RefreshTokenRequest, RefreshTokenResponse, RevokeTokenRequest};
use crate::models::refresh_token::RefreshToken;
use crate::models::auth_user::AuthUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: i64,
    pub iat: i64,
}

#[derive(Clone)]
pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }

    pub async fn register(&self, data: CreateUserRequest, device_info: Option<serde_json::Value>) -> Result<AuthResponse> {
        // Hash password
        let password_hash = hash(&data.password, DEFAULT_COST)?;
        
        // Generate slug from display_name
        let (_, final_slug) = self.find_available_slug(Self::generate_slug(&data.display_name)).await?;
        
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash, display_name, slug)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, password_hash, display_name, slug, created_at, updated_at
            "#,
            data.email,
            password_hash,
            data.display_name,
            final_slug
        )
        .fetch_one(&self.pool)
        .await?;

        // Assign default 'user' role
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'user'
            "#,
            user.id
        )
        .execute(&self.pool)
        .await?;

        // Get user roles
        let roles = self.get_user_roles(user.id).await?;
        
        // Generate JWT token and refresh token
        let token = self.generate_token(&user)?;
        let refresh_token = self.create_refresh_token(user.id, device_info).await?;

        Ok(AuthResponse {
            token,
            refresh_token,
            user: UserResponse::from_user_with_roles(user, roles),
        })
    }

    pub async fn login(&self, data: LoginRequest, device_info: Option<serde_json::Value>) -> Result<Option<AuthResponse>> {
        // Get user by email
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, display_name, slug, created_at, updated_at FROM users WHERE email = $1",
            data.email
        )
        .fetch_optional(&self.pool)
        .await?;

        let user = match user {
            Some(user) => user,
            None => return Ok(None), // User not found
        };

        // Verify password
        if !verify(&data.password, &user.password_hash)? {
            return Ok(None); // Invalid password
        }

        // Get user roles
        let roles = self.get_user_roles(user.id).await?;
        
        // Generate JWT token and refresh token
        let token = self.generate_token(&user)?;
        let refresh_token = self.create_refresh_token(user.id, device_info).await?;

        Ok(Some(AuthResponse {
            token,
            refresh_token,
            user: UserResponse::from_user_with_roles(user, roles),
        }))
    }

    pub async fn verify_token(&self, token: &str) -> Result<Option<Claims>> {
        let validation = Validation::default();
        let token_data: TokenData<Claims> = decode(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )?;

        Ok(Some(token_data.claims))
    }

    // Refresh token operations

    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> Result<Option<RefreshTokenResponse>> {
        // Hash the provided token to lookup in database
        let token_hash = self.hash_token(&request.refresh_token);

        // Get the refresh token from database
        let refresh_token = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT id, user_id, token_hash, device_info, expires_at, created_at, updated_at, last_used_at
            FROM refresh_tokens
            WHERE token_hash = $1 AND expires_at > NOW()
            "#,
            token_hash
        )
        .fetch_optional(&self.pool)
        .await?;

        let refresh_token = match refresh_token {
            Some(token) => token,
            None => return Ok(None), // Token not found or expired
        };

        // Check 6-month hard limit
        let six_months_ago = Utc::now() - Duration::days(180);
        if refresh_token.created_at < six_months_ago {
            // Delete the expired token
            self.delete_refresh_token_by_hash(&token_hash).await?;
            return Ok(None);
        }

        // Get user
        let user = self.get_user_by_id(refresh_token.user_id).await?;
        let user = match user {
            Some(user) => user,
            None => return Ok(None), // User no longer exists
        };

        // Generate new JWT and refresh token
        let new_jwt = self.generate_token(&user)?;
        let new_refresh_token = self.generate_refresh_token_string();
        let new_token_hash = self.hash_token(&new_refresh_token);

        // Delete old token and create new token in transaction
        let mut tx = self.pool.begin().await?;

        // Delete old token (rolling token system)
        sqlx::query!(
            "DELETE FROM refresh_tokens WHERE token_hash = $1",
            token_hash
        )
        .execute(&mut *tx)
        .await?;

        // Create new refresh token
        let expires_at = Utc::now() + Duration::days(30);
        sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, device_info, expires_at)
            VALUES ($1, $2, $3, $4)
            "#,
            refresh_token.user_id,
            new_token_hash,
            refresh_token.device_info,
            expires_at
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(Some(RefreshTokenResponse {
            token: new_jwt,
            refresh_token: new_refresh_token,
        }))
    }

    pub async fn create_refresh_token(&self, user_id: Uuid, device_info: Option<serde_json::Value>) -> Result<String> {
        let refresh_token_string = self.generate_refresh_token_string();
        let token_hash = self.hash_token(&refresh_token_string);
        let expires_at = Utc::now() + Duration::days(30);

        sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, device_info, expires_at)
            VALUES ($1, $2, $3, $4)
            "#,
            user_id,
            token_hash,
            device_info,
            expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(refresh_token_string)
    }

    pub async fn revoke_refresh_token(&self, request: RevokeTokenRequest) -> Result<bool> {
        let token_hash = self.hash_token(&request.refresh_token);
        self.delete_refresh_token_by_hash(&token_hash).await
    }

    pub async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<u64> {
        let result = sqlx::query!(
            "DELETE FROM refresh_tokens WHERE user_id = $1",
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }


    // Helper methods for refresh tokens

    fn generate_refresh_token_string(&self) -> String {
        let mut token_bytes = [0u8; 32]; // 256 bits
        thread_rng().fill(&mut token_bytes);
        hex::encode(token_bytes)
    }

    fn hash_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }

    async fn delete_refresh_token_by_hash(&self, token_hash: &str) -> Result<bool> {
        let result = sqlx::query!(
            "DELETE FROM refresh_tokens WHERE token_hash = $1",
            token_hash
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // Generic, reusable user lookup
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, display_name, slug, created_at, updated_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    // Business logic for current user API endpoint
    pub async fn get_current_user(&self, user_id: Uuid) -> Result<Option<UserResponse>> {
        if let Some(user) = self.get_user_by_id(user_id).await? {
            let roles = self.get_user_roles(user.id).await?;
            Ok(Some(UserResponse::from_user_with_roles(user, roles)))
        } else {
            Ok(None)
        }
    }

    // Helper for middleware/route handlers that need AuthUser with roles
    pub async fn get_auth_user(&self, user_id: Uuid) -> Result<Option<AuthUser>> {
        if let Some(user) = self.get_user_by_id(user_id).await? {
            let roles = self.get_user_roles(user.id).await?;
            let roles_set = roles.into_iter().collect();
            Ok(Some(AuthUser {
                id: user.id,
                email: user.email,
                roles: roles_set,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>> {
        let roles = sqlx::query!(
            r#"
            SELECT r.name
            FROM roles r
            JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(roles.into_iter().map(|r| r.name).collect())
    }

    fn generate_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(1); // Reduced to 1 hour since we have refresh tokens

        let claims = Claims {
            sub: user.id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }

    pub async fn preview_slug(&self, request: SlugPreviewRequest) -> Result<SlugPreviewResponse> {
        let base_slug = Self::generate_slug(&request.display_name);
        let (available, final_slug) = self.find_available_slug(base_slug.clone()).await?;
        
        Ok(SlugPreviewResponse {
            slug: base_slug,
            available,
            final_slug,
        })
    }

    pub fn generate_slug(display_name: &str) -> String {
        display_name
            .to_lowercase()
            .trim()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }

    async fn find_available_slug(&self, base_slug: String) -> Result<(bool, String)> {
        // Check if base slug exists
        if !self.slug_exists(&base_slug).await? {
            return Ok((true, base_slug));
        }
        
        // Try numbered variants: slug-2, slug-3, etc.
        for i in 2..=999 {
            let candidate = format!("{}-{}", base_slug, i);
            if !self.slug_exists(&candidate).await? {
                return Ok((false, candidate));
            }
        }
        
        // Fallback: append timestamp if all numbered variants taken
        let timestamp = chrono::Utc::now().timestamp();
        Ok((false, format!("{}-{}", base_slug, timestamp)))
    }

    async fn slug_exists(&self, slug: &str) -> Result<bool> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE slug = $1")
            .bind(slug)
            .fetch_one(&self.pool)
            .await?;
        Ok(count > 0)
    }
}