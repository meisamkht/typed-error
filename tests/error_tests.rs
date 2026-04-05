use std::error::Error;
use std::io;
use std::time::Duration;

use typed_error::{AppError, ErrorKind, RetryHint};

#[test]
fn creates_error_with_new() {
    let error = AppError::new(ErrorKind::Validation, "INVALID_INPUT");

    assert_eq!(error.kind(), ErrorKind::Validation);
    assert_eq!(error.code().as_str(), "INVALID_INPUT");
    assert_eq!(error.message_str(), "");
    assert!(error.details_map().is_empty());
    assert_eq!(error.retry_hint_ref(), &RetryHint::Never);
    assert!(!error.is_retryable());
}

#[test]
fn helper_constructor_creates_expected_kind() {
    let error = AppError::not_found("USER_NOT_FOUND");
    assert_eq!(error.kind(), ErrorKind::NotFound);
    assert_eq!(error.code().as_str(), "USER_NOT_FOUND");
}

#[test]
fn message_sets_human_readable_text() {
    let error = AppError::internal("INTERNAL_ERROR").message("unexpected internal failure");

    assert_eq!(error.message_str(), "unexpected internal failure");
}

#[test]
fn detail_adds_structured_metadata() {
    let error = AppError::validation("EMAIL_REQUIRED")
        .detail("field", "email")
        .detail("attempt", 2_u32);

    assert_eq!(
        error.details_map().get("field").map(|v| v.to_string()),
        Some("email".to_string())
    );
    assert_eq!(
        error.details_map().get("attempt").map(|v| v.to_string()),
        Some("2".to_string())
    );
}

#[test]
fn details_adds_multiple_entries() {
    let entries = vec![
        ("field".to_string(), "email".into()),
        ("retryable".to_string(), false.into()),
    ];

    let error = AppError::validation("INVALID_EMAIL").details(entries);

    assert_eq!(
        error.details_map().get("field").map(|v| v.to_string()),
        Some("email".to_string())
    );
    assert_eq!(
        error.details_map().get("retryable").map(|v| v.to_string()),
        Some("false".to_string())
    );
}

#[test]
fn retry_hint_is_exposed() {
    let error =
        AppError::external("PAYMENT_TIMEOUT").retry_hint(RetryHint::After(Duration::from_secs(5)));

    assert!(error.is_retryable());
    assert_eq!(
        error.retry_hint_ref().retry_after(),
        Some(Duration::from_secs(5))
    );
}

#[test]
fn source_error_is_preserved() {
    let source = io::Error::new(io::ErrorKind::NotFound, "file missing");

    let error = AppError::internal("CONFIG_READ_FAILED")
        .message("failed to read config")
        .source_error(source);

    let source = error.source().expect("expected source error");
    assert_eq!(source.to_string(), "file missing");
}

#[test]
fn display_uses_code_and_message_when_message_exists() {
    let error = AppError::not_found("USER_NOT_FOUND").message("user was not found");

    assert_eq!(error.to_string(), "USER_NOT_FOUND: user was not found");
}

#[test]
fn display_uses_code_and_kind_when_message_is_empty() {
    let error = AppError::internal("INTERNAL_ERROR");

    assert_eq!(error.to_string(), "INTERNAL_ERROR (internal)");
}

#[test]
fn into_parts_returns_owned_components() {
    let error = AppError::conflict("USER_EXISTS")
        .message("user already exists")
        .detail("user_id", 42_u64)
        .retry_hint(RetryHint::Safe);

    let parts = error.into_parts();

    assert_eq!(parts.kind, ErrorKind::Conflict);
    assert_eq!(parts.code.as_str(), "USER_EXISTS");
    assert_eq!(parts.message, "user already exists");
    assert_eq!(
        parts.details.get("user_id").map(|v| v.to_string()),
        Some("42".to_string())
    );
    assert_eq!(parts.retry_hint, RetryHint::Safe);
    assert!(parts.source.is_none());
}
