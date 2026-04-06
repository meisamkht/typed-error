#![cfg(feature = "tonic")]

use tonic::{Code, Status};
use typed_error::AppError;

#[test]
fn not_found_error_converts_to_grpc_status() {
    let error = AppError::not_found("USER_NOT_FOUND").message("user was not found");

    let status: Status = error.into();

    assert_eq!(status.code(), Code::NotFound);
    assert_eq!(status.message(), "USER_NOT_FOUND: user was not found");
}

#[test]
fn validation_error_converts_to_invalid_argument() {
    let error = AppError::validation("INVALID_EMAIL").message("email format is invalid");

    let status: Status = error.into();

    assert_eq!(status.code(), Code::InvalidArgument);
    assert_eq!(status.message(), "INVALID_EMAIL: email format is invalid");
}

#[test]
fn external_error_converts_to_unavailable() {
    let error =
        AppError::external("PAYMENT_PROVIDER_FAILED").message("payment provider is unavailable");

    let status: Status = error.into();

    assert_eq!(status.code(), Code::Unavailable);
    assert_eq!(
        status.message(),
        "PAYMENT_PROVIDER_FAILED: payment provider is unavailable"
    );
}

#[test]
fn borrowed_app_error_can_convert_to_status() {
    let error = AppError::internal("INTERNAL_ERROR").message("unexpected internal failure");

    let status = Status::from(&error);

    assert_eq!(status.code(), Code::Internal);
    assert_eq!(
        status.message(),
        "INTERNAL_ERROR: unexpected internal failure"
    );
}

#[test]
fn cancelled_error_maps_to_cancelled() {
    let error = AppError::cancelled("REQUEST_CANCELLED").message("request was cancelled");

    let status: Status = error.into();

    assert_eq!(status.code(), Code::Cancelled);
    assert_eq!(status.message(), "REQUEST_CANCELLED: request was cancelled");
}
