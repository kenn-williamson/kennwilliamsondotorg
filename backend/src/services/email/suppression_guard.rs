use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;

use super::{Email, EmailService};
use crate::models::db::EmailType;
use crate::repositories::traits::email_suppression_repository::EmailSuppressionRepository;

/// Email service wrapper that adds suppression checking
///
/// This wrapper can be composed with any EmailService implementation to add
/// suppression list checking before sending emails. It follows the decorator
/// pattern, delegating actual email sending to the wrapped service.
///
/// # Example
///
/// ```rust,no_run
/// use backend::services::email::{SesEmailService, SuppressionGuard, MockEmailService};
/// use backend::repositories::postgres::postgres_email_suppression_repository::PostgresEmailSuppressionRepository;
///
/// // Production: Wrap SES with suppression
/// # fn example() {
/// # let from = "test@example.com".to_string();
/// # let reply_to = None;
/// # let config_set = None;
/// # let pool = todo!();
/// let ses = SesEmailService::new(from, reply_to, config_set);
/// let suppression_repo = Box::new(PostgresEmailSuppressionRepository::new(pool));
/// let guarded = SuppressionGuard::new(Box::new(ses), suppression_repo);
///
/// // Testing: Wrap mock with suppression
/// # use backend::repositories::mocks::MockEmailSuppressionRepository;
/// let mock = MockEmailService::new();
/// let suppression_repo = Box::new(MockEmailSuppressionRepository::new());
/// let guarded = SuppressionGuard::new(Box::new(mock), suppression_repo);
/// # }
/// ```
pub struct SuppressionGuard {
    /// The wrapped email service (can be any EmailService implementation)
    inner: Box<dyn EmailService>,

    /// Repository for checking email suppression status
    suppression_repo: Box<dyn EmailSuppressionRepository>,
}

impl SuppressionGuard {
    /// Create a new SuppressionGuard that wraps any EmailService
    ///
    /// # Arguments
    ///
    /// * `inner` - The email service to wrap (SesEmailService, MockEmailService, etc.)
    /// * `suppression_repo` - Repository for checking suppression status
    pub fn new(
        inner: Box<dyn EmailService>,
        suppression_repo: Box<dyn EmailSuppressionRepository>,
    ) -> Self {
        Self {
            inner,
            suppression_repo,
        }
    }
}

#[async_trait]
impl EmailService for SuppressionGuard {
    async fn send_email(&self, email: Email) -> Result<()> {
        // Check suppression list for all recipients
        for recipient in &email.to {
            let is_suppressed = self.suppression_repo
                .is_email_suppressed(recipient, EmailType::Transactional)
                .await
                .context("Failed to check email suppression status")?;

            if is_suppressed {
                log::warn!(
                    "Email blocked by suppression list: {} (transactional)",
                    recipient
                );
                return Err(anyhow!(
                    "Email address {} is suppressed and cannot receive emails",
                    recipient
                ));
            }
        }

        // All recipients passed suppression check - delegate to wrapped service
        self.inner.send_email(email).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::email::MockEmailService;
    use crate::repositories::mocks::MockEmailSuppressionRepository;
    use crate::repositories::traits::email_suppression_repository::CreateSuppressionData;

    #[tokio::test]
    async fn test_suppression_guard_blocks_suppressed_email() {
        // Given: A suppression guard with a suppressed email
        let mock_email_service = MockEmailService::new();
        let suppression_repo = MockEmailSuppressionRepository::new();

        suppression_repo
            .create_suppression(&CreateSuppressionData {
                email: "suppressed@example.com".to_string(),
                suppression_type: "bounce".to_string(),
                reason: Some("Hard bounce".to_string()),
                suppress_transactional: true,
                suppress_marketing: true,
            })
            .await
            .unwrap();

        let guard = SuppressionGuard::new(
            Box::new(mock_email_service.clone()),
            Box::new(suppression_repo),
        );

        // When: Attempting to send to suppressed address
        let email = Email::builder()
            .to("suppressed@example.com")
            .subject("Test")
            .text_body("Test body")
            .build()
            .unwrap();

        let result = guard.send_email(email).await;

        // Then: Email should be blocked
        assert!(result.is_err());
        assert_eq!(mock_email_service.count(), 0, "No email should reach inner service");
    }

    #[tokio::test]
    async fn test_suppression_guard_allows_non_suppressed_email() {
        // Given: A suppression guard with no suppressions
        let mock_email_service = MockEmailService::new();
        let suppression_repo = MockEmailSuppressionRepository::new();

        let guard = SuppressionGuard::new(
            Box::new(mock_email_service.clone()),
            Box::new(suppression_repo),
        );

        // When: Sending to non-suppressed address
        let email = Email::builder()
            .to("allowed@example.com")
            .subject("Test")
            .text_body("Test body")
            .build()
            .unwrap();

        let result = guard.send_email(email).await;

        // Then: Email should be sent to inner service
        assert!(result.is_ok());
        assert_eq!(mock_email_service.count(), 1, "Email should reach inner service");
    }
}
