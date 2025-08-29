#[macro_use]
extern crate criterion;
use criterion::Criterion;
use veloxx::series::Series;

fn bench_percentile_i32(c: &mut Criterion) {
    let data: Vec<i32> = (0..100_000).map(|x| x % 1000).collect();
    let bitmap: Vec<bool> = vec![true; data.len()];
    let series = Series::I32("test".to_string(), data, bitmap);
    c.bench_function("percentile_i32_parallel", |b| {
        b.iter(|| {
            let _ = series.percentile(90.0);
        })
    });
}

fn bench_percentile_f64(c: &mut Criterion) {
    let data: Vec<f64> = (0..100_000).map(|x| (x as f64) * 0.5).collect();
    let bitmap: Vec<bool> = vec![true; data.len()];
    let series = Series::F64("test".to_string(), data, bitmap);
    c.bench_function("percentile_f64_parallel", |b| {
        b.iter(|| {
            let _ = series.percentile(90.0);
        })
    });
}

criterion_group!(benches, bench_percentile_i32, bench_percentile_f64);
criterion_main!(benches);
