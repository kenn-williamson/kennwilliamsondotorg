use anyhow::Result;
use uuid::Uuid;

use super::IncidentTimerService;
use crate::models::api::CreateIncidentTimer;

impl IncidentTimerService {
    /// Create a new incident timer
    pub async fn create(
        &self,
        user_id: Uuid,
        data: CreateIncidentTimer,
    ) -> Result<crate::models::db::IncidentTimer> {
        let reset_timestamp = data.reset_timestamp.unwrap_or_else(|| chrono::Utc::now());

        // Validate that reset_timestamp is not in the future
        validate_timestamp(&reset_timestamp)?;

        let timer_data = crate::repositories::traits::incident_timer_repository::CreateTimerData {
            user_id,
            reset_timestamp,
            notes: data.notes,
        };

        self.repository.create_timer(&timer_data).await
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::api::CreateIncidentTimer;
    use crate::repositories::traits::incident_timer_repository::CreateTimerData;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_timer_success() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let timer_data = CreateIncidentTimer {
            reset_timestamp: Some(now),
            notes: Some("Test notes".to_string()),
        };

        let expected_timer = crate::test_utils::IncidentTimerBuilder::new()
            .with_user_id(user_id)
            .with_reset_timestamp(now)
            .with_notes("Test notes")
            .created_at(now)
            .updated_at(now)
            .build();

        mock_repo
            .expect_create_timer()
            .with(eq(CreateTimerData {
                user_id,
                reset_timestamp: now,
                notes: Some("Test notes".to_string()),
            }))
            .times(1)
            .returning(move |_| Ok(expected_timer.clone()));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.create(user_id, timer_data).await;

        // Verify
        assert!(result.is_ok());
        let timer = result.unwrap();
        assert_eq!(timer.user_id, user_id);
        assert_eq!(timer.notes, Some("Test notes".to_string()));
    }

    #[tokio::test]
    async fn test_create_timer_with_default_timestamp() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_id = Uuid::new_v4();
        let timer_data = CreateIncidentTimer {
            reset_timestamp: None,
            notes: None,
        };

        let expected_timer = crate::test_utils::IncidentTimerBuilder::new()
            .with_user_id(user_id)
            .without_notes()
            .build();

        mock_repo
            .expect_create_timer()
            .times(1)
            .returning(move |_| Ok(expected_timer.clone()));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.create(user_id, timer_data).await;

        // Verify
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_timer_future_timestamp_error() {
        // Setup
        let mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_id = Uuid::new_v4();
        let future_time = chrono::Utc::now() + chrono::Duration::hours(1);
        let timer_data = CreateIncidentTimer {
            reset_timestamp: Some(future_time),
            notes: None,
        };

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.create(user_id, timer_data).await;

        // Verify
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Reset timestamp cannot be in the future"));
    }

    #[tokio::test]
    async fn test_create_timer_repository_error() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let timer_data = CreateIncidentTimer {
            reset_timestamp: Some(now),
            notes: None,
        };

        mock_repo
            .expect_create_timer()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.create(user_id, timer_data).await;

        // Verify
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }
}
