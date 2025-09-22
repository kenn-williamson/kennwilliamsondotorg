use std::sync::Arc;
use uuid::Uuid;

use crate::models::api::{
    CreatePhraseRequest, PhraseResponse, PhraseSuggestionRequest, 
    UpdatePhraseRequest, UserPhrasesResponse
};
use crate::repositories::traits::PhraseRepository;

pub mod public_access;
pub mod user_management;
pub mod admin_management;
pub mod exclusions;
pub mod suggestions;

pub struct PhraseService {
    repository: Arc<dyn PhraseRepository>,
}

impl PhraseService {
    pub fn new(repository: Box<dyn PhraseRepository>) -> Self {
        Self { 
            repository: Arc::from(repository)
        }
    }

    /// Get a random active phrase for a user by slug, excluding phrases the user has excluded
    pub async fn get_random_phrase_by_slug(&self, user_slug: &str) -> anyhow::Result<String> {
        public_access::get_random_phrase_by_slug(&self.repository, user_slug).await
    }

    /// Get a random active phrase, excluding phrases the user has excluded (for authenticated users)
    pub async fn get_random_phrase(&self, user_id: Uuid) -> anyhow::Result<String> {
        user_management::get_random_phrase(&self.repository, user_id).await
    }

    /// Get all active phrases for a user (excluding their excluded phrases)
    pub async fn get_user_phrases(
        &self, 
        user_id: Uuid, 
        limit: Option<i64>, 
        offset: Option<i64>
    ) -> anyhow::Result<Vec<PhraseResponse>> {
        user_management::get_user_phrases(&self.repository, user_id, limit, offset).await
    }

    /// Get all active phrases for a user with exclusion status (single API call)
    pub async fn get_user_phrases_with_exclusions(&self, user_id: Uuid) -> anyhow::Result<UserPhrasesResponse> {
        user_management::get_user_phrases_with_exclusions(&self.repository, user_id).await
    }

    /// Get all phrases (admin only)
    pub async fn get_all_phrases(
        &self, 
        include_inactive: bool, 
        limit: Option<i64>, 
        offset: Option<i64>
    ) -> anyhow::Result<Vec<PhraseResponse>> {
        admin_management::get_all_phrases(&self.repository, include_inactive, limit, offset).await
    }

    /// Create a new phrase (admin only)
    pub async fn create_phrase(&self, request: CreatePhraseRequest, created_by: Uuid) -> anyhow::Result<PhraseResponse> {
        admin_management::create_phrase(&self.repository, request, created_by).await
    }

    /// Update a phrase (admin only)
    pub async fn update_phrase(&self, phrase_id: Uuid, request: UpdatePhraseRequest) -> anyhow::Result<PhraseResponse> {
        admin_management::update_phrase(&self.repository, phrase_id, request).await
    }

    /// Exclude a phrase for a user
    pub async fn exclude_phrase_for_user(&self, user_id: Uuid, phrase_id: Uuid) -> anyhow::Result<()> {
        exclusions::exclude_phrase_for_user(&self.repository, user_id, phrase_id).await
    }

    /// Remove phrase exclusion for a user
    pub async fn remove_phrase_exclusion(&self, user_id: Uuid, phrase_id: Uuid) -> anyhow::Result<()> {
        exclusions::remove_phrase_exclusion(&self.repository, user_id, phrase_id).await
    }

    /// Get user's excluded phrases
    pub async fn get_user_excluded_phrases(&self, user_id: Uuid) -> anyhow::Result<Vec<(Uuid, String, chrono::DateTime<chrono::Utc>)>> {
        self.repository.get_user_excluded_phrases(user_id).await
    }

    /// Submit a phrase suggestion
    pub async fn submit_phrase_suggestion(&self, user_id: Uuid, request: PhraseSuggestionRequest) -> anyhow::Result<crate::models::db::PhraseSuggestion> {
        suggestions::submit_phrase_suggestion(&self.repository, user_id, request).await
    }

    /// Get user's phrase suggestions
    pub async fn get_user_suggestions(&self, user_id: Uuid) -> anyhow::Result<Vec<crate::models::db::PhraseSuggestion>> {
        self.repository.get_user_suggestions(user_id).await
    }
}
