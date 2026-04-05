use crate::{AppError, ErrorDetails, ErrorKind};

/// A transport-friendly representation of an [`AppError`].
///
/// `ErrorResponse` is an owned, serializable view of an application error.
/// It is intended for use in:
///
/// - HTTP responses
/// - gRPC metadata mapping layers
/// - logs and events
/// - structured diagnostics
///
/// Unlike [`AppError`], this type does not preserve the source error chain.
/// It only exposes the structured, safe-to-serialize parts of an error.
///
/// # Fields
///
/// - `kind`: high-level classification of the error
/// - `code`: stable machine-readable error code
/// - `message`: human-readable message
/// - `details`: structured metadata
/// - `retryable`: whether retry is allowed
/// - `retry_after_ms`: optional retry delay in milliseconds
///
/// # Examples
///
/// ```rust
/// # use std::time::Duration;
/// # use typed_error::{AppError, ErrorResponse, RetryHint};
/// let error = AppError::external("PAYMENT_GATEWAY_TIMEOUT")
///     .message("payment gateway did not respond in time")
///     .detail("provider", "stripe")
///     .retry_hint(RetryHint::After(Duration::from_secs(2)));
///
/// let response = ErrorResponse::from(&error);
///
/// assert_eq!(response.code, "PAYMENT_GATEWAY_TIMEOUT");
/// assert_eq!(response.message, "payment gateway did not respond in time");
/// assert!(response.retryable);
/// assert_eq!(response.retry_after_ms, Some(2000));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorResponse {
    /// The high-level classification of the error.
    pub kind: ErrorKind,

    /// A stable machine-readable error code.
    pub code: String,

    /// A human-readable message.
    pub message: String,

    /// Structured error metadata.
    pub details: ErrorDetails,

    /// Whether the error is retryable.
    pub retryable: bool,

    /// Suggested retry delay in milliseconds, if any.
    pub retry_after_ms: Option<u64>,
}

impl ErrorResponse {
    /// Creates a new response from explicit fields.
    ///
    /// This constructor is useful when building transport-level errors without
    /// going through [`AppError`] directly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::{ErrorDetails, ErrorKind, ErrorResponse};
    /// let response = ErrorResponse::new(
    ///     ErrorKind::Validation,
    ///     "INVALID_EMAIL",
    ///     "email format is invalid",
    ///     ErrorDetails::new(),
    ///     false,
    ///     None,
    /// );
    ///
    /// assert_eq!(response.kind, ErrorKind::Validation);
    /// assert_eq!(response.code, "INVALID_EMAIL");
    /// ```
    pub fn new(
        kind: ErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
        details: ErrorDetails,
        retryable: bool,
        retry_after_ms: Option<u64>,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            message: message.into(),
            details,
            retryable,
            retry_after_ms,
        }
    }
}

impl From<&AppError> for ErrorResponse {
    fn from(error: &AppError) -> Self {
        Self {
            kind: error.kind(),
            code: error.code().as_str().to_owned(),
            message: error.message_str().to_owned(),
            details: error.details_map().clone(),
            retryable: error.is_retryable(),
            retry_after_ms: error.retry_hint_ref().retry_after_ms(),
        }
    }
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        let parts = error.into_parts();

        Self {
            kind: parts.kind,
            code: parts.code.into_string(),
            message: parts.message,
            details: parts.details,
            retryable: parts.retry_hint.is_retryable(),
            retry_after_ms: parts.retry_hint.retry_after_ms(),
        }
    }
}
