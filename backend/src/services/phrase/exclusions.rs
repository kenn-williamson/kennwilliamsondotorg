use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::repositories::traits::PhraseRepository;

/// Exclude a phrase for a user
pub async fn exclude_phrase_for_user(
    repository: &Arc<dyn PhraseRepository>,
    user_id: Uuid,
    phrase_id: Uuid,
) -> Result<()> {
    // Validate input
    if user_id.is_nil() {
        return Err(anyhow::anyhow!("User ID cannot be nil"));
    }
    if phrase_id.is_nil() {
        return Err(anyhow::anyhow!("Phrase ID cannot be nil"));
    }

    // Exclude phrase in repository (handles duplicate exclusion gracefully)
    repository
        .exclude_phrase_for_user(user_id, phrase_id)
        .await?;

    Ok(())
}

/// Remove phrase exclusion for a user
pub async fn remove_phrase_exclusion(
    repository: &Arc<dyn PhraseRepository>,
    user_id: Uuid,
    phrase_id: Uuid,
) -> Result<()> {
    // Validate input
    if user_id.is_nil() {
        return Err(anyhow::anyhow!("User ID cannot be nil"));
    }
    if phrase_id.is_nil() {
        return Err(anyhow::anyhow!("Phrase ID cannot be nil"));
    }

    // Remove phrase exclusion in repository
    repository
        .remove_phrase_exclusion(user_id, phrase_id)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::traits::PhraseRepository;
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

    #[tokio::test]
    async fn test_exclude_phrase_for_user_success() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        let phrase_id = Uuid::new_v4();

        mock_repo
            .expect_exclude_phrase_for_user()
            .with(
                mockall::predicate::eq(user_id),
                mockall::predicate::eq(phrase_id),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = exclude_phrase_for_user(&repo, user_id, phrase_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_exclude_phrase_for_user_nil_user_id() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = exclude_phrase_for_user(&repo, Uuid::nil(), Uuid::new_v4()).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("User ID cannot be nil"));
    }

    #[tokio::test]
    async fn test_exclude_phrase_for_user_nil_phrase_id() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = exclude_phrase_for_user(&repo, Uuid::new_v4(), Uuid::nil()).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Phrase ID cannot be nil"));
    }

    #[tokio::test]
    async fn test_remove_phrase_exclusion_success() {
        let mut mock_repo = MockPhraseRepository::new();
        let user_id = Uuid::new_v4();
        let phrase_id = Uuid::new_v4();

        mock_repo
            .expect_remove_phrase_exclusion()
            .with(
                mockall::predicate::eq(user_id),
                mockall::predicate::eq(phrase_id),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = remove_phrase_exclusion(&repo, user_id, phrase_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_phrase_exclusion_nil_user_id() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = remove_phrase_exclusion(&repo, Uuid::nil(), Uuid::new_v4()).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("User ID cannot be nil"));
    }

    #[tokio::test]
    async fn test_remove_phrase_exclusion_nil_phrase_id() {
        let mock_repo = MockPhraseRepository::new();
        let repo = Arc::new(mock_repo) as Arc<dyn PhraseRepository>;
        let result = remove_phrase_exclusion(&repo, Uuid::new_v4(), Uuid::nil()).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Phrase ID cannot be nil"));
    }
}
