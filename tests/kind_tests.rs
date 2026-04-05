use typed_error::ErrorKind;

#[test]
fn as_str_returns_expected_values() {
    assert_eq!(ErrorKind::Validation.as_str(), "validation");
    assert_eq!(ErrorKind::Unauthorized.as_str(), "unauthorized");
    assert_eq!(ErrorKind::Forbidden.as_str(), "forbidden");
    assert_eq!(ErrorKind::NotFound.as_str(), "not_found");
    assert_eq!(ErrorKind::Conflict.as_str(), "conflict");
    assert_eq!(ErrorKind::RateLimited.as_str(), "rate_limited");
    assert_eq!(ErrorKind::Timeout.as_str(), "timeout");
    assert_eq!(ErrorKind::Cancelled.as_str(), "cancelled");
    assert_eq!(ErrorKind::Unavailable.as_str(), "unavailable");
    assert_eq!(ErrorKind::External.as_str(), "external");
    assert_eq!(ErrorKind::Internal.as_str(), "internal");
}

#[test]
fn display_matches_machine_readable_name() {
    assert_eq!(ErrorKind::Validation.to_string(), "validation");
    assert_eq!(ErrorKind::NotFound.to_string(), "not_found");
    assert_eq!(ErrorKind::Internal.to_string(), "internal");
}

#[test]
fn client_fault_kinds_are_classified_correctly() {
    assert!(ErrorKind::Validation.is_client_fault());
    assert!(ErrorKind::Unauthorized.is_client_fault());
    assert!(ErrorKind::Forbidden.is_client_fault());
    assert!(ErrorKind::NotFound.is_client_fault());
    assert!(ErrorKind::Conflict.is_client_fault());
    assert!(ErrorKind::RateLimited.is_client_fault());
}

#[test]
fn server_fault_kinds_are_classified_correctly() {
    assert!(ErrorKind::Timeout.is_server_fault());
    assert!(ErrorKind::Cancelled.is_server_fault());
    assert!(ErrorKind::Unavailable.is_server_fault());
    assert!(ErrorKind::External.is_server_fault());
    assert!(ErrorKind::Internal.is_server_fault());
}

#[test]
fn client_fault_and_server_fault_are_complementary() {
    let values = [
        ErrorKind::Validation,
        ErrorKind::Unauthorized,
        ErrorKind::Forbidden,
        ErrorKind::NotFound,
        ErrorKind::Conflict,
        ErrorKind::RateLimited,
        ErrorKind::Timeout,
        ErrorKind::Cancelled,
        ErrorKind::Unavailable,
        ErrorKind::External,
        ErrorKind::Internal,
    ];

    for kind in values {
        assert_eq!(kind.is_server_fault(), !kind.is_client_fault());
    }
}
