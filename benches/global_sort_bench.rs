#[macro_use]
extern crate criterion;
use criterion::Criterion;
// ...existing code...

fn bench_global_sort_f64(_c: &mut Criterion) {
    let _data: Vec<f64> = (0..100_000).map(|_x| rand::random::<f64>()).collect();
    // c.bench_function("global_sort_f64_parallel", |b| {
    //     b.iter(|| {
    //         GlobalSort::sort_f64(&mut data).unwrap();
    //     })
    // });
}

criterion_group!(benches, bench_global_sort_f64);
criterion_main!(benches);
