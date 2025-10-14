use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::user_profile::UserProfile;

/// Data structure for updating profile fields
#[derive(Debug, Clone)]
pub struct UpdateProfile {
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

/// Repository trait for user profile data (bio, avatar, etc.)
#[async_trait]
pub trait UserProfileRepository: Send + Sync {
    /// Create an empty profile for a user
    async fn create(&self, user_id: Uuid) -> Result<UserProfile>;

    /// Find profile by user ID
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>>;

    /// Update profile fields (only updates provided fields)
    async fn update(&self, user_id: Uuid, data: UpdateProfile) -> Result<UserProfile>;
}
