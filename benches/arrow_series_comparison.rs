// benches/arrow_series_comparison.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polars::prelude::*;
use veloxx::arrow::series::ArrowSeries;

fn create_test_data(size: usize) -> Vec<f64> {
    (0..size).map(|i| i as f64).collect()
}

fn create_arrow_series(size: usize) -> ArrowSeries {
    let data: Vec<f64> = create_test_data(size);
    let opt_data: Vec<Option<f64>> = data.into_iter().map(Some).collect();
    ArrowSeries::new_f64("test", opt_data)
}

fn create_polars_series(size: usize) -> Series {
    let data: Vec<f64> = create_test_data(size);
    Series::new("test".into(), data)
}

fn bench_arrow_vs_polars(c: &mut Criterion) {
    let size = 100_000;

    // ArrowSeries sum benchmark
    c.bench_function("arrow_series_simd_sum", |b| {
        let series = create_arrow_series(size);
        b.iter(|| {
            let result = series.simd_sum().unwrap();
            black_box(result)
        })
    });

    // Polars sum benchmark
    c.bench_function("polars_series_sum", |b| {
        let series = create_polars_series(size);
        b.iter(|| {
            let result = series.sum::<f64>().unwrap();
            black_box(result)
        })
    });

    // ArrowSeries add benchmark
    c.bench_function("arrow_series_simd_add", |b| {
        let series1 = create_arrow_series(size);
        let series2 = create_arrow_series(size);
        b.iter(|| {
            let result = series1.simd_add(&series2).unwrap();
            black_box(result)
        })
    });

    // Polars add benchmark
    c.bench_function("polars_series_add", |b| {
        let series1 = create_polars_series(size);
        let series2 = create_polars_series(size);
        b.iter(|| {
            let result = &series1 + &series2;
            black_box(result)
        })
    });
}

criterion_group!(arrow_benchmarks, bench_arrow_vs_polars);
criterion_main!(arrow_benchmarks);
