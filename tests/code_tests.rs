use typed_error::code::{ErrorCode, InvalidErrorCode};

#[test]
fn creates_error_code_with_new() {
    let code = ErrorCode::new("USER_NOT_FOUND");
    assert_eq!(code.as_str(), "USER_NOT_FOUND");
}

#[test]
fn creates_error_code_from_str() {
    let code = ErrorCode::from("EMAIL_REQUIRED");
    assert_eq!(code.as_str(), "EMAIL_REQUIRED");
}

#[test]
fn creates_error_code_from_string() {
    let code = ErrorCode::from(String::from("AUTH_INVALID_TOKEN"));
    assert_eq!(code.as_str(), "AUTH_INVALID_TOKEN");
}

#[test]
fn creates_error_code_with_try_new() {
    let code = ErrorCode::try_new("PAYMENT_GATEWAY_TIMEOUT").unwrap();
    assert_eq!(code.as_str(), "PAYMENT_GATEWAY_TIMEOUT");
}

#[test]
fn rejects_empty_error_code() {
    assert_eq!(ErrorCode::try_new(""), Err(InvalidErrorCode));
}

#[test]
fn rejects_whitespace_only_error_code() {
    assert_eq!(ErrorCode::try_new("   "), Err(InvalidErrorCode));
}

#[test]
fn display_returns_inner_string() {
    let code = ErrorCode::new("USER_NOT_FOUND");
    assert_eq!(code.to_string(), "USER_NOT_FOUND");
}

#[test]
fn as_ref_returns_inner_str() {
    let code = ErrorCode::new("USER_NOT_FOUND");
    assert_eq!(code.as_ref(), "USER_NOT_FOUND");
}

#[test]
fn into_string_returns_owned_value() {
    let code = ErrorCode::new("USER_NOT_FOUND");
    let value = code.into_string();
    assert_eq!(value, "USER_NOT_FOUND");
}

#[test]
fn preserves_original_text_when_using_new() {
    let code = ErrorCode::new(" user_not_found ");
    assert_eq!(code.as_str(), " user_not_found ");
}

#[test]
fn invalid_error_code_display_is_readable() {
    let err = InvalidErrorCode;
    assert_eq!(
        err.to_string(),
        "error code must not be empty or whitespace only"
    );
}
