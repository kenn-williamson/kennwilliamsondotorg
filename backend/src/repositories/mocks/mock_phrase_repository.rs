use mockall::mock;
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::repositories::traits::PhraseRepository;
use crate::models::api::{
    CreatePhraseRequest, UpdatePhraseRequest, PhraseSuggestionRequest
};
use crate::models::db::{Phrase, PhraseSuggestion, PhraseWithUserExclusionView};

// Generate mock for PhraseRepository trait
mock! {
    pub PhraseRepository {}
    
    #[async_trait]
    impl PhraseRepository for PhraseRepository {
        async fn get_random_phrase_by_slug(&self, user_slug: &str) -> Result<String>;
        async fn get_random_phrase(&self, user_id: Uuid) -> Result<String>;
        async fn get_user_phrases(
            &self, 
            user_id: Uuid, 
            limit: Option<i64>, 
            offset: Option<i64>
        ) -> Result<Vec<Phrase>>;
        async fn get_user_phrases_with_exclusions(&self, user_id: Uuid) -> Result<Vec<PhraseWithUserExclusionView>>;
        async fn get_all_phrases(
            &self, 
            include_inactive: bool, 
            limit: Option<i64>, 
            offset: Option<i64>
        ) -> Result<Vec<Phrase>>;
        async fn create_phrase(&self, request: CreatePhraseRequest, created_by: Uuid) -> Result<Phrase>;
        async fn update_phrase(&self, phrase_id: Uuid, request: UpdatePhraseRequest) -> Result<Phrase>;
        async fn exclude_phrase_for_user(&self, user_id: Uuid, phrase_id: Uuid) -> Result<()>;
        async fn remove_phrase_exclusion(&self, user_id: Uuid, phrase_id: Uuid) -> Result<()>;
        async fn get_user_excluded_phrases(&self, user_id: Uuid) -> Result<Vec<(Uuid, String, chrono::DateTime<chrono::Utc>)>>;
        async fn submit_phrase_suggestion(&self, user_id: Uuid, request: PhraseSuggestionRequest) -> Result<PhraseSuggestion>;
        async fn get_user_suggestions(&self, user_id: Uuid) -> Result<Vec<PhraseSuggestion>>;
        async fn get_pending_suggestions(&self) -> Result<Vec<crate::repositories::traits::phrase_repository::PendingSuggestionWithUser>>;
        async fn approve_suggestion(
            &self,
            suggestion_id: Uuid,
            admin_id: Uuid,
            admin_reason: Option<String>,
        ) -> Result<()>;
        async fn reject_suggestion(
            &self,
            suggestion_id: Uuid,
            admin_id: Uuid,
            admin_reason: Option<String>,
        ) -> Result<()>;
        
        async fn count_all_phrases(&self) -> Result<i64>;
        
        async fn count_pending_suggestions(&self) -> Result<i64>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;
    use mockall::predicate::eq;

    // Helper function to create a test phrase
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

    // Helper function to create test phrase suggestion
    fn create_test_phrase_suggestion() -> PhraseSuggestion {
        PhraseSuggestion {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            phrase_text: "Test suggestion".to_string(),
            status: "pending".to_string(),
            admin_id: None,
            admin_reason: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_mock_get_random_phrase() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        
        // Setup mock expectation
        mock_repo
            .expect_get_random_phrase()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok("Test phrase".to_string()));
        
        // Test the mock
        let result = mock_repo.get_random_phrase(user_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test phrase");
    }

    #[tokio::test]
    async fn test_mock_get_random_phrase_by_slug() {
        let mut mock_repo = MockPhraseRepository::new();
        
        // Setup mock expectation
        mock_repo
            .expect_get_random_phrase_by_slug()
            .times(1)
            .with(eq("test-user"))
            .returning(|_| Ok("Test phrase for user".to_string()));
        
        // Test the mock
        let result = mock_repo.get_random_phrase_by_slug("test-user").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test phrase for user");
    }

    #[tokio::test]
    async fn test_mock_get_user_phrases() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        
        // Setup mock expectation
        mock_repo
            .expect_get_user_phrases()
            .times(1)
            .with(eq(user_id), eq(Some(10)), eq(Some(0)))
            .returning(|_, _, _| Ok(vec![create_test_phrase()]));
        
        // Test the mock
        let result = mock_repo.get_user_phrases(user_id, Some(10), Some(0)).await;
        assert!(result.is_ok());
        let phrases = result.unwrap();
        assert_eq!(phrases.len(), 1);
        assert_eq!(phrases[0].phrase_text, "Test phrase");
    }

    #[tokio::test]
    async fn test_mock_submit_phrase_suggestion() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        let request = PhraseSuggestionRequest {
            phrase_text: "New suggestion".to_string(),
        };
        
        // Setup mock expectation
        mock_repo
            .expect_submit_phrase_suggestion()
            .times(1)
            .with(eq(user_id), eq(request.clone()))
            .returning(|_, _| Ok(create_test_phrase_suggestion()));
        
        // Test the mock
        let result = mock_repo.submit_phrase_suggestion(user_id, request).await;
        assert!(result.is_ok());
        let suggestion = result.unwrap();
        assert_eq!(suggestion.phrase_text, "Test suggestion");
    }

    #[tokio::test]
    async fn test_mock_error_handling() {
        let mut mock_repo = MockPhraseRepository::new();
        
        // Setup mock to return an error
        mock_repo
            .expect_get_random_phrase()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database connection failed")));
        
        // Test error handling
        let result = mock_repo.get_random_phrase(Uuid::new_v4()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database connection failed"));
    }
}
