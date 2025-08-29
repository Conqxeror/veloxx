// benches/simd_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use veloxx::performance::simd::SimdOps;
use veloxx::performance::SeriesPerformanceExt;
use veloxx::series::Series;

fn bench_simd_operations(c: &mut Criterion) {
    // Create test data
    let size = 100_000;
    let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
    let data2: Vec<Option<f64>> = (0..size).map(|i| Some((i as f64) * 2.0)).collect();

    let series1 = Series::new_f64("a", data1);
    let series2 = Series::new_f64("b", data2);

    c.bench_with_input(
        BenchmarkId::new("simd_add_series", size),
        &(&series1, &series2),
        |b, (s1, s2)| b.iter(|| s1.simd_add(s2).unwrap()),
    );

    // Test raw slice operations
    let values1: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let values2: Vec<f64> = (0..size).map(|i| (i as f64) * 2.0).collect();

    c.bench_with_input(
        BenchmarkId::new("simd_add_slices", size),
        &(&values1, &values2),
        |b, (v1, v2)| b.iter(|| v1.simd_add(v2)),
    );

    // Compare with traditional operations
    c.bench_with_input(
        BenchmarkId::new("traditional_add", size),
        &(&values1, &values2),
        |b, (v1, v2)| {
            b.iter(|| {
                let mut result = Vec::with_capacity(v1.len());
                for i in 0..v1.len() {
                    result.push(v1[i] + v2[i]);
                }
                black_box(result)
            })
        },
    );
}

fn bench_simd_sum(c: &mut Criterion) {
    let size = 100_000;
    let values: Vec<f64> = (0..size).map(|i| i as f64).collect();

    c.bench_with_input(BenchmarkId::new("simd_sum", size), &values, |b, v| {
        b.iter(|| v.simd_sum())
    });

    c.bench_with_input(
        BenchmarkId::new("traditional_sum", size),
        &values,
        |b, v| {
            b.iter(|| {
                let mut sum = 0.0;
                for &val in v {
                    sum += val;
                }
                black_box(sum)
            })
        },
    );
}

fn bench_optimized_simd(c: &mut Criterion) {
    let size = 100_000;
    let values: Vec<f64> = (0..size).map(|i| i as f64).collect();

    c.bench_with_input(
        BenchmarkId::new("simd_sum_optimized", size),
        &values,
        |b, v| b.iter(|| veloxx::performance::simd::optimized::simd_sum_optimized(v)),
    );
}

criterion_group!(
    simd_benches,
    bench_simd_operations,
    bench_simd_sum,
    bench_optimized_simd
);
criterion_main!(simd_benches);
