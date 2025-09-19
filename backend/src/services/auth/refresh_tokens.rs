use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use rand::{thread_rng, Rng};

use crate::models::api::{RefreshTokenRequest, RefreshTokenResponse, RevokeTokenRequest};
use crate::models::refresh_token::{RefreshToken, CreateRefreshToken};
use super::jwt::JwtService;

#[derive(Clone)]
pub struct RefreshTokenService {
    pool: PgPool,
    jwt_service: JwtService,
}

impl RefreshTokenService {

    pub fn with_jwt_service(pool: PgPool, jwt_service: JwtService) -> Self {
        Self { pool, jwt_service }
    }

    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> Result<Option<RefreshTokenResponse>> {
        log::debug!("Starting token refresh process");
        
        // Hash the provided token to lookup in database
        let token_hash = self.hash_token(&request.refresh_token);
        log::debug!("Token hashed for database lookup");

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
            Some(token) => {
                log::debug!("Refresh token found for user: {}", token.user_id);
                token
            },
            None => {
                log::warn!("Refresh token not found or expired");
                return Ok(None);
            }
        };

        // Check 6-month hard limit
        let six_months_ago = Utc::now() - Duration::days(180);
        if refresh_token.created_at < six_months_ago {
            log::info!("Refresh token exceeded 6-month hard limit for user: {}, created: {}", 
                      refresh_token.user_id, refresh_token.created_at);
            // Delete the expired token
            self.delete_refresh_token_by_hash(&token_hash).await?;
            return Ok(None);
        }

        // Get user
        let user = self.get_user_by_id(refresh_token.user_id).await?;
        let user = match user {
            Some(user) => {
                log::debug!("User found for token refresh: {}", user.id);
                user
            },
            None => {
                log::warn!("User no longer exists for refresh token: {}", refresh_token.user_id);
                return Ok(None);
            }
        };

        // Generate new JWT and refresh token
        log::debug!("Generating new JWT and refresh token for user: {}", user.id);
        let new_jwt = self.jwt_service.generate_token(&user)?;
        let new_refresh_token = self.generate_refresh_token_string();
        let new_token_hash = self.hash_token(&new_refresh_token);

        // Delete old token and create new token in transaction
        let mut tx = self.pool.begin().await?;
        log::debug!("Starting database transaction for token refresh");

        // Delete old token (rolling token system)
        sqlx::query!(
            "DELETE FROM refresh_tokens WHERE token_hash = $1",
            token_hash
        )
        .execute(&mut *tx)
        .await?;
        log::debug!("Old refresh token deleted");

        // Create new refresh token (aligned with 1-week session expiration)
        let expires_at = Utc::now() + Duration::days(7);
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
        log::debug!("New refresh token created");

        tx.commit().await?;
        log::info!("Token refresh completed successfully for user: {}", user.id);

        Ok(Some(RefreshTokenResponse {
            token: new_jwt,
            refresh_token: new_refresh_token,
        }))
    }

    pub async fn create_refresh_token(&self, user_id: Uuid, device_info: Option<serde_json::Value>) -> Result<String> {
        let refresh_token_string = self.generate_refresh_token_string();
        let token_hash = self.hash_token(&refresh_token_string);
        let expires_at = Utc::now() + Duration::days(7); // Aligned with 1-week session expiration

        let request = CreateRefreshToken {
            user_id,
            token_hash,
            device_info,
            expires_at,
        };

        self.create_refresh_token_from_dto(request).await?;
        Ok(refresh_token_string)
    }

    pub async fn create_refresh_token_from_dto(&self, request: CreateRefreshToken) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, device_info, expires_at)
            VALUES ($1, $2, $3, $4)
            "#,
            request.user_id,
            request.token_hash,
            request.device_info,
            request.expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
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

    // Helper methods

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

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<crate::models::db::User>> {
        let user = sqlx::query_as!(
            crate::models::db::User,
            "SELECT id, email, password_hash, display_name, slug, active, created_at, updated_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

}
