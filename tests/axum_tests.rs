#![cfg(feature = "axum")]

use axum::response::IntoResponse;
use http::StatusCode;
use typed_error::AppError;

#[tokio::test]
async fn app_error_converts_to_response() {
    let error = AppError::not_found("USER_NOT_FOUND").message("user not found");

    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn validation_error_maps_to_bad_request() {
    let error = AppError::validation("INVALID_EMAIL");

    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn external_error_maps_to_bad_gateway() {
    let error = AppError::external("PAYMENT_FAILED");

    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
}
