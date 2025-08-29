//! Tests for Arrow mathematical operations

#[cfg(feature = "arrow")]
use veloxx::arrow::{ArrowOps, ArrowSeries};

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_math_sub() {
    let data1 = vec![Some(5.0f64), Some(10.0), Some(15.0)];
    let data2 = vec![Some(2.0f64), Some(3.0), Some(4.0)];
    let series1 = ArrowSeries::new_f64("a", data1);
    let series2 = ArrowSeries::new_f64("b", data2);

    let result = series1.arrow_sub(&series2).unwrap();

    match result {
        ArrowSeries::F64(_, array, _) => {
            let arr = array
                .as_any()
                .downcast_ref::<arrow_array::Float64Array>()
                .unwrap();
            assert_eq!(arr.value(0), 3.0);
            assert_eq!(arr.value(1), 7.0);
            assert_eq!(arr.value(2), 11.0);
        }
        _ => panic!("Expected F64 series"),
    }
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_math_mul() {
    let data1 = vec![Some(5.0f64), Some(10.0), Some(15.0)];
    let data2 = vec![Some(2.0f64), Some(3.0), Some(4.0)];
    let series1 = ArrowSeries::new_f64("a", data1);
    let series2 = ArrowSeries::new_f64("b", data2);

    let result = series1.arrow_mul(&series2).unwrap();

    match result {
        ArrowSeries::F64(_, array, _) => {
            let arr = array
                .as_any()
                .downcast_ref::<arrow_array::Float64Array>()
                .unwrap();
            assert_eq!(arr.value(0), 10.0);
            assert_eq!(arr.value(1), 30.0);
            assert_eq!(arr.value(2), 60.0);
        }
        _ => panic!("Expected F64 series"),
    }
}

#[cfg(feature = "arrow")]
#[test]
fn test_arrow_math_div() {
    let data1 = vec![Some(10.0f64), Some(20.0), Some(30.0)];
    let data2 = vec![Some(2.0f64), Some(4.0), Some(5.0)];
    let series1 = ArrowSeries::new_f64("a", data1);
    let series2 = ArrowSeries::new_f64("b", data2);

    let result = series1.arrow_div(&series2).unwrap();

    match result {
        ArrowSeries::F64(_, array, _) => {
            let arr = array
                .as_any()
                .downcast_ref::<arrow_array::Float64Array>()
                .unwrap();
            assert_eq!(arr.value(0), 5.0);
            assert_eq!(arr.value(1), 5.0);
            assert_eq!(arr.value(2), 6.0);
        }
        _ => panic!("Expected F64 series"),
    }
}
