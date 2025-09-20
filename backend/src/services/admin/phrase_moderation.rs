use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;

use crate::models::api::{PendingSuggestionResponse, PendingSuggestionsResponse};
use crate::repositories::traits::PhraseRepository;

/// Phrase moderation service for admin operations
pub struct PhraseModerationService {
    phrase_repository: Arc<dyn PhraseRepository>,
}

impl PhraseModerationService {
    pub fn new(phrase_repository: Box<dyn PhraseRepository>) -> Self {
        Self {
            phrase_repository: Arc::from(phrase_repository),
        }
    }

    /// Get all pending phrase suggestions
    pub async fn get_pending_suggestions(&self) -> Result<PendingSuggestionsResponse> {
        // Get pending suggestions from repository
        let suggestions = self.phrase_repository.get_pending_suggestions().await?;
        
        // Convert to response format
        let pending_suggestions: Vec<PendingSuggestionResponse> = suggestions
            .into_iter()
            .map(|suggestion| PendingSuggestionResponse {
                id: suggestion.id,
                phrase_text: suggestion.phrase_text,
                created_at: suggestion.created_at,
                user_display_name: suggestion.user_display_name.unwrap_or_else(|| "Unknown".to_string()),
                user_email: suggestion.user_email.unwrap_or_else(|| "unknown@example.com".to_string()),
            })
            .collect();

        let total = pending_suggestions.len() as i64;

        Ok(PendingSuggestionsResponse {
            suggestions: pending_suggestions,
            total,
        })
    }

    /// Approve a phrase suggestion
    pub async fn approve_suggestion(
        &self,
        suggestion_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        self.phrase_repository
            .approve_suggestion(suggestion_id, admin_id, admin_reason)
            .await
    }

    /// Reject a phrase suggestion
    pub async fn reject_suggestion(
        &self,
        suggestion_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        self.phrase_repository
            .reject_suggestion(suggestion_id, admin_id, admin_reason)
            .await
    }
}
