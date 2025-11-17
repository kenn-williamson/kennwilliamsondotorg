use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::events::EventPublisher;
use crate::events::types::{PhraseSuggestionApprovedEvent, PhraseSuggestionRejectedEvent};
use crate::models::api::{PendingSuggestionResponse, PendingSuggestionsResponse};
use crate::repositories::traits::PhraseRepository;

/// Phrase moderation service for admin operations
pub struct PhraseModerationService {
    phrase_repository: Arc<dyn PhraseRepository>,
    event_bus: Option<Arc<dyn EventPublisher>>,
}

/// Builder for PhraseModerationService
pub struct PhraseModerationServiceBuilder {
    phrase_repository: Option<Box<dyn PhraseRepository>>,
    event_bus: Option<Arc<dyn EventPublisher>>,
}

impl Default for PhraseModerationServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PhraseModerationServiceBuilder {
    pub fn new() -> Self {
        Self {
            phrase_repository: None,
            event_bus: None,
        }
    }

    pub fn with_repository(mut self, repository: Box<dyn PhraseRepository>) -> Self {
        self.phrase_repository = Some(repository);
        self
    }

    pub fn with_event_bus(mut self, event_bus: Arc<dyn EventPublisher>) -> Self {
        self.event_bus = Some(event_bus);
        self
    }

    pub fn build(self) -> Result<PhraseModerationService> {
        let phrase_repository = self
            .phrase_repository
            .ok_or_else(|| anyhow::anyhow!("PhraseRepository is required"))?;

        Ok(PhraseModerationService {
            phrase_repository: Arc::from(phrase_repository),
            event_bus: self.event_bus,
        })
    }
}

impl PhraseModerationService {
    /// Create a new builder for PhraseModerationService
    pub fn builder() -> PhraseModerationServiceBuilder {
        PhraseModerationServiceBuilder::new()
    }

    /// Create service with repository only (backward compatibility)
    ///
    /// For new code, prefer using the builder pattern:
    /// ```ignore
    /// PhraseModerationService::builder()
    ///     .with_repository(repository)
    ///     .with_event_bus(event_bus)
    ///     .build()?
    /// ```
    pub fn new(phrase_repository: Box<dyn PhraseRepository>) -> Self {
        Self {
            phrase_repository: Arc::from(phrase_repository),
            event_bus: None,
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
                user_display_name: suggestion
                    .user_display_name
                    .unwrap_or_else(|| "Unknown".to_string()),
                user_email: suggestion
                    .user_email
                    .unwrap_or_else(|| "unknown@example.com".to_string()),
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
        // Fetch suggestion details before approving (needed for event)
        let suggestion = self
            .phrase_repository
            .get_suggestion_by_id(suggestion_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Phrase suggestion not found"))?;

        // Approve in repository
        self.phrase_repository
            .approve_suggestion(suggestion_id, admin_id, admin_reason.clone())
            .await?;

        // Emit event if event bus is configured
        if let Some(event_bus) = &self.event_bus {
            let event = PhraseSuggestionApprovedEvent::new(
                suggestion.user_id,
                suggestion.phrase_text,
                admin_reason,
            );

            if let Err(e) = event_bus.publish(Box::new(event)).await {
                log::error!(
                    "Failed to publish PhraseSuggestionApprovedEvent for suggestion {}: {}",
                    suggestion_id,
                    e
                );
            } else {
                log::debug!(
                    "Published PhraseSuggestionApprovedEvent for suggestion {}",
                    suggestion_id
                );
            }
        }

        Ok(())
    }

    /// Reject a phrase suggestion
    pub async fn reject_suggestion(
        &self,
        suggestion_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        // Fetch suggestion details before rejecting (needed for event)
        let suggestion = self
            .phrase_repository
            .get_suggestion_by_id(suggestion_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Phrase suggestion not found"))?;

        // Reject in repository
        self.phrase_repository
            .reject_suggestion(suggestion_id, admin_id, admin_reason.clone())
            .await?;

        // Emit event if event bus is configured
        if let Some(event_bus) = &self.event_bus {
            let event = PhraseSuggestionRejectedEvent::new(
                suggestion.user_id,
                suggestion.phrase_text,
                admin_reason,
            );

            if let Err(e) = event_bus.publish(Box::new(event)).await {
                log::error!(
                    "Failed to publish PhraseSuggestionRejectedEvent for suggestion {}: {}",
                    suggestion_id,
                    e
                );
            } else {
                log::debug!(
                    "Published PhraseSuggestionRejectedEvent for suggestion {}",
                    suggestion_id
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::MockPhraseRepository;
    use crate::repositories::traits::phrase_repository::PendingSuggestionWithUser;
    use crate::test_utils::PhraseSuggestionBuilder;
    use chrono::Utc;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_pending_suggestions_success() {
        // Setup mocks
        let mut mock_repo = MockPhraseRepository::new();

        // Create test data
        let suggestion = PendingSuggestionWithUser {
            id: Uuid::new_v4(),
            phrase_text: "Test phrase".to_string(),
            created_at: Utc::now(),
            user_display_name: Some("Test User".to_string()),
            user_email: Some("test@example.com".to_string()),
        };

        // Configure mock expectations
        mock_repo
            .expect_get_pending_suggestions()
            .times(1)
            .returning(move || Ok(vec![suggestion.clone()]));

        // Create service
        let service = PhraseModerationService::new(Box::new(mock_repo));

        // Test
        let result = service.get_pending_suggestions().await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.total, 1);
        assert_eq!(response.suggestions.len(), 1);
        assert_eq!(response.suggestions[0].phrase_text, "Test phrase");
        assert_eq!(response.suggestions[0].user_display_name, "Test User");
    }

    #[tokio::test]
    async fn test_get_pending_suggestions_empty() {
        // Setup mocks
        let mut mock_repo = MockPhraseRepository::new();

        // Configure mock expectations
        mock_repo
            .expect_get_pending_suggestions()
            .times(1)
            .returning(|| Ok(vec![]));

        // Create service
        let service = PhraseModerationService::new(Box::new(mock_repo));

        // Test
        let result = service.get_pending_suggestions().await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.total, 0);
        assert_eq!(response.suggestions.len(), 0);
    }

    #[tokio::test]
    async fn test_get_pending_suggestions_repo_error() {
        // Setup mocks
        let mut mock_repo = MockPhraseRepository::new();

        // Configure mock expectations
        mock_repo
            .expect_get_pending_suggestions()
            .times(1)
            .returning(|| Err(anyhow::anyhow!("Database error")));

        // Create service
        let service = PhraseModerationService::new(Box::new(mock_repo));

        // Test
        let result = service.get_pending_suggestions().await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_approve_suggestion_success() {
        // Setup mocks
        let mut mock_repo = MockPhraseRepository::new();
        let suggestion_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();

        // Configure mock expectations
        mock_repo
            .expect_get_suggestion_by_id()
            .with(eq(suggestion_id))
            .times(1)
            .returning(move |id| {
                Ok(Some(
                    PhraseSuggestionBuilder::new()
                        .with_id(id)
                        .with_text("Test phrase suggestion")
                        .build(),
                ))
            });

        mock_repo
            .expect_approve_suggestion()
            .with(
                eq(suggestion_id),
                eq(admin_id),
                eq(Some("Approved".to_string())),
            )
            .times(1)
            .returning(|_, _, _| Ok(()));

        // Create service
        let service = PhraseModerationService::new(Box::new(mock_repo));

        // Test
        let result = service
            .approve_suggestion(suggestion_id, admin_id, Some("Approved".to_string()))
            .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reject_suggestion_success() {
        // Setup mocks
        let mut mock_repo = MockPhraseRepository::new();
        let suggestion_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();

        // Configure mock expectations
        mock_repo
            .expect_get_suggestion_by_id()
            .with(eq(suggestion_id))
            .times(1)
            .returning(move |id| {
                Ok(Some(
                    PhraseSuggestionBuilder::new()
                        .with_id(id)
                        .with_text("Test phrase suggestion")
                        .build(),
                ))
            });

        mock_repo
            .expect_reject_suggestion()
            .with(
                eq(suggestion_id),
                eq(admin_id),
                eq(Some("Rejected".to_string())),
            )
            .times(1)
            .returning(|_, _, _| Ok(()));

        // Create service
        let service = PhraseModerationService::new(Box::new(mock_repo));

        // Test
        let result = service
            .reject_suggestion(suggestion_id, admin_id, Some("Rejected".to_string()))
            .await;

        // Assert
        assert!(result.is_ok());
    }
}
