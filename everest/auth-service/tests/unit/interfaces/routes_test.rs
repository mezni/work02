use actix_web::{test, App};
use auth_service::interfaces::routes::configure_routes;

#[actix_web::test]
async fn test_all_routes_configured() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Test health endpoint
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test Swagger UI endpoint
    let req = test::TestRequest::get().uri("/swagger-ui/").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test auth endpoints exist (they should return errors without proper setup)
    let auth_endpoints = [
        "/api/v1/auth/register",
        "/api/v1/auth/login",
        "/api/v1/auth/refresh",
        "/api/v1/auth/validate",
        "/api/v1/auth/logout",
    ];

    for endpoint in auth_endpoints.iter() {
        let req = test::TestRequest::post().uri(endpoint).to_request();
        let resp = test::call_service(&app, req).await;
        // These should return some kind of response (not 404)
        assert!(resp.status() != actix_web::http::StatusCode::NOT_FOUND);
    }

    // Test user endpoints exist
    let user_endpoints = [
        "/api/v1/users",
        "/api/v1/users/", // Test with trailing slash
    ];

    for endpoint in user_endpoints.iter() {
        let req = test::TestRequest::get().uri(endpoint).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status() != actix_web::http::StatusCode::NOT_FOUND);
    }

    // Test company endpoints exist
    let company_endpoints = [
        "/api/v1/companies",
        "/api/v1/companies/", // Test with trailing slash
    ];

    for endpoint in company_endpoints.iter() {
        let req = test::TestRequest::get().uri(endpoint).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status() != actix_web::http::StatusCode::NOT_FOUND);
    }
}

#[actix_web::test]
async fn test_api_version_prefix() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    // Test that all API routes are under /api/v1 prefix
    let non_api_req = test::TestRequest::get()
        .uri("/users") // Without /api/v1 prefix
        .to_request();
    let resp = test::call_service(&app, non_api_req).await;
    // This should be 404 since routes are under /api/v1
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);

    let api_req = test::TestRequest::get()
        .uri("/api/v1/users") // With /api/v1 prefix
        .to_request();
    let resp = test::call_service(&app, api_req).await;
    // This should not be 404
    assert!(resp.status() != actix_web::http::StatusCode::NOT_FOUND);
}
