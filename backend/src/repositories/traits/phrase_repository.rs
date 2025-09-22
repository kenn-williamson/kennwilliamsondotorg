use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::models::db::{Phrase, PhraseSuggestion, PhraseWithUserExclusionView};
use crate::models::api::{
    CreatePhraseRequest, UpdatePhraseRequest
};

/// Repository trait for phrase operations
#[async_trait]
pub trait PhraseRepository: Send + Sync {
    /// Get a random active phrase for a user by slug, excluding phrases the user has excluded
    async fn get_random_phrase_by_slug(&self, user_slug: &str) -> Result<String>;
    
    /// Get a random active phrase, excluding phrases the user has excluded (for authenticated users)
    async fn get_random_phrase(&self, user_id: Uuid) -> Result<String>;
    
    /// Get all active phrases for a user (excluding their excluded phrases)
    async fn get_user_phrases(
        &self, 
        user_id: Uuid, 
        limit: Option<i64>, 
        offset: Option<i64>
    ) -> Result<Vec<Phrase>>;
    
    /// Get all active phrases for a user with exclusion status (single API call)
    async fn get_user_phrases_with_exclusions(&self, user_id: Uuid) -> Result<Vec<PhraseWithUserExclusionView>>;
    
    /// Get all phrases (admin only)
    async fn get_all_phrases(
        &self, 
        include_inactive: bool, 
        limit: Option<i64>, 
        offset: Option<i64>
    ) -> Result<Vec<Phrase>>;
    
    /// Create a new phrase (admin only)
    async fn create_phrase(&self, request: CreatePhraseRequest, created_by: Uuid) -> Result<Phrase>;
    
    /// Update a phrase (admin only)
    async fn update_phrase(&self, phrase_id: Uuid, request: UpdatePhraseRequest) -> Result<Phrase>;
    
    /// Exclude a phrase for a user
    async fn exclude_phrase_for_user(&self, user_id: Uuid, phrase_id: Uuid) -> Result<()>;
    
    /// Remove phrase exclusion for a user
    async fn remove_phrase_exclusion(&self, user_id: Uuid, phrase_id: Uuid) -> Result<()>;
    
    /// Get user's excluded phrases
    async fn get_user_excluded_phrases(&self, user_id: Uuid) -> Result<Vec<(Uuid, String, chrono::DateTime<chrono::Utc>)>>;
    
    /// Submit a phrase suggestion
    async fn submit_phrase_suggestion(&self, user_id: Uuid, request: crate::models::api::PhraseSuggestionRequest) -> Result<PhraseSuggestion>;
    
    /// Get user's phrase suggestions
    async fn get_user_suggestions(&self, user_id: Uuid) -> Result<Vec<PhraseSuggestion>>;
    
    /// Get all pending phrase suggestions (admin only)
    async fn get_pending_suggestions(&self) -> Result<Vec<PendingSuggestionWithUser>>;
    
    /// Approve a phrase suggestion (admin only)
    async fn approve_suggestion(
        &self,
        suggestion_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()>;
    
    /// Reject a phrase suggestion (admin only)
    async fn reject_suggestion(
        &self,
        suggestion_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()>;
    
    /// Count all phrases
    async fn count_all_phrases(&self) -> Result<i64>;
    
    /// Count pending suggestions
    async fn count_pending_suggestions(&self) -> Result<i64>;
}

/// Internal struct for pending suggestions with user info
#[derive(Debug, Clone)]
pub struct PendingSuggestionWithUser {
    pub id: Uuid,
    pub phrase_text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user_display_name: Option<String>,
    pub user_email: Option<String>,
}
