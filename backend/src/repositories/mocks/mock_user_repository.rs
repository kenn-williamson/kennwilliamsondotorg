use mockall::mock;
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::models::db::user::User;
use crate::repositories::traits::user_repository::{UserRepository, CreateUserData, UserUpdates};

// Generate mock for UserRepository trait
mock! {
    pub UserRepository {}
    
    #[async_trait]
    impl UserRepository for UserRepository {
        async fn create_user(&self, user_data: &CreateUserData) -> Result<User>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
        async fn update_user(&self, id: Uuid, updates: &UserUpdates) -> Result<User>;
        async fn slug_exists(&self, slug: &str) -> Result<bool>;
        async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()>;
        async fn slug_exists_excluding_user(&self, slug: &str, user_id: Uuid) -> Result<bool>;
        async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;
    use mockall::predicate::eq;

    // Helper function to create a test user
    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // Helper function to create test data
    fn create_test_user_data() -> CreateUserData {
        CreateUserData {
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
        }
    }

    #[tokio::test]
    async fn test_mock_create_user() {
        let mut mock_repo = MockUserRepository::new();
        let user_data = create_test_user_data();
        
        // Setup mock expectation - create user inside closure
        mock_repo
            .expect_create_user()
            .times(1)
            .returning(|_| Ok(create_test_user()));
        
        // Test the mock
        let result = mock_repo.create_user(&user_data).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_mock_find_by_email() {
        let mut mock_repo = MockUserRepository::new();
        
        // Setup mock expectation - create user inside closure
        mock_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("test@example.com"))
            .returning(|_| Ok(Some(create_test_user())));
        
        // Test the mock
        let result = mock_repo.find_by_email("test@example.com").await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert!(user.is_some());
        assert_eq!(user.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_mock_slug_exists() {
        let mut mock_repo = MockUserRepository::new();
        
        // Setup mock expectation
        mock_repo
            .expect_slug_exists()
            .times(1)
            .with(eq("test-slug"))
            .returning(|_| Ok(true));
        
        // Test the mock
        let result = mock_repo.slug_exists("test-slug").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_mock_error_handling() {
        let mut mock_repo = MockUserRepository::new();
        
        // Setup mock to return an error
        mock_repo
            .expect_find_by_email()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database connection failed")));
        
        // Test error handling
        let result = mock_repo.find_by_email("error@example.com").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database connection failed"));
    }
}
