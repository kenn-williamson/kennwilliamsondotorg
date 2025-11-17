mod fixtures;

use backend::repositories::postgres::postgres_access_request_repository::PostgresAccessRequestRepository;
use backend::repositories::traits::AccessRequestRepository;
use backend::test_utils::AccessRequestBuilder;
use fixtures::TestContainer;
use fixtures::database::{create_test_user, get_user_roles};

#[tokio::test]
async fn test_approve_access_request_grants_role() {
    // Setup test database
    let container = TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let pool = &container.pool;

    // Create a regular user without trusted-contact role
    let user = create_test_user(pool, "test@example.com", "password123", false).await;

    // Verify user doesn't have trusted-contact role initially
    let initial_roles = get_user_roles(pool, user.id).await;
    assert!(
        !initial_roles.contains(&"trusted-contact".to_string()),
        "User should not have trusted-contact role initially"
    );

    // Create an access request for trusted-contact role using builder
    let request = AccessRequestBuilder::new()
        .with_user_id(user.id)
        .with_message("I would like access please")
        .with_requested_role("trusted-contact")
        .pending()
        .persist(pool)
        .await
        .expect("Failed to create access request");

    // Create an admin user to approve the request
    let admin = create_test_user(pool, "admin@example.com", "password123", true).await;

    // Approve the access request using the repository method
    let access_request_repo = PostgresAccessRequestRepository::new(pool.clone());

    access_request_repo
        .approve_request(request.id, admin.id, Some("Approved!".to_string()))
        .await
        .expect("Failed to approve request");

    // Verify the request status is updated
    let updated_request = access_request_repo
        .get_request_by_id(request.id)
        .await
        .expect("Failed to fetch updated request")
        .expect("Request not found");

    assert_eq!(updated_request.status, "approved");
    assert_eq!(updated_request.admin_id, Some(admin.id));
    assert_eq!(updated_request.admin_reason, Some("Approved!".to_string()));

    // MOST IMPORTANT: Verify the user now has the trusted-contact role
    let final_roles = get_user_roles(pool, user.id).await;
    assert!(
        final_roles.contains(&"trusted-contact".to_string()),
        "User should have trusted-contact role after approval. Roles: {:?}",
        final_roles
    );
}

#[tokio::test]
async fn test_approve_access_request_idempotent() {
    // Setup test database
    let container = TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let pool = &container.pool;

    // Create a user who already has the trusted-contact role
    let user = create_test_user(pool, "test@example.com", "password123", false).await;

    // Grant trusted-contact role manually using raw query (since we don't have a role builder)
    sqlx::query(
        r#"
        INSERT INTO user_roles (user_id, role_id)
        SELECT $1, id FROM roles WHERE name = 'trusted-contact'
        "#,
    )
    .bind(user.id)
    .execute(pool)
    .await
    .expect("Failed to grant trusted-contact role");

    // Create an access request using builder
    let request = AccessRequestBuilder::new()
        .with_user_id(user.id)
        .with_message("I would like access please")
        .with_requested_role("trusted-contact")
        .pending()
        .persist(pool)
        .await
        .expect("Failed to create access request");

    // Create an admin user
    let admin = create_test_user(pool, "admin@example.com", "password123", true).await;

    // Approve the request (should not fail even though user already has the role)
    let access_request_repo = PostgresAccessRequestRepository::new(pool.clone());

    let result = access_request_repo
        .approve_request(request.id, admin.id, None)
        .await;

    assert!(
        result.is_ok(),
        "Approving request should succeed even if user already has the role"
    );

    // Verify user still has the role (no duplicates)
    let roles_count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM user_roles ur
        JOIN roles r ON ur.role_id = r.id
        WHERE ur.user_id = $1 AND r.name = 'trusted-contact'
        "#,
    )
    .bind(user.id)
    .fetch_one(pool)
    .await
    .expect("Failed to count roles");

    assert_eq!(
        roles_count, 1,
        "User should have exactly one trusted-contact role (no duplicates)"
    );
}

#[tokio::test]
async fn test_reject_access_request_does_not_grant_role() {
    // Setup test database
    let container = TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let pool = &container.pool;

    // Create a regular user
    let user = create_test_user(pool, "test@example.com", "password123", false).await;

    // Create an access request using builder
    let request = AccessRequestBuilder::new()
        .with_user_id(user.id)
        .with_message("I would like access please")
        .with_requested_role("trusted-contact")
        .pending()
        .persist(pool)
        .await
        .expect("Failed to create access request");

    // Create an admin user
    let admin = create_test_user(pool, "admin@example.com", "password123", true).await;

    // Reject the request
    let access_request_repo = PostgresAccessRequestRepository::new(pool.clone());

    access_request_repo
        .reject_request(request.id, admin.id, Some("Not appropriate".to_string()))
        .await
        .expect("Failed to reject request");

    // Verify the request status is updated
    let updated_request = access_request_repo
        .get_request_by_id(request.id)
        .await
        .expect("Failed to fetch updated request")
        .expect("Request not found");

    assert_eq!(updated_request.status, "rejected");

    // Verify the user does NOT have the trusted-contact role
    let roles = get_user_roles(pool, user.id).await;
    assert!(
        !roles.contains(&"trusted-contact".to_string()),
        "User should NOT have trusted-contact role after rejection"
    );
}
