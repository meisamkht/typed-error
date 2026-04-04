use core::fmt;
use std::borrow::Borrow;

/// A stable, machine-readable error code.
///
/// `ErrorCode` is a lightweight wrapper around `String` used to represent
/// application or service error identifiers such as:
///
/// - `USER_NOT_FOUND`
/// - `EMAIL_REQUIRED`
/// - `PAYMENT_GATEWAY_TIMEOUT`
///
/// Unlike a free-form error message, an `ErrorCode` is intended to be:
///
/// - stable across releases
/// - machine-readable
/// - suitable for logs, metrics, APIs, and clients
///
/// # Design goals
///
/// This type is designed to:
///
/// - provide a dedicated type for error identifiers
/// - avoid mixing user-facing messages with machine-readable codes
/// - remain flexible enough for different naming conventions
///
/// # Validation
///
/// [`ErrorCode::new`] does not reject empty or whitespace-only values.
/// If validation is required, use [`ErrorCode::try_new`] instead.
///
/// # Examples
///
/// Creating an error code from a string:
///
/// ```rust
/// # use typed_error::ErrorCode;
/// let code = ErrorCode::new("USER_NOT_FOUND");
/// assert_eq!(code.as_str(), "USER_NOT_FOUND");
/// ```
///
/// Using the fallible constructor:
///
/// ```rust
/// # use typed_error::ErrorCode;
/// let code = ErrorCode::try_new("EMAIL_REQUIRED").unwrap();
/// assert_eq!(code.as_str(), "EMAIL_REQUIRED");
/// ```
///
/// Invalid values are rejected by `try_new`:
///
/// ```rust
/// # use typed_error::ErrorCode;
/// assert!(ErrorCode::try_new("").is_err());
/// assert!(ErrorCode::try_new("   ").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorCode(String);

impl ErrorCode {
    /// Creates a new error code from any string-like input.
    ///
    /// This constructor is intentionally ergonomic and does not perform
    /// strict validation. If you need to reject empty or whitespace-only
    /// values, use [`ErrorCode::try_new`] instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorCode;
    /// let code = ErrorCode::new("USER_NOT_FOUND");
    /// assert_eq!(code.as_str(), "USER_NOT_FOUND");
    /// ```
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }

    /// Tries to create a validated error code.
    ///
    /// This constructor rejects:
    ///
    /// - empty strings
    /// - whitespace-only strings
    ///
    /// It does not enforce a specific naming convention such as upper snake
    /// case. That policy is intentionally left to the user so the type can be
    /// reused across different projects.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidErrorCode`] if the provided value is empty or contains
    /// only whitespace.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorCode;
    /// let code = ErrorCode::try_new("AUTH_INVALID_TOKEN").unwrap();
    /// assert_eq!(code.as_str(), "AUTH_INVALID_TOKEN");
    /// ```
    ///
    /// ```rust
    /// # use typed_error::ErrorCode;
    /// assert!(ErrorCode::try_new("").is_err());
    /// assert!(ErrorCode::try_new("   ").is_err());
    /// ```
    pub fn try_new(code: impl Into<String>) -> Result<Self, InvalidErrorCode> {
        let code = code.into();

        if code.trim().is_empty() {
            return Err(InvalidErrorCode);
        }

        Ok(Self(code))
    }

    /// Returns the underlying code as a string slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorCode;
    /// let code = ErrorCode::new("USER_NOT_FOUND");
    /// assert_eq!(code.as_str(), "USER_NOT_FOUND");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the error code and returns the inner `String`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorCode;
    /// let code = ErrorCode::new("USER_NOT_FOUND");
    /// let value = code.into_string();
    /// assert_eq!(value, "USER_NOT_FOUND");
    /// ```
    pub fn into_string(self) -> String {
        self.0
    }
}

/// An error returned when constructing an invalid [`ErrorCode`] with
/// [`ErrorCode::try_new`].
///
/// This error indicates that the provided error code was empty or consisted
/// only of whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidErrorCode;

impl fmt::Display for InvalidErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error code must not be empty or whitespace only")
    }
}

impl std::error::Error for InvalidErrorCode {}

impl fmt::Display for ErrorCode {
    /// Formats the error code as its raw string value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorCode;
    /// let code = ErrorCode::new("USER_NOT_FOUND");
    /// assert_eq!(code.to_string(), "USER_NOT_FOUND");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for ErrorCode {
    /// Creates an `ErrorCode` from a borrowed string slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorCode;
    /// let code = ErrorCode::from("USER_NOT_FOUND");
    /// assert_eq!(code.as_str(), "USER_NOT_FOUND");
    /// ```
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ErrorCode {
    /// Creates an `ErrorCode` from an owned `String`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::ErrorCode;
    /// let code = ErrorCode::from(String::from("USER_NOT_FOUND"));
    /// assert_eq!(code.as_str(), "USER_NOT_FOUND");
    /// ```
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for ErrorCode {
    /// Returns this error code as `&str`.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for ErrorCode {
    /// Borrows this error code as `&str`.
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
