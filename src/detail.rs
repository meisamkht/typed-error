use core::fmt;
use std::collections::BTreeMap;

/// A structured value stored inside [`ErrorDetails`].
///
/// `DetailValue` represents machine-readable metadata attached to an error.
/// It is designed to hold small, structured values that can be included in:
///
/// - logs
/// - API responses
/// - retry logic
/// - diagnostics
/// - observability pipelines
///
/// The type intentionally supports a limited set of primitive-like variants
/// to keep formatting, comparison, and serialization predictable.
///
/// # Design notes
///
/// - `String` is used for textual values.
/// - signed and unsigned integer values are stored separately.
/// - floating-point values are stored as strings to avoid issues with
///   `NaN`, total ordering, and equality semantics.
/// - booleans are stored directly.
///
/// # Examples
///
/// ```rust
/// # use typed_error::DetailValue;
/// let value = DetailValue::from("user_id");
/// assert_eq!(value.to_string(), "user_id");
/// ```
///
/// ```rust
/// # use typed_error::DetailValue;
/// let value = DetailValue::from(42_u64);
/// assert_eq!(value.to_string(), "42");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum DetailValue {
    /// A textual metadata value.
    String(String),

    /// A signed integer metadata value.
    Integer(i64),

    /// An unsigned integer metadata value.
    Unsigned(u64),

    /// A floating-point metadata value stored in string form.
    ///
    /// This representation keeps ordering and equality predictable.
    Float(String),

    /// A boolean metadata value.
    Boolean(bool),
}

impl DetailValue {
    /// Returns `true` if this value is a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::DetailValue;
    /// let value = DetailValue::from("email");
    /// assert!(value.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns `true` if this value is a signed integer.
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// Returns `true` if this value is an unsigned integer.
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Self::Unsigned(_))
    }

    /// Returns `true` if this value is a float stored as text.
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Returns `true` if this value is a boolean.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    /// Returns the inner string if this is [`DetailValue::String`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use typed_error::DetailValue;
    /// let value = DetailValue::from("email");
    /// assert_eq!(value.as_string(), Some("email"));
    /// ```
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value.as_str()),
            _ => None,
        }
    }

    /// Returns the inner `i64` if this is [`DetailValue::Integer`].
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the inner `u64` if this is [`DetailValue::Unsigned`].
    pub fn as_unsigned(&self) -> Option<u64> {
        match self {
            Self::Unsigned(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the inner float representation if this is [`DetailValue::Float`].
    pub fn as_float_str(&self) -> Option<&str> {
        match self {
            Self::Float(value) => Some(value.as_str()),
            _ => None,
        }
    }

    /// Returns the inner boolean if this is [`DetailValue::Boolean`].
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }
}

impl fmt::Display for DetailValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => f.write_str(value),
            Self::Integer(value) => write!(f, "{value}"),
            Self::Unsigned(value) => write!(f, "{value}"),
            Self::Float(value) => f.write_str(value),
            Self::Boolean(value) => write!(f, "{value}"),
        }
    }
}

impl From<String> for DetailValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for DetailValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<bool> for DetailValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i8> for DetailValue {
    fn from(value: i8) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i16> for DetailValue {
    fn from(value: i16) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i32> for DetailValue {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i64> for DetailValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<isize> for DetailValue {
    fn from(value: isize) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<u8> for DetailValue {
    fn from(value: u8) -> Self {
        Self::Unsigned(value as u64)
    }
}

impl From<u16> for DetailValue {
    fn from(value: u16) -> Self {
        Self::Unsigned(value as u64)
    }
}

impl From<u32> for DetailValue {
    fn from(value: u32) -> Self {
        Self::Unsigned(value as u64)
    }
}

impl From<u64> for DetailValue {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl From<usize> for DetailValue {
    fn from(value: usize) -> Self {
        Self::Unsigned(value as u64)
    }
}

impl From<f32> for DetailValue {
    fn from(value: f32) -> Self {
        Self::Float(value.to_string())
    }
}

impl From<f64> for DetailValue {
    fn from(value: f64) -> Self {
        Self::Float(value.to_string())
    }
}

/// A deterministic map of structured error metadata.
///
/// `ErrorDetails` is a `BTreeMap<String, DetailValue>` used to attach
/// contextual information to errors in a stable and serializable form.
///
/// A `BTreeMap` is used instead of `HashMap` to keep ordering deterministic,
/// which is useful for:
///
/// - test assertions
/// - snapshot testing
/// - stable JSON output
/// - reproducible logs
///
/// # Examples
///
/// ```rust
/// # use typed_error::{DetailValue, ErrorDetails};
/// let mut details = ErrorDetails::new();
/// details.insert("field".to_string(), DetailValue::from("email"));
/// details.insert("retryable".to_string(), DetailValue::from(true));
///
/// assert_eq!(details.get("field").unwrap().to_string(), "email");
/// ```
pub type ErrorDetails = BTreeMap<String, DetailValue>;
