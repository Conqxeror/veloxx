//! Benchmark for Arrow SIMD operations

#[cfg(feature = "arrow")]
use arrow_array::{Float64Array, Int32Array};
#[cfg(feature = "arrow")]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[cfg(feature = "arrow")]
use std::sync::Arc;

#[cfg(feature = "arrow")]
use veloxx::arrow::ops::ArrowOps;
#[cfg(feature = "arrow")]
use veloxx::arrow::series::ArrowSeries;

#[cfg(feature = "arrow")]
fn create_test_data_f64(size: usize) -> (ArrowSeries, ArrowSeries) {
    let values1: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let values2: Vec<f64> = (0..size).map(|i| (i * 2) as f64).collect();

    let series1 = ArrowSeries::F64(
        "test1".to_string(),
        Arc::new(Float64Array::from(values1)),
        None,
    );

    let series2 = ArrowSeries::F64(
        "test2".to_string(),
        Arc::new(Float64Array::from(values2)),
        None,
    );

    (series1, series2)
}

#[cfg(feature = "arrow")]
fn create_test_data_i32(size: usize) -> (ArrowSeries, ArrowSeries) {
    let values1: Vec<i32> = (0..size).map(|i| i as i32).collect();
    let values2: Vec<i32> = (0..size).map(|i| (i * 2) as i32).collect();

    let series1 = ArrowSeries::I32(
        "test1".to_string(),
        Arc::new(Int32Array::from(values1)),
        None,
    );

    let series2 = ArrowSeries::I32(
        "test2".to_string(),
        Arc::new(Int32Array::from(values2)),
        None,
    );

    (series1, series2)
}

#[cfg(feature = "arrow")]
fn bench_arrow_add_f64(c: &mut Criterion) {
    let (series1, series2) = create_test_data_f64(100000);

    c.bench_function("arrow_add_f64", |b| {
        b.iter(|| {
            let result = series1.arrow_add(black_box(&series2)).unwrap();
            black_box(result);
        })
    });
}

#[cfg(feature = "arrow")]
fn bench_simd_add_f64(c: &mut Criterion) {
    let (series1, series2) = create_test_data_f64(100000);

    c.bench_function("simd_add_f64", |b| {
        b.iter(|| {
            let result = series1.simd_add(black_box(&series2)).unwrap();
            black_box(result);
        })
    });
}

#[cfg(feature = "arrow")]
fn bench_arrow_add_i32(c: &mut Criterion) {
    let (series1, series2) = create_test_data_i32(100000);

    c.bench_function("arrow_add_i32", |b| {
        b.iter(|| {
            let result = series1.arrow_add(black_box(&series2)).unwrap();
            black_box(result);
        })
    });
}

#[cfg(feature = "arrow")]
fn bench_simd_add_i32(c: &mut Criterion) {
    let (series1, series2) = create_test_data_i32(100000);

    c.bench_function("simd_add_i32", |b| {
        b.iter(|| {
            let result = series1.simd_add(black_box(&series2)).unwrap();
            black_box(result);
        })
    });
}

#[cfg(feature = "arrow")]
fn bench_arrow_sum_f64(c: &mut Criterion) {
    let (series1, _) = create_test_data_f64(100000);

    c.bench_function("arrow_sum_f64", |b| {
        b.iter(|| {
            let result = series1.arrow_sum().unwrap();
            black_box(result);
        })
    });
}

#[cfg(feature = "arrow")]
fn bench_simd_sum_f64(c: &mut Criterion) {
    let (series1, _) = create_test_data_f64(100000);

    c.bench_function("simd_sum_f64", |b| {
        b.iter(|| {
            let result = series1.simd_sum().unwrap();
            black_box(result);
        })
    });
}

#[cfg(feature = "arrow")]
fn bench_arrow_sum_i32(c: &mut Criterion) {
    let (series1, _) = create_test_data_i32(100000);

    c.bench_function("arrow_sum_i32", |b| {
        b.iter(|| {
            let result = series1.arrow_sum().unwrap();
            black_box(result);
        })
    });
}

#[cfg(feature = "arrow")]
fn bench_simd_sum_i32(c: &mut Criterion) {
    let (series1, _) = create_test_data_i32(100000);

    c.bench_function("simd_sum_i32", |b| {
        b.iter(|| {
            let result = series1.simd_sum().unwrap();
            black_box(result);
        })
    });
}

#[cfg(feature = "arrow")]
criterion_group!(
    benches,
    bench_arrow_add_f64,
    bench_simd_add_f64,
    bench_arrow_add_i32,
    bench_simd_add_i32,
    bench_arrow_sum_f64,
    bench_simd_sum_f64,
    bench_arrow_sum_i32,
    bench_simd_sum_i32,
);

#[cfg(feature = "arrow")]
criterion_main!(benches);

#[cfg(not(feature = "arrow"))]
fn main() {
    println!("Benchmarks require the 'arrow' feature to be enabled");
}
