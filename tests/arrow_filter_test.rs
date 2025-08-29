//! Tests for Arrow filtering operations

#[cfg(feature = "arrow")]
use arrow_array::BooleanArray;

#[cfg(feature = "arrow")]
use veloxx::arrow::{ArrowFilter, ArrowSeries};
#[cfg(feature = "arrow")]
use veloxx::types::Value;

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_filter_equal() {
    let data1 = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
    let data2 = vec![Some(1.0f64), Some(2.0), Some(2.0), Some(4.0)];

    let series1 = ArrowSeries::new_f64("a", data1);
    let series2 = ArrowSeries::new_f64("b", data2);

    let mask = series1.equal(&series2).unwrap();
    let expected = BooleanArray::from(vec![true, true, false, true]);

    assert_eq!(mask, expected);
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_filter_gt() {
    let data1 = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
    let data2 = vec![Some(1.0f64), Some(1.0), Some(2.0), Some(3.0)];

    let series1 = ArrowSeries::new_f64("a", data1);
    let series2 = ArrowSeries::new_f64("b", data2);

    let mask = series1.gt(&series2).unwrap();
    let expected = BooleanArray::from(vec![false, true, true, true]);

    assert_eq!(mask, expected);
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_filter_filter() {
    let data = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
    let series = ArrowSeries::new_f64("test", data);

    let mask = BooleanArray::from(vec![true, false, true, false]);
    let filtered = series.filter(&mask).unwrap();

    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered.get(0), Some(Value::F64(1.0)));
    assert_eq!(filtered.get(1), Some(Value::F64(3.0)));
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_filter_and_or() {
    let bool_data1 = vec![Some(true), Some(false), Some(true), Some(false)];
    let bool_data2 = vec![Some(true), Some(true), Some(false), Some(false)];

    let bool_series1 = ArrowSeries::new_bool("a", bool_data1);
    let bool_series2 = ArrowSeries::new_bool("b", bool_data2);

    let _mask1 = bool_series1.equal(&bool_series2).unwrap();
    let mask2 = BooleanArray::from(vec![true, true, false, true]);

    let and_result = bool_series1.and(&mask2).unwrap();
    let expected_and = BooleanArray::from(vec![true, false, false, false]);
    assert_eq!(and_result, expected_and);

    let or_result = bool_series1.or(&mask2).unwrap();
    let expected_or = BooleanArray::from(vec![true, true, true, true]);
    assert_eq!(or_result, expected_or);
}
