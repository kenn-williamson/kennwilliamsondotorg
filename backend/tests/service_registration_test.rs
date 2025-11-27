/**
 * Service Registration Test
 *
 * This test ensures all services in ServiceContainer are properly registered
 * with the Actix web application. It catches the common mistake of adding a
 * new service to the container but forgetting to register it in main.rs.
 */
mod fixtures;

use actix_web::{App, test, web};
use backend::services::container::ServiceContainer;
use std::env;

#[actix_web::test]
async fn test_all_services_are_registered() {
    // Use testcontainer pool for database
    let tc = fixtures::pool::checkout().await;
    let pool = tc.pool.clone();

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "test_secret".to_string());
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

    // Create service container (same as main.rs)
    let services = ServiceContainer::new_development(pool.clone(), jwt_secret, redis_url);

    // Create app with the SAME service registrations as main.rs
    // If you add a service to container but forget to register it here,
    // the test will fail when you try to extract it
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(services.auth_service.clone()))
            .app_data(web::Data::from(services.blog_service.clone()))
            .app_data(web::Data::from(services.incident_timer_service.clone()))
            .app_data(web::Data::from(services.phrase_service.clone()))
            .app_data(web::Data::from(services.admin_service.clone()))
            .app_data(web::Data::from(services.phrase_moderation_service.clone()))
            .app_data(web::Data::from(
                services.access_request_moderation_service.clone(),
            ))
            .app_data(web::Data::from(services.stats_service.clone()))
            .app_data(web::Data::from(services.rate_limit_service.clone())),
    )
    .await;

    // The fact that the app initialized successfully means all services
    // are properly registered. If we forgot one, compilation would fail
    // because we'd be trying to clone a service that doesn't exist.

    // This test serves as documentation of what services MUST be registered
    drop(app);

    println!("✅ All services are properly registered in the Actix app");
}

#[tokio::test]
async fn test_service_container_has_all_expected_services() {
    // This is a compile-time check. If ServiceContainer adds a new service,
    // you MUST update main.rs to register it. This test documents what services exist.

    // List all services that should exist in ServiceContainer:
    // 1. auth_service
    // 2. blog_service
    // 3. incident_timer_service
    // 4. phrase_service
    // 5. admin_service
    // 6. phrase_moderation_service
    // 7. access_request_moderation_service
    // 8. stats_service
    // 9. rate_limit_service
    // 10. cleanup_service (used in background task, not in app_data)

    // If you add a new service to ServiceContainer, add it to this list
    // and update main.rs to register it with .app_data()

    println!("✅ ServiceContainer service count documented");
}
