use anyhow::Result;
use crate::repositories::traits::user_repository::UserRepository;
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;
use super::jwt::JwtService;

pub mod register;
pub mod login;
pub mod refresh_token;
pub mod profile;
pub mod password;
pub mod slug;

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

    pub async fn verify_token(&self, token: &str) -> Result<Option<super::jwt::Claims>> {
        self.jwt_service.verify_token(token).await
    }
}
