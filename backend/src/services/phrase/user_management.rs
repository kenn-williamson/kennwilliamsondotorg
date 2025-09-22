use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;

use crate::repositories::traits::PhraseRepository;
use crate::models::api::{PhraseResponse, UserPhrasesResponse, PhraseWithExclusion as ApiPhraseWithExclusion};

/// Get a random active phrase, excluding phrases the user has excluded (for authenticated users)
pub async fn get_random_phrase(
    repository: &Arc<dyn PhraseRepository>,
    user_id: Uuid,
) -> Result<String> {
    // Validate input
    if user_id.is_nil() {
        return Err(anyhow::anyhow!("User ID cannot be nil"));
    }

    // Get phrase from repository
    let phrase_text = repository.get_random_phrase(user_id).await?;
    
    // Handle empty result case
    if phrase_text.trim().is_empty() {
        return Err(anyhow::anyhow!("No phrases available for user"));
    }

    Ok(phrase_text)
}

/// Get all active phrases for a user (excluding their excluded phrases)
pub async fn get_user_phrases(
    repository: &Arc<dyn PhraseRepository>,
    user_id: Uuid,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<PhraseResponse>> {
    // Validate input
    if user_id.is_nil() {
        return Err(anyhow::anyhow!("User ID cannot be nil"));
    }

    // Validate pagination parameters
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    
    if limit < 0 {
        return Err(anyhow::anyhow!("Limit cannot be negative"));
    }
    if offset < 0 {
        return Err(anyhow::anyhow!("Offset cannot be negative"));
    }

    // Get phrases from repository
    let phrases = repository.get_user_phrases(user_id, Some(limit), Some(offset)).await?;
    
    // Transform to API response format
    let phrase_responses: Vec<PhraseResponse> = phrases
        .into_iter()
        .map(PhraseResponse::from)
        .collect();

    Ok(phrase_responses)
}

/// Get all active phrases for a user with exclusion status (single API call)
pub async fn get_user_phrases_with_exclusions(
    repository: &Arc<dyn PhraseRepository>,
    user_id: Uuid,
) -> Result<UserPhrasesResponse> {
    // Validate input
    if user_id.is_nil() {
        return Err(anyhow::anyhow!("User ID cannot be nil"));
    }

    // Get phrases with exclusion status from repository
    let phrases_with_exclusions = repository.get_user_phrases_with_exclusions(user_id).await?;
    
    // Transform to API response format
    let phrases: Vec<ApiPhraseWithExclusion> = phrases_with_exclusions
        .into_iter()
        .map(|item| ApiPhraseWithExclusion {
            id: item.phrase.id,
            phrase_text: item.phrase.phrase_text,
            active: item.phrase.active,
            created_by: item.phrase.created_by,
            created_at: item.phrase.created_at,
            updated_at: item.phrase.updated_at,
            is_excluded: item.is_excluded,
        })
        .collect();

    let total = phrases.len() as i64;

    Ok(UserPhrasesResponse {
        phrases,
        total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use mockall::mock;
    use crate::repositories::traits::PhraseRepository;
    use crate::models::db::{Phrase, PhraseWithUserExclusionView};
    use chrono::Utc;
    use async_trait::async_trait;

    mock! {
        PhraseRepository {}

        #[async_trait]
        impl PhraseRepository for PhraseRepository {
            async fn get_random_phrase_by_slug(&self, user_slug: &str) -> Result<String>;
            async fn get_random_phrase(&self, user_id: uuid::Uuid) -> Result<String>;
            async fn get_user_phrases(&self, user_id: uuid::Uuid, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<crate::models::db::Phrase>>;
            async fn get_user_phrases_with_exclusions(&self, user_id: uuid::Uuid) -> Result<Vec<crate::models::db::PhraseWithUserExclusionView>>;
            async fn get_all_phrases(&self, include_inactive: bool, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<crate::models::db::Phrase>>;
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
    async fn test_get_random_phrase_success() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        mock_repo
            .expect_get_random_phrase()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok("Test phrase".to_string()));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = get_random_phrase(&repo, user_id).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test phrase");
    }

    #[tokio::test]
    async fn test_get_random_phrase_nil_user_id() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = get_random_phrase(&repo, Uuid::nil()).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("User ID cannot be nil"));
    }

    #[tokio::test]
    async fn test_get_user_phrases_success() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        let test_phrase = create_test_phrase();
        
        mock_repo
            .expect_get_user_phrases()
            .with(mockall::predicate::eq(user_id), mockall::predicate::eq(Some(50)), mockall::predicate::eq(Some(0)))
            .times(1)
            .returning(move |_, _, _| Ok(vec![test_phrase.clone()]));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = get_user_phrases(&repo, user_id, Some(50), Some(0)).await;
        
        assert!(result.is_ok());
        let phrases = result.unwrap();
        assert_eq!(phrases.len(), 1);
        assert_eq!(phrases[0].phrase_text, "Test phrase");
    }

    #[tokio::test]
    async fn test_get_user_phrases_negative_limit() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = get_user_phrases(&repo, Uuid::new_v4(), Some(-1), Some(0)).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Limit cannot be negative"));
    }

    #[tokio::test]
    async fn test_get_user_phrases_with_exclusions_success() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        let test_phrase = create_test_phrase();
        
        let phrase_with_exclusion = PhraseWithUserExclusionView {
            phrase: test_phrase.clone(),
            is_excluded: false,
        };
        
        mock_repo
            .expect_get_user_phrases_with_exclusions()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(vec![phrase_with_exclusion.clone()]));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = get_user_phrases_with_exclusions(&repo, user_id).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.phrases.len(), 1);
        assert_eq!(response.total, 1);
        assert!(!response.phrases[0].is_excluded);
    }

    #[tokio::test]
    async fn test_get_user_phrases_with_exclusions_nil_user_id() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = get_user_phrases_with_exclusions(&repo, Uuid::nil()).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("User ID cannot be nil"));
    }
}
