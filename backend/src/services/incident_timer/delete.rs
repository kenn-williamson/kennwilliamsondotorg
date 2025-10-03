use anyhow::Result;
use uuid::Uuid;

use super::IncidentTimerService;

impl IncidentTimerService {
    /// Delete an incident timer
    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<bool> {
        // Check if timer belongs to user
        let belongs_to_user = validate_ownership(self, id, user_id).await?;
        if !belongs_to_user {
            return Ok(false);
        }

        self.repository.delete_timer(id).await?;
        Ok(true)
    }
}

/// Validate that timer belongs to user
async fn validate_ownership(
    service: &IncidentTimerService,
    timer_id: Uuid,
    user_id: Uuid,
) -> Result<bool> {
    service
        .repository
        .timer_belongs_to_user(timer_id, user_id)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_delete_timer_success() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_timer_belongs_to_user()
            .with(eq(timer_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(true));

        mock_repo
            .expect_delete_timer()
            .with(eq(timer_id))
            .times(1)
            .returning(|_| Ok(()));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.delete(timer_id, user_id).await;

        // Verify
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_delete_timer_not_owned_by_user() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_timer_belongs_to_user()
            .with(eq(timer_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(false));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.delete(timer_id, user_id).await;

        // Verify
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_delete_timer_ownership_check_error() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_timer_belongs_to_user()
            .with(eq(timer_id), eq(user_id))
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Database error")));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.delete(timer_id, user_id).await;

        // Verify
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_delete_timer_repository_error() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_timer_belongs_to_user()
            .with(eq(timer_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(true));

        mock_repo
            .expect_delete_timer()
            .with(eq(timer_id))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.delete(timer_id, user_id).await;

        // Verify
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }
}
