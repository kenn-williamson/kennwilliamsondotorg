//! Testcontainer pool for fast parallel integration tests.
//!
//! Uses deadpool to maintain a pool of pre-warmed PostgreSQL containers.
//! Each test checks out a container, and it's reset (TRUNCATE + re-migrate) on return.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │              deadpool::managed::Pool                 │
//! │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐   │
//! │  │Container│ │Container│ │Container│ │Container│   │
//! │  │    1    │ │    2    │ │    3    │ │    4    │   │
//! │  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘   │
//! │       └───────────┴───────────┴───────────┘         │
//! │                TestcontainerManager                  │
//! │                (create / recycle)                    │
//! └─────────────────────────────────────────────────────┘
//!          │ checkout()           ▲ drop(guard) → recycle
//!          ▼                      │
//!     ┌─────────┐            ┌─────────┐
//!     │  Test   │ ────────▶  │  Test   │
//!     │ (runs)  │            │ (done)  │
//!     └─────────┘            └─────────┘
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! use crate::fixtures::pool::checkout;
//!
//! #[tokio::test]
//! async fn test_example() {
//!     let container = checkout().await;
//!     // Use container.pool for database operations
//!     sqlx::query("SELECT 1").fetch_one(&container.pool).await.unwrap();
//!     // Container is automatically returned to pool on drop
//! }
//! ```

pub mod config;
pub mod container;
pub mod manager;
pub mod reset;

use deadpool::managed::Pool;
use std::sync::OnceLock;

use config::PoolConfig;
use manager::TestcontainerManager;

#[allow(unused_imports)]
pub use container::PooledPgContainer;

/// Type alias for the testcontainer pool.
pub type TestcontainerPool = Pool<TestcontainerManager>;

/// Global pool singleton.
static POOL: OnceLock<TestcontainerPool> = OnceLock::new();

/// Get or initialize the global testcontainer pool.
pub fn get_pool() -> &'static TestcontainerPool {
    POOL.get_or_init(|| {
        let config = PoolConfig::auto();

        tracing::info!(
            max_size = config.max_size,
            ci_mode = std::env::var("CI").is_ok(),
            "Initializing testcontainer pool"
        );

        let manager = TestcontainerManager::new(config.clone());

        Pool::builder(manager)
            .max_size(config.max_size)
            .wait_timeout(Some(config.checkout_timeout))
            .create_timeout(Some(config.create_timeout))
            .recycle_timeout(Some(config.recycle_timeout))
            .runtime(deadpool::Runtime::Tokio1)
            .build()
            .expect("Failed to create testcontainer pool")
    })
}

/// Convenience function to checkout a container from the pool.
///
/// # Panics
/// Panics if checkout fails (pool exhaustion, container creation failure).
pub async fn checkout() -> deadpool::managed::Object<TestcontainerManager> {
    let pool = get_pool();

    match pool.get().await {
        Ok(container) => container,
        Err(deadpool::managed::PoolError::Timeout(e)) => {
            let status = pool.status();
            tracing::error!(
                available = status.available,
                size = status.size,
                max_size = status.max_size,
                "Pool exhaustion - consider increasing TESTCONTAINER_POOL_SIZE"
            );
            panic!("Container pool exhausted after {:?}", e);
        }
        Err(e) => {
            tracing::error!(error = %e, "Container checkout failed");
            panic!("Container checkout failed: {e}");
        }
    }
}

/// Get pool status for diagnostics.
#[allow(dead_code)]
pub fn status() -> deadpool::Status {
    get_pool().status()
}
