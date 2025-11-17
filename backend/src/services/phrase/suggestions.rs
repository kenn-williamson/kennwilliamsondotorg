use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::api::PhraseSuggestionRequest;
use crate::models::db::PhraseSuggestion;
use crate::repositories::traits::PhraseRepository;

/// Submit a phrase suggestion
pub async fn submit_phrase_suggestion(
    repository: &Arc<dyn PhraseRepository>,
    user_id: Uuid,
    request: PhraseSuggestionRequest,
) -> Result<PhraseSuggestion> {
    // Validate input
    if user_id.is_nil() {
        return Err(anyhow::anyhow!("User ID cannot be nil"));
    }

    // Validate phrase text
    if request.phrase_text.trim().is_empty() {
        return Err(anyhow::anyhow!("Phrase text cannot be empty"));
    }

    if request.phrase_text.len() > 500 {
        return Err(anyhow::anyhow!("Phrase text cannot exceed 500 characters"));
    }

    // Sanitize phrase text
    let sanitized_text = sanitize_phrase_text(&request.phrase_text);

    // Create sanitized request
    let sanitized_request = PhraseSuggestionRequest {
        phrase_text: sanitized_text,
    };

    // Submit suggestion to repository
    let suggestion = repository
        .submit_phrase_suggestion(user_id, sanitized_request)
        .await?;

    Ok(suggestion)
}

/// Sanitize phrase text by trimming whitespace and normalizing
fn sanitize_phrase_text(text: &str) -> String {
    text.trim()
        .chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::PhraseSuggestion;
    use crate::repositories::traits::PhraseRepository;
    use crate::test_utils::PhraseSuggestionBuilder;
    use async_trait::async_trait;
    use mockall::mock;
    use std::sync::Arc;

    mock! {
        PhraseRepository {}

        #[async_trait]
        impl PhraseRepository for PhraseRepository {
            async fn get_random_phrase_by_slug(&self, user_slug: &str) -> Result<String>;
            async fn get_random_phrase(&self, user_id: uuid::Uuid) -> Result<String>;
            async fn get_user_phrases(&self, user_id: uuid::Uuid, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<crate::models::db::Phrase>>;
            async fn get_user_phrases_with_exclusions(&self, user_id: uuid::Uuid, limit: Option<i64>, offset: Option<i64>, search: Option<String>) -> Result<Vec<crate::models::db::PhraseSearchResultWithUserExclusionView>>;
            async fn get_phrases(&self, include_inactive: bool, limit: Option<i64>, offset: Option<i64>, search: Option<String>) -> Result<Vec<crate::models::db::Phrase>>;
            async fn create_phrase(&self, request: crate::models::api::CreatePhraseRequest, created_by: uuid::Uuid) -> Result<crate::models::db::Phrase>;
            async fn update_phrase(&self, phrase_id: uuid::Uuid, request: crate::models::api::UpdatePhraseRequest) -> Result<crate::models::db::Phrase>;
            async fn exclude_phrase_for_user(&self, user_id: uuid::Uuid, phrase_id: uuid::Uuid) -> Result<()>;
            async fn remove_phrase_exclusion(&self, user_id: uuid::Uuid, phrase_id: uuid::Uuid) -> Result<()>;
            async fn get_user_excluded_phrases(&self, user_id: uuid::Uuid) -> Result<Vec<(uuid::Uuid, String, chrono::DateTime<chrono::Utc>)>>;
            async fn submit_phrase_suggestion(&self, user_id: uuid::Uuid, request: crate::models::api::PhraseSuggestionRequest) -> Result<crate::models::db::PhraseSuggestion>;
            async fn get_user_suggestions(&self, user_id: uuid::Uuid) -> Result<Vec<crate::models::db::PhraseSuggestion>>;
            async fn get_suggestion_by_id(&self, suggestion_id: uuid::Uuid) -> Result<Option<crate::models::db::PhraseSuggestion>>;
            async fn get_pending_suggestions(&self) -> Result<Vec<crate::repositories::traits::phrase_repository::PendingSuggestionWithUser>>;
            async fn approve_suggestion(&self, suggestion_id: uuid::Uuid, admin_id: uuid::Uuid, admin_reason: Option<String>) -> Result<()>;
            async fn reject_suggestion(&self, suggestion_id: uuid::Uuid, admin_id: uuid::Uuid, admin_reason: Option<String>) -> Result<()>;
            async fn count_all_phrases(&self) -> Result<i64>;
            async fn count_pending_suggestions(&self) -> Result<i64>;
        }
    }

    fn create_test_suggestion() -> PhraseSuggestion {
        PhraseSuggestionBuilder::new()
            .with_text("Test suggestion")
            .pending()
            .build()
    }

    #[tokio::test]
    async fn test_submit_phrase_suggestion_success() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        let request = PhraseSuggestionRequest {
            phrase_text: "Test suggestion".to_string(),
        };
        let test_suggestion = create_test_suggestion();

        mock_repo
            .expect_submit_phrase_suggestion()
            .with(
                mockall::predicate::eq(user_id),
                mockall::predicate::eq(request.clone()),
            )
            .times(1)
            .returning(move |_, _| Ok(test_suggestion.clone()));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = submit_phrase_suggestion(&repo, user_id, request).await;

        assert!(result.is_ok());
        let suggestion = result.unwrap();
        assert_eq!(suggestion.phrase_text, "Test suggestion");
    }

    #[tokio::test]
    async fn test_submit_phrase_suggestion_nil_user_id() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let request = PhraseSuggestionRequest {
            phrase_text: "Test suggestion".to_string(),
        };
        let result = submit_phrase_suggestion(&repo, Uuid::nil(), request).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("User ID cannot be nil")
        );
    }

    #[tokio::test]
    async fn test_submit_phrase_suggestion_empty_text() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let request = PhraseSuggestionRequest {
            phrase_text: "".to_string(),
        };
        let result = submit_phrase_suggestion(&repo, Uuid::new_v4(), request).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Phrase text cannot be empty")
        );
    }

    #[tokio::test]
    async fn test_submit_phrase_suggestion_too_long() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let request = PhraseSuggestionRequest {
            phrase_text: "a".repeat(501),
        };
        let result = submit_phrase_suggestion(&repo, Uuid::new_v4(), request).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Phrase text cannot exceed 500 characters")
        );
    }

    #[tokio::test]
    async fn test_submit_phrase_suggestion_sanitization() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        let request = PhraseSuggestionRequest {
            phrase_text: "  Test   suggestion  ".to_string(),
        };
        let test_suggestion = create_test_suggestion();

        // Expect sanitized text (trimmed and normalized whitespace)
        let expected_request = PhraseSuggestionRequest {
            phrase_text: "Test suggestion".to_string(),
        };

        mock_repo
            .expect_submit_phrase_suggestion()
            .with(
                mockall::predicate::eq(user_id),
                mockall::predicate::eq(expected_request),
            )
            .times(1)
            .returning(move |_, _| Ok(test_suggestion.clone()));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = submit_phrase_suggestion(&repo, user_id, request).await;

        assert!(result.is_ok());
    }
}
