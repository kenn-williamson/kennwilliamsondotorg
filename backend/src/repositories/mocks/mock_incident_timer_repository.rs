use mockall::mock;
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::models::db::incident_timer::IncidentTimer;
use crate::repositories::traits::incident_timer_repository::{IncidentTimerRepository, CreateTimerData, TimerUpdates};

// Generate mock for IncidentTimerRepository trait
mock! {
    pub IncidentTimerRepository {}
    
    #[async_trait]
    impl IncidentTimerRepository for IncidentTimerRepository {
        async fn create_timer(&self, timer_data: &CreateTimerData) -> Result<IncidentTimer>;
        async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<IncidentTimer>>;
        async fn find_latest_by_user_slug_with_display_name(&self, slug: &str) -> Result<Option<(IncidentTimer, String)>>;
        async fn update_timer(&self, id: Uuid, updates: &TimerUpdates) -> Result<IncidentTimer>;
        async fn delete_timer(&self, id: Uuid) -> Result<()>;
        async fn timer_belongs_to_user(&self, timer_id: Uuid, user_id: Uuid) -> Result<bool>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;
    use mockall::predicate::eq;

    // Helper function to create a test incident timer
    fn create_test_incident_timer() -> IncidentTimer {
        IncidentTimer {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            reset_timestamp: Utc::now(),
            notes: Some("Test incident".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // Helper function to create test data
    fn create_test_timer_data() -> CreateTimerData {
        CreateTimerData {
            user_id: Uuid::new_v4(),
            reset_timestamp: Utc::now(),
            notes: Some("Test incident".to_string()),
        }
    }

    #[tokio::test]
    async fn test_mock_create_timer() {
        let mut mock_repo = MockIncidentTimerRepository::new();
        let timer_data = create_test_timer_data();
        
        // Setup mock expectation
        mock_repo
            .expect_create_timer()
            .times(1)
            .returning(|_| Ok(create_test_incident_timer()));
        
        // Test the mock
        let result = mock_repo.create_timer(&timer_data).await;
        assert!(result.is_ok());
        let timer = result.unwrap();
        assert_eq!(timer.notes, Some("Test incident".to_string()));
    }

    #[tokio::test]
    async fn test_mock_find_by_user_id() {
        let mut mock_repo = MockIncidentTimerRepository::new();
        let user_id = Uuid::new_v4();
        
        // Setup mock expectation
        mock_repo
            .expect_find_by_user_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(vec![create_test_incident_timer()]));
        
        // Test the mock
        let result = mock_repo.find_by_user_id(user_id).await;
        assert!(result.is_ok());
        let timers = result.unwrap();
        assert_eq!(timers.len(), 1);
    }


    #[tokio::test]
    async fn test_mock_timer_belongs_to_user() {
        let mut mock_repo = MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Setup mock expectation
        mock_repo
            .expect_timer_belongs_to_user()
            .times(1)
            .with(eq(timer_id), eq(user_id))
            .returning(|_, _| Ok(true));
        
        // Test the mock
        let result = mock_repo.timer_belongs_to_user(timer_id, user_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_mock_delete_timer() {
        let mut mock_repo = MockIncidentTimerRepository::new();
        let timer_id = Uuid::new_v4();
        
        // Setup mock expectation
        mock_repo
            .expect_delete_timer()
            .times(1)
            .with(eq(timer_id))
            .returning(|_| Ok(()));
        
        // Test the mock
        let result = mock_repo.delete_timer(timer_id).await;
        assert!(result.is_ok());
    }

}
