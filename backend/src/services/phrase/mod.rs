use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::events::EventPublisher;
use crate::models::api::{
    CreatePhraseRequest, PhraseResponse, PhraseSuggestionRequest, UpdatePhraseRequest,
    UserPhrasesResponse,
};
use crate::repositories::traits::PhraseRepository;

pub mod admin_management;
pub mod exclusions;
pub mod public_access;
pub mod suggestions;
pub mod user_management;

pub struct PhraseService {
    repository: Arc<dyn PhraseRepository>,
    event_bus: Option<Arc<dyn EventPublisher>>,
}

/// Builder for PhraseService
pub struct PhraseServiceBuilder {
    repository: Option<Box<dyn PhraseRepository>>,
    event_bus: Option<Arc<dyn EventPublisher>>,
}

impl PhraseServiceBuilder {
    pub fn new() -> Self {
        Self {
            repository: None,
            event_bus: None,
        }
    }

    pub fn with_repository(mut self, repository: Box<dyn PhraseRepository>) -> Self {
        self.repository = Some(repository);
        self
    }

    pub fn with_event_bus(mut self, event_bus: Arc<dyn EventPublisher>) -> Self {
        self.event_bus = Some(event_bus);
        self
    }

    pub fn build(self) -> Result<PhraseService> {
        let repository = self.repository
            .ok_or_else(|| anyhow::anyhow!("PhraseRepository is required"))?;

        Ok(PhraseService {
            repository: Arc::from(repository),
            event_bus: self.event_bus,
        })
    }
}

impl PhraseService {
    /// Create a new builder for PhraseService
    pub fn builder() -> PhraseServiceBuilder {
        PhraseServiceBuilder::new()
    }

    /// Create service with repository only (backward compatibility)
    ///
    /// For new code, prefer using the builder pattern:
    /// ```ignore
    /// PhraseService::builder()
    ///     .with_repository(repository)
    ///     .with_event_bus(event_bus)
    ///     .build()?
    /// ```
    pub fn new(repository: Box<dyn PhraseRepository>) -> Self {
        Self {
            repository: Arc::from(repository),
            event_bus: None,
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
        offset: Option<i64>,
    ) -> anyhow::Result<Vec<PhraseResponse>> {
        user_management::get_user_phrases(&self.repository, user_id, limit, offset).await
    }

    /// Get all active phrases for a user with exclusion status (single API call)
    pub async fn get_user_phrases_with_exclusions(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
        search: Option<String>,
    ) -> anyhow::Result<UserPhrasesResponse> {
        user_management::get_user_phrases_with_exclusions(
            &self.repository,
            user_id,
            limit,
            offset,
            search,
        )
        .await
    }

    /// Get phrases (admin only)
    pub async fn get_phrases(
        &self,
        include_inactive: bool,
        limit: Option<i64>,
        offset: Option<i64>,
        search: Option<String>,
    ) -> anyhow::Result<Vec<PhraseResponse>> {
        admin_management::get_phrases(&self.repository, include_inactive, limit, offset, search)
            .await
    }

    /// Create a new phrase (admin only)
    pub async fn create_phrase(
        &self,
        request: CreatePhraseRequest,
        created_by: Uuid,
    ) -> anyhow::Result<PhraseResponse> {
        admin_management::create_phrase(&self.repository, request, created_by).await
    }

    /// Update a phrase (admin only)
    pub async fn update_phrase(
        &self,
        phrase_id: Uuid,
        request: UpdatePhraseRequest,
    ) -> anyhow::Result<PhraseResponse> {
        admin_management::update_phrase(&self.repository, phrase_id, request).await
    }

    /// Exclude a phrase for a user
    pub async fn exclude_phrase_for_user(
        &self,
        user_id: Uuid,
        phrase_id: Uuid,
    ) -> anyhow::Result<()> {
        exclusions::exclude_phrase_for_user(&self.repository, user_id, phrase_id).await
    }

    /// Remove phrase exclusion for a user
    pub async fn remove_phrase_exclusion(
        &self,
        user_id: Uuid,
        phrase_id: Uuid,
    ) -> anyhow::Result<()> {
        exclusions::remove_phrase_exclusion(&self.repository, user_id, phrase_id).await
    }

    /// Get user's excluded phrases
    pub async fn get_user_excluded_phrases(
        &self,
        user_id: Uuid,
    ) -> anyhow::Result<Vec<(Uuid, String, chrono::DateTime<chrono::Utc>)>> {
        self.repository.get_user_excluded_phrases(user_id).await
    }

    /// Submit a phrase suggestion
    ///
    /// # Arguments
    /// * `user_id` - ID of the user submitting the suggestion
    /// * `request` - The phrase suggestion request
    ///
    /// # Domain Events
    /// Emits `PhraseSuggestionCreatedEvent` if EventBus is configured.
    /// Event handlers are responsible for fetching user details as needed.
    pub async fn submit_phrase_suggestion(
        &self,
        user_id: Uuid,
        request: PhraseSuggestionRequest,
    ) -> anyhow::Result<crate::models::db::PhraseSuggestion> {
        // Submit suggestion to repository
        let suggestion = suggestions::submit_phrase_suggestion(&self.repository, user_id, request).await?;

        // Emit domain event if EventBus is configured
        if let Some(event_bus) = &self.event_bus {
            let event = crate::events::types::PhraseSuggestionCreatedEvent::new(
                user_id,
                &suggestion.phrase_text,
            );

            // Fire-and-forget event publishing
            if let Err(e) = event_bus.publish(Box::new(event)).await {
                log::error!("Failed to publish PhraseSuggestionCreatedEvent: {}", e);
            } else {
                log::debug!("Published PhraseSuggestionCreatedEvent for user_id {}", user_id);
            }
        }

        Ok(suggestion)
    }

    /// Get user's phrase suggestions
    pub async fn get_user_suggestions(
        &self,
        user_id: Uuid,
    ) -> anyhow::Result<Vec<crate::models::db::PhraseSuggestion>> {
        self.repository.get_user_suggestions(user_id).await
    }
}
