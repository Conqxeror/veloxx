use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(all(feature = "polars", feature = "arrow"))]
use polars::prelude::*;

use arrow::array::Float64Array;
use std::sync::Arc;
use veloxx::arrow::series::ArrowSeries;
use veloxx::performance::{
    advanced_parallel::{
        parallel_fused_add_mul_advanced, parallel_simd_add_advanced, parallel_simd_sum_advanced,
    },
    optimized_simd::OptimizedSimdOps,
};

fn create_test_data(size: usize) -> Vec<f64> {
    (0..size).map(|i| (i as f64) * 0.1).collect()
}

/// Comprehensive benchmark comparing all optimization layers
fn bench_optimization_layers(c: &mut Criterion) {
    let size = 100_000;
    let data_a = create_test_data(size);
    let data_b = create_test_data(size);
    let mut result = vec![0.0; size];

    let mut group = c.benchmark_group("optimization_layers");

    // 1. Naive scalar implementation
    group.bench_function("naive_scalar", |b| {
        b.iter(|| {
            for i in 0..size {
                result[i] = data_a[i] + data_b[i];
            }
            black_box(&result);
        })
    });

    // 2. Basic SIMD with optimized_simd
    group.bench_function("basic_simd", |b| {
        b.iter(|| {
            data_a.optimized_simd_add(&data_b, &mut result);
            black_box(&result);
        })
    });

    // 3. Advanced parallel SIMD
    group.bench_function("advanced_parallel_simd", |b| {
        b.iter(|| {
            parallel_simd_add_advanced(&data_a, &data_b, &mut result).unwrap();
            black_box(&result);
        })
    });

    // 4. ArrowSeries with aligned memory pools
    group.bench_function("arrow_series_aligned", |b| {
        let array_a = Arc::new(Float64Array::from_iter_values(data_a.clone()));
        let array_b = Arc::new(Float64Array::from_iter_values(data_b.clone()));
        let series_a = ArrowSeries::F64("a".to_string(), array_a, None);
        let series_b = ArrowSeries::F64("b".to_string(), array_b, None);

        b.iter(|| {
            let result = series_a.simd_add_raw(&series_b).unwrap();
            black_box(result);
        })
    });

    #[cfg(all(feature = "polars", feature = "arrow"))]
    {
        // 5. Polars baseline
        group.bench_function("polars_baseline", |b| {
            let series_a = Series::new("a", &data_a);
            let series_b = Series::new("b", &data_b);

            b.iter(|| {
                let result = &series_a + &series_b;
                black_box(result);
            })
        });
    }

    group.finish();
}

/// Test fused operations performance
fn bench_fused_operations(c: &mut Criterion) {
    let size = 100_000;
    let data_a = create_test_data(size);
    let data_b = create_test_data(size);
    let data_c = create_test_data(size);
    let mut result = vec![0.0; size];

    let mut group = c.benchmark_group("fused_operations");

    // Separate operations: (a + b) * c
    group.bench_function("separate_ops", |b| {
        let mut temp = vec![0.0; size];
        b.iter(|| {
            data_a.optimized_simd_add(&data_b, &mut temp);
            temp.optimized_simd_mul(&data_c, &mut result);
            black_box(&result);
        })
    });

    // Fused operation: (a + b) * c in single pass
    group.bench_function("fused_parallel", |b| {
        b.iter(|| {
            parallel_fused_add_mul_advanced(&data_a, &data_b, &data_c, &mut result).unwrap();
            black_box(&result);
        })
    });

    group.finish();
}

/// Memory bandwidth comparison
fn bench_memory_optimization(c: &mut Criterion) {
    let size = 100_000; // Medium dataset to test memory bandwidth (reduced from 1M)
    let data = create_test_data(size);

    let mut group = c.benchmark_group("memory_optimization");

    // Standard Vec allocation
    group.bench_function("standard_vec", |b| {
        b.iter(|| {
            let result = data.optimized_simd_sum();
            black_box(result);
        })
    });

    // Aligned buffer allocation
    group.bench_function("aligned_buffer", |b| {
        b.iter(|| {
            let result = parallel_simd_sum_advanced(&data).unwrap();
            black_box(result);
        })
    });

    #[cfg(all(feature = "polars", feature = "arrow"))]
    {
        // Polars sum
        group.bench_function("polars_sum", |b| {
            let series = Series::new("data", &data);
            b.iter(|| {
                let result = series.sum::<f64>().unwrap();
                black_box(result);
            })
        });
    }

    group.finish();
}

/// Scalability test with different data sizes
fn bench_scalability(c: &mut Criterion) {
    let sizes = vec![1_000, 10_000, 100_000]; // Removed 1_000_000 to reduce memory usage

    for size in sizes {
        let data_a = create_test_data(size);
        let data_b = create_test_data(size);
        let mut result = vec![0.0; size];

        let mut group = c.benchmark_group(&format!("scalability_{}", size));

        group.bench_function("veloxx_advanced", |b| {
            b.iter(|| {
                parallel_simd_add_advanced(&data_a, &data_b, &mut result).unwrap();
                black_box(&result);
            })
        });

        #[cfg(all(feature = "polars", feature = "arrow"))]
        group.bench_function("polars", |b| {
            let series_a = Series::new("a", &data_a);
            let series_b = Series::new("b", &data_b);

            b.iter(|| {
                let result = &series_a + &series_b;
                black_box(result);
            })
        });

        group.finish();
    }
}

/// Performance summary showing our competitive advantage
fn bench_final_comparison(c: &mut Criterion) {
    let size = 100_000;
    let data = create_test_data(size);
    let data_b = create_test_data(size);

    let mut group = c.benchmark_group("final_comparison");

    // Veloxx sum (our strength)
    group.bench_function("veloxx_sum", |b| {
        b.iter(|| {
            let result = parallel_simd_sum_advanced(&data).unwrap();
            black_box(result);
        })
    });

    // Veloxx add with all optimizations
    group.bench_function("veloxx_add_optimized", |b| {
        let mut result = vec![0.0; size];
        b.iter(|| {
            parallel_simd_add_advanced(&data, &data_b, &mut result).unwrap();
            black_box(&result);
        })
    });

    #[cfg(all(feature = "polars", feature = "arrow"))]
    {
        // Polars sum
        group.bench_function("polars_sum", |b| {
            let series = Series::new("data", &data);
            b.iter(|| {
                let result = series.sum::<f64>().unwrap();
                black_box(result);
            })
        });

        // Polars add
        group.bench_function("polars_add", |b| {
            let series_a = Series::new("a", &data);
            let series_b = Series::new("b", &data_b);

            b.iter(|| {
                let result = &series_a + &series_b;
                black_box(result);
            })
        });
    }

    group.finish();
}

criterion_group!(
    comprehensive_benchmarks,
    bench_optimization_layers,
    bench_fused_operations,
    bench_memory_optimization,
    bench_scalability,
    bench_final_comparison
);

criterion_main!(comprehensive_benchmarks);
