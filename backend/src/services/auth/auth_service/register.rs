use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};

use crate::models::api::{AuthResponse, CreateUserRequest, UserResponse};
use crate::repositories::traits::user_repository::CreateUserData;
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;
use crate::models::db::refresh_token::CreateRefreshToken;
use super::AuthService;
use super::slug::generate_slug;

impl AuthService {
    /// Register a new user
    pub async fn register(&self, data: CreateUserRequest, device_info: Option<serde_json::Value>) -> Result<AuthResponse> {
        // Generate slug from display_name
        let slug = generate_slug(&data.display_name, &*self.user_repository).await?;
        
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
        let refresh_token = create_refresh_token(user.id, device_info, &*self.refresh_token_repository).await?;

        Ok(AuthResponse {
            token,
            refresh_token,
            user: UserResponse::from_user_with_roles(user, roles),
        })
    }
}

/// Create refresh token
async fn create_refresh_token(
    user_id: uuid::Uuid,
    device_info: Option<serde_json::Value>,
    refresh_token_repository: &dyn RefreshTokenRepository,
) -> Result<String> {
    // Generate random token
    let token = generate_refresh_token_string();
    let token_hash = hash_token(&token);
    
    // Set expiration (7 days)
    let expires_at = chrono::Utc::now() + chrono::Duration::days(7);
    
    // Create token data
    let token_data = CreateRefreshToken {
        user_id,
        token_hash,
        device_info,
        expires_at,
    };
    
    // Store in database
    refresh_token_repository.create_token(&token_data).await?;
    
    // Return plain token (not hash)
    Ok(token)
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
    use anyhow::Result;
    use crate::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::repositories::mocks::mock_refresh_token_repository::MockRefreshTokenRepository;
    use uuid::Uuid;
    use chrono::Utc;
    use crate::models::db::refresh_token::RefreshToken;
    use crate::services::auth::jwt::JwtService;

    fn create_test_user() -> crate::models::db::User {
        crate::models::db::User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: "hashed".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_refresh_token() -> RefreshToken {
        RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token_hash: "test_token_hash".to_string(),
            device_info: None,
            expires_at: Utc::now() + chrono::Duration::days(7),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_used_at: None,
        }
    }

    #[tokio::test]
    async fn registers_user_successfully() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Ok(false)); // Slug doesn't exist

        user_repo
            .expect_create_user()
            .times(1)
            .returning(|_| Ok(create_test_user()));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.register(request, None).await?;
        
        assert!(!result.token.is_empty());
        assert!(!result.refresh_token.is_empty());
        assert_eq!(result.user.email, "test@example.com");
        assert_eq!(result.user.display_name, "Test User");
        assert_eq!(result.user.roles, vec!["user"]);
        
        Ok(())
    }

    #[tokio::test]
    async fn handles_database_error_during_user_creation() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Ok(false)); // Slug doesn't exist

        user_repo
            .expect_create_user()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.register(request, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
        
        Ok(())
    }

    #[tokio::test]
    async fn handles_database_error_during_refresh_token_creation() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Ok(false)); // Slug doesn't exist

        user_repo
            .expect_create_user()
            .times(1)
            .returning(|_| Ok(create_test_user()));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.register(request, None).await;
        assert!(result.is_err());
        
        Ok(())
    }
}

