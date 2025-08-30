// benches/arrow_string_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[cfg(feature = "arrow")]
use veloxx::arrow::{ArrowSeries, ArrowStringOps};

fn bench_arrow_string_ops(c: &mut Criterion) {
    let size = 10_000;

    // Create test data
    let data: Vec<Option<String>> = (0..size)
        .map(|i| Some(format!("test_string_{}", i)))
        .collect();

    // Test Arrow Series
    #[cfg(feature = "arrow")]
    {
        let arrow_series = ArrowSeries::new_string("test", data.clone());

        c.bench_with_input(
            BenchmarkId::new("arrow_string_to_uppercase", size),
            &arrow_series,
            |b, s| b.iter(|| s.to_uppercase().unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_string_to_lowercase", size),
            &arrow_series,
            |b, s| b.iter(|| s.to_lowercase().unwrap()),
        );
    }

    // Traditional string operations for comparison
    let values: Vec<String> = (0..size).map(|i| format!("test_string_{}", i)).collect();

    c.bench_with_input(
        BenchmarkId::new("traditional_to_uppercase", size),
        &values,
        |b, v| {
            b.iter(|| {
                let result: Vec<String> = v.iter().map(|s| s.to_uppercase()).collect();
                black_box(result)
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("traditional_to_lowercase", size),
        &values,
        |b, v| {
            b.iter(|| {
                let result: Vec<String> = v.iter().map(|s| s.to_lowercase()).collect();
                black_box(result)
            })
        },
    );
}

criterion_group!(arrow_string_benches, bench_arrow_string_ops);
criterion_main!(arrow_string_benches);
