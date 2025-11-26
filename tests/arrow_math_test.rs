//! Tests for Arrow mathematical operations

#[cfg(feature = "arrow")]
use veloxx::series::Series;

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_math_sub() {
    let data1 = vec![Some(5.0f64), Some(10.0), Some(15.0)];
    let data2 = vec![Some(2.0f64), Some(3.0), Some(4.0)];
    let series1 = Series::new_f64("a", data1);
    let series2 = Series::new_f64("b", data2);

    let result = series1.arrow_sub(&series2).unwrap();

    assert_eq!(result.get_value(0), Some(veloxx::types::Value::F64(3.0)));
    assert_eq!(result.get_value(1), Some(veloxx::types::Value::F64(7.0)));
    assert_eq!(result.get_value(2), Some(veloxx::types::Value::F64(11.0)));
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_math_mul() {
    let data1 = vec![Some(5.0f64), Some(10.0), Some(15.0)];
    let data2 = vec![Some(2.0f64), Some(3.0), Some(4.0)];
    let series1 = Series::new_f64("a", data1);
    let series2 = Series::new_f64("b", data2);

    let result = series1.arrow_mul(&series2).unwrap();

    assert_eq!(result.get_value(0), Some(veloxx::types::Value::F64(10.0)));
    assert_eq!(result.get_value(1), Some(veloxx::types::Value::F64(30.0)));
    assert_eq!(result.get_value(2), Some(veloxx::types::Value::F64(60.0)));
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_math_div() {
    let data1 = vec![Some(10.0f64), Some(20.0), Some(30.0)];
    let data2 = vec![Some(2.0f64), Some(4.0), Some(5.0)];
    let series1 = Series::new_f64("a", data1);
    let series2 = Series::new_f64("b", data2);

    let result = series1.arrow_div(&series2).unwrap();

    assert_eq!(result.get_value(0), Some(veloxx::types::Value::F64(5.0)));
    assert_eq!(result.get_value(1), Some(veloxx::types::Value::F64(5.0)));
    assert_eq!(result.get_value(2), Some(veloxx::types::Value::F64(6.0)));
}
