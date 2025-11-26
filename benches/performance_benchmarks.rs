//! Benchmark suite for Veloxx performance optimizations
//!
//! This benchmark compares the performance of traditional operations
//! with SIMD-optimized and parallel operations.

use criterion::{criterion_group, criterion_main, Criterion};
use veloxx::dataframe::DataFrame;
use veloxx::performance::series_ext::SeriesPerformanceExt;
use veloxx::series::Series;
// ...existing code...

fn create_test_series(size: usize) -> (Series, Series) {
    let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
    let data2: Vec<Option<f64>> = (0..size).map(|i| Some((i * 2) as f64)).collect();

    (
        Series::new_f64("data1", data1),
        Series::new_f64("data2", data2),
    )
}

use indexmap::IndexMap;

fn create_test_dataframe(size: usize) -> DataFrame {
    let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
    let data2: Vec<Option<i32>> = (0..size).map(|i| Some(i as i32)).collect();

    let mut columns = IndexMap::new();
    columns.insert("col1".to_string(), Series::new_f64("col1", data1));
    columns.insert("col2".to_string(), Series::new_i32("col2", data2));

    DataFrame::new(columns)
}

fn bench_traditional_add(c: &mut Criterion) {
    let (series1, series2) = create_test_series(100_000);

    c.bench_function("traditional_add_100k", |b| {
        b.iter(|| match (&series1, &series2) {
            (Series::F64(_, a, a_bitmap), Series::F64(_, b, b_bitmap)) => {
                let result: Vec<Option<f64>> = a
                    .iter()
                    .zip(a_bitmap.iter())
                    .zip(b.iter().zip(b_bitmap.iter()))
                    .map(|((a_val, a_valid), (b_val, b_valid))| {
                        if *a_valid && *b_valid {
                            Some(a_val + b_val)
                        } else {
                            None
                        }
                    })
                    .collect();
                Series::new_f64("result", result)
            }
            _ => panic!("Unexpected series types"),
        })
    });
}

fn bench_simd_add(c: &mut Criterion) {
    let (series1, series2) = create_test_series(100_000);

    c.bench_function("simd_add_100k", |b| {
        b.iter(|| series1.simd_add(&series2).unwrap())
    });
}

fn bench_traditional_sum(c: &mut Criterion) {
    let (series, _) = create_test_series(100_000);

    c.bench_function("traditional_sum_100k", |b| b.iter(|| series.sum().unwrap()));
}

fn bench_parallel_sum(c: &mut Criterion) {
    let (series, _) = create_test_series(100_000);

    c.bench_function("parallel_sum_100k", |b| {
        b.iter(|| series.par_sum().unwrap())
    });
}

fn bench_dataframe_operations(c: &mut Criterion) {
    let df = create_test_dataframe(10_000);

    c.bench_function("dataframe_column_access", |b| {
        b.iter(|| df.get_column("col1").unwrap())
    });
}

criterion_group!(
    benches,
    bench_traditional_add,
    bench_simd_add,
    bench_traditional_sum,
    bench_parallel_sum,
    bench_dataframe_operations
);
criterion_main!(benches);
