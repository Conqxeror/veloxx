use arrow_array::Float64Array;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use veloxx::arrow::ops::ArrowOps;
use veloxx::arrow::series::ArrowSeries;

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

fn bench_sequential_operations(c: &mut Criterion) {
    let (series1, series2) = create_test_data_f64(10000);

    c.bench_function("sequential_operations", |b| {
        b.iter(|| {
            let result1 = series1.arrow_add(black_box(&series2)).unwrap();
            let result2 = result1.arrow_pow(2.0).unwrap();
            let result3 = result2.arrow_sqrt().unwrap();
            black_box(result3);
        })
    });
}

fn bench_fused_operations(c: &mut Criterion) {
    let (_series1, _series2) = create_test_data_f64(10000);

    c.bench_function("fused_operations", |b| {
        b.iter(|| {
            // let fused_expr = series1.fuse().add().pow(2.0).sqrt();
            // let result = fused_expr.evaluate_binary(&series1, &series2).unwrap();
            // black_box(result);
        })
    });
}

fn bench_sequential_operations_with_simd(c: &mut Criterion) {
    let (series1, series2) = create_test_data_f64(10000);

    c.bench_function("sequential_operations_with_simd", |b| {
        b.iter(|| {
            let result1 = series1.simd_add(black_box(&series2)).unwrap();
            let result2 = result1.arrow_pow(2.0).unwrap();
            let result3 = result2.arrow_sqrt().unwrap();
            black_box(result3);
        })
    });
}

fn bench_fused_operations_with_simd(c: &mut Criterion) {
    let (_series1, _series2) = create_test_data_f64(10000);

    c.bench_function("fused_operations_with_simd", |b| {
        b.iter(|| {
            // let fused_expr = series1.fuse().add().pow(2.0).sqrt();
            // let result = fused_expr.evaluate_binary(&series1, &series2).unwrap();
            // black_box(result);
        })
    });
}

criterion_group!(
    benches,
    bench_sequential_operations,
    bench_fused_operations,
    bench_sequential_operations_with_simd,
    bench_fused_operations_with_simd,
);

criterion_main!(benches);
