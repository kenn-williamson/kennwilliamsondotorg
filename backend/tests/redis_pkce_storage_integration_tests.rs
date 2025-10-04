use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};
use backend::repositories::redis::RedisPkceStorage;
use backend::repositories::traits::PkceStorage;

#[actix_web::test]
async fn test_pkce_storage_store_and_retrieve() {
    // Start Redis container
    let redis_image = GenericImage::new("redis", "alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    let redis_container = redis_image.start().await.expect("Failed to start Redis container");
    let redis_port = redis_container.get_host_port_ipv4(6379).await.unwrap();
    let redis_url = format!("redis://127.0.0.1:{}", redis_port);

    // Create PKCE storage
    let pkce_storage = RedisPkceStorage::new(&redis_url)
        .expect("Failed to create PKCE storage");

    let state = "test-state-123";
    let verifier = "test-verifier-abc";

    // Store PKCE verifier
    pkce_storage.store_pkce(state, verifier, 300).await
        .expect("Failed to store PKCE verifier");

    // Retrieve and delete PKCE verifier
    let retrieved = pkce_storage.retrieve_and_delete_pkce(state).await
        .expect("Failed to retrieve PKCE verifier");

    assert_eq!(retrieved, Some(verifier.to_string()));

    // Second retrieval should return None (single-use)
    let second_retrieval = pkce_storage.retrieve_and_delete_pkce(state).await
        .expect("Failed to retrieve PKCE verifier");

    assert_eq!(second_retrieval, None);
}

#[actix_web::test]
async fn test_pkce_storage_expiration() {
    // Start Redis container
    let redis_image = GenericImage::new("redis", "alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    let redis_container = redis_image.start().await.expect("Failed to start Redis container");
    let redis_port = redis_container.get_host_port_ipv4(6379).await.unwrap();
    let redis_url = format!("redis://127.0.0.1:{}", redis_port);

    // Create PKCE storage
    let pkce_storage = RedisPkceStorage::new(&redis_url)
        .expect("Failed to create PKCE storage");

    let state = "expiring-state";
    let verifier = "expiring-verifier";

    // Store with 2 second TTL
    pkce_storage.store_pkce(state, verifier, 2).await
        .expect("Failed to store PKCE verifier");

    // Should be retrievable immediately
    let retrieved = pkce_storage.retrieve_and_delete_pkce(state).await
        .expect("Failed to retrieve PKCE verifier");

    assert_eq!(retrieved, Some(verifier.to_string()));
}

#[actix_web::test]
async fn test_pkce_storage_not_found() {
    // Start Redis container
    let redis_image = GenericImage::new("redis", "alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    let redis_container = redis_image.start().await.expect("Failed to start Redis container");
    let redis_port = redis_container.get_host_port_ipv4(6379).await.unwrap();
    let redis_url = format!("redis://127.0.0.1:{}", redis_port);

    // Create PKCE storage
    let pkce_storage = RedisPkceStorage::new(&redis_url)
        .expect("Failed to create PKCE storage");

    // Retrieve non-existent state
    let retrieved = pkce_storage.retrieve_and_delete_pkce("nonexistent-state").await
        .expect("Failed to retrieve PKCE verifier");

    assert_eq!(retrieved, None);
}

#[actix_web::test]
async fn test_pkce_storage_multiple_states() {
    // Start Redis container
    let redis_image = GenericImage::new("redis", "alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    let redis_container = redis_image.start().await.expect("Failed to start Redis container");
    let redis_port = redis_container.get_host_port_ipv4(6379).await.unwrap();
    let redis_url = format!("redis://127.0.0.1:{}", redis_port);

    // Create PKCE storage
    let pkce_storage = RedisPkceStorage::new(&redis_url)
        .expect("Failed to create PKCE storage");

    // Store multiple PKCE verifiers
    pkce_storage.store_pkce("state1", "verifier1", 300).await
        .expect("Failed to store first verifier");
    pkce_storage.store_pkce("state2", "verifier2", 300).await
        .expect("Failed to store second verifier");
    pkce_storage.store_pkce("state3", "verifier3", 300).await
        .expect("Failed to store third verifier");

    // Retrieve them in different order
    let v2 = pkce_storage.retrieve_and_delete_pkce("state2").await.unwrap();
    assert_eq!(v2, Some("verifier2".to_string()));

    let v1 = pkce_storage.retrieve_and_delete_pkce("state1").await.unwrap();
    assert_eq!(v1, Some("verifier1".to_string()));

    let v3 = pkce_storage.retrieve_and_delete_pkce("state3").await.unwrap();
    assert_eq!(v3, Some("verifier3".to_string()));

    // All should be gone now
    assert_eq!(pkce_storage.retrieve_and_delete_pkce("state1").await.unwrap(), None);
    assert_eq!(pkce_storage.retrieve_and_delete_pkce("state2").await.unwrap(), None);
    assert_eq!(pkce_storage.retrieve_and_delete_pkce("state3").await.unwrap(), None);
}

#[actix_web::test]
async fn test_pkce_storage_overwrites_existing_state() {
    // Start Redis container
    let redis_image = GenericImage::new("redis", "alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    let redis_container = redis_image.start().await.expect("Failed to start Redis container");
    let redis_port = redis_container.get_host_port_ipv4(6379).await.unwrap();
    let redis_url = format!("redis://127.0.0.1:{}", redis_port);

    // Create PKCE storage
    let pkce_storage = RedisPkceStorage::new(&redis_url)
        .expect("Failed to create PKCE storage");

    let state = "same-state";

    // Store first verifier
    pkce_storage.store_pkce(state, "first-verifier", 300).await
        .expect("Failed to store first verifier");

    // Overwrite with second verifier
    pkce_storage.store_pkce(state, "second-verifier", 300).await
        .expect("Failed to store second verifier");

    // Should retrieve the second verifier (overwrites)
    let retrieved = pkce_storage.retrieve_and_delete_pkce(state).await
        .expect("Failed to retrieve PKCE verifier");

    assert_eq!(retrieved, Some("second-verifier".to_string()));
}
