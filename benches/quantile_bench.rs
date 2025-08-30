#[macro_use]
extern crate criterion;
use criterion::Criterion;
use veloxx::series::Series;

fn bench_quantile_i32(c: &mut Criterion) {
    let data: Vec<i32> = (0..100_000).map(|x| x % 1000).collect();
    let bitmap: Vec<bool> = vec![true; data.len()];
    let series = Series::I32("test".to_string(), data, bitmap);
    c.bench_function("quantile_i32_parallel", |b| {
        b.iter(|| {
            let _ = series.quantile(0.75);
        })
    });
}

fn bench_quantile_f64(c: &mut Criterion) {
    let data: Vec<f64> = (0..100_000).map(|x| (x as f64) * 0.5).collect();
    let bitmap: Vec<bool> = vec![true; data.len()];
    let series = Series::F64("test".to_string(), data, bitmap);
    c.bench_function("quantile_f64_parallel", |b| {
        b.iter(|| {
            let _ = series.quantile(0.75);
        })
    });
}

criterion_group!(benches, bench_quantile_i32, bench_quantile_f64);
criterion_main!(benches);
