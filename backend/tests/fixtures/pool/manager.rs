//! Deadpool Manager implementation for testcontainers.

use deadpool::managed::{Manager, Metrics, RecycleError, RecycleResult};
use sqlx::PgPool;
use testcontainers::{
    GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

use super::config::PoolConfig;
use super::container::PooledPgContainer;
use super::reset::RESET_SQL;

/// Deadpool Manager for PostgreSQL testcontainers.
pub struct TestcontainerManager {
    config: PoolConfig,
}

impl TestcontainerManager {
    pub fn new(config: PoolConfig) -> Self {
        Self { config }
    }

    /// Wait for database to be ready with retries.
    async fn connect_with_retry(connection_string: &str) -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
        let max_attempts = 10;

        for attempt in 1..=max_attempts {
            match sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .acquire_timeout(std::time::Duration::from_secs(10))
                .idle_timeout(std::time::Duration::from_secs(600))
                .connect(connection_string)
                .await
            {
                Ok(pool) => {
                    // Test the connection
                    match sqlx::query("SELECT 1").fetch_one(&pool).await {
                        Ok(_) => return Ok(pool),
                        Err(e) => {
                            if attempt == max_attempts {
                                return Err(format!("Connection test failed after {max_attempts} attempts: {e}").into());
                            }
                        }
                    }
                }
                Err(e) => {
                    if attempt == max_attempts {
                        return Err(format!("Connection failed after {max_attempts} attempts: {e}").into());
                    }
                }
            }

            // Exponential backoff: 1, 2, 4, 8... seconds (max 8)
            let delay = std::cmp::min(1 << (attempt - 1), 8);
            tokio::time::sleep(std::time::Duration::from_secs(delay)).await;
        }

        Err("Failed to connect after all attempts".into())
    }
}

impl Manager for TestcontainerManager {
    type Type = PooledPgContainer;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn create(&self) -> Result<PooledPgContainer, Self::Error> {
        let image = GenericImage::new(&self.config.postgres_image, &self.config.postgres_tag)
            .with_exposed_port(5432.tcp())
            .with_wait_for(WaitFor::message_on_stdout(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_DB", "testdb")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "postgres");

        let container = image
            .start()
            .await
            .map_err(|e| format!("Container start failed: {e}"))?;

        let port = container
            .get_host_port_ipv4(5432)
            .await
            .map_err(|e| format!("Failed to get port: {e}"))?;

        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/testdb?sslmode=disable",
            port
        );

        // Connect with retry logic
        let pool = Self::connect_with_retry(&connection_string).await?;

        // Enable pg_uuidv7 extension
        sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_uuidv7")
            .execute(&pool)
            .await
            .map_err(|e| format!("Extension creation failed: {e}"))?;

        // Run migrations (seeds roles, system user, phrases)
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| format!("Migration failed: {e}"))?;

        tracing::info!(port, container_id = %uuid::Uuid::new_v4(), "Created new pooled container");

        Ok(PooledPgContainer::new(container, pool, port))
    }

    async fn recycle(
        &self,
        container: &mut PooledPgContainer,
        _metrics: &Metrics,
    ) -> RecycleResult<Self::Error> {
        // Health check
        sqlx::query("SELECT 1")
            .fetch_one(&container.pool)
            .await
            .map_err(|e| RecycleError::Backend(format!("Health check failed: {e}").into()))?;

        // TRUNCATE all test data
        sqlx::raw_sql(RESET_SQL)
            .execute(&container.pool)
            .await
            .map_err(|e| RecycleError::Backend(format!("Truncate failed: {e}").into()))?;

        let count = container.increment_recycle();
        tracing::debug!(
            port = container.port,
            container_id = %container.id,
            recycle_count = count + 1,
            "Recycled container"
        );

        Ok(())
    }
}
