use anyhow::Result;

use crate::repositories::traits::user_repository::UserRepository;

/// Generate a unique slug from a display name
pub async fn generate_slug(
    display_name: &str,
    user_repository: &dyn UserRepository,
) -> Result<String> {
    let base_slug = display_name
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace() || *c == '-')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-");

    // Reject if no alphanumeric characters remain after filtering
    if base_slug.chars().all(|c| !c.is_ascii_alphanumeric()) {
        return Err(anyhow::anyhow!("Display name must contain at least one letter or number"));
    }

    let mut slug = base_slug.clone();
    let mut counter = 1;

    while user_repository.slug_exists(&slug).await? {
        slug = format!("{}-{}", base_slug, counter);
        counter += 1;
    }

    Ok(slug)
}


#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use crate::repositories::mocks::mock_user_repository::MockUserRepository;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn generates_slugs_with_various_inputs() -> Result<()> {
        let mut mock_repo = MockUserRepository::new();
        
        // Test cases: (input, expected_output)
        let test_cases = vec![
            ("John Doe", "john-doe"),
            ("John's Café & Bar!", "johns-caf-bar"), // Special chars + accented chars
            ("José María", "jos-mara"), // Accented characters (é and í filtered out)
            ("John    Doe   Smith", "john-doe-smith"), // Multiple spaces
            ("User123", "user123"), // Numbers
            ("Test-User", "test-user"), // Already has hyphens
            ("UPPERCASE", "uppercase"), // Case conversion
            ("Mixed123Case", "mixed123case"), // Mixed case + numbers
            ("Special@#$%", "special"), // Only special chars (some valid)
            ("   Spaces   ", "spaces"), // Leading/trailing spaces
        ];
        
        // Setup mock expectations for all test cases
        for (_, expected) in &test_cases {
            mock_repo
                .expect_slug_exists()
                .times(1)
                .with(eq(*expected))
                .returning(|_| Ok(false));
        }
        
        // Run all test cases
        for (input, expected) in test_cases {
            let result = generate_slug(input, &mock_repo).await?;
            assert_eq!(result, expected, "Failed for input: '{}'", input);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn generates_unique_slug_when_exists() -> Result<()> {
        let mut mock_repo = MockUserRepository::new();
        
        // Setup mock expectations - first slug exists, second doesn't
        mock_repo
            .expect_slug_exists()
            .times(1)
            .with(eq("john-doe"))
            .returning(|_| Ok(true));
        
        mock_repo
            .expect_slug_exists()
            .times(1)
            .with(eq("john-doe-1"))
            .returning(|_| Ok(false));
        
        let result = generate_slug("John Doe", &mock_repo).await?;
        assert_eq!(result, "john-doe-1");
        Ok(())
    }


    #[tokio::test]
    async fn handles_error_cases() -> Result<()> {
        // Test invalid inputs that should be rejected
        let invalid_inputs = vec![
            "!!!@@@###",
            "   ",
            "---", // Only hyphens
            "!!!",
            "   !!!   ",
            "😀🎉🎊", // Emojis only
        ];
        
        let mock_repo = MockUserRepository::new();
        
        for input in invalid_inputs {
            let result = generate_slug(input, &mock_repo).await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Display name must contain at least one letter or number"));
        }
        
        // Test database error
        let mut mock_repo = MockUserRepository::new();
        mock_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database connection failed")));
        
        let result = generate_slug("John Doe", &mock_repo).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database connection failed"));
        
        Ok(())
    }
}
