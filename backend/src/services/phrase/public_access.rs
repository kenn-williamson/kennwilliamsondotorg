use crate::repositories::traits::PhraseRepository;
use anyhow::Result;
use std::sync::Arc;

/// Get a random active phrase for a user by slug, excluding phrases the user has excluded
pub async fn get_random_phrase_by_slug(
    repository: &Arc<dyn PhraseRepository>,
    user_slug: &str,
) -> Result<String> {
    // Validate input
    if user_slug.trim().is_empty() {
        return Err(anyhow::anyhow!("User slug cannot be empty"));
    }

    // Get phrase from repository
    let phrase_text = repository.get_random_phrase_by_slug(user_slug).await?;

    // Handle empty result case
    if phrase_text.trim().is_empty() {
        return Err(anyhow::anyhow!("No phrases available for user"));
    }

    Ok(phrase_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::MockPhraseRepository;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_get_random_phrase_by_slug_success() {
        let mut mock_repo = MockPhraseRepository::new();
        mock_repo
            .expect_get_random_phrase_by_slug()
            .with(mockall::predicate::eq("test-user"))
            .times(1)
            .returning(|_| Ok("Test phrase".to_string()));

        let repo: Arc<dyn crate::repositories::traits::PhraseRepository> = Arc::new(mock_repo);
        let result = get_random_phrase_by_slug(&repo, "test-user").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test phrase");
    }

    #[tokio::test]
    async fn test_get_random_phrase_by_slug_empty_slug() {
        let mock_repo = MockPhraseRepository::new();
        let repo: Arc<dyn crate::repositories::traits::PhraseRepository> = Arc::new(mock_repo);
        let result = get_random_phrase_by_slug(&repo, "").await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("User slug cannot be empty")
        );
    }

    #[tokio::test]
    async fn test_get_random_phrase_by_slug_repository_error() {
        let mut mock_repo = MockPhraseRepository::new();
        mock_repo
            .expect_get_random_phrase_by_slug()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let repo: Arc<dyn crate::repositories::traits::PhraseRepository> = Arc::new(mock_repo);
        let result = get_random_phrase_by_slug(&repo, "test-user").await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }
}
