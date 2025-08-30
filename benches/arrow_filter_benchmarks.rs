// benches/arrow_filter_benchmarks.rs
use arrow_array::BooleanArray;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[cfg(feature = "arrow")]
use veloxx::arrow::{ArrowFilter, ArrowSeries};

fn bench_arrow_filtering(c: &mut Criterion) {
    let size = 100_000;

    // Create test data
    let data1: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64)).collect();
    let data2: Vec<Option<f64>> = (0..size).map(|i| Some((i as f64) * 2.0)).collect();

    // Test Arrow Series
    #[cfg(feature = "arrow")]
    {
        let arrow_series1 = ArrowSeries::new_f64("a", data1.clone());
        let arrow_series2 = ArrowSeries::new_f64("b", data2.clone());

        c.bench_with_input(
            BenchmarkId::new("arrow_series_equal", size),
            &(&arrow_series1, &arrow_series2),
            |b, (s1, s2)| b.iter(|| s1.equal(s2).unwrap()),
        );

        // Create a mask for filtering
        let mask_data: Vec<bool> = (0..size).map(|i| i % 2 == 0).collect();
        let mask = BooleanArray::from(mask_data);

        c.bench_with_input(
            BenchmarkId::new("arrow_series_filter", size),
            &(&arrow_series1, &mask),
            |b, (s, m)| b.iter(|| s.filter(m).unwrap()),
        );
    }

    // Traditional filtering for comparison
    let values1: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let values2: Vec<f64> = (0..size).map(|i| (i as f64) * 2.0).collect();

    c.bench_with_input(
        BenchmarkId::new("traditional_equal", size),
        &(&values1, &values2),
        |b, (v1, v2)| {
            b.iter(|| {
                let mut result = Vec::with_capacity(v1.len());
                for i in 0..v1.len() {
                    result.push(v1[i] == v2[i]);
                }
                black_box(result)
            })
        },
    );

    let mask_data: Vec<bool> = (0..size).map(|i| i % 2 == 0).collect();

    c.bench_with_input(
        BenchmarkId::new("traditional_filter", size),
        &(&values1, &mask_data),
        |b, (v, m)| {
            b.iter(|| {
                let mut result = Vec::with_capacity(v.len());
                for i in 0..v.len() {
                    if m[i] {
                        result.push(v[i]);
                    }
                }
                black_box(result)
            })
        },
    );
}

criterion_group!(arrow_filter_benches, bench_arrow_filtering);
criterion_main!(arrow_filter_benches);
