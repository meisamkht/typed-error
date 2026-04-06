#![cfg(feature = "http")]

use http::StatusCode;
use typed_error::{AppError, ErrorKind, HttpErrorExt};

#[test]
fn error_kind_maps_to_expected_http_statuses() {
    assert_eq!(ErrorKind::Validation.http_status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        ErrorKind::Unauthorized.http_status(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(ErrorKind::Forbidden.http_status(), StatusCode::FORBIDDEN);
    assert_eq!(ErrorKind::NotFound.http_status(), StatusCode::NOT_FOUND);
    assert_eq!(ErrorKind::Conflict.http_status(), StatusCode::CONFLICT);
    assert_eq!(
        ErrorKind::RateLimited.http_status(),
        StatusCode::TOO_MANY_REQUESTS
    );
    assert_eq!(
        ErrorKind::Timeout.http_status(),
        StatusCode::REQUEST_TIMEOUT
    );
    assert_eq!(ErrorKind::Cancelled.http_status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        ErrorKind::Unavailable.http_status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
    assert_eq!(ErrorKind::External.http_status(), StatusCode::BAD_GATEWAY);
    assert_eq!(
        ErrorKind::Internal.http_status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn app_error_maps_using_its_kind() {
    let error = AppError::not_found("USER_NOT_FOUND").message("user was not found");

    assert_eq!(error.http_status(), StatusCode::NOT_FOUND);
}

#[test]
fn app_error_validation_maps_to_bad_request() {
    let error = AppError::validation("INVALID_EMAIL");

    assert_eq!(error.http_status(), StatusCode::BAD_REQUEST);
}

#[test]
fn app_error_external_maps_to_bad_gateway() {
    let error = AppError::external("PAYMENT_PROVIDER_FAILED");

    assert_eq!(error.http_status(), StatusCode::BAD_GATEWAY);
}
