# typed-error

Stop treating errors as strings.

`typed-error` is a typed, transport-aware error system for Rust services. It provides a structured, consistent, and production-ready foundation for modeling service errors across:

- core domain and application logic
- HTTP APIs
- Axum handlers
- gRPC services built with Tonic
- logs, diagnostics, and structured error responses

It separates concerns clearly:

- `ErrorKind` classifies the error at a high level
- `ErrorCode` identifies the concrete error in a stable, machine-readable way
- `DetailValue` / `ErrorDetails` hold structured metadata
- `RetryHint` carries retry guidance
- `AppError` ties all of the above together
- `ErrorResponse` exposes a transport-safe response shape

---

## Why typed-error?

Instead of ad-hoc strings and loosely structured errors, `typed-error` gives you:

- a clear error taxonomy for backend systems
- strongly typed error codes for machine processing
- structured metadata for debugging and observability
- built-in retry semantics
- clean mapping to HTTP and gRPC transports
- framework integrations for Axum and Tonic

Built for real-world systems: APIs, microservices, and distributed architectures.

---

## Installation

```toml
[dependencies]
typed-error = "0.1.0"
```

Enable features as needed:

```toml
typed-error = { version = "0.1.0", features = ["axum"] }
```

Because `axum` depends on `http` and `json`, you do not need to enable those separately.

---

## Feature flags

The crate is feature-based so consumers can keep dependencies small.

```toml
[features]
default = ["http", "json"]

http = ["dep:http"]
serde = ["dep:serde"]
json = ["serde", "dep:serde_json"]

axum = ["http", "json", "dep:axum"]
tonic = ["dep:tonic"]

full = ["http", "json", "axum", "tonic"]
```

### Recommended feature usage

Core + default features (`http` + `json`):

```toml
typed-error = "0.1.0"
```

Core + serde derives only:

```toml
typed-error = { version = "0.1.0", default-features = false, features = ["serde"] }
```

HTTP mapping only:

```toml
typed-error = { version = "0.1.0", default-features = false, features = ["http"] }
```

JSON-friendly error responses:

```toml
typed-error = { version = "0.1.0", default-features = false, features = ["json"] }
```

Axum integration:

```toml
typed-error = { version = "0.1.0", default-features = false, features = ["axum"] }
```

Tonic integration:

```toml
typed-error = { version = "0.1.0", default-features = false, features = ["tonic"] }
```

Everything for local development:

```toml
typed-error = { version = "0.1.0", default-features = false, features = ["full"] }
```

> Note: `json` already enables `serde`, and `axum` already enables both `http` and `json`.

---

## Core model

The main public types are:

- `ErrorCode`
- `InvalidErrorCode`
- `DetailValue`
- `ErrorDetails`
- `RetryHint`
- `ErrorKind`
- `AppError`
- `AppErrorParts`
- `ErrorResponse`
- `HttpErrorExt` (behind `http`)

---

## ErrorCode

`ErrorCode` is a stable, machine-readable identifier for a concrete error.

Examples:

- `USER_NOT_FOUND`
- `EMAIL_REQUIRED`
- `PAYMENT_GATEWAY_TIMEOUT`

### Basic usage

```rust
use typed_error::ErrorCode;

let code = ErrorCode::new("USER_NOT_FOUND");
assert_eq!(code.as_str(), "USER_NOT_FOUND");
```

### Validated construction

```rust
use typed_error::ErrorCode;

let code = ErrorCode::try_new("EMAIL_REQUIRED").unwrap();
assert_eq!(code.as_str(), "EMAIL_REQUIRED");

assert!(ErrorCode::try_new("").is_err());
assert!(ErrorCode::try_new("   ").is_err());
```

---

## DetailValue and ErrorDetails

`DetailValue` stores structured error metadata.

Supported value kinds:

- `String`
- `Integer`
- `Unsigned`
- `Float` (stored as string)
- `Boolean`

`ErrorDetails` is a `BTreeMap<String, DetailValue>`.

### Basic usage

```rust
use typed_error::{DetailValue, ErrorDetails};

let mut details = ErrorDetails::new();
details.insert("field".to_string(), DetailValue::from("email"));
details.insert("attempt".to_string(), DetailValue::from(2_u32));
details.insert("retryable".to_string(), DetailValue::from(true));

assert_eq!(details.get("field").unwrap().to_string(), "email");
```

### Why `BTreeMap`?

A `BTreeMap` is used to keep ordering deterministic for:

- predictable tests
- stable JSON output
- reproducible logs
- snapshot-style assertions

---

## RetryHint

`RetryHint` carries retry guidance without implementing retry logic itself.

Variants:

- `RetryHint::Never`
- `RetryHint::Safe`
- `RetryHint::After(Duration)`

### Example

```rust
use std::time::Duration;
use typed_error::RetryHint;

let hint = RetryHint::After(Duration::from_secs(5));
assert!(hint.is_retryable());
assert_eq!(hint.retry_after_ms(), Some(5000));
```

---

## ErrorKind

`ErrorKind` is the coarse-grained classification layer.

Available kinds:

- `Validation`
- `Unauthorized`
- `Forbidden`
- `NotFound`
- `Conflict`
- `RateLimited`
- `Timeout`
- `Cancelled`
- `Unavailable`
- `External`
- `Internal`

### Example

```rust
use typed_error::ErrorKind;

let kind = ErrorKind::NotFound;
assert_eq!(kind.as_str(), "not_found");
assert!(kind.is_client_fault());
```

### Design guideline

Use `ErrorKind` for classification and `ErrorCode` for specificity.

Good:

- `ErrorKind::NotFound`
- `ErrorCode("USER_NOT_FOUND")`

Not recommended:

- creating domain-specific variants directly in `ErrorKind`

---

## AppError

`AppError` is the central error type.

It contains:

- `kind: ErrorKind`
- `code: ErrorCode`
- `message: Cow<'static, str>`
- `details: ErrorDetails`
- `retry_hint: RetryHint`
- `source: Option<Box<dyn Error + Send + Sync>>`

### Example: building an error

```rust
use typed_error::{AppError, ErrorKind, RetryHint};

let error = AppError::new(ErrorKind::NotFound, "USER_NOT_FOUND")
    .message("user was not found")
    .detail("user_id", 42_u64)
    .retry_hint(RetryHint::Never);

assert_eq!(error.kind(), ErrorKind::NotFound);
assert_eq!(error.code().as_str(), "USER_NOT_FOUND");
assert_eq!(error.message_str(), "user was not found");
assert!(!error.is_retryable());
```

### Example: helper constructors

```rust
use typed_error::AppError;

let validation = AppError::validation("INVALID_EMAIL")
    .message("email format is invalid");

let not_found = AppError::not_found("USER_NOT_FOUND")
    .message("user was not found");

let internal = AppError::internal("INTERNAL_ERROR")
    .message("unexpected internal failure");
```

### Example: attaching source errors

```rust
use std::error::Error;
use std::io;
use typed_error::AppError;

let source = io::Error::new(io::ErrorKind::NotFound, "config file missing");

let error = AppError::internal("CONFIG_READ_FAILED")
    .message("failed to read config")
    .source_error(source);

assert!(error.source().is_some());
```

### Example: moving into owned parts

```rust
use typed_error::{AppError, RetryHint};

let error = AppError::conflict("USER_EXISTS")
    .message("user already exists")
    .detail("user_id", 42_u64)
    .retry_hint(RetryHint::Safe);

let parts = error.into_parts();
assert_eq!(parts.code.as_str(), "USER_EXISTS");
assert_eq!(parts.message, "user already exists");
```

---

## ErrorResponse

`ErrorResponse` is the transport-safe, owned representation of an `AppError`.

It is suitable for:

- HTTP/JSON responses
- structured logs and events
- serialization
- diagnostics pipelines

Fields:

- `kind`
- `code`
- `message`
- `details`
- `retryable`
- `retry_after_ms`

### Example

```rust
use std::time::Duration;
use typed_error::{AppError, ErrorResponse, RetryHint};

let error = AppError::external("PAYMENT_GATEWAY_TIMEOUT")
    .message("payment gateway did not respond in time")
    .detail("provider", "stripe")
    .retry_hint(RetryHint::After(Duration::from_secs(2)));

let response = ErrorResponse::from(&error);

assert_eq!(response.code, "PAYMENT_GATEWAY_TIMEOUT");
assert_eq!(response.message, "payment gateway did not respond in time");
assert!(response.retryable);
assert_eq!(response.retry_after_ms, Some(2000));
```

### JSON serialization

Requires the `json` feature.

```rust
use typed_error::{AppError, ErrorResponse};

let error = AppError::validation("INVALID_EMAIL")
    .message("email format is invalid")
    .detail("field", "email");

let response = ErrorResponse::from(error);
let json = serde_json::to_string(&response).unwrap();

assert!(json.contains("INVALID_EMAIL"));
```

---

## HTTP status mapping

Enable the `http` feature to map `ErrorKind` and `AppError` to `http::StatusCode`.

```rust
use ::http::StatusCode;
use typed_error::{AppError, ErrorKind, HttpErrorExt};

assert_eq!(ErrorKind::Validation.http_status(), StatusCode::BAD_REQUEST);
assert_eq!(ErrorKind::NotFound.http_status(), StatusCode::NOT_FOUND);

let error = AppError::not_found("USER_NOT_FOUND");
assert_eq!(error.http_status(), StatusCode::NOT_FOUND);
```

### Current HTTP mapping

- `Validation` -> `400 Bad Request`
- `Unauthorized` -> `401 Unauthorized`
- `Forbidden` -> `403 Forbidden`
- `NotFound` -> `404 Not Found`
- `Conflict` -> `409 Conflict`
- `RateLimited` -> `429 Too Many Requests`
- `Timeout` -> `408 Request Timeout`
- `Cancelled` -> `400 Bad Request`
- `Unavailable` -> `503 Service Unavailable`
- `External` -> `502 Bad Gateway`
- `Internal` -> `500 Internal Server Error`

---

## Axum integration

Enable the `axum` feature to return `AppError` directly from Axum handlers.

```rust
use typed_error::AppError;

async fn handler() -> Result<String, AppError> {
    Err(
        AppError::not_found("USER_NOT_FOUND")
            .message("user was not found")
    )
}
```

The integration:

- derives the status code from `HttpErrorExt`
- serializes the body as `ErrorResponse`

Typical JSON body:

```json
{
  "kind": "not_found",
  "code": "USER_NOT_FOUND",
  "message": "user was not found",
  "details": {},
  "retryable": false,
  "retry_after_ms": null
}
```

---

## Tonic integration

Enable the `tonic` feature to convert `AppError` into `tonic::Status`.

```rust
use tonic::Status;
use typed_error::AppError;

let error = AppError::not_found("USER_NOT_FOUND")
    .message("user was not found");

let status: Status = error.into();
assert_eq!(status.message(), "USER_NOT_FOUND: user was not found");
```

### Current gRPC mapping

- `Validation` -> `InvalidArgument`
- `Unauthorized` -> `Unauthenticated`
- `Forbidden` -> `PermissionDenied`
- `NotFound` -> `NotFound`
- `Conflict` -> `AlreadyExists`
- `RateLimited` -> `ResourceExhausted`
- `Timeout` -> `DeadlineExceeded`
- `Cancelled` -> `Cancelled`
- `Unavailable` -> `Unavailable`
- `External` -> `Unavailable`
- `Internal` -> `Internal`

---

## Recommended usage pattern

A good pattern in real services is:

1. choose a coarse `ErrorKind`
2. choose a stable `ErrorCode`
3. attach a concise message
4. add structured `details`
5. add `RetryHint` when operationally relevant
6. attach a source error when preserving lower-level failures helps diagnostics

Example:

```rust
use std::time::Duration;
use typed_error::{AppError, RetryHint};

let error = AppError::external("PAYMENT_GATEWAY_TIMEOUT")
    .message("payment gateway did not respond in time")
    .detail("provider", "stripe")
    .detail("attempt", 3_u32)
    .retry_hint(RetryHint::After(Duration::from_secs(2)));
```

---

## Testing locally

Recommended commands:

```bash
cargo fmt --all
cargo clippy --features full --all-targets -- -D warnings
cargo test --features full
cargo doc --no-deps --features full
```

---

## Project status

This crate is under active development. The foundational model is already in place:

- core error model
- transport-safe response model
- HTTP mapping
- Axum integration
- Tonic integration

Planned improvements may include:

- richer response policies
- optional redaction strategies
- enhanced tracing integration
- more advanced transport metadata

---

## License

Licensed under Apache-2.0.
