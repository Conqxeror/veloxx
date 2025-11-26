//! Tests for Arrow filtering operations

#[cfg(feature = "arrow")]
use veloxx::series::Series;

#[cfg(feature = "arrow")]
use veloxx::types::Value;

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_filter_equal() {
    let data1 = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
    let data2 = vec![Some(1.0f64), Some(2.0), Some(2.0), Some(4.0)];

    let series1 = Series::new_f64("a", data1);
    let series2 = Series::new_f64("b", data2);

    let mask = series1.equal(&series2).unwrap();
    let expected = Series::new_bool(
        "expected",
        vec![Some(true), Some(true), Some(false), Some(true)],
    );

    assert_eq!(mask, expected);
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_filter_gt() {
    let data1 = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
    let data2 = vec![Some(1.0f64), Some(1.0), Some(2.0), Some(3.0)];

    let series1 = Series::new_f64("a", data1);
    let series2 = Series::new_f64("b", data2);

    let mask = series1.gt(&series2).unwrap();
    let expected = Series::new_bool(
        "expected",
        vec![Some(false), Some(true), Some(true), Some(true)],
    );

    assert_eq!(mask, expected);
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_filter_filter() {
    let data = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
    let series = Series::new_f64("test", data);

    let mask = Series::new_bool(
        "mask",
        vec![Some(true), Some(false), Some(true), Some(false)],
    );
    let filtered = series.filter_by_mask(&mask).unwrap();

    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered.get_value(0), Some(Value::F64(1.0)));
    assert_eq!(filtered.get_value(1), Some(Value::F64(3.0)));
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_filter_and_or() {
    let bool_data1 = vec![Some(true), Some(false), Some(true), Some(false)];
    let bool_data2 = vec![Some(true), Some(true), Some(false), Some(false)];

    let bool_series1 = Series::new_bool("a", bool_data1);
    let _bool_series2 = Series::new_bool("b", bool_data2);

    let mask2 = Series::new_bool(
        "mask2",
        vec![Some(true), Some(true), Some(false), Some(true)],
    );

    let and_result = bool_series1.and(&mask2).unwrap();
    let expected_and = Series::new_bool(
        "expected_and",
        vec![Some(true), Some(false), Some(false), Some(false)],
    );
    assert_eq!(and_result, expected_and);

    let or_result = bool_series1.or(&mask2).unwrap();
    let expected_or = Series::new_bool(
        "expected_or",
        vec![Some(true), Some(true), Some(true), Some(true)],
    );
    assert_eq!(or_result, expected_or);
}
