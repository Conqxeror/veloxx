// benches/arrow_aggregate_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[cfg(feature = "arrow")]
use veloxx::arrow::{ArrowAggregate, ArrowSeries};

fn bench_arrow_aggregations(c: &mut Criterion) {
    let size = 100_000;

    // Create test data
    let data_f64: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
    let data_i32: Vec<Option<i32>> = (0..size).map(|i| Some(i as i32)).collect();

    // Test Arrow Series
    #[cfg(feature = "arrow")]
    {
        let arrow_series_f64 = ArrowSeries::new_f64("a", data_f64.clone());
        let arrow_series_i32 = ArrowSeries::new_i32("b", data_i32.clone());

        c.bench_with_input(
            BenchmarkId::new("arrow_series_mean_f64", size),
            &arrow_series_f64,
            |b, s| b.iter(|| s.mean().unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_series_min_i32", size),
            &arrow_series_i32,
            |b, s| b.iter(|| s.min().unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_series_max_i32", size),
            &arrow_series_i32,
            |b, s| b.iter(|| s.max().unwrap()),
        );
    }

    // Traditional aggregation for comparison
    let values_f64: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let values_i32: Vec<i32> = (0..size).map(|i| i as i32).collect();

    c.bench_with_input(
        BenchmarkId::new("traditional_mean_f64", size),
        &values_f64,
        |b, v| {
            b.iter(|| {
                let sum: f64 = v.iter().sum();
                black_box(sum / v.len() as f64)
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("traditional_min_i32", size),
        &values_i32,
        |b, v| {
            b.iter(|| {
                let mut min = v[0];
                for &val in v.iter() {
                    if val < min {
                        min = val;
                    }
                }
                black_box(min)
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("traditional_max_i32", size),
        &values_i32,
        |b, v| {
            b.iter(|| {
                let mut max = v[0];
                for &val in v.iter() {
                    if val > max {
                        max = val;
                    }
                }
                black_box(max)
            })
        },
    );
}

criterion_group!(arrow_aggregate_benches, bench_arrow_aggregations);
criterion_main!(arrow_aggregate_benches);
