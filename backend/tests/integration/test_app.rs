use actix_web::{web, App};
use anyhow::Result;
use actix_test;

use backend::services::container::ServiceContainer;
use backend::routes;
use crate::integration::test_database::TestDatabase;

/// Create a test application with isolated database
pub async fn create_test_app(test_name: &str) -> Result<(actix_test::TestServer, sqlx::PgPool)> {
    let test_db = TestDatabase::new(test_name).await?;
    
    // Create service container with test database
    let jwt_secret = "test-jwt-secret-for-integration-tests".to_string();
    let container = ServiceContainer::new_development(test_db.pool.clone(), jwt_secret);
    
    // Create test app with all services
    let pool_clone = test_db.pool.clone();
    let srv = actix_test::start(move || {
        App::new()
            .app_data(web::Data::new(pool_clone.clone()))
            .app_data(web::Data::from(container.auth_service.clone()))
            .app_data(web::Data::from(container.incident_timer_service.clone()))
            .app_data(web::Data::from(container.phrase_service.clone()))
            .app_data(web::Data::from(container.admin_service.clone()))
            .app_data(web::Data::from(container.phrase_moderation_service.clone()))
            .app_data(web::Data::from(container.stats_service.clone()))
            .configure(routes::configure_app_routes)
    });
    
    Ok((srv, test_db.pool.clone()))
}

/// Create a test app with a specific user pre-created
pub async fn create_test_app_with_user(
    test_name: &str,
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<(actix_test::TestServer, sqlx::PgPool, backend::models::db::user::User)> {
    let (srv, pool) = create_test_app(test_name).await?;

    // Create test user directly in database
    let user = crate::test_helpers::create_test_user_in_db(
        &pool,
        email,
        password_hash,
        display_name,
        slug,
    ).await?;

    Ok((srv, pool, user))
}

/// Create a test app with admin user pre-created
pub async fn create_test_app_with_admin_user(
    test_name: &str,
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<(actix_test::TestServer, sqlx::PgPool, backend::models::db::user::User)> {
    let (srv, pool) = create_test_app(test_name).await?;

    // Create test user
    let user = crate::test_helpers::create_test_user_in_db(
        &pool,
        email,
        password_hash,
        display_name,
        slug,
    ).await?;

    // Add admin role
    add_admin_role_to_user(&pool, user.id).await?;

    Ok((srv, pool, user))
}

/// Add admin role to a user
async fn add_admin_role_to_user(pool: &sqlx::PgPool, user_id: uuid::Uuid) -> Result<()> {
    // Get admin role ID
    let admin_role_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM roles WHERE name = 'admin'"
    )
    .fetch_one(pool)
    .await?;
    
    // Add user-role relationship
    sqlx::query(
        "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)"
    )
    .bind(user_id)
    .bind(admin_role_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Helper to create JWT token for testing
pub async fn create_test_jwt_token(
    user: &backend::models::db::user::User,
) -> Result<String> {
    use backend::services::auth::jwt::JwtService;
    
    let jwt_secret = "test-jwt-secret-for-integration-tests".to_string();
    let jwt_service = JwtService::new(jwt_secret);
    
    let token = jwt_service.generate_token(user)?;
    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[actix_web::test]
    async fn test_app_creation() {
        let (app, _pool) = create_test_app("test_app_creation").await.unwrap();

        // Test that we can make a request
        let resp = app.get("/backend/public/health").send().await.unwrap();
        assert!(resp.status().is_success());
    }
    
    #[actix_web::test]
    async fn test_app_with_user_creation() {
        let (_app, _pool, user) = create_test_app_with_user(
            "test_app_with_user_creation",
            &crate::test_helpers::unique_test_email(),
            &crate::test_helpers::test_password_hash(),
            "Test User",
            &crate::test_helpers::unique_test_slug(),
        ).await.unwrap();

        assert_eq!(user.display_name, "Test User");
    }
}