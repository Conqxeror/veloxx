//! Tests for Arrow string operations

#[cfg(feature = "arrow")]
use veloxx::series::Series;

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_string_to_uppercase() {
    let data = vec![
        Some("hello".to_string()),
        Some("world".to_string()),
        None,
        Some("rust".to_string()),
    ];
    let series = Series::new_string("test", data);

    let uppercase_series = series.to_uppercase().unwrap();

    assert_eq!(
        uppercase_series.get_value(0),
        Some(veloxx::types::Value::String("HELLO".to_string()))
    );
    assert_eq!(
        uppercase_series.get_value(1),
        Some(veloxx::types::Value::String("WORLD".to_string()))
    );
    assert_eq!(uppercase_series.get_value(2), None);
    assert_eq!(
        uppercase_series.get_value(3),
        Some(veloxx::types::Value::String("RUST".to_string()))
    );
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_string_to_lowercase() {
    let data = vec![
        Some("HELLO".to_string()),
        Some("WORLD".to_string()),
        None,
        Some("RUST".to_string()),
    ];
    let series = Series::new_string("test", data);

    let lowercase_series = series.to_lowercase().unwrap();

    assert_eq!(
        lowercase_series.get_value(0),
        Some(veloxx::types::Value::String("hello".to_string()))
    );
    assert_eq!(
        lowercase_series.get_value(1),
        Some(veloxx::types::Value::String("world".to_string()))
    );
    assert_eq!(lowercase_series.get_value(2), None);
    assert_eq!(
        lowercase_series.get_value(3),
        Some(veloxx::types::Value::String("rust".to_string()))
    );
}
