use actix_web::{App, test, web};
use serial_test::serial;
use uuid::Uuid;

use configurator_service::{
    api::config_network_routes,
    application::NetworkApplicationService,
    infrastructure::{PostgresNetworkRepository, create_pool_from_env},
};

fn setup_test_env() {
    dotenvy::dotenv().ok();
}

// Test helper to clean up test data
async fn cleanup_test_data() {
    setup_test_env();
    let pool = create_pool_from_env()
        .await
        .expect("Failed to create cleanup pool");

    sqlx::query("DELETE FROM networks WHERE name LIKE 'Test API Network%' OR created_at > NOW() - INTERVAL '1 hour'")
        .execute(&pool)
        .await
        .expect("Failed to clean up test data");
}

// Helper to create service data
async fn create_service_data() -> web::Data<NetworkApplicationService<PostgresNetworkRepository>> {
    setup_test_env();
    let pool = create_pool_from_env()
        .await
        .expect("Failed to create test database pool");

    let repository = PostgresNetworkRepository::new(pool);
    let service = NetworkApplicationService::new(repository);
    web::Data::new(service)
}

// Helper to extract network ID from response with better debugging
fn extract_network_id(body: &serde_json::Value) -> String {
    println!("\n=== DEBUG RESPONSE ===");
    println!("Full response: {}", body);
    println!(
        "Response type: {}",
        body.as_object().map(|_| "object").unwrap_or("not object")
    );

    if let Some(obj) = body.as_object() {
        println!("Available fields: {:?}", obj.keys().collect::<Vec<_>>());

        for (key, value) in obj {
            println!(
                "Field '{}': {:?} (type: {})",
                key,
                value,
                value.as_str().map(|_| "string").unwrap_or("other")
            );
        }
    }

    // Try different possible field names
    if let Some(network_id) = body["network_id"].as_str() {
        println!("Found network_id: {}", network_id);
        return network_id.to_string();
    }

    if let Some(network_id) = body["id"].as_str() {
        println!("Found id: {}", network_id);
        return network_id.to_string();
    }

    if let Some(network_id) = body["uuid"].as_str() {
        println!("Found uuid: {}", network_id);
        return network_id.to_string();
    }

    // Check if it's an error response
    if let Some(error) = body["error"].as_str() {
        println!("Error response: {}", error);
    }

    panic!("Failed to extract network_id from response");
}

#[actix_web::test]
#[serial]
async fn debug_api_response() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data.clone())
            .configure(config_network_routes),
    )
    .await;

    // Test simple create request
    let create_body = serde_json::json!({
        "name": "Debug Network",
        "network_type": "COMPANY"
    });

    println!("=== DEBUG: Testing API Response ===");
    let create_req = test::TestRequest::post()
        .uri("/api/networks")
        .set_json(&create_body)
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    println!("Status: {}", create_resp.status());

    // Read response as bytes first to see raw content
    let body_bytes = actix_web::body::to_bytes(create_resp.into_body())
        .await
        .unwrap();
    let body_str = String::from_utf8_lossy(&body_bytes);
    println!("Raw response: {}", body_str);

    // Try to parse as JSON
    match serde_json::from_str::<serde_json::Value>(&body_str) {
        Ok(parsed) => {
            println!("Parsed JSON: {}", parsed);
            println!(
                "JSON keys: {:?}",
                parsed.as_object().map(|obj| obj.keys().collect::<Vec<_>>())
            );
        }
        Err(e) => {
            println!("Failed to parse as JSON: {}", e);
            println!("Response might be HTML or plain text");
        }
    }
}

#[actix_web::test]
#[serial]
async fn test_create_network_success() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data)
            .configure(config_network_routes),
    )
    .await;

    let request_body = serde_json::json!({
        "name": "Test API Network",
        "network_type": "COMPANY"
    });

    let req = test::TestRequest::post()
        .uri("/api/networks")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;

    // Check if we got a successful response with expected fields
    assert!(body.is_object(), "Response should be a JSON object");
    assert!(
        body.get("network_id").is_some() || body.get("id").is_some(),
        "Response should contain network_id or id field"
    );

    // Extract network_id using our helper
    let _network_id = extract_network_id(&body);

    // Verify other fields
    assert_eq!(body["name"], "Test API Network");
    assert_eq!(body["network_type"], "COMPANY");
    assert_eq!(body["is_verified"], false);
    assert_eq!(body["is_active"], true);
    assert_eq!(body["is_live"], true);
}

#[actix_web::test]
#[serial]
async fn test_create_network_without_name() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data)
            .configure(config_network_routes),
    )
    .await;

    let request_body = serde_json::json!({
        "network_type": "INDIVIDUAL"
    });

    let req = test::TestRequest::post()
        .uri("/api/networks")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["name"].is_null());
    assert_eq!(body["network_type"], "INDIVIDUAL");
}

#[actix_web::test]
#[serial]
async fn test_get_network_success() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data.clone())
            .configure(config_network_routes),
    )
    .await;

    // First create a network
    let create_body = serde_json::json!({
        "name": "Test API Network - Get",
        "network_type": "COMPANY"
    });

    println!("=== SENDING CREATE REQUEST ===");
    let create_req = test::TestRequest::post()
        .uri("/api/networks")
        .set_json(&create_body)
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    println!("Create response status: {}", create_resp.status());

    let created_network: serde_json::Value = test::read_body_json(create_resp).await;
    let network_id = extract_network_id(&created_network);

    println!("=== SENDING GET REQUEST ===");
    let get_req = test::TestRequest::get()
        .uri(&format!("/api/networks/{}", network_id))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;
    println!("Get response status: {}", get_resp.status());

    let body: serde_json::Value = test::read_body_json(get_resp).await;
    let retrieved_network_id = extract_network_id(&body);

    assert_eq!(retrieved_network_id, network_id);
    assert_eq!(body["name"], "Test API Network - Get");
    assert_eq!(body["network_type"], "COMPANY");
}

#[actix_web::test]
#[serial]
async fn test_get_network_not_found() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data)
            .configure(config_network_routes),
    )
    .await;

    let non_existent_id = Uuid::new_v4();
    let req = test::TestRequest::get()
        .uri(&format!("/api/networks/{}", non_existent_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);

    let body: serde_json::Value = test::read_body_json(resp).await;
    // Check for either error format - your actual error response might be different
    let error_field = body["error"].as_str().unwrap_or("");
    assert!(error_field.contains("Not Found") || error_field.contains("404"));
}

#[actix_web::test]
#[serial]
async fn test_list_networks() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data.clone())
            .configure(config_network_routes),
    )
    .await;

    // Create multiple networks
    let networks = vec![
        serde_json::json!({
            "name": "Test API Network - List 1",
            "network_type": "INDIVIDUAL"
        }),
        serde_json::json!({
            "name": "Test API Network - List 2",
            "network_type": "COMPANY"
        }),
    ];

    for network_body in &networks {
        let req = test::TestRequest::post()
            .uri("/api/networks")
            .set_json(network_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);
    }

    // List all networks
    let list_req = test::TestRequest::get().uri("/api/networks").to_request();

    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(list_resp.status(), 200);

    let body: Vec<serde_json::Value> = test::read_body_json(list_resp).await;

    // We might have other networks, so filter for our test ones
    let test_networks: Vec<_> = body
        .into_iter()
        .filter(|n| {
            n["name"]
                .as_str()
                .map(|name| name.starts_with("Test API Network - List"))
                .unwrap_or(false)
        })
        .collect();

    assert_eq!(test_networks.len(), 2);

    let has_individual = test_networks
        .iter()
        .any(|n| n["network_type"] == "INDIVIDUAL");
    let has_company = test_networks.iter().any(|n| n["network_type"] == "COMPANY");

    assert!(has_individual);
    assert!(has_company);
}

#[actix_web::test]
#[serial]
async fn test_verify_network_success() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data.clone())
            .configure(config_network_routes),
    )
    .await;

    // First create a network
    let create_body = serde_json::json!({
        "name": "Test API Network - Verify",
        "network_type": "COMPANY"
    });

    let create_req = test::TestRequest::post()
        .uri("/api/networks")
        .set_json(&create_body)
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), 201);

    let created_network: serde_json::Value = test::read_body_json(create_resp).await;
    let network_id = extract_network_id(&created_network);

    // Verify the network
    let verify_body = serde_json::json!({
        "verified_by": Uuid::new_v4().to_string()
    });

    let verify_req = test::TestRequest::post()
        .uri(&format!("/api/networks/{}/verify", network_id))
        .set_json(&verify_body)
        .to_request();

    let verify_resp = test::call_service(&app, verify_req).await;
    assert_eq!(verify_resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(verify_resp).await;
    assert_eq!(extract_network_id(&body), network_id);
    assert_eq!(body["is_verified"], true);
    assert!(body["updated_by"].is_string());
    assert!(body["updated_at"].is_string());
}

#[actix_web::test]
#[serial]
async fn test_verify_network_not_found() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data)
            .configure(config_network_routes),
    )
    .await;

    let non_existent_id = Uuid::new_v4();
    let verify_body = serde_json::json!({
        "verified_by": Uuid::new_v4().to_string()
    });

    let req = test::TestRequest::post()
        .uri(&format!("/api/networks/{}/verify", non_existent_id))
        .set_json(&verify_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
#[serial]
async fn test_delete_network_success() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data.clone())
            .configure(config_network_routes),
    )
    .await;

    // First create a network
    let create_body = serde_json::json!({
        "name": "Test API Network - Delete",
        "network_type": "INDIVIDUAL"
    });

    let create_req = test::TestRequest::post()
        .uri("/api/networks")
        .set_json(&create_body)
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), 201);

    let created_network: serde_json::Value = test::read_body_json(create_resp).await;
    let network_id = extract_network_id(&created_network);

    // Delete the network
    let delete_req = test::TestRequest::delete()
        .uri(&format!("/api/networks/{}", network_id))
        .to_request();

    let delete_resp = test::call_service(&app, delete_req).await;
    assert_eq!(delete_resp.status(), 204);

    // Verify it's gone
    let get_req = test::TestRequest::get()
        .uri(&format!("/api/networks/{}", network_id))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;
    assert_eq!(get_resp.status(), 404);
}

#[actix_web::test]
#[serial]
async fn test_delete_network_not_found() {
    setup_test_env();
    cleanup_test_data().await;

    let service_data = create_service_data().await;
    let app = test::init_service(
        App::new()
            .app_data(service_data)
            .configure(config_network_routes),
    )
    .await;

    let non_existent_id = Uuid::new_v4();
    let req = test::TestRequest::delete()
        .uri(&format!("/api/networks/{}", non_existent_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
