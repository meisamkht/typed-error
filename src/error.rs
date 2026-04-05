use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;

use crate::{DetailValue, ErrorCode, ErrorDetails, ErrorKind, RetryHint};

/// A boxed standard error used as the optional source of an [`AppError`].
pub type BoxError = Box<dyn StdError + Send + Sync + 'static>;

/// A structured application error.
///
/// `AppError` is the central error type of this crate. It combines:
///
/// - a high-level error classification via [`ErrorKind`]
/// - a stable machine-readable identifier via [`ErrorCode`]
/// - a human-readable message
/// - structured metadata via [`ErrorDetails`]
/// - retry guidance via [`RetryHint`]
/// - an optional source error
///
/// # Design goals
///
/// `AppError` is designed to be:
///
/// - type-safe
/// - transport-agnostic
/// - easy to construct
/// - extensible over time
/// - suitable for backend and service applications
///
/// # Examples
///
/// ```rust
/// # use typed_error::{AppError, ErrorKind, RetryHint};
/// let error = AppError::new(ErrorKind::NotFound, "USER_NOT_FOUND")
///     .message("user was not found")
///     .detail("user_id", 42_u64)
///     .retry_hint(RetryHint::Never);
///
/// assert_eq!(error.kind(), ErrorKind::NotFound);
/// assert_eq!(error.code().as_str(), "USER_NOT_FOUND");
/// assert_eq!(error.message_str(), "user was not found");
/// assert!(!error.is_retryable());
/// ```
#[derive(Debug)]
pub struct AppError {
    kind: ErrorKind,
    code: ErrorCode,
    message: Cow<'static, str>,
    details: ErrorDetails,
    retry_hint: RetryHint,
    source: Option<BoxError>,
}

impl AppError {
    /// Creates a new application error from an [`ErrorKind`] and [`ErrorCode`].
    ///
    /// The created error has:
    ///
    /// - an empty message
    /// - empty details
    /// - [`RetryHint::Never`]
    /// - no source error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::{AppError, ErrorKind};
    /// let error = AppError::new(ErrorKind::Internal, "INTERNAL_ERROR");
    ///
    /// assert_eq!(error.kind(), ErrorKind::Internal);
    /// assert_eq!(error.code().as_str(), "INTERNAL_ERROR");
    /// assert_eq!(error.message_str(), "");
    /// ```
    pub fn new(kind: ErrorKind, code: impl Into<ErrorCode>) -> Self {
        Self {
            kind,
            code: code.into(),
            message: Cow::Borrowed(""),
            details: ErrorDetails::new(),
            retry_hint: RetryHint::Never,
            source: None,
        }
    }

    /// Creates a validation error.
    pub fn validation(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::Validation, code)
    }

    /// Creates an unauthorized error.
    pub fn unauthorized(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::Unauthorized, code)
    }

    /// Creates a forbidden error.
    pub fn forbidden(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::Forbidden, code)
    }

    /// Creates a not-found error.
    pub fn not_found(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::NotFound, code)
    }

    /// Creates a conflict error.
    pub fn conflict(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::Conflict, code)
    }

    /// Creates a rate-limited error.
    pub fn rate_limited(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::RateLimited, code)
    }

    /// Creates a timeout error.
    pub fn timeout(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::Timeout, code)
    }

    /// Creates a cancelled error.
    pub fn cancelled(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::Cancelled, code)
    }

    /// Creates an unavailable error.
    pub fn unavailable(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::Unavailable, code)
    }

    /// Creates an external error.
    pub fn external(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::External, code)
    }

    /// Creates an internal error.
    pub fn internal(code: impl Into<ErrorCode>) -> Self {
        Self::new(ErrorKind::Internal, code)
    }

    /// Sets the human-readable message of the error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::AppError;
    /// let error = AppError::internal("INTERNAL_ERROR")
    ///     .message("unexpected internal failure");
    ///
    /// assert_eq!(error.message_str(), "unexpected internal failure");
    /// ```
    pub fn message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.message = message.into();
        self
    }

    /// Adds a single structured metadata entry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::AppError;
    /// let error = AppError::validation("EMAIL_REQUIRED")
    ///     .detail("field", "email");
    ///
    /// assert_eq!(
    ///     error.details_map().get("field").map(|v| v.to_string()),
    ///     Some("email".to_string())
    /// );
    /// ```
    pub fn detail(mut self, key: impl Into<String>, value: impl Into<DetailValue>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }

    /// Adds multiple structured metadata entries.
    pub fn details(mut self, entries: impl IntoIterator<Item = (String, DetailValue)>) -> Self {
        self.details.extend(entries);
        self
    }

    /// Sets the retry guidance for this error.
    pub fn retry_hint(mut self, retry_hint: RetryHint) -> Self {
        self.retry_hint = retry_hint;
        self
    }

    /// Sets the source error.
    ///
    /// This can be used to preserve the underlying failure while still exposing
    /// a structured application-level error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use typed_error::AppError;
    /// # use std::io;
    /// let source = io::Error::new(io::ErrorKind::NotFound, "config file missing");
    ///
    /// let error = AppError::internal("CONFIG_READ_FAILED")
    ///     .message("failed to read config")
    ///     .source_error(source);
    ///
    /// assert!(error.source().is_some());
    /// ```
    pub fn source_error<E>(mut self, error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(error));
        self
    }

    /// Returns the high-level error classification.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the stable machine-readable error code.
    pub fn code(&self) -> &ErrorCode {
        &self.code
    }

    /// Returns the human-readable error message.
    pub fn message_str(&self) -> &str {
        &self.message
    }

    /// Returns the structured error metadata.
    pub fn details_map(&self) -> &ErrorDetails {
        &self.details
    }

    /// Returns the retry hint.
    pub fn retry_hint_ref(&self) -> &RetryHint {
        &self.retry_hint
    }

    /// Returns `true` if the error is retryable.
    pub fn is_retryable(&self) -> bool {
        self.retry_hint.is_retryable()
    }

    /// Decomposes the error into owned parts.
    ///
    /// This is useful for response mapping, logging, and integration layers.
    pub fn into_parts(self) -> AppErrorParts {
        AppErrorParts {
            kind: self.kind,
            code: self.code,
            message: self.message.into_owned(),
            details: self.details,
            retry_hint: self.retry_hint,
            source: self.source,
        }
    }
}

/// Owned components of an [`AppError`].
///
/// This type is useful when moving an error into another representation such as:
///
/// - an HTTP response body
/// - a serialized error payload
/// - a logging/event pipeline
pub struct AppErrorParts {
    pub kind: ErrorKind,
    pub code: ErrorCode,
    pub message: String,
    pub details: ErrorDetails,
    pub retry_hint: RetryHint,
    pub source: Option<BoxError>,
}

impl fmt::Display for AppError {
    /// Formats the error as `CODE: message` when a message is present,
    /// otherwise as `CODE (kind)`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.message.is_empty() {
            write!(f, "{} ({})", self.code, self.kind)
        } else {
            write!(f, "{}: {}", self.code, self.message)
        }
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|error| error as &(dyn StdError + 'static))
    }
}
