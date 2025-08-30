#[macro_use]
extern crate criterion;
use criterion::Criterion;
use veloxx::series::Series;

fn bench_correlation(c: &mut Criterion) {
    let data1: Vec<f64> = (0..100_000).map(|x| (x as f64) * 0.5).collect();
    let data2: Vec<f64> = (0..100_000).map(|x| (x as f64) * 0.3 + 10.0).collect();
    let bitmap: Vec<bool> = vec![true; data1.len()];
    let s1 = Series::F64("s1".to_string(), data1, bitmap.clone());
    let s2 = Series::F64("s2".to_string(), data2, bitmap);
    c.bench_function("correlation_parallel", |b| {
        b.iter(|| {
            let _ = s1.correlation(&s2);
        })
    });
}

criterion_group!(benches, bench_correlation);
criterion_main!(benches);
