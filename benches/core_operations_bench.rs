use veloxx::prelude::*;
use veloxx::expressions::col;

use criterion::{criterion_group, criterion_main, Criterion};
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use std::collections::BTreeMap;

fn filter_benchmark(c: &mut Criterion) {
    let mut columns = BTreeMap::new();
    columns.insert("a".to_string(), Series::new_i32("a", (0..1000).map(Some).collect()));
    let df = DataFrame::new(columns).unwrap();

    c.bench_function("filter", |b| {
        b.iter(|| {
            df.filter(col("a").eq(500.into())).unwrap();
        });
    });
}

fn aggregation_benchmark(c: &mut Criterion) {
    let mut columns = BTreeMap::new();
    columns.insert("a".to_string(), Series::new_i32("a", (0..1000).map(Some).collect()));
    let df = DataFrame::new(columns).unwrap();

    c.bench_function("aggregate", |b| {
        b.iter(|| {
            df.aggregate(vec![("a", "mean")]).unwrap();
        });
    });
}

criterion_group!(benches, filter_benchmark, aggregation_benchmark);
criterion_main!(benches);
