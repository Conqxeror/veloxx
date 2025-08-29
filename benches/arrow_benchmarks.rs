// benches/arrow_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use veloxx::performance::simd::SimdOps;
use veloxx::performance::SeriesPerformanceExt;
use veloxx::series::Series;

fn bench_arrow_vs_veloxx(c: &mut Criterion) {
    let size = 100_000;

    // Create test data
    let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
    let data2: Vec<Option<f64>> = (0..size).map(|i| Some((i as f64) * 2.0)).collect();

    // Test Veloxx Series
    let series1 = Series::new_f64("a", data1.clone());
    let series2 = Series::new_f64("b", data2.clone());

    c.bench_with_input(
        BenchmarkId::new("veloxx_series_simd_add", size),
        &(&series1, &series2),
        |b, (s1, s2): &(&Series, &Series)| b.iter(|| s1.simd_add(s2).unwrap()),
    );

    let values1: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let values2: Vec<f64> = (0..size).map(|i| (i as f64) * 2.0).collect();

    c.bench_with_input(
        BenchmarkId::new("veloxx_slice_simd_add", size),
        &(&values1, &values2),
        |b, (v1, v2): &(&Vec<f64>, &Vec<f64>)| b.iter(|| v1.simd_add(v2)),
    );

    // Test Arrow Series (if feature is enabled)
    #[cfg(feature = "arrow")]
    {
        use veloxx::arrow::{ArrowOps, ArrowSeries};

        let arrow_series1 = ArrowSeries::new_f64("a", data1.clone());
        let arrow_series2 = ArrowSeries::new_f64("b", data2.clone());

        c.bench_with_input(
            BenchmarkId::new("arrow_series_simd_add", size),
            &(&arrow_series1, &arrow_series2),
            |b, (s1, s2): &(&ArrowSeries, &ArrowSeries)| b.iter(|| s1.simd_add(s2).unwrap()),
        );

        // Test optimized Arrow operations
        c.bench_with_input(
            BenchmarkId::new("arrow_series_optimized_add", size),
            &(&arrow_series1, &arrow_series2),
            |b, (s1, s2): &(&ArrowSeries, &ArrowSeries)| b.iter(|| s1.arrow_add(s2).unwrap()),
        );
    }

    // Traditional operations for comparison
    c.bench_with_input(
        BenchmarkId::new("veloxx_traditional_add", size),
        &(&values1, &values2),
        |b, (v1, v2): &(&Vec<f64>, &Vec<f64>)| {
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

fn bench_arrow_sum(c: &mut Criterion) {
    let size = 100_000;
    let values: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();

    // Test Veloxx Series sum
    let series = Series::new_f64("test", values.clone());

    c.bench_with_input(
        BenchmarkId::new("veloxx_series_simd_sum", size),
        &series,
        |b, s: &Series| b.iter(|| s.sum().unwrap()),
    );

    let slice_values: Vec<f64> = (0..size).map(|i| i as f64).collect();

    c.bench_with_input(
        BenchmarkId::new("veloxx_slice_simd_sum", size),
        &slice_values,
        |b, v: &Vec<f64>| b.iter(|| v.simd_sum()),
    );

    // Test Arrow Series sum (if feature is enabled)
    #[cfg(feature = "arrow")]
    {
        use veloxx::arrow::{ArrowOps, ArrowSeries};

        let arrow_series = ArrowSeries::new_f64("test", values.clone());

        c.bench_with_input(
            BenchmarkId::new("arrow_series_simd_sum", size),
            &arrow_series,
            |b, s: &ArrowSeries| b.iter(|| s.simd_sum().unwrap()),
        );

        // Test optimized Arrow operations
        c.bench_with_input(
            BenchmarkId::new("arrow_series_optimized_sum", size),
            &arrow_series,
            |b, s: &ArrowSeries| b.iter(|| s.arrow_sum().unwrap()),
        );
    }

    // Traditional sum for comparison
    c.bench_with_input(
        BenchmarkId::new("veloxx_traditional_sum", size),
        &slice_values,
        |b, v: &Vec<f64>| {
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

criterion_group!(arrow_benches, bench_arrow_vs_veloxx, bench_arrow_sum);
criterion_main!(arrow_benches);
