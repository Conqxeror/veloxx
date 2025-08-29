#[cfg(test)]
#[cfg(feature = "arrow")]
mod tests {
    use veloxx::series::Series;
    use veloxx::types::{DataType, Value};

    #[test]
    fn test_unified_series_creation() {
        let data = vec![Some(1i32), Some(2), None, Some(4)];
        let series = Series::new_i32("test", data);

        assert_eq!(series.len(), 4);
        assert_eq!(series.name(), "test");
        assert_eq!(series.get_value(0), Some(Value::I32(1)));
        assert_eq!(series.get_value(1), Some(Value::I32(2)));
        assert_eq!(series.get_value(2), None);
        assert_eq!(series.get_value(3), Some(Value::I32(4)));
    }

    #[test]
    fn test_unified_series_f64() {
        let data = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
        let series = Series::new_f64("test", data);

        assert_eq!(series.len(), 4);
        assert_eq!(series.name(), "test");
        assert_eq!(series.get_value(0), Some(Value::F64(1.0)));
        assert_eq!(series.get_value(1), Some(Value::F64(2.0)));
        assert_eq!(series.get_value(2), Some(Value::F64(3.0)));
        assert_eq!(series.get_value(3), Some(Value::F64(4.0)));
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_simd_optimized_addition() {
        let data1 = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
        let data2 = vec![Some(1.0f64), Some(1.0), Some(1.0), Some(1.0)];

        let series1 = Series::new_f64("test1", data1);
        let series2 = Series::new_f64("test2", data2);

        let result = series1.add(&series2).expect("Addition should succeed");

        assert_eq!(result.len(), 4);
        assert_eq!(result.get_value(0), Some(Value::F64(2.0))); // 1.0 + 1.0
        assert_eq!(result.get_value(1), Some(Value::F64(3.0))); // 2.0 + 1.0
        assert_eq!(result.get_value(2), Some(Value::F64(4.0))); // 3.0 + 1.0
        assert_eq!(result.get_value(3), Some(Value::F64(5.0))); // 4.0 + 1.0
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_simd_optimized_addition_i32() {
        let data1 = vec![Some(1i32), Some(2), Some(3), Some(4)];
        let data2 = vec![Some(1i32), Some(2), Some(3), Some(4)];

        let series1 = Series::new_i32("test1", data1);
        let series2 = Series::new_i32("test2", data2);

        let result = series1.add(&series2).expect("Addition should succeed");

        assert_eq!(result.len(), 4);
        assert_eq!(result.get_value(0), Some(Value::I32(2))); // 1 + 1
        assert_eq!(result.get_value(1), Some(Value::I32(4))); // 2 + 2
        assert_eq!(result.get_value(2), Some(Value::I32(6))); // 3 + 3
        assert_eq!(result.get_value(3), Some(Value::I32(8))); // 4 + 4
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_simd_optimized_multiplication() {
        let data1 = vec![Some(2.0f64), Some(3.0), Some(4.0), Some(5.0)];
        let data2 = vec![Some(2.0f64), Some(2.0), Some(2.0), Some(2.0)];

        let series1 = Series::new_f64("test1", data1);
        let series2 = Series::new_f64("test2", data2);

        let result = series1
            .multiply(&series2)
            .expect("Multiplication should succeed");

        assert_eq!(result.len(), 4);
        assert_eq!(result.get_value(0), Some(Value::F64(4.0))); // 2.0 * 2.0
        assert_eq!(result.get_value(1), Some(Value::F64(6.0))); // 3.0 * 2.0
        assert_eq!(result.get_value(2), Some(Value::F64(8.0))); // 4.0 * 2.0
        assert_eq!(result.get_value(3), Some(Value::F64(10.0))); // 5.0 * 2.0
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_simd_optimized_multiplication_i32() {
        let data1 = vec![Some(2i32), Some(3), Some(4), Some(5)];
        let data2 = vec![Some(3i32), Some(3), Some(3), Some(3)];

        let series1 = Series::new_i32("test1", data1);
        let series2 = Series::new_i32("test2", data2);

        let result = series1
            .multiply(&series2)
            .expect("Multiplication should succeed");

        assert_eq!(result.len(), 4);
        assert_eq!(result.get_value(0), Some(Value::I32(6))); // 2 * 3
        assert_eq!(result.get_value(1), Some(Value::I32(9))); // 3 * 3
        assert_eq!(result.get_value(2), Some(Value::I32(12))); // 4 * 3
        assert_eq!(result.get_value(3), Some(Value::I32(15))); // 5 * 3
    }

    #[test]
    fn test_arithmetic_with_nulls() {
        let data1 = vec![Some(1.0f64), None, Some(3.0), Some(4.0)];
        let data2 = vec![Some(1.0f64), Some(2.0), None, Some(4.0)];

        let series1 = Series::new_f64("test1", data1);
        let series2 = Series::new_f64("test2", data2);

        let result = series1
            .add(&series2)
            .expect("Addition should handle nulls gracefully");

        assert_eq!(result.len(), 4);
        assert_eq!(result.get_value(0), Some(Value::F64(2.0))); // 1.0 + 1.0
        assert_eq!(result.get_value(1), None); // null + 2.0 = null
        assert_eq!(result.get_value(2), None); // 3.0 + null = null
        assert_eq!(result.get_value(3), Some(Value::F64(8.0))); // 4.0 + 4.0
    }

    #[test]
    fn test_arithmetic_mismatched_lengths() {
        let data1 = vec![Some(1.0f64), Some(2.0), Some(3.0)];
        let data2 = vec![Some(1.0f64), Some(2.0)]; // Different length

        let series1 = Series::new_f64("test1", data1);
        let series2 = Series::new_f64("test2", data2);

        let result = series1.add(&series2);
        assert!(
            result.is_err(),
            "Should return error for mismatched lengths"
        );
    }

    #[test]
    fn test_arithmetic_mismatched_types() {
        let data1 = vec![Some(1.0f64), Some(2.0), Some(3.0)];
        let data2 = vec![Some(1i32), Some(2), Some(3)];

        let series1 = Series::new_f64("test1", data1);
        let series2 = Series::new_i32("test2", data2);

        // Mixed-type arithmetic should work (F64 + I32 -> F64)
        let result = series1.add(&series2);
        assert!(
            result.is_ok(),
            "Mixed-type arithmetic should work (F64 + I32 -> F64)"
        );

        let result_series = result.unwrap();
        assert_eq!(result_series.data_type(), DataType::F64);

        // Check values
        if let Some(Value::F64(val)) = result_series.get_value(0) {
            assert_eq!(val, 2.0);
        } else {
            panic!("Expected F64 value");
        }
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_large_simd_operations() {
        // Test with larger datasets to ensure SIMD optimization works
        let size = 1000;
        let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
        let data2: Vec<Option<f64>> = (0..size).map(|_| Some(2.0)).collect();

        let series1 = Series::new_f64("large1", data1);
        let series2 = Series::new_f64("large2", data2);

        let result = series1
            .add(&series2)
            .expect("Large addition should succeed");

        assert_eq!(result.len(), size);
        // Check a few values
        assert_eq!(result.get_value(0), Some(Value::F64(2.0))); // 0 + 2
        assert_eq!(result.get_value(500), Some(Value::F64(502.0))); // 500 + 2
        assert_eq!(result.get_value(999), Some(Value::F64(1001.0))); // 999 + 2
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_performance_comparison() {
        use std::time::Instant;

        let size = 10000;
        let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
        let data2: Vec<Option<f64>> = (0..size).map(|i| Some((i + 1) as f64)).collect();

        let series1 = Series::new_f64("perf1", data1);
        let series2 = Series::new_f64("perf2", data2);

        // Time the SIMD-optimized operation
        let start = Instant::now();
        let result = series1
            .add(&series2)
            .expect("Performance test should succeed");
        let duration = start.elapsed();

        assert_eq!(result.len(), size);
        println!(
            "SIMD-optimized addition of {} elements took: {:?}",
            size, duration
        );

        // The operation should complete reasonably quickly
        assert!(duration.as_millis() < 1000, "SIMD operation should be fast");
    }
}
