use actix_web::{http::StatusCode, test};
use auth_service::application::errors::ApplicationError;
use auth_service::interfaces::errors::{InterfaceError, WebResult};

#[test]
fn test_interface_error_codes() {
    let auth_error = InterfaceError::ApplicationError(ApplicationError::AuthenticationFailed);
    assert_eq!(auth_error.code(), "APP_AUTHENTICATION_FAILED");

    let validation_error = InterfaceError::ValidationError("Test error".to_string());
    assert_eq!(validation_error.to_string(), "Validation error: Test error");
}

#[test]
fn test_interface_error_responses() {
    let auth_error = InterfaceError::ApplicationError(ApplicationError::AuthenticationFailed);
    let response = auth_error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let validation_error = InterfaceError::ValidationError("Test error".to_string());
    let response = validation_error.error_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let not_found_error = InterfaceError::NotFound;
    let response = not_found_error.error_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let forbidden_error = InterfaceError::InsufficientPermissions;
    let response = forbidden_error.error_response();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    let internal_error = InterfaceError::InternalServerError;
    let response = internal_error.error_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_web_result_type() {
    let success_result: WebResult<i32> = Ok(42);
    assert!(success_result.is_ok());

    let error_result: WebResult<i32> = Err(InterfaceError::NotFound);
    assert!(error_result.is_err());
}

#[test]
fn test_error_messages() {
    let auth_required = InterfaceError::AuthenticationRequired;
    assert_eq!(auth_required.to_string(), "Authentication required");

    let bad_request = InterfaceError::BadRequest("Invalid input".to_string());
    assert_eq!(bad_request.to_string(), "Bad request: Invalid input");

    let app_error = InterfaceError::ApplicationError(ApplicationError::UserNotFound);
    assert_eq!(app_error.to_string(), "Application error: User not found");
}
