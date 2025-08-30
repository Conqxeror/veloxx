//! Tests for Arrow string operations

#[cfg(feature = "arrow")]
use arrow_array::Array;
#[cfg(feature = "arrow")]
use veloxx::arrow::{ArrowSeries, ArrowStringOps};

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_string_to_uppercase() {
    let data = vec![
        Some("hello".to_string()),
        Some("world".to_string()),
        None,
        Some("rust".to_string()),
    ];
    let series = ArrowSeries::new_string("test", data);

    let uppercase_series = series.to_uppercase().unwrap();

    match uppercase_series {
        ArrowSeries::String(_, array, _) => {
            let arr = array
                .as_any()
                .downcast_ref::<arrow_array::StringArray>()
                .unwrap();
            assert_eq!(arr.value(0), "HELLO");
            assert_eq!(arr.value(1), "WORLD");
            assert!(arr.is_null(2));
            assert_eq!(arr.value(3), "RUST");
        }
        _ => panic!("Expected String series"),
    }
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
    let series = ArrowSeries::new_string("test", data);

    let lowercase_series = series.to_lowercase().unwrap();

    match lowercase_series {
        ArrowSeries::String(_, array, _) => {
            let arr = array
                .as_any()
                .downcast_ref::<arrow_array::StringArray>()
                .unwrap();
            assert_eq!(arr.value(0), "hello");
            assert_eq!(arr.value(1), "world");
            assert!(arr.is_null(2));
            assert_eq!(arr.value(3), "rust");
        }
        _ => panic!("Expected String series"),
    }
}
