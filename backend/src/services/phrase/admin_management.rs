use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::api::{CreatePhraseRequest, PhraseResponse, UpdatePhraseRequest};
use crate::repositories::traits::PhraseRepository;

/// Get phrases (admin only)
pub async fn get_phrases(
    repository: &Arc<dyn PhraseRepository>,
    include_inactive: bool,
    limit: Option<i64>,
    offset: Option<i64>,
    search: Option<String>,
) -> Result<Vec<PhraseResponse>> {
    // Validate pagination parameters
    let limit = limit.unwrap_or(100); // Reasonable limit to prevent performance issues
    let offset = offset.unwrap_or(0);

    if limit < 0 {
        return Err(anyhow::anyhow!("Limit cannot be negative"));
    }
    if offset < 0 {
        return Err(anyhow::anyhow!("Offset cannot be negative"));
    }

    // Get phrases from repository
    let phrases = repository
        .get_phrases(include_inactive, Some(limit), Some(offset), search)
        .await?;

    // Transform to API response format
    let phrase_responses: Vec<PhraseResponse> =
        phrases.into_iter().map(PhraseResponse::from).collect();

    Ok(phrase_responses)
}

/// Create a new phrase (admin only)
pub async fn create_phrase(
    repository: &Arc<dyn PhraseRepository>,
    request: CreatePhraseRequest,
    created_by: Uuid,
) -> Result<PhraseResponse> {
    // Validate input
    if created_by.is_nil() {
        return Err(anyhow::anyhow!("Created by user ID cannot be nil"));
    }

    // Validate phrase text
    if request.phrase_text.trim().is_empty() {
        return Err(anyhow::anyhow!("Phrase text cannot be empty"));
    }

    if request.phrase_text.len() > 500 {
        return Err(anyhow::anyhow!("Phrase text cannot exceed 500 characters"));
    }

    // Create phrase in repository
    let phrase = repository.create_phrase(request, created_by).await?;

    // Transform to API response format
    Ok(PhraseResponse::from(phrase))
}

/// Update a phrase (admin only)
pub async fn update_phrase(
    repository: &Arc<dyn PhraseRepository>,
    phrase_id: Uuid,
    request: UpdatePhraseRequest,
) -> Result<PhraseResponse> {
    // Validate input
    if phrase_id.is_nil() {
        return Err(anyhow::anyhow!("Phrase ID cannot be nil"));
    }

    // Validate phrase text if provided
    if let Some(ref phrase_text) = request.phrase_text {
        if phrase_text.trim().is_empty() {
            return Err(anyhow::anyhow!("Phrase text cannot be empty"));
        }
        if phrase_text.len() > 500 {
            return Err(anyhow::anyhow!("Phrase text cannot exceed 500 characters"));
        }
    }

    // Update phrase in repository
    let phrase = repository.update_phrase(phrase_id, request).await?;

    // Transform to API response format
    Ok(PhraseResponse::from(phrase))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::Phrase;
    use crate::repositories::traits::PhraseRepository;
    use async_trait::async_trait;
    use chrono::Utc;
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
            async fn get_pending_suggestions(&self) -> Result<Vec<crate::repositories::traits::phrase_repository::PendingSuggestionWithUser>>;
            async fn approve_suggestion(&self, suggestion_id: uuid::Uuid, admin_id: uuid::Uuid, admin_reason: Option<String>) -> Result<()>;
            async fn reject_suggestion(&self, suggestion_id: uuid::Uuid, admin_id: uuid::Uuid, admin_reason: Option<String>) -> Result<()>;
            async fn count_all_phrases(&self) -> Result<i64>;
            async fn count_pending_suggestions(&self) -> Result<i64>;
        }
    }

    fn create_test_phrase() -> Phrase {
        Phrase {
            id: Uuid::new_v4(),
            phrase_text: "Test phrase".to_string(),
            active: true,
            created_by: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_get_all_phrases_success() {
        let mut mock_repo = MockPhraseRepository::new();
        let test_phrase = create_test_phrase();

        mock_repo
            .expect_get_phrases()
            .with(
                mockall::predicate::eq(false),
                mockall::predicate::eq(Some(50)),
                mockall::predicate::eq(Some(0)),
                mockall::predicate::eq(None),
            )
            .times(1)
            .returning(move |_, _, _, _| Ok(vec![test_phrase.clone()]));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = get_phrases(&repo, false, Some(50), Some(0), None).await;

        assert!(result.is_ok());
        let phrases = result.unwrap();
        assert_eq!(phrases.len(), 1);
        assert_eq!(phrases[0].phrase_text, "Test phrase");
    }

    #[tokio::test]
    async fn test_get_all_phrases_negative_limit() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = get_phrases(&repo, false, Some(-1), Some(0), None).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Limit cannot be negative"));
    }

    #[tokio::test]
    async fn test_create_phrase_success() {
        let mut mock_repo = MockPhraseRepository::new();
        let created_by = Uuid::new_v4();
        let test_phrase = create_test_phrase();
        let request = CreatePhraseRequest {
            phrase_text: "New phrase".to_string(),
            active: Some(true),
        };

        mock_repo
            .expect_create_phrase()
            .with(
                mockall::predicate::eq(request.clone()),
                mockall::predicate::eq(created_by),
            )
            .times(1)
            .returning(move |_, _| Ok(test_phrase.clone()));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = create_phrase(&repo, request, created_by).await;

        assert!(result.is_ok());
        let phrase = result.unwrap();
        assert_eq!(phrase.phrase_text, "Test phrase");
    }

    #[tokio::test]
    async fn test_create_phrase_empty_text() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let request = CreatePhraseRequest {
            phrase_text: "".to_string(),
            active: Some(true),
        };
        let result = create_phrase(&repo, request, Uuid::new_v4()).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Phrase text cannot be empty"));
    }

    #[tokio::test]
    async fn test_create_phrase_too_long() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let request = CreatePhraseRequest {
            phrase_text: "a".repeat(501),
            active: Some(true),
        };
        let result = create_phrase(&repo, request, Uuid::new_v4()).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Phrase text cannot exceed 500 characters"));
    }

    #[tokio::test]
    async fn test_update_phrase_success() {
        let mut mock_repo = MockPhraseRepository::new();
        let phrase_id = Uuid::new_v4();
        let test_phrase = create_test_phrase();
        let request = UpdatePhraseRequest {
            phrase_text: Some("Updated phrase".to_string()),
            active: Some(false),
        };

        mock_repo
            .expect_update_phrase()
            .with(
                mockall::predicate::eq(phrase_id),
                mockall::predicate::eq(request.clone()),
            )
            .times(1)
            .returning(move |_, _| Ok(test_phrase.clone()));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = update_phrase(&repo, phrase_id, request).await;

        assert!(result.is_ok());
        let phrase = result.unwrap();
        assert_eq!(phrase.phrase_text, "Test phrase");
    }

    #[tokio::test]
    async fn test_update_phrase_nil_id() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let request = UpdatePhraseRequest {
            phrase_text: Some("Updated phrase".to_string()),
            active: Some(false),
        };
        let result = update_phrase(&repo, Uuid::nil(), request).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Phrase ID cannot be nil"));
    }
}
