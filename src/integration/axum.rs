use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;

use crate::{AppError, ErrorResponse, HttpErrorExt};

/// Axum integration for [`AppError`].
///
/// This implementation allows returning `AppError` directly from Axum handlers:
///
/// ```rust
/// # use axum::response::IntoResponse;
/// # use typed_error::AppError;
/// async fn handler() -> Result<String, AppError> {
///     Err(AppError::not_found("USER_NOT_FOUND"))
/// }
/// ```
///
/// Axum will automatically:
///
/// - map the error to an HTTP status code
/// - serialize it into JSON
///
/// # Behavior
///
/// - Status code is derived from [`HttpErrorExt::http_status`]
/// - Body is serialized from [`ErrorResponse`]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status: StatusCode = self.http_status();

        let body = ErrorResponse::from(self);

        (status, Json(body)).into_response()
    }
}

/// Allows converting a borrowed [`AppError`] into an Axum response.
///
/// Useful when working with references.
impl IntoResponse for &AppError {
    fn into_response(self) -> Response {
        let status: StatusCode = self.http_status();

        let body = ErrorResponse::from(self);

        (status, Json(body)).into_response()
    }
}
