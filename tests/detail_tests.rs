use typed_error::{DetailValue, ErrorDetails};

#[test]
fn creates_string_detail_value_from_str() {
    let value = DetailValue::from("email");
    assert_eq!(value.as_string(), Some("email"));
    assert!(value.is_string());
}

#[test]
fn creates_string_detail_value_from_string() {
    let value = DetailValue::from(String::from("user_id"));
    assert_eq!(value.as_string(), Some("user_id"));
    assert!(value.is_string());
}

#[test]
fn creates_integer_detail_value() {
    let value = DetailValue::from(-42_i32);
    assert_eq!(value.as_integer(), Some(-42));
    assert!(value.is_integer());
}

#[test]
fn creates_unsigned_detail_value() {
    let value = DetailValue::from(42_u64);
    assert_eq!(value.as_unsigned(), Some(42));
    assert!(value.is_unsigned());
}

#[test]
fn creates_boolean_detail_value() {
    let value = DetailValue::from(true);
    assert_eq!(value.as_boolean(), Some(true));
    assert!(value.is_boolean());
}

#[test]
fn creates_float_detail_value() {
    let value = DetailValue::from(12.5_f64);
    assert_eq!(value.as_float_str(), Some("12.5"));
    assert!(value.is_float());
}

#[test]
fn display_for_string_value_returns_inner_text() {
    let value = DetailValue::from("email");
    assert_eq!(value.to_string(), "email");
}

#[test]
fn display_for_integer_value_returns_number_text() {
    let value = DetailValue::from(-15_i64);
    assert_eq!(value.to_string(), "-15");
}

#[test]
fn display_for_unsigned_value_returns_number_text() {
    let value = DetailValue::from(99_u32);
    assert_eq!(value.to_string(), "99");
}

#[test]
fn display_for_boolean_value_returns_boolean_text() {
    let value = DetailValue::from(false);
    assert_eq!(value.to_string(), "false");
}

#[test]
fn display_for_float_value_returns_original_float_text() {
    let value = DetailValue::from(3.25_f32);
    assert_eq!(value.to_string(), "3.25");
}

#[test]
fn accessors_return_none_for_wrong_variant() {
    let value = DetailValue::from("email");

    assert_eq!(value.as_integer(), None);
    assert_eq!(value.as_unsigned(), None);
    assert_eq!(value.as_float_str(), None);
    assert_eq!(value.as_boolean(), None);
}

#[test]
fn error_details_can_store_multiple_values() {
    let mut details = ErrorDetails::new();
    details.insert("field".to_string(), DetailValue::from("email"));
    details.insert("attempt".to_string(), DetailValue::from(2_u32));
    details.insert("retryable".to_string(), DetailValue::from(true));

    assert_eq!(
        details.get("field").and_then(DetailValue::as_string),
        Some("email")
    );
    assert_eq!(
        details.get("attempt").and_then(DetailValue::as_unsigned),
        Some(2)
    );
    assert_eq!(
        details.get("retryable").and_then(DetailValue::as_boolean),
        Some(true)
    );
}

#[test]
fn btreemap_order_is_deterministic() {
    let mut details = ErrorDetails::new();
    details.insert("z".to_string(), DetailValue::from(1_u32));
    details.insert("a".to_string(), DetailValue::from(2_u32));
    details.insert("m".to_string(), DetailValue::from(3_u32));

    let keys: Vec<_> = details.keys().cloned().collect();
    assert_eq!(
        keys,
        vec!["a".to_string(), "m".to_string(), "z".to_string()]
    );
}
