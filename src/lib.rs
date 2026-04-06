pub mod code;
pub mod detail;
pub mod error;

#[cfg(feature = "http")]
pub mod http;
pub mod integration;
pub mod kind;
pub mod response;
pub mod result;
pub mod retry;

pub use code::{ErrorCode, InvalidErrorCode};
pub use detail::{DetailValue, ErrorDetails};
pub use error::{AppError, AppErrorParts, BoxError};

#[cfg(feature = "http")]
pub use http::HttpErrorExt;

pub use kind::ErrorKind;
pub use response::ErrorResponse;
pub use retry::RetryHint;
