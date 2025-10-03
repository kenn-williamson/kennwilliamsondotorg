use anyhow::Result;
use uuid::Uuid;

use super::IncidentTimerService;
use crate::models::api::UpdateIncidentTimer;
use crate::models::db::IncidentTimer;

impl IncidentTimerService {
    /// Update an existing incident timer
    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        data: UpdateIncidentTimer,
    ) -> Result<Option<IncidentTimer>> {
        // Validate that reset_timestamp is not in the future
        if let Some(reset_timestamp) = &data.reset_timestamp {
            validate_timestamp(reset_timestamp)?;
        }

        // Check if timer belongs to user
        let belongs_to_user = validate_ownership(self, id, user_id).await?;
        if !belongs_to_user {
            return Ok(None);
        }

        let updates = crate::repositories::traits::incident_timer_repository::TimerUpdates {
            reset_timestamp: data.reset_timestamp,
            notes: data.notes,
        };

        let timer = self.repository.update_timer(id, &updates).await?;
        Ok(Some(timer))
    }
}

/// Validate that timestamp is not in the future
fn validate_timestamp(timestamp: &chrono::DateTime<chrono::Utc>) -> Result<()> {
    let now = chrono::Utc::now();
    if timestamp > &now {
        return Err(anyhow::anyhow!("Reset timestamp cannot be in the future"));
    }
    Ok(())
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
    use crate::models::api::UpdateIncidentTimer;
    use crate::models::db::IncidentTimer;
    use crate::repositories::traits::incident_timer_repository::TimerUpdates;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_update_timer_success() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let update_data = UpdateIncidentTimer {
            reset_timestamp: Some(now),
            notes: Some("Updated notes".to_string()),
        };

        let expected_timer = IncidentTimer {
            id: timer_id,
            user_id,
            reset_timestamp: now,
            notes: Some("Updated notes".to_string()),
            created_at: now - chrono::Duration::hours(1),
            updated_at: now,
        };

        mock_repo
            .expect_timer_belongs_to_user()
            .with(eq(timer_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(true));

        mock_repo
            .expect_update_timer()
            .with(
                eq(timer_id),
                eq(TimerUpdates {
                    reset_timestamp: Some(now),
                    notes: Some("Updated notes".to_string()),
                }),
            )
            .times(1)
            .returning(move |_, _| Ok(expected_timer.clone()));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.update(timer_id, user_id, update_data).await;

        // Verify
        assert!(result.is_ok());
        let timer = result.unwrap().unwrap();
        assert_eq!(timer.id, timer_id);
        assert_eq!(timer.notes, Some("Updated notes".to_string()));
    }

    #[tokio::test]
    async fn test_update_timer_not_owned_by_user() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let update_data = UpdateIncidentTimer {
            reset_timestamp: None,
            notes: Some("Updated notes".to_string()),
        };

        mock_repo
            .expect_timer_belongs_to_user()
            .with(eq(timer_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(false));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.update(timer_id, user_id, update_data).await;

        // Verify
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_timer_future_timestamp_error() {
        // Setup
        let mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let future_time = chrono::Utc::now() + chrono::Duration::hours(1);
        let update_data = UpdateIncidentTimer {
            reset_timestamp: Some(future_time),
            notes: None,
        };

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.update(timer_id, user_id, update_data).await;

        // Verify
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Reset timestamp cannot be in the future"));
    }

    #[tokio::test]
    async fn test_update_timer_ownership_check_error() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let update_data = UpdateIncidentTimer {
            reset_timestamp: None,
            notes: None,
        };

        mock_repo
            .expect_timer_belongs_to_user()
            .with(eq(timer_id), eq(user_id))
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Database error")));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.update(timer_id, user_id, update_data).await;

        // Verify
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_update_timer_repository_error() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let update_data = UpdateIncidentTimer {
            reset_timestamp: Some(now),
            notes: None,
        };

        mock_repo
            .expect_timer_belongs_to_user()
            .with(eq(timer_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(true));

        mock_repo
            .expect_update_timer()
            .with(
                eq(timer_id),
                eq(TimerUpdates {
                    reset_timestamp: Some(now),
                    notes: None,
                }),
            )
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Database error")));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.update(timer_id, user_id, update_data).await;

        // Verify
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }
}
