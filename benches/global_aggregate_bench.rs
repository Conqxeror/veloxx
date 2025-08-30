#[macro_use]
extern crate criterion;
use criterion::Criterion;
use veloxx::performance::global_aggregate::GlobalAggregate;

fn bench_global_sum_mean_f64(c: &mut Criterion) {
    let data: Vec<f64> = (0..100_000).map(|_| rand::random::<f64>()).collect();
    c.bench_function("global_sum_f64_parallel", |b| {
        b.iter(|| {
            let _ = GlobalAggregate::sum_f64(&data);
        })
    });
    c.bench_function("global_mean_f64_parallel", |b| {
        b.iter(|| {
            let _ = GlobalAggregate::mean_f64(&data).unwrap();
        })
    });
}

criterion_group!(benches, bench_global_sum_mean_f64);
criterion_main!(benches);
