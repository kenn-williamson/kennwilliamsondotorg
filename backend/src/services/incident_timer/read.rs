use anyhow::Result;
use uuid::Uuid;

use crate::models::db::IncidentTimer;
use super::IncidentTimerService;

impl IncidentTimerService {
    /// Get the latest timer for a user by their slug (public access)
    pub async fn get_latest_by_user_slug(&self, user_slug: &str) -> Result<Option<(IncidentTimer, String)>> {
        // Use the new repository method that returns both timer and display name
        self.repository.find_latest_by_user_slug_with_display_name(user_slug).await
    }

    /// Get all timers for a specific user
    pub async fn get_all_by_user(&self, user_id: Uuid) -> Result<Vec<IncidentTimer>> {
        self.repository.find_by_user_id(user_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::IncidentTimer;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_latest_by_user_slug_success() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_slug = "test-user";
        let now = chrono::Utc::now();
        let timer = IncidentTimer {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            reset_timestamp: now,
            notes: Some("Test notes".to_string()),
            created_at: now,
            updated_at: now,
        };
        let display_name = "Test User".to_string();

        mock_repo
            .expect_find_latest_by_user_slug_with_display_name()
            .with(eq(user_slug))
            .times(1)
            .returning(move |_| Ok(Some((timer.clone(), display_name.clone()))));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.get_latest_by_user_slug(user_slug).await;

        // Verify
        assert!(result.is_ok());
        let (returned_timer, returned_display_name) = result.unwrap().unwrap();
        assert_eq!(returned_timer.notes, Some("Test notes".to_string()));
        assert_eq!(returned_display_name, "Test User");
    }

    #[tokio::test]
    async fn test_get_latest_by_user_slug_not_found() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_slug = "nonexistent-user";

        mock_repo
            .expect_find_latest_by_user_slug_with_display_name()
            .with(eq(user_slug))
            .times(1)
            .returning(|_| Ok(None));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.get_latest_by_user_slug(user_slug).await;

        // Verify
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_all_by_user_success() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let timers = vec![
            IncidentTimer {
                id: Uuid::new_v4(),
                user_id,
                reset_timestamp: now,
                notes: Some("Timer 1".to_string()),
                created_at: now,
                updated_at: now,
            },
            IncidentTimer {
                id: Uuid::new_v4(),
                user_id,
                reset_timestamp: now - chrono::Duration::hours(1),
                notes: Some("Timer 2".to_string()),
                created_at: now - chrono::Duration::hours(1),
                updated_at: now - chrono::Duration::hours(1),
            },
        ];

        mock_repo
            .expect_find_by_user_id()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(timers.clone()));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.get_all_by_user(user_id).await;

        // Verify
        assert!(result.is_ok());
        let returned_timers = result.unwrap();
        assert_eq!(returned_timers.len(), 2);
        assert_eq!(returned_timers[0].notes, Some("Timer 1".to_string()));
        assert_eq!(returned_timers[1].notes, Some("Timer 2".to_string()));
    }

    #[tokio::test]
    async fn test_get_all_by_user_empty() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_find_by_user_id()
            .with(eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.get_all_by_user(user_id).await;

        // Verify
        assert!(result.is_ok());
        let returned_timers = result.unwrap();
        assert_eq!(returned_timers.len(), 0);
    }

    #[tokio::test]
    async fn test_get_latest_by_user_slug_repository_error() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_slug = "test-user";

        mock_repo
            .expect_find_latest_by_user_slug_with_display_name()
            .with(eq(user_slug))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.get_latest_by_user_slug(user_slug).await;

        // Verify
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_get_all_by_user_repository_error() {
        // Setup
        let mut mock_repo = crate::repositories::mocks::MockIncidentTimerRepository::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_find_by_user_id()
            .with(eq(user_id))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let service = IncidentTimerService::new(Box::new(mock_repo));

        // Execute
        let result = service.get_all_by_user(user_id).await;

        // Verify
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }
}
