//! Tests for Arrow aggregation operations

#[cfg(feature = "arrow")]
use veloxx::arrow::{ArrowAggregate, ArrowSeries};
#[cfg(feature = "arrow")]
use veloxx::types::Value;

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_aggregate_mean() {
    let data = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];
    let series = ArrowSeries::new_f64("test", data);

    let mean = series.mean().unwrap().unwrap();
    assert_eq!(mean, Value::F64(3.0));
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_aggregate_min() {
    let data = vec![Some(5i32), Some(2), Some(8), Some(1), Some(9)];
    let series = ArrowSeries::new_i32("test", data);

    let min = series.min().unwrap().unwrap();
    assert_eq!(min, Value::I32(1));
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_aggregate_max() {
    let data = vec![Some(5i32), Some(2), Some(8), Some(1), Some(9)];
    let series = ArrowSeries::new_i32("test", data);

    let max = series.max().unwrap().unwrap();
    assert_eq!(max, Value::I32(9));
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_aggregate_std() {
    let data = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];
    let series = ArrowSeries::new_f64("test", data);

    let std = series.std().unwrap().unwrap();
    // Standard deviation of [1,2,3,4,5] is sqrt(2) ≈ 1.414
    assert!((std.as_f64().unwrap() - 1.414).abs() < 0.01);
}
