// benches/arrow_math_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[cfg(feature = "arrow")]
use veloxx::arrow::{ArrowOps, ArrowSeries};

fn bench_arrow_math_ops(c: &mut Criterion) {
    let size = 100_000;

    // Create test data with smaller values to avoid overflow and zero values for division
    let data1_f64: Vec<Option<f64>> = (0..size).map(|i| Some((i % 1000 + 1) as f64)).collect();
    let data2_f64: Vec<Option<f64>> = (0..size)
        .map(|i| Some(((i % 1000) as f64) * 2.0 + 1.0))
        .collect();
    let data1_i32: Vec<Option<i32>> = (0..size).map(|i| Some(i % 1000 + 1)).collect();
    let data2_i32: Vec<Option<i32>> = (0..size).map(|i| Some((i % 1000) * 2 + 1)).collect();

    // Test Arrow Series
    #[cfg(feature = "arrow")]
    {
        let arrow_series1_f64 = ArrowSeries::new_f64("a", data1_f64.clone());
        let arrow_series2_f64 = ArrowSeries::new_f64("b", data2_f64.clone());
        let arrow_series1_i32 = ArrowSeries::new_i32("c", data1_i32.clone());
        let arrow_series2_i32 = ArrowSeries::new_i32("d", data2_i32.clone());

        c.bench_with_input(
            BenchmarkId::new("arrow_f64_add", size),
            &(&arrow_series1_f64, &arrow_series2_f64),
            |b, (s1, s2)| b.iter(|| s1.arrow_add(s2).unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_f64_sub", size),
            &(&arrow_series1_f64, &arrow_series2_f64),
            |b, (s1, s2)| b.iter(|| s1.arrow_sub(s2).unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_f64_mul", size),
            &(&arrow_series1_f64, &arrow_series2_f64),
            |b, (s1, s2)| b.iter(|| s1.arrow_mul(s2).unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_f64_div", size),
            &(&arrow_series1_f64, &arrow_series2_f64),
            |b, (s1, s2)| b.iter(|| s1.arrow_div(s2).unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_i32_add", size),
            &(&arrow_series1_i32, &arrow_series2_i32),
            |b, (s1, s2)| b.iter(|| s1.arrow_add(s2).unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_i32_sub", size),
            &(&arrow_series1_i32, &arrow_series2_i32),
            |b, (s1, s2)| b.iter(|| s1.arrow_sub(s2).unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_i32_mul", size),
            &(&arrow_series1_i32, &arrow_series2_i32),
            |b, (s1, s2)| b.iter(|| s1.arrow_mul(s2).unwrap()),
        );

        c.bench_with_input(
            BenchmarkId::new("arrow_i32_div", size),
            &(&arrow_series1_i32, &arrow_series2_i32),
            |b, (s1, s2)| b.iter(|| s1.arrow_div(s2).unwrap()),
        );
    }

    // Traditional mathematical operations for comparison
    let values1_f64: Vec<f64> = (0..size).map(|i| (i % 1000 + 1) as f64).collect();
    let values2_f64: Vec<f64> = (0..size).map(|i| ((i % 1000) as f64) * 2.0 + 1.0).collect();
    let values1_i32: Vec<i32> = (0..size).map(|i| i % 1000 + 1).collect();
    let values2_i32: Vec<i32> = (0..size).map(|i| (i % 1000) * 2 + 1).collect();

    c.bench_with_input(
        BenchmarkId::new("traditional_f64_add", size),
        &(&values1_f64, &values2_f64),
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

    c.bench_with_input(
        BenchmarkId::new("traditional_f64_sub", size),
        &(&values1_f64, &values2_f64),
        |b, (v1, v2)| {
            b.iter(|| {
                let mut result = Vec::with_capacity(v1.len());
                for i in 0..v1.len() {
                    result.push(v1[i] - v2[i]);
                }
                black_box(result)
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("traditional_f64_mul", size),
        &(&values1_f64, &values2_f64),
        |b, (v1, v2)| {
            b.iter(|| {
                let mut result = Vec::with_capacity(v1.len());
                for i in 0..v1.len() {
                    result.push(v1[i] * v2[i]);
                }
                black_box(result)
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("traditional_f64_div", size),
        &(&values1_f64, &values2_f64),
        |b, (v1, v2)| {
            b.iter(|| {
                let mut result = Vec::with_capacity(v1.len());
                for i in 0..v1.len() {
                    result.push(v1[i] / v2[i]);
                }
                black_box(result)
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("traditional_i32_add", size),
        &(&values1_i32, &values2_i32),
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

    c.bench_with_input(
        BenchmarkId::new("traditional_i32_sub", size),
        &(&values1_i32, &values2_i32),
        |b, (v1, v2)| {
            b.iter(|| {
                let mut result = Vec::with_capacity(v1.len());
                for i in 0..v1.len() {
                    result.push(v1[i] - v2[i]);
                }
                black_box(result)
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("traditional_i32_mul", size),
        &(&values1_i32, &values2_i32),
        |b, (v1, v2)| {
            b.iter(|| {
                let mut result = Vec::with_capacity(v1.len());
                for i in 0..v1.len() {
                    result.push(v1[i] * v2[i]);
                }
                black_box(result)
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("traditional_i32_div", size),
        &(&values1_i32, &values2_i32),
        |b, (v1, v2)| {
            b.iter(|| {
                let mut result = Vec::with_capacity(v1.len());
                for i in 0..v1.len() {
                    result.push(v1[i] / v2[i]);
                }
                black_box(result)
            })
        },
    );
}

criterion_group!(arrow_math_benches, bench_arrow_math_ops);
criterion_main!(arrow_math_benches);
