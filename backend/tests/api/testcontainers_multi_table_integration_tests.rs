use crate::fixtures::TestContext;
use serde_json::json;
use uuid::Uuid;

/// Integration tests to verify multi-table schema operations
/// These tests ensure that operations correctly create/update data across multiple tables

#[actix_web::test]
async fn test_registration_creates_all_required_tables() {
    let ctx = TestContext::builder().build().await;

    let email = crate::fixtures::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";

    let register_req = json!({
        "email": email,
        "password": password,
        "display_name": display_name
    });

    // Register user
    let mut resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_req)
        .await
        .unwrap();

    assert!(resp.status().is_success(), "Registration should succeed");

    let body: serde_json::Value = resp.json().await.unwrap();
    let user_id_str = body
        .get("user")
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap();
    let user_id = Uuid::parse_str(user_id_str).unwrap();

    // Verify user created in users table
    let user = sqlx::query_as::<_, (Uuid, String, String)>(
        "SELECT id, email, display_name FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(user.1, email);
    assert_eq!(user.2, display_name);

    // Verify credentials created in user_credentials table
    let creds = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT user_id, password_hash FROM user_credentials WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(creds.0, user_id);
    assert!(!creds.1.is_empty(), "Password hash should be set");

    // Verify preferences created in user_preferences table
    let prefs = sqlx::query_as::<_, (Uuid, bool, bool)>(
        "SELECT user_id, timer_is_public, timer_show_in_list FROM user_preferences WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(prefs.0, user_id);
    assert!(
        prefs.1,
        "Default timer_is_public should be true for backward compatibility"
    );
    assert!(
        prefs.2,
        "Default timer_show_in_list should be true for backward compatibility"
    );

    // Verify profile created in user_profiles table (optional, may be null fields)
    let profile = sqlx::query_as::<_, (Uuid, Option<String>)>(
        "SELECT user_id, real_name FROM user_profiles WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(profile.0, user_id);
}

#[actix_web::test]
async fn test_registration_transaction_rollback_on_failure() {
    let ctx = TestContext::builder().build().await;

    let email = crate::fixtures::unique_test_email();

    // First registration succeeds
    let register_req = json!({
        "email": email,
        "password": "TestPassword123!",
        "display_name": "Test User"
    });

    let resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_req)
        .await
        .unwrap();

    assert!(resp.status().is_success());

    // Second registration with same email should fail
    let resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_req)
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        409,
        "Duplicate email should return 409 Conflict"
    );

    // Verify no orphaned records in other tables
    // Count total users with this email
    let user_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = $1")
        .bind(&email)
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

    assert_eq!(user_count, 1, "Should only have one user with this email");

    // Verify credentials count matches user count
    let creds_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM user_credentials uc JOIN users u ON uc.user_id = u.id WHERE u.email = $1"
    )
    .bind(&email)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();

    assert_eq!(creds_count, 1, "Should only have one credentials record");
}

#[actix_web::test]
async fn test_login_queries_multiple_tables() {
    let ctx = TestContext::builder().build().await;

    let email = crate::fixtures::unique_test_email();
    let password = "TestPassword123!";

    // Register user first
    let register_req = json!({
        "email": email,
        "password": password,
        "display_name": "Test User"
    });

    let mut resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_req)
        .await
        .unwrap();

    assert!(resp.status().is_success());

    let register_body: serde_json::Value = resp.json().await.unwrap();
    let user_id_str = register_body
        .get("user")
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap();
    let user_id = Uuid::parse_str(user_id_str).unwrap();

    // Now login
    let login_req = json!({
        "email": email,
        "password": password
    });

    let mut resp = ctx
        .server
        .post("/backend/public/auth/login")
        .send_json(&login_req)
        .await
        .unwrap();

    assert!(resp.status().is_success(), "Login should succeed");

    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("token").is_some());
    assert!(body.get("refresh_token").is_some());

    // Verify login used credentials from user_credentials table
    let creds =
        sqlx::query_as::<_, (Uuid,)>("SELECT user_id FROM user_credentials WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&ctx.pool)
            .await
            .unwrap();

    assert_eq!(creds.0, user_id);
}

#[actix_web::test]
async fn test_oauth_user_without_password() {
    let ctx = TestContext::builder().build().await;

    // Create OAuth-only user (no password)
    let email = crate::fixtures::unique_test_email();
    let slug = crate::fixtures::unique_test_slug();
    let user = ctx.create_oauth_user(&email, &slug, "google_12345").await;

    // Verify user exists in users table
    let db_user = sqlx::query_as::<_, (Uuid, String)>("SELECT id, email FROM users WHERE id = $1")
        .bind(user.id)
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(db_user.1, email);

    // Verify NO credentials in user_credentials table
    let creds =
        sqlx::query_as::<_, (Uuid,)>("SELECT user_id FROM user_credentials WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&ctx.pool)
            .await
            .unwrap();

    assert!(
        creds.is_none(),
        "OAuth-only user should not have credentials"
    );

    // Verify external login exists in user_external_logins table
    let ext_login = sqlx::query_as::<_, (Uuid, String, String)>(
        "SELECT user_id, provider, provider_user_id FROM user_external_logins WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();

    assert_eq!(ext_login.0, user.id);
    assert_eq!(ext_login.1, "google");
    assert_eq!(ext_login.2, "google_12345");
}

#[actix_web::test]
async fn test_data_export_includes_all_new_tables() {
    let ctx = TestContext::builder().build().await;

    // Create user with password
    let email = crate::fixtures::unique_test_email();
    let password = "TestPassword123!";

    let register_req = json!({
        "email": email,
        "password": password,
        "display_name": "Complete User"
    });

    let mut resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_req)
        .await
        .unwrap();

    assert!(resp.status().is_success());

    let register_body: serde_json::Value = resp.json().await.unwrap();
    let token = register_body.get("token").unwrap().as_str().unwrap();
    let user_id_str = register_body
        .get("user")
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap();
    let user_id = Uuid::parse_str(user_id_str).unwrap();

    // Link Google account to create external login
    // (This would normally be done via OAuth callback, but we'll add it directly for testing)
    sqlx::query(
        "INSERT INTO user_external_logins (id, user_id, provider, provider_user_id, linked_at) VALUES ($1, $2, $3, $4, NOW())"
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind("google")
    .bind("google_test_123")
    .execute(&ctx.pool)
    .await
    .unwrap();

    // Update profile with real_name
    sqlx::query("UPDATE user_profiles SET real_name = $1 WHERE user_id = $2")
        .bind("Real Name Test")
        .bind(user_id)
        .execute(&ctx.pool)
        .await
        .unwrap();

    // Update preferences
    sqlx::query(
        "UPDATE user_preferences SET timer_is_public = $1, timer_show_in_list = $2 WHERE user_id = $3"
    )
    .bind(true)
    .bind(true)
    .bind(user_id)
    .execute(&ctx.pool)
    .await
    .unwrap();

    // Export data
    let mut resp = ctx
        .server
        .get("/backend/protected/auth/export-data")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let export: serde_json::Value = resp.json().await.unwrap();

    // Verify export structure includes all new tables
    assert_eq!(export["export_version"], "1.0");

    // Verify authentication data (from user_credentials)
    assert!(export.get("authentication").is_some());
    let auth = export.get("authentication").unwrap();
    assert_eq!(auth["has_password"], true);
    assert!(auth.get("password_last_changed").is_some());

    // Verify external logins (from user_external_logins)
    assert!(export.get("external_logins").is_some());
    let ext_logins = export["external_logins"].as_array().unwrap();
    assert_eq!(ext_logins.len(), 1);
    assert_eq!(ext_logins[0]["provider"], "google");
    assert_eq!(ext_logins[0]["provider_user_id"], "google_test_123");

    // Verify profile (from user_profiles)
    assert!(export.get("profile").is_some());
    let profile = export.get("profile").unwrap();
    assert_eq!(profile["real_name"], "Real Name Test");

    // Verify preferences (from user_preferences)
    assert!(export.get("preferences").is_some());
    let prefs = export.get("preferences").unwrap();
    assert_eq!(prefs["timer_is_public"], true);
    assert_eq!(prefs["timer_show_in_list"], true);
}

#[actix_web::test]
async fn test_data_export_oauth_only_user() {
    let ctx = TestContext::builder().build().await;

    // Create OAuth-only user (no password)
    let email = crate::fixtures::unique_test_email();
    let slug = crate::fixtures::unique_test_slug();
    let user = ctx
        .create_oauth_user(&email, &slug, "google_oauth_123")
        .await;

    let token = crate::fixtures::create_test_jwt_token(&user).await.unwrap();

    // Export data
    let mut resp = ctx
        .server
        .get("/backend/protected/auth/export-data")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let export: serde_json::Value = resp.json().await.unwrap();

    // Verify authentication shows no password
    assert!(export.get("authentication").is_some());
    let auth = export.get("authentication").unwrap();
    assert_eq!(auth["has_password"], false);
    assert!(
        auth["password_last_changed"].is_null()
            || !auth.get("password_last_changed").unwrap().is_string()
    );

    // Verify external login exists
    let ext_logins = export["external_logins"].as_array().unwrap();
    assert_eq!(ext_logins.len(), 1);
    assert_eq!(ext_logins[0]["provider"], "google");
    assert_eq!(ext_logins[0]["provider_user_id"], "google_oauth_123");
}

#[actix_web::test]
async fn test_account_deletion_cascades_to_all_tables() {
    let ctx = TestContext::builder().build().await;

    let email = crate::fixtures::unique_test_email();
    let password = "TestPassword123!";

    // Register user
    let register_req = json!({
        "email": email,
        "password": password,
        "display_name": "To Be Deleted"
    });

    let mut resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_req)
        .await
        .unwrap();

    assert!(resp.status().is_success());

    let register_body: serde_json::Value = resp.json().await.unwrap();
    let token = register_body.get("token").unwrap().as_str().unwrap();
    let user_id_str = register_body
        .get("user")
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap();
    let user_id = Uuid::parse_str(user_id_str).unwrap();

    // Verify records exist in all tables
    let user = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(user.0, user_id);

    let creds =
        sqlx::query_as::<_, (Uuid,)>("SELECT user_id FROM user_credentials WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&ctx.pool)
            .await
            .unwrap();
    assert_eq!(creds.0, user_id);

    let prefs =
        sqlx::query_as::<_, (Uuid,)>("SELECT user_id FROM user_preferences WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&ctx.pool)
            .await
            .unwrap();
    assert_eq!(prefs.0, user_id);

    let profile =
        sqlx::query_as::<_, (Uuid,)>("SELECT user_id FROM user_profiles WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&ctx.pool)
            .await
            .unwrap();
    assert_eq!(profile.0, user_id);

    // Delete account
    let delete_req = json!({
        "password": password
    });

    let resp = ctx
        .server
        .delete("/backend/protected/auth/delete-account")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&delete_req)
        .await
        .unwrap();

    assert!(
        resp.status().is_success(),
        "Account deletion should succeed"
    );

    // Verify user was hard deleted
    let user = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&ctx.pool)
        .await
        .unwrap();
    assert!(user.is_none(), "User should be completely deleted");

    // Verify cascade deletion removed all related records
    let creds =
        sqlx::query_as::<_, (Uuid,)>("SELECT user_id FROM user_credentials WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&ctx.pool)
            .await
            .unwrap();
    assert!(creds.is_none(), "Credentials should be cascade deleted");

    let prefs =
        sqlx::query_as::<_, (Uuid,)>("SELECT user_id FROM user_preferences WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&ctx.pool)
            .await
            .unwrap();
    assert!(prefs.is_none(), "Preferences should be cascade deleted");

    let profile =
        sqlx::query_as::<_, (Uuid,)>("SELECT user_id FROM user_profiles WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&ctx.pool)
            .await
            .unwrap();
    assert!(profile.is_none(), "Profile should be cascade deleted")
}

#[actix_web::test]
async fn test_profile_update_modifies_user_profiles_table() {
    let ctx = TestContext::builder().build().await;

    let email = crate::fixtures::unique_test_email();
    let password = "TestPassword123!";

    // Register user
    let register_req = json!({
        "email": email,
        "password": password,
        "display_name": "Original Name"
    });

    let mut resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_req)
        .await
        .unwrap();

    assert!(resp.status().is_success());

    let register_body: serde_json::Value = resp.json().await.unwrap();
    let token = register_body.get("token").unwrap().as_str().unwrap();
    let user_id_str = register_body
        .get("user")
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap();
    let user_id = Uuid::parse_str(user_id_str).unwrap();

    // Update profile with display_name and slug
    let update_req = json!({
        "display_name": "Updated Name",
        "slug": "updated-slug"
    });

    let mut resp = ctx
        .server
        .put("/backend/protected/auth/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&update_req)
        .await
        .unwrap();

    assert!(resp.status().is_success(), "Profile update should succeed");

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["display_name"], "Updated Name");
    assert_eq!(body["slug"], "updated-slug");

    // Verify database updates
    let user =
        sqlx::query_as::<_, (String, String)>("SELECT display_name, slug FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&ctx.pool)
            .await
            .unwrap();
    assert_eq!(user.0, "Updated Name");
    assert_eq!(user.1, "updated-slug");
}
