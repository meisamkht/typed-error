use tonic::{Code, Status};

use crate::{AppError, ErrorKind};

/// Maps an [`ErrorKind`] to a gRPC [`tonic::Code`].
///
/// This helper keeps transport mapping logic separate from the core error model.
///
/// # Current mapping
///
/// - `Validation` -> `InvalidArgument`
/// - `Unauthorized` -> `Unauthenticated`
/// - `Forbidden` -> `PermissionDenied`
/// - `NotFound` -> `NotFound`
/// - `Conflict` -> `AlreadyExists`
/// - `RateLimited` -> `ResourceExhausted`
/// - `Timeout` -> `DeadlineExceeded`
/// - `Cancelled` -> `Cancelled`
/// - `Unavailable` -> `Unavailable`
/// - `External` -> `Unavailable`
/// - `Internal` -> `Internal`
fn grpc_code(kind: ErrorKind) -> Code {
    match kind {
        ErrorKind::Validation => Code::InvalidArgument,
        ErrorKind::Unauthorized => Code::Unauthenticated,
        ErrorKind::Forbidden => Code::PermissionDenied,
        ErrorKind::NotFound => Code::NotFound,
        ErrorKind::Conflict => Code::AlreadyExists,
        ErrorKind::RateLimited => Code::ResourceExhausted,
        ErrorKind::Timeout => Code::DeadlineExceeded,
        ErrorKind::Cancelled => Code::Cancelled,
        ErrorKind::Unavailable => Code::Unavailable,
        ErrorKind::External => Code::Unavailable,
        ErrorKind::Internal => Code::Internal,
    }
}

impl From<AppError> for Status {
    /// Converts an owned [`AppError`] into a gRPC [`Status`].
    ///
    /// The status code is derived from the error kind, and the status message
    /// is derived from the formatted application error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tonic::Code;
    /// # use tonic::Status;
    /// # use typed_error::AppError;
    /// let error = AppError::not_found("USER_NOT_FOUND")
    ///     .message("user was not found");
    ///
    /// let status: Status = error.into();
    ///
    /// assert_eq!(status.code(), Code::NotFound);
    /// assert_eq!(status.message(), "USER_NOT_FOUND: user was not found");
    /// ```
    fn from(error: AppError) -> Self {
        Status::new(grpc_code(error.kind()), error.to_string())
    }
}

impl From<&AppError> for Status {
    /// Converts a borrowed [`AppError`] into a gRPC [`Status`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tonic::Code;
    /// # use tonic::Status;
    /// # use typed_error::AppError;
    /// let error = AppError::validation("INVALID_EMAIL")
    ///     .message("email format is invalid");
    ///
    /// let status = Status::from(&error);
    ///
    /// assert_eq!(status.code(), Code::InvalidArgument);
    /// assert_eq!(status.message(), "INVALID_EMAIL: email format is invalid");
    /// ```
    fn from(error: &AppError) -> Self {
        Status::new(grpc_code(error.kind()), error.to_string())
    }
}
