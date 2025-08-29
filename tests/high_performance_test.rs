#[cfg(test)]
#[cfg(all(feature = "arrow", feature = "simd"))]
mod high_performance_tests {
    use std::time::Instant;
    use veloxx::conditions::Condition;
    use veloxx::dataframe::DataFrame;
    use veloxx::series::Series;
    use veloxx::types::Value;

    // Use smaller data sizes by default to keep CI/unit tests fast. Set
    // VELOXX_SLOW_TESTS=1 to run the original large sizes locally/CI.
    fn cfg_size(original: usize, fast: usize) -> usize {
        match std::env::var("VELOXX_SLOW_TESTS") {
            Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => original,
            _ => fast,
        }
    }

    #[test]
    fn test_vectorized_filter_performance() {
    // Create a dataset for performance testing (smaller by default)
    let size = cfg_size(100_000, 20_000);
        let data: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
        let series = Series::new_f64("values", data);

        let mut columns = std::collections::HashMap::new();
        columns.insert("values".to_string(), series);
        let df = DataFrame::new(columns).expect("DataFrame creation should succeed");

        // Test vectorized filtering
    let start = Instant::now();
    // Use a dynamic threshold at half the range so roughly half the rows match
    let threshold = (size as f64) / 2.0;
    let condition = Condition::Gt("values".to_string(), Value::F64(threshold));
        let filtered = df.filter(&condition).expect("Filter should succeed");
        let duration = start.elapsed();

    // Should filter approximately half the data (±10%)
    let expected = size as f64 / 2.0;
    let lower = (expected * 0.9) as usize;
    let upper = (expected * 1.1) as usize;
    assert!(filtered.row_count() > lower && filtered.row_count() < upper);
        println!("Vectorized filter of {} rows took: {:?}", size, duration);

        // Should be reasonably fast
    assert!(duration.as_millis() < 5_000, "Vectorized filter should be performant");
    }

    #[test]
    fn test_simd_sum_performance() {
    let size = cfg_size(1_000_000, 200_000);
        let data: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
        let series = Series::new_f64("values", data);

        let start = Instant::now();
        let sum = series.sum().expect("Sum should succeed");
        let duration = start.elapsed();

        // Mathematical sum: 0 + 1 + 2 + ... + (n-1) = n*(n-1)/2
        let expected = (size as f64) * (size as f64 - 1.0) / 2.0;

        if let Value::F64(actual) = sum {
            assert!((actual - expected).abs() < 1e-6, "Sum should be accurate");
        } else {
            panic!("Sum should return F64 value");
        }

        println!("SIMD sum of {} elements took: {:?}", size, duration);
    assert!(duration.as_millis() < 1_000, "SIMD sum should be very fast");
    }

    #[test]
    fn test_simd_arithmetic_performance() {
    let size = cfg_size(500_000, 100_000);
        let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
        let data2: Vec<Option<f64>> = (0..size).map(|i| Some((i + 1) as f64)).collect();

        let series1 = Series::new_f64("a", data1);
        let series2 = Series::new_f64("b", data2);

        // Test addition
        let start = Instant::now();
        let added = series1.add(&series2).expect("Addition should succeed");
        let add_duration = start.elapsed();

        // Test multiplication
        let start = Instant::now();
        let multiplied = series1
            .multiply(&series2)
            .expect("Multiplication should succeed");
        let mul_duration = start.elapsed();

        assert_eq!(added.len(), size);
        assert_eq!(multiplied.len(), size);

        // Verify some results
        assert_eq!(added.get_value(0), Some(Value::F64(1.0))); // 0 + 1
        assert_eq!(multiplied.get_value(1), Some(Value::F64(2.0))); // 1 * 2

        println!(
            "SIMD addition of {} elements took: {:?}",
            size, add_duration
        );
        println!(
            "SIMD multiplication of {} elements took: {:?}",
            size, mul_duration
        );

    assert!(add_duration.as_millis() < 2_000, "SIMD addition should be fast");
    assert!(mul_duration.as_millis() < 2_000, "SIMD multiplication should be fast");
    }

    #[test]
    fn test_optimized_group_by_performance() {
    let size = cfg_size(100_000, 50_000);
        let categories = vec!["A", "B", "C", "D", "E"];

        // Create test data with groups
        let cat_data: Vec<Option<String>> = (0..size)
            .map(|i| Some(categories[i % categories.len()].to_string()))
            .collect();
        let value_data: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();

        let cat_series = Series::new_string("category", cat_data);
        let val_series = Series::new_f64("value", value_data);

        let mut columns = std::collections::HashMap::new();
        columns.insert("category".to_string(), cat_series);
        columns.insert("value".to_string(), val_series);
        let df = DataFrame::new(columns).expect("DataFrame creation should succeed");

        // Test group by with sum
        let start = Instant::now();
        let grouped = df
            .group_by(vec!["category".to_string()])
            .expect("Group by should succeed");
        let result = grouped
            .agg(vec![("value", "sum")])
            .expect("Group sum should succeed");
        let duration = start.elapsed();

        // Should have 5 groups
        assert_eq!(result.row_count(), 5);

        println!("Optimized group by on {} rows took: {:?}", size, duration);
    assert!(duration.as_millis() < 10_000, "Optimized group by should be fast");
    }

    #[test]
    fn test_memory_pool_efficiency() {
    // Test that our memory pool reduces allocation overhead
    let iterations = cfg_size(1_000, 200);
    let size = cfg_size(1_000, 500);

        let start = Instant::now();
        for i in 0..iterations {
            let data: Vec<Option<f64>> = (0..size).map(|j| Some((i * size + j) as f64)).collect();
            let series = Series::new_f64(&format!("test_{}", i), data);

            // Perform some operations to trigger memory usage
            let doubled = series
                .multiply(&series)
                .expect("Multiplication should succeed");
            let sum = doubled.sum().expect("Sum should succeed");

            // Verify the operation worked
            if let Value::F64(_) = sum {
                // Good
            } else {
                panic!("Sum should return F64");
            }
        }
        let duration = start.elapsed();

        println!(
            "Memory pool test ({} iterations of {} elements) took: {:?}",
            iterations, size, duration
        );

        // Should complete in reasonable time (memory pool should help)
    assert!(duration.as_secs() < 10, "Memory pool should provide efficiency gains");
    }

    #[test]
    fn test_expression_fusion_optimization() {
    let size = cfg_size(50_000, 20_000);
        let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
        let data2: Vec<Option<f64>> = (0..size).map(|i| Some((i + 1) as f64)).collect();
        let data3: Vec<Option<f64>> = (0..size).map(|i| Some((i * 2) as f64)).collect();

        let series1 = Series::new_f64("a", data1);
        let series2 = Series::new_f64("b", data2);
        let series3 = Series::new_f64("c", data3);

        // Test chained operations (should benefit from expression fusion)
        let start = Instant::now();
        let step1 = series1.add(&series2).expect("Addition should succeed");
        let result = step1
            .multiply(&series3)
            .expect("Multiplication should succeed");
        let duration = start.elapsed();

        assert_eq!(result.len(), size);

        // Verify result: (a + b) * c = (i + (i+1)) * (i*2) = (2i + 1) * (2i)
        assert_eq!(result.get_value(1), Some(Value::F64(6.0))); // (2 + 1) * 2 = 6
        assert_eq!(result.get_value(2), Some(Value::F64(20.0))); // (4 + 1) * 4 = 20

        println!(
            "Expression fusion test on {} elements took: {:?}",
            size, duration
        );
    assert!(duration.as_millis() < 3_000, "Expression fusion should optimize chained operations");
    }

    #[test]
    fn test_parallel_processing_scaling() {
    let size = cfg_size(200_000, 100_000);
        let data: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64 * 1.5)).collect();
        
        // Create a categorical column with reasonable number of groups (10)
        let categories = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
        let category_data: Vec<Option<String>> = (0..size)
            .map(|i| Some(categories[i % categories.len()].to_string()))
            .collect();
        
        let series = Series::new_f64("values", data);
        let cat_series = Series::new_string("category", category_data);

        let mut columns = std::collections::HashMap::new();
        columns.insert("values".to_string(), series);
        columns.insert("category".to_string(), cat_series);
        let df = DataFrame::new(columns).expect("DataFrame creation should succeed");

        // Test operations that should benefit from parallel processing
        let start = Instant::now();

        // Multiple operations in sequence
        let condition1 = Condition::Gt("values".to_string(), Value::F64(100.0));
        let condition2 = Condition::Lt("values".to_string(), Value::F64(250000.0));
        let combined_condition = Condition::And(Box::new(condition1), Box::new(condition2));
        let filtered = df
            .filter(&combined_condition)
            .expect("Filter should succeed");

        // Group by the categorical column instead of the values column
        let _grouped = filtered
            .group_by(vec!["category".to_string()])
            .expect("Group by should succeed");

        let duration = start.elapsed();

        println!(
            "Parallel processing test on {} rows took: {:?}",
            size, duration
        );
    assert!(duration.as_millis() < 25_000, "Parallel processing should provide performance benefits");
    }

    #[test]
    fn test_cross_platform_simd_consistency() {
        // Test that SIMD operations produce consistent results across platforms
        let data1 = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];
        let data2 = vec![Some(1.1f64), Some(2.2), Some(3.3), Some(4.4), Some(5.5)];

        let series1 = Series::new_f64("test1", data1);
        let series2 = Series::new_f64("test2", data2);

        let added = series1.add(&series2).expect("Addition should succeed");
        let multiplied = series1
            .multiply(&series2)
            .expect("Multiplication should succeed");

        // Verify exact results (should be consistent across platforms)
        assert_eq!(added.get_value(0), Some(Value::F64(2.1)));
        assert_eq!(added.get_value(1), Some(Value::F64(4.2)));
        assert_eq!(multiplied.get_value(0), Some(Value::F64(1.1)));
        assert_eq!(multiplied.get_value(1), Some(Value::F64(4.4)));

        println!("Cross-platform SIMD consistency test passed");
    }
}

#[cfg(test)]
mod benchmark_comparisons {
    use std::time::Instant;
    use veloxx::series::Series;
    use veloxx::types::Value;

    #[test]
    #[ignore] // Mark as ignored for normal test runs, run with --ignored for benchmarks
    fn benchmark_simd_vs_scalar() {
        let size = 1_000_000;
        let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
        let data2: Vec<Option<f64>> = (0..size).map(|i| Some((i + 1) as f64)).collect();

        let series1 = Series::new_f64("a", data1.clone());
        let series2 = Series::new_f64("b", data2.clone());

        // SIMD-optimized operation
        let start = Instant::now();
        let simd_result = series1.add(&series2).expect("SIMD addition should succeed");
        let simd_duration = start.elapsed();

        // Scalar comparison (manual implementation)
        let start = Instant::now();
        let mut scalar_result = Vec::new();
        for i in 0..size {
            if let (Some(a), Some(b)) = (data1[i], data2[i]) {
                scalar_result.push(Some(a + b));
            } else {
                scalar_result.push(None);
            }
        }
        let scalar_duration = start.elapsed();

        println!("SIMD addition of {} elements: {:?}", size, simd_duration);
        println!(
            "Scalar addition of {} elements: {:?}",
            size, scalar_duration
        );

        let speedup = scalar_duration.as_nanos() as f64 / simd_duration.as_nanos() as f64;
        println!("SIMD speedup: {:.2}x", speedup);

        // SIMD should be faster
        assert!(
            simd_duration < scalar_duration,
            "SIMD should outperform scalar operations"
        );

        // Verify results are identical
        assert_eq!(simd_result.len(), scalar_result.len());
        for i in 0..std::cmp::min(100, size) {
            // Check first 100 elements
            let simd_val = simd_result.get_value(i);
            let scalar_val = scalar_result[i].map(Value::F64);
            assert_eq!(
                simd_val, scalar_val,
                "Results should be identical at index {}",
                i
            );
        }
    }
}
