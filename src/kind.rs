use core::fmt;

/// A high-level classification of an application error.
///
/// `ErrorKind` describes the general category of an error without encoding
/// domain-specific details.
///
/// It is intended to answer questions such as:
///
/// - Was the request invalid?
/// - Was the resource missing?
/// - Was access denied?
/// - Was the failure caused by an upstream dependency?
/// - Was this an internal server-side problem?
///
/// `ErrorKind` is deliberately coarse-grained. More specific identification
/// should be represented using [`crate::ErrorCode`], while structured context
/// should be stored in [`crate::ErrorDetails`].
///
/// # Design goals
///
/// This type is designed to:
///
/// - provide a stable taxonomy for common service-layer failures
/// - remain reusable across different applications and domains
/// - support transport-layer mapping such as HTTP and gRPC
/// - avoid embedding business-specific error variants
///
/// # Non-goals
///
/// `ErrorKind` is not intended to:
///
/// - replace domain-specific error codes
/// - carry human-readable messages
/// - hold structured metadata
///
/// # Examples
///
/// ```rust
/// # use typed_error::ErrorKind;
/// let kind = ErrorKind::NotFound;
/// assert_eq!(kind.as_str(), "not_found");
/// ```
///
/// ```rust
/// # use typed_error::ErrorKind;
/// let kind = ErrorKind::Validation;
/// assert_eq!(kind.to_string(), "validation");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum ErrorKind {
    /// The request or input data is invalid.
    ///
    /// Typical examples:
    ///
    /// - missing required fields
    /// - invalid format
    /// - invalid range
    /// - malformed input
    Validation,

    /// Authentication is required or has failed.
    ///
    /// Typical examples:
    ///
    /// - missing authentication token
    /// - invalid credentials
    /// - expired token
    Unauthorized,

    /// The caller is authenticated but not allowed to perform the action.
    ///
    /// Typical examples:
    ///
    /// - insufficient permissions
    /// - access denied by policy
    Forbidden,

    /// The requested resource does not exist.
    ///
    /// Typical examples:
    ///
    /// - missing user
    /// - missing order
    /// - unknown identifier
    NotFound,

    /// The operation conflicts with the current state of the system.
    ///
    /// Typical examples:
    ///
    /// - duplicate resource
    /// - version mismatch
    /// - state transition conflict
    Conflict,

    /// The caller has exceeded a limit and should slow down.
    ///
    /// Typical examples:
    ///
    /// - rate limiting
    /// - quota exceeded
    RateLimited,

    /// The operation exceeded its allowed time window.
    ///
    /// Typical examples:
    ///
    /// - request timeout
    /// - upstream deadline exceeded
    Timeout,

    /// The operation was intentionally cancelled.
    ///
    /// Typical examples:
    ///
    /// - client cancellation
    /// - shutdown-triggered cancellation
    Cancelled,

    /// The service is temporarily unable to handle the request.
    ///
    /// Typical examples:
    ///
    /// - temporary outage
    /// - service overloaded
    /// - dependency unavailable
    Unavailable,

    /// The failure originated from an external dependency or upstream system.
    ///
    /// Typical examples:
    ///
    /// - payment gateway failure
    /// - third-party API error
    /// - downstream service failure
    External,

    /// An unexpected internal failure occurred.
    ///
    /// Typical examples:
    ///
    /// - bug
    /// - invariant violation
    /// - unhandled internal error
    Internal,
}

impl ErrorKind {
    /// Returns a stable, machine-readable string representation.
    ///
    /// This string is intended for logging, serialization, diagnostics,
    /// and response payloads.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorKind;
    /// assert_eq!(ErrorKind::Validation.as_str(), "validation");
    /// assert_eq!(ErrorKind::NotFound.as_str(), "not_found");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Unauthorized => "unauthorized",
            Self::Forbidden => "forbidden",
            Self::NotFound => "not_found",
            Self::Conflict => "conflict",
            Self::RateLimited => "rate_limited",
            Self::Timeout => "timeout",
            Self::Cancelled => "cancelled",
            Self::Unavailable => "unavailable",
            Self::External => "external",
            Self::Internal => "internal",
        }
    }

    /// Returns `true` if this error kind typically represents a client-side failure.
    ///
    /// This is a coarse operational helper and should not be treated as a strict
    /// transport-layer rule.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorKind;
    /// assert!(ErrorKind::Validation.is_client_fault());
    /// assert!(ErrorKind::NotFound.is_client_fault());
    /// assert!(!ErrorKind::Internal.is_client_fault());
    /// ```
    pub const fn is_client_fault(&self) -> bool {
        matches!(
            self,
            Self::Validation
                | Self::Unauthorized
                | Self::Forbidden
                | Self::NotFound
                | Self::Conflict
                | Self::RateLimited
        )
    }

    /// Returns `true` if this error kind typically represents a server-side failure.
    ///
    /// This is a coarse operational helper and should not be treated as a strict
    /// transport-layer rule.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorKind;
    /// assert!(ErrorKind::Internal.is_server_fault());
    /// assert!(ErrorKind::External.is_server_fault());
    /// assert!(!ErrorKind::Validation.is_server_fault());
    /// ```
    pub const fn is_server_fault(&self) -> bool {
        !self.is_client_fault()
    }
}

impl fmt::Display for ErrorKind {
    /// Formats the error kind as its stable machine-readable name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorKind;
    /// assert_eq!(ErrorKind::Conflict.to_string(), "conflict");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
