use std::time::Duration;

use typed_error::{AppError, ErrorDetails, ErrorKind, ErrorResponse, RetryHint};

#[test]
fn creates_response_with_new() {
    let response = ErrorResponse::new(
        ErrorKind::Validation,
        "INVALID_EMAIL",
        "email format is invalid",
        ErrorDetails::new(),
        false,
        None,
    );

    assert_eq!(response.kind, ErrorKind::Validation);
    assert_eq!(response.code, "INVALID_EMAIL");
    assert_eq!(response.message, "email format is invalid");
    assert!(response.details.is_empty());
    assert!(!response.retryable);
    assert_eq!(response.retry_after_ms, None);
}

#[test]
fn converts_from_app_error_by_reference() {
    let error = AppError::external("PAYMENT_GATEWAY_TIMEOUT")
        .message("payment gateway did not respond in time")
        .detail("provider", "stripe")
        .detail("attempt", 3_u32)
        .retry_hint(RetryHint::After(Duration::from_secs(2)));

    let response = ErrorResponse::from(&error);

    assert_eq!(response.kind, ErrorKind::External);
    assert_eq!(response.code, "PAYMENT_GATEWAY_TIMEOUT");
    assert_eq!(response.message, "payment gateway did not respond in time");
    assert_eq!(
        response.details.get("provider").map(|v| v.to_string()),
        Some("stripe".to_string())
    );
    assert_eq!(
        response.details.get("attempt").map(|v| v.to_string()),
        Some("3".to_string())
    );
    assert!(response.retryable);
    assert_eq!(response.retry_after_ms, Some(2000));
}

#[test]
fn converts_from_owned_app_error() {
    let error = AppError::not_found("USER_NOT_FOUND")
        .message("user was not found")
        .detail("user_id", 42_u64)
        .retry_hint(RetryHint::Never);

    let response = ErrorResponse::from(error);

    assert_eq!(response.kind, ErrorKind::NotFound);
    assert_eq!(response.code, "USER_NOT_FOUND");
    assert_eq!(response.message, "user was not found");
    assert_eq!(
        response.details.get("user_id").map(|v| v.to_string()),
        Some("42".to_string())
    );
    assert!(!response.retryable);
    assert_eq!(response.retry_after_ms, None);
}

#[test]
fn response_clones_details_when_borrowed_conversion_is_used() {
    let error = AppError::validation("EMAIL_REQUIRED")
        .message("email is required")
        .detail("field", "email");

    let response = ErrorResponse::from(&error);

    assert_eq!(
        response.details.get("field").map(|v| v.to_string()),
        Some("email".to_string())
    );
    assert_eq!(
        error.details_map().get("field").map(|v| v.to_string()),
        Some("email".to_string())
    );
}

#[cfg(feature = "serde")]
#[test]
fn serializes_to_json() {
    let error = AppError::validation("INVALID_EMAIL")
        .message("email format is invalid")
        .detail("field", "email");

    let response = ErrorResponse::from(error);
    let json = serde_json::to_string(&response).expect("serialization should succeed");

    assert!(json.contains("\"code\":\"INVALID_EMAIL\""));
    assert!(json.contains("\"message\":\"email format is invalid\""));
    assert!(json.contains("\"retryable\":false"));
}
