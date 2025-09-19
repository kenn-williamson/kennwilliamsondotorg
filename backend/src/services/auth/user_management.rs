use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;

use crate::models::db::User;
use crate::models::api::{CreateUserRequest, LoginRequest, UserResponse, ProfileUpdateRequest, PasswordChangeRequest};

#[derive(Clone)]
pub struct UserManagementService {
    pool: PgPool,
}

impl UserManagementService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, data: CreateUserRequest, final_slug: String) -> Result<User> {
        // Hash password
        let password_hash = hash(&data.password, DEFAULT_COST)?;
        
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash, display_name, slug)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, password_hash, display_name, slug, active, created_at, updated_at
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

        Ok(user)
    }

    pub async fn authenticate_user(&self, data: LoginRequest) -> Result<Option<User>> {
        // Get user by email
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, display_name, slug, active, created_at, updated_at FROM users WHERE email = $1",
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

        Ok(Some(user))
    }

    pub async fn get_current_user(&self, user_id: Uuid) -> Result<Option<UserResponse>> {
        if let Some(user) = self.get_user_by_id(user_id).await? {
            let roles = self.get_user_roles(user.id).await?;
            Ok(Some(UserResponse::from_user_with_roles(user, roles)))
        } else {
            Ok(None)
        }
    }


    pub async fn update_profile(&self, user_id: Uuid, request: ProfileUpdateRequest) -> Result<UserResponse> {
        // Validate slug format using the slug utils
        use super::slug_utils::SlugUtils;
        
        // Validate that the slug follows proper format (but doesn't need to match display name)
        let generated_slug = SlugUtils::generate_slug(&request.slug);
        if generated_slug != request.slug {
            return Err(anyhow::anyhow!("Invalid slug format"));
        }

        // Check if slug is available (excluding current user)
        if self.slug_exists_excluding_user(&request.slug, user_id).await? {
            return Err(anyhow::anyhow!("Slug already taken"));
        }

        // Update user profile
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users 
            SET display_name = $1, slug = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING id, email, password_hash, display_name, slug, active, created_at, updated_at
            "#,
            request.display_name,
            request.slug,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Get user roles
        let roles = self.get_user_roles(user.id).await?;

        Ok(UserResponse::from_user_with_roles(user, roles))
    }

    pub async fn change_password(&self, user_id: Uuid, request: PasswordChangeRequest) -> Result<()> {
        // Get current user
        let user = self.get_user_by_id(user_id).await?;
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
        sqlx::query!(
            "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
            new_password_hash,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Helper methods

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, display_name, slug, active, created_at, updated_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>> {
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

    async fn slug_exists_excluding_user(&self, slug: &str, user_id: Uuid) -> Result<bool> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE slug = $1 AND id != $2"
        )
        .bind(slug)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(count > 0)
    }
}
