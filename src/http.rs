use ::http::StatusCode;

use crate::{AppError, ErrorKind};

/// Extension trait for mapping error types to HTTP status codes.
///
/// This trait provides a transport-layer mapping from structured error types
/// to [`http::StatusCode`].
///
/// It is intentionally small and focused:
///
/// - it does not build a full HTTP response
/// - it does not serialize error bodies
/// - it only determines the appropriate status code
///
/// # Design goals
///
/// This trait is designed to:
///
/// - keep HTTP mapping separate from core error modeling
/// - support reuse across frameworks such as `axum`
/// - provide a stable and predictable mapping layer
///
/// # Examples
///
/// ```rust
/// # use http::StatusCode;
/// # use typed_error::{ErrorKind, HttpErrorExt};
/// assert_eq!(ErrorKind::Validation.http_status(), StatusCode::BAD_REQUEST);
/// assert_eq!(ErrorKind::NotFound.http_status(), StatusCode::NOT_FOUND);
/// ```
///
/// ```rust
/// # use http::StatusCode;
/// # use typed_error::{AppError, HttpErrorExt};
/// let error = AppError::not_found("USER_NOT_FOUND");
/// assert_eq!(error.http_status(), StatusCode::NOT_FOUND);
/// ```
pub trait HttpErrorExt {
    /// Returns the HTTP status code corresponding to this error value.
    fn http_status(&self) -> StatusCode;
}

impl HttpErrorExt for ErrorKind {
    /// Maps an [`ErrorKind`] to an HTTP status code.
    ///
    /// The mapping is intentionally conservative and suitable for common
    /// backend and service scenarios.
    ///
    /// Current mapping:
    ///
    /// - `Validation` -> `400 Bad Request`
    /// - `Unauthorized` -> `401 Unauthorized`
    /// - `Forbidden` -> `403 Forbidden`
    /// - `NotFound` -> `404 Not Found`
    /// - `Conflict` -> `409 Conflict`
    /// - `RateLimited` -> `429 Too Many Requests`
    /// - `Timeout` -> `408 Request Timeout`
    /// - `Cancelled` -> `400 Bad Request`
    /// - `Unavailable` -> `503 Service Unavailable`
    /// - `External` -> `502 Bad Gateway`
    /// - `Internal` -> `500 Internal Server Error`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use http::StatusCode;
    /// # use typed_error::{ErrorKind, HttpErrorExt};
    /// assert_eq!(ErrorKind::Conflict.http_status(), StatusCode::CONFLICT);
    /// ```
    fn http_status(&self) -> StatusCode {
        match self {
            ErrorKind::Validation => StatusCode::BAD_REQUEST,
            ErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorKind::Forbidden => StatusCode::FORBIDDEN,
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::Conflict => StatusCode::CONFLICT,
            ErrorKind::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            ErrorKind::Timeout => StatusCode::REQUEST_TIMEOUT,
            ErrorKind::Cancelled => StatusCode::BAD_REQUEST,
            ErrorKind::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            ErrorKind::External => StatusCode::BAD_GATEWAY,
            ErrorKind::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpErrorExt for AppError {
    /// Maps an [`AppError`] to an HTTP status code using its [`ErrorKind`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use http::StatusCode;
    /// # use typed_error::{AppError, HttpErrorExt};
    /// let error = AppError::validation("INVALID_EMAIL");
    /// assert_eq!(error.http_status(), StatusCode::BAD_REQUEST);
    /// ```
    fn http_status(&self) -> StatusCode {
        self.kind().http_status()
    }
}
