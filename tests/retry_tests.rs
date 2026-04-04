use std::time::Duration;

use typed_error::RetryHint;

#[test]
fn never_is_not_retryable() {
    let hint = RetryHint::Never;
    assert!(!hint.is_retryable());
}

#[test]
fn safe_is_retryable() {
    let hint = RetryHint::Safe;
    assert!(hint.is_retryable());
}

#[test]
fn after_is_retryable() {
    let hint = RetryHint::After(Duration::from_secs(3));
    assert!(hint.is_retryable());
}

#[test]
fn never_has_no_retry_delay() {
    let hint = RetryHint::Never;
    assert_eq!(hint.retry_after(), None);
    assert_eq!(hint.retry_after_ms(), None);
    assert!(!hint.has_delay());
}

#[test]
fn safe_has_no_retry_delay() {
    let hint = RetryHint::Safe;
    assert_eq!(hint.retry_after(), None);
    assert_eq!(hint.retry_after_ms(), None);
    assert!(!hint.has_delay());
}

#[test]
fn after_returns_retry_delay() {
    let hint = RetryHint::After(Duration::from_secs(5));

    assert_eq!(hint.retry_after(), Some(Duration::from_secs(5)));
    assert_eq!(hint.retry_after_ms(), Some(5000));
    assert!(hint.has_delay());
}

#[test]
fn display_for_never_is_readable() {
    let hint = RetryHint::Never;
    assert_eq!(hint.to_string(), "never");
}

#[test]
fn display_for_safe_is_readable() {
    let hint = RetryHint::Safe;
    assert_eq!(hint.to_string(), "safe");
}

#[test]
fn display_for_after_contains_milliseconds() {
    let hint = RetryHint::After(Duration::from_millis(1500));
    assert_eq!(hint.to_string(), "after(1500ms)");
}

#[test]
fn clone_and_eq_work_as_expected() {
    let left = RetryHint::After(Duration::from_secs(1));
    let right = left.clone();

    assert_eq!(left, right);
}
