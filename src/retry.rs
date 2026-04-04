use core::fmt;
use std::time::Duration;

/// Retry guidance attached to an error.
///
/// `RetryHint` describes whether an operation may be retried and, if so,
/// whether a delay is recommended before the next attempt.
///
/// This type is intended to provide **operational metadata** rather than
/// enforce retry logic by itself. In other words, it tells callers what is
/// reasonable to do, but it does not schedule, execute, or limit retries.
///
/// # Variants
///
/// - [`RetryHint::Never`] means the operation should not be retried.
/// - [`RetryHint::Safe`] means retrying is allowed immediately.
/// - [`RetryHint::After`] means retrying is allowed, but a delay is advised.
///
/// # Typical use cases
///
/// `RetryHint` is useful in:
///
/// - HTTP/gRPC services
/// - job runners
/// - message consumers
/// - queue processors
/// - integrations with external systems
///
/// # Design notes
///
/// This type is intentionally small and transport-agnostic:
///
/// - it does not encode backoff strategies
/// - it does not track retry counts
/// - it does not decide whether a retry will eventually succeed
///
/// Those concerns belong in higher-level retry policies.
///
/// # Examples
///
/// A non-retryable validation failure:
///
/// ```rust
/// # use typed_error::RetryHint;
/// let hint = RetryHint::Never;
/// assert!(!hint.is_retryable());
/// ```
///
/// A retryable transient failure:
///
/// ```rust
/// # use typed_error::RetryHint;
/// let hint = RetryHint::Safe;
/// assert!(hint.is_retryable());
/// assert_eq!(hint.retry_after(), None);
/// ```
///
/// A retryable upstream timeout with a suggested delay:
///
/// ```rust
/// # use std::time::Duration;
/// # use typed_error::RetryHint;
/// let hint = RetryHint::After(Duration::from_secs(5));
/// assert!(hint.is_retryable());
/// assert_eq!(hint.retry_after(), Some(Duration::from_secs(5)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryHint {
    /// The operation should not be retried.
    ///
    /// This is appropriate for errors such as:
    ///
    /// - validation failures
    /// - malformed requests
    /// - permission errors
    /// - permanent domain rule violations
    Never,

    /// The operation may be retried immediately.
    ///
    /// This is appropriate for transient failures where the caller may try
    /// again without waiting for a specific duration.
    Safe,

    /// The operation may be retried after a suggested delay.
    ///
    /// This is useful for:
    ///
    /// - rate limiting
    /// - temporary upstream unavailability
    /// - backpressure
    /// - cooldown-style retry semantics
    After(Duration),
}

impl RetryHint {
    /// Returns `true` if the operation is retryable.
    ///
    /// This returns:
    ///
    /// - `false` for [`RetryHint::Never`]
    /// - `true` for [`RetryHint::Safe`]
    /// - `true` for [`RetryHint::After`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use typed_error::RetryHint;
    /// assert!(!RetryHint::Never.is_retryable());
    /// assert!(RetryHint::Safe.is_retryable());
    /// assert!(RetryHint::After(Duration::from_secs(1)).is_retryable());
    /// ```
    pub fn is_retryable(&self) -> bool {
        !matches!(self, Self::Never)
    }

    /// Returns the suggested retry delay, if any.
    ///
    /// This returns:
    ///
    /// - `None` for [`RetryHint::Never`]
    /// - `None` for [`RetryHint::Safe`]
    /// - `Some(duration)` for [`RetryHint::After`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use typed_error::RetryHint;
    /// assert_eq!(RetryHint::Never.retry_after(), None);
    /// assert_eq!(RetryHint::Safe.retry_after(), None);
    /// assert_eq!(
    ///     RetryHint::After(Duration::from_millis(250)).retry_after(),
    ///     Some(Duration::from_millis(250))
    /// );
    /// ```
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::After(duration) => Some(*duration),
            Self::Never | Self::Safe => None,
        }
    }

    /// Returns `true` if this hint contains a delayed retry recommendation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use typed_error::RetryHint;
    /// assert!(!RetryHint::Never.has_delay());
    /// assert!(!RetryHint::Safe.has_delay());
    /// assert!(RetryHint::After(Duration::from_secs(2)).has_delay());
    /// ```
    pub fn has_delay(&self) -> bool {
        matches!(self, Self::After(_))
    }

    /// Returns the suggested retry delay in milliseconds, if any.
    ///
    /// This is a convenience helper for transport or response layers where
    /// retry delays are commonly expressed as numeric values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use typed_error::RetryHint;
    /// assert_eq!(RetryHint::Safe.retry_after_ms(), None);
    /// assert_eq!(
    ///     RetryHint::After(Duration::from_secs(2)).retry_after_ms(),
    ///     Some(2000)
    /// );
    /// ```
    pub fn retry_after_ms(&self) -> Option<u64> {
        self.retry_after()
            .map(|duration| duration.as_millis() as u64)
    }
}

impl fmt::Display for RetryHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Never => f.write_str("never"),
            Self::Safe => f.write_str("safe"),
            Self::After(duration) => write!(f, "after({}ms)", duration.as_millis()),
        }
    }
}
