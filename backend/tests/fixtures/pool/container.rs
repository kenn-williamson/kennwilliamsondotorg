//! Pooled container wrapper with connection pool and metadata.

use sqlx::PgPool;
use std::sync::atomic::{AtomicU64, Ordering};
use testcontainers::ContainerAsync;
use testcontainers::GenericImage;

/// A pooled PostgreSQL container with its connection pool.
pub struct PooledPgContainer {
    /// The Docker container (kept alive for lifetime).
    #[allow(dead_code)]
    pub(crate) container: ContainerAsync<GenericImage>,

    /// SQLx connection pool.
    pub pool: PgPool,

    /// The mapped host port (useful for debugging).
    pub port: u16,

    /// Unique container ID for logging.
    pub id: uuid::Uuid,

    /// Number of times this container has been recycled.
    recycle_count: AtomicU64,
}

impl PooledPgContainer {
    /// Create a new pooled container.
    pub fn new(container: ContainerAsync<GenericImage>, pool: PgPool, port: u16) -> Self {
        Self {
            container,
            pool,
            port,
            id: uuid::Uuid::new_v4(),
            recycle_count: AtomicU64::new(0),
        }
    }

    /// Get the connection string for this container.
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://postgres:postgres@127.0.0.1:{}/testdb?sslmode=disable",
            self.port
        )
    }

    /// Get recycle count.
    #[allow(dead_code)]
    pub fn recycled_times(&self) -> u64 {
        self.recycle_count.load(Ordering::Relaxed)
    }

    /// Increment recycle count and return previous value.
    pub fn increment_recycle(&self) -> u64 {
        self.recycle_count.fetch_add(1, Ordering::Relaxed)
    }
}
