use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::models::db::{EmailSuppression, EmailType};
use crate::repositories::traits::email_suppression_repository::{
    CreateSuppressionData, EmailSuppressionRepository,
};

/// Mock implementation of EmailSuppressionRepository for testing
#[derive(Clone)]
pub struct MockEmailSuppressionRepository {
    suppressions: Arc<Mutex<HashMap<String, EmailSuppression>>>,
}

impl MockEmailSuppressionRepository {
    pub fn new() -> Self {
        Self {
            suppressions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for MockEmailSuppressionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailSuppressionRepository for MockEmailSuppressionRepository {
    async fn create_suppression(
        &self,
        data: &CreateSuppressionData,
    ) -> Result<EmailSuppression> {
        let mut suppressions = self.suppressions.lock().unwrap();

        // Check for duplicate email
        if suppressions.contains_key(&data.email) {
            return Err(anyhow!("Email already suppressed"));
        }

        let now = Utc::now();
        let suppression = EmailSuppression {
            id: Uuid::new_v4(),
            email: data.email.clone(),
            suppression_type: data.suppression_type.clone(),
            reason: data.reason.clone(),
            suppress_transactional: data.suppress_transactional,
            suppress_marketing: data.suppress_marketing,
            bounce_count: 0,
            last_bounce_at: None,
            created_at: now,
            updated_at: now,
        };

        suppressions.insert(data.email.clone(), suppression.clone());
        Ok(suppression)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<EmailSuppression>> {
        let suppressions = self.suppressions.lock().unwrap();
        Ok(suppressions.get(email).cloned())
    }

    async fn is_email_suppressed(&self, email: &str, email_type: EmailType) -> Result<bool> {
        let suppressions = self.suppressions.lock().unwrap();

        if let Some(suppression) = suppressions.get(email) {
            match email_type {
                EmailType::Transactional => Ok(suppression.suppress_transactional),
                EmailType::Marketing => Ok(suppression.suppress_marketing),
            }
        } else {
            Ok(false)
        }
    }

    async fn increment_bounce_count(
        &self,
        email: &str,
        bounced_at: DateTime<Utc>,
    ) -> Result<()> {
        let mut suppressions = self.suppressions.lock().unwrap();

        if let Some(suppression) = suppressions.get_mut(email) {
            suppression.bounce_count += 1;
            suppression.last_bounce_at = Some(bounced_at);
            suppression.updated_at = Utc::now();
            Ok(())
        } else {
            Err(anyhow!("Email suppression not found"))
        }
    }

    async fn delete_suppression(&self, email: &str) -> Result<()> {
        let mut suppressions = self.suppressions.lock().unwrap();
        suppressions.remove(email);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_suppression() {
        let repo = MockEmailSuppressionRepository::new();

        let data = CreateSuppressionData {
            email: "test@example.com".to_string(),
            suppression_type: "bounce".to_string(),
            reason: Some("Hard bounce".to_string()),
            suppress_transactional: true,
            suppress_marketing: true,
        };

        let result = repo.create_suppression(&data).await;
        assert!(result.is_ok());

        let suppression = result.unwrap();
        assert_eq!(suppression.email, "test@example.com");
        assert_eq!(suppression.suppression_type, "bounce");
        assert_eq!(suppression.reason, Some("Hard bounce".to_string()));
        assert!(suppression.suppress_transactional);
        assert!(suppression.suppress_marketing);
        assert_eq!(suppression.bounce_count, 0);
    }

    #[tokio::test]
    async fn test_create_duplicate_email_fails() {
        let repo = MockEmailSuppressionRepository::new();

        let data = CreateSuppressionData {
            email: "test@example.com".to_string(),
            suppression_type: "bounce".to_string(),
            reason: None,
            suppress_transactional: true,
            suppress_marketing: true,
        };

        // First creation should succeed
        let result1 = repo.create_suppression(&data).await;
        assert!(result1.is_ok());

        // Second creation with same email should fail
        let result2 = repo.create_suppression(&data).await;
        assert!(result2.is_err());
        assert!(result2.unwrap_err().to_string().contains("already suppressed"));
    }

    #[tokio::test]
    async fn test_find_by_email() {
        let repo = MockEmailSuppressionRepository::new();

        let data = CreateSuppressionData {
            email: "test@example.com".to_string(),
            suppression_type: "bounce".to_string(),
            reason: None,
            suppress_transactional: true,
            suppress_marketing: true,
        };

        repo.create_suppression(&data).await.unwrap();

        // Should find existing suppression
        let found = repo.find_by_email("test@example.com").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().email, "test@example.com");

        // Should not find non-existent suppression
        let not_found = repo.find_by_email("notfound@example.com").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_is_email_suppressed_transactional() {
        let repo = MockEmailSuppressionRepository::new();

        // Create suppression that blocks transactional emails
        let data = CreateSuppressionData {
            email: "bounce@example.com".to_string(),
            suppression_type: "bounce".to_string(),
            reason: None,
            suppress_transactional: true,
            suppress_marketing: true,
        };
        repo.create_suppression(&data).await.unwrap();

        // Should be suppressed for transactional
        let is_suppressed = repo
            .is_email_suppressed("bounce@example.com", EmailType::Transactional)
            .await
            .unwrap();
        assert!(is_suppressed);

        // Should be suppressed for marketing
        let is_suppressed = repo
            .is_email_suppressed("bounce@example.com", EmailType::Marketing)
            .await
            .unwrap();
        assert!(is_suppressed);
    }

    #[tokio::test]
    async fn test_is_email_suppressed_marketing_only() {
        let repo = MockEmailSuppressionRepository::new();

        // Create suppression that blocks only marketing emails
        let data = CreateSuppressionData {
            email: "unsubscribe@example.com".to_string(),
            suppression_type: "unsubscribe".to_string(),
            reason: None,
            suppress_transactional: false,
            suppress_marketing: true,
        };
        repo.create_suppression(&data).await.unwrap();

        // Should NOT be suppressed for transactional
        let is_suppressed = repo
            .is_email_suppressed("unsubscribe@example.com", EmailType::Transactional)
            .await
            .unwrap();
        assert!(!is_suppressed);

        // Should be suppressed for marketing
        let is_suppressed = repo
            .is_email_suppressed("unsubscribe@example.com", EmailType::Marketing)
            .await
            .unwrap();
        assert!(is_suppressed);
    }

    #[tokio::test]
    async fn test_is_email_suppressed_not_in_list() {
        let repo = MockEmailSuppressionRepository::new();

        // Non-existent email should not be suppressed
        let is_suppressed = repo
            .is_email_suppressed("clean@example.com", EmailType::Transactional)
            .await
            .unwrap();
        assert!(!is_suppressed);

        let is_suppressed = repo
            .is_email_suppressed("clean@example.com", EmailType::Marketing)
            .await
            .unwrap();
        assert!(!is_suppressed);
    }

    #[tokio::test]
    async fn test_increment_bounce_count() {
        let repo = MockEmailSuppressionRepository::new();

        let data = CreateSuppressionData {
            email: "bounce@example.com".to_string(),
            suppression_type: "bounce".to_string(),
            reason: None,
            suppress_transactional: true,
            suppress_marketing: true,
        };
        repo.create_suppression(&data).await.unwrap();

        // Increment bounce count
        let bounced_at = Utc::now();
        repo.increment_bounce_count("bounce@example.com", bounced_at)
            .await
            .unwrap();

        // Verify count incremented
        let suppression = repo.find_by_email("bounce@example.com").await.unwrap().unwrap();
        assert_eq!(suppression.bounce_count, 1);
        assert!(suppression.last_bounce_at.is_some());

        // Increment again
        repo.increment_bounce_count("bounce@example.com", Utc::now())
            .await
            .unwrap();

        let suppression = repo.find_by_email("bounce@example.com").await.unwrap().unwrap();
        assert_eq!(suppression.bounce_count, 2);
    }

    #[tokio::test]
    async fn test_increment_bounce_count_nonexistent_email() {
        let repo = MockEmailSuppressionRepository::new();

        // Should fail for non-existent email
        let result = repo
            .increment_bounce_count("notfound@example.com", Utc::now())
            .await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_delete_suppression() {
        let repo = MockEmailSuppressionRepository::new();

        let data = CreateSuppressionData {
            email: "test@example.com".to_string(),
            suppression_type: "manual".to_string(),
            reason: None,
            suppress_transactional: false,
            suppress_marketing: true,
        };
        repo.create_suppression(&data).await.unwrap();

        // Verify it exists
        let found = repo.find_by_email("test@example.com").await.unwrap();
        assert!(found.is_some());

        // Delete it
        repo.delete_suppression("test@example.com").await.unwrap();

        // Verify it's gone
        let not_found = repo.find_by_email("test@example.com").await.unwrap();
        assert!(not_found.is_none());

        // Deleting again should not error (idempotent)
        let result = repo.delete_suppression("test@example.com").await;
        assert!(result.is_ok());
    }
}
