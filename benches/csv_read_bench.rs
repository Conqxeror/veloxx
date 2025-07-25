use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use veloxx::dataframe::DataFrame;

fn bench_csv_read(c: &mut Criterion) {
    c.bench_function("bench_csv_read", |b| {
        b.iter(|| {
            let df = DataFrame::from_csv(black_box("examples/test.csv"));
            let _ = black_box(df);
        })
    });
}

criterion_group!(benches, bench_csv_read);
criterion_main!(benches);
