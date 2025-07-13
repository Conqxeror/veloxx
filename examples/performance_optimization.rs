//! Performance optimization example for Velox
//!
//! This example demonstrates the performance improvements available
//! through SIMD operations, parallel processing, and memory optimizations.

use std::collections::BTreeMap;
use std::time::Instant;
use veloxx::dataframe::DataFrame;
use veloxx::performance::memory::CompressedColumn;
use veloxx::performance::simd::has_avx_support;
use veloxx::performance::SeriesPerformanceExt;
use veloxx::series::Series;
use veloxx::types::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Velox Performance Optimization Demo");
    println!("=====================================\n");

    // Check CPU capabilities
    println!("CPU Features:");
    println!("  AVX Support: {}", has_avx_support());
    println!(
        "  AVX2 Support: {}",
        veloxx::performance::simd::has_avx2_support()
    );
    println!();

    // Create large datasets for performance testing
    let size = 100_000;
    println!("Creating datasets with {} elements...", size);

    let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
    let data2: Vec<Option<f64>> = (0..size).map(|i| Some((i * 2) as f64)).collect();

    let series1 = Series::new_f64("data1", data1);
    let series2 = Series::new_f64("data2", data2);

    // SIMD Operations Demo
    println!("🔥 SIMD Operations Performance");
    println!("-----------------------------");

    // Traditional addition
    let start = Instant::now();
    let traditional_result = traditional_add(&series1, &series2)?;
    let traditional_time = start.elapsed();

    // SIMD addition
    let start = Instant::now();
    let simd_result = series1.simd_add(&series2)?;
    let simd_time = start.elapsed();

    println!("Traditional addition: {:?}", traditional_time);
    println!("SIMD addition: {:?}", simd_time);
    println!(
        "SIMD speedup: {:.2}x",
        traditional_time.as_nanos() as f64 / simd_time.as_nanos() as f64
    );

    // Verify results are equivalent
    println!(
        "Results match: {}",
        verify_series_equal(&traditional_result, &simd_result)
    );
    println!();

    // Parallel Processing Demo
    println!("⚡ Parallel Processing Performance");
    println!("--------------------------------");

    let large_series = Series::new_i32("large", (0..10_000).map(|i| Some(i)).collect());

    // Traditional sum
    let start = Instant::now();
    let traditional_sum = large_series.sum()?.unwrap_or(Value::Null);
    let traditional_sum_time = start.elapsed();

    // Parallel sum
    let start = Instant::now();
    let parallel_sum = large_series.par_sum()?;
    let parallel_sum_time = start.elapsed();

    println!("Traditional sum: {:?}", traditional_sum_time);
    println!("Parallel sum: {:?}", parallel_sum_time);
    println!(
        "Parallel speedup: {:.2}x",
        traditional_sum_time.as_nanos() as f64 / parallel_sum_time.as_nanos() as f64
    );
    println!("Results match: {}", traditional_sum == parallel_sum);
    println!();

    // Memory Optimization Demo
    println!("💾 Memory Optimization Analysis");
    println!("------------------------------");

    // Create different types of series for compression analysis
    let string_series = Series::new_string(
        "cities",
        vec![
            Some("New York".to_string()),
            Some("London".to_string()),
            Some("New York".to_string()),
            Some("Paris".to_string()),
            Some("London".to_string()),
            Some("New York".to_string()),
            Some("Tokyo".to_string()),
            Some("Paris".to_string()),
            Some("London".to_string()),
        ],
    );

    let bool_series = Series::new_bool(
        "flags",
        vec![
            Some(true),
            Some(false),
            Some(true),
            Some(true),
            Some(false),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
            Some(false),
        ],
    );

    let sequential_series = Series::new_datetime("sequential", (1..=1000).map(Some).collect());

    // Analyze memory usage and compression
    analyze_series_memory(&string_series, "String Series");
    analyze_series_memory(&bool_series, "Boolean Series");
    analyze_series_memory(&sequential_series, "Sequential Series");

    // DataFrame Performance Demo
    println!("📊 DataFrame Performance with Optimizations");
    println!("------------------------------------------");

    let mut columns = BTreeMap::new();
    columns.insert(
        "price".to_string(),
        Series::new_f64(
            "price",
            (0..10000).map(|i| Some(100.0 + (i as f64 * 0.1))).collect(),
        ),
    );
    columns.insert(
        "volume".to_string(),
        Series::new_i32("volume", (0..10000).map(|i| Some(1000 + i)).collect()),
    );
    columns.insert(
        "symbol".to_string(),
        Series::new_string(
            "symbol",
            (0..10000)
                .map(|i| Some(format!("STOCK{}", i % 100)))
                .collect(),
        ),
    );

    let df = DataFrame::new(columns)?;
    println!(
        "Created DataFrame with {} rows and {} columns",
        df.row_count(),
        df.column_count()
    );

    // Parallel aggregations on DataFrame columns
    if let Some(price_series) = df.get_column("price") {
        let start = Instant::now();
        let par_mean = price_series.par_mean()?;
        let par_time = start.elapsed();

        let start = Instant::now();
        let traditional_mean = price_series.mean()?.unwrap_or(Value::Null);
        let traditional_time = start.elapsed();

        println!(
            "Price column parallel mean: {:?} (time: {:?})",
            par_mean, par_time
        );
        println!(
            "Price column traditional mean: {:?} (time: {:?})",
            traditional_mean, traditional_time
        );
        println!(
            "Parallel mean speedup: {:.2}x",
            traditional_time.as_nanos() as f64 / par_time.as_nanos() as f64
        );
    }

    println!("\n✅ Performance optimization demo completed!");
    println!("Key takeaways:");
    println!("  • SIMD operations provide significant speedups for numeric computations");
    println!("  • Parallel processing scales well with available CPU cores");
    println!("  • Memory compression can reduce storage requirements substantially");
    println!("  • Performance optimizations maintain result accuracy");

    Ok(())
}

fn traditional_add(s1: &Series, s2: &Series) -> Result<Series, Box<dyn std::error::Error>> {
    match (s1, s2) {
        (Series::F64(_, a), Series::F64(_, b)) => {
            let result: Vec<Option<f64>> = a
                .iter()
                .zip(b.iter())
                .map(|(x, y)| match (x, y) {
                    (Some(x), Some(y)) => Some(x + y),
                    _ => None,
                })
                .collect();
            Ok(Series::new_f64("traditional_add", result))
        }
        _ => Err("Unsupported series types".into()),
    }
}

fn verify_series_equal(s1: &Series, s2: &Series) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    for i in 0..s1.len() {
        if s1.get_value(i) != s2.get_value(i) {
            return false;
        }
    }
    true
}

fn analyze_series_memory(series: &Series, name: &str) {
    println!("\n{} Analysis:", name);

    let memory_usage = series.memory_usage();
    println!("  Memory usage: {} bytes", memory_usage);

    let suggestions = series.compression_suggestions();
    println!("  Compression suggestions: {:?}", suggestions);

    // Try different compression methods
    for suggestion in &suggestions {
        match *suggestion {
            "dictionary" => {
                if let Ok(compressed) = CompressedColumn::from_dictionary(series) {
                    let ratio = compressed.compression_ratio(series);
                    println!("  Dictionary compression ratio: {:.2}x", ratio);
                }
            }
            "run_length" => {
                if let Ok(compressed) = CompressedColumn::from_run_length(series) {
                    let ratio = compressed.compression_ratio(series);
                    println!("  Run-length compression ratio: {:.2}x", ratio);
                }
            }
            _ => {}
        }
    }
}
