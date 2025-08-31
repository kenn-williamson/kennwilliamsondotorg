use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use chrono::{Duration, Utc};

use crate::models::user::{AuthResponse, CreateUserRequest, LoginRequest, User, UserResponse, SlugPreviewRequest, SlugPreviewResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub roles: Vec<String>,
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

    pub async fn register(&self, data: CreateUserRequest) -> Result<AuthResponse> {
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
        
        // Generate JWT token
        let token = self.generate_token(&user, &roles)?;
        
        Ok(AuthResponse {
            token,
            user: UserResponse::from_user_with_roles(user, roles),
        })
    }

    pub async fn login(&self, data: LoginRequest) -> Result<Option<AuthResponse>> {
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
        
        // Generate JWT token
        let token = self.generate_token(&user, &roles)?;
        
        Ok(Some(AuthResponse {
            token,
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

    fn generate_token(&self, user: &User, roles: &[String]) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(24);

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            roles: roles.to_vec(),
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