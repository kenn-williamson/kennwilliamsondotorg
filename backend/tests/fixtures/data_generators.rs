// Test data generators for unique test values

use std::sync::atomic::{AtomicU64, Ordering};

/// Generates a unique test email
#[allow(dead_code)]
pub fn unique_test_email() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("test_{}_{}@test.com",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        counter
    )
}

/// Generates a unique test slug
#[allow(dead_code)]
pub fn unique_test_slug() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("test-user-{}-{}",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        counter
    )
}

/// Test password hash for testing
#[allow(dead_code)]
pub fn test_password_hash() -> String {
    "$2b$04$6X8Q8Q8Q8Q8Q8Q8Q8Q8Q8O8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q".to_string()
}

/// Generates a unique test phrase
#[allow(dead_code)]
pub fn unique_test_phrase() -> String {
    format!("Test phrase {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
}
