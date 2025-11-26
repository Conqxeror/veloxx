#[macro_use]
extern crate criterion;
use criterion::Criterion;
use std::collections::HashMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use indexmap::IndexMap;

fn bench_groupby_sum(c: &mut Criterion) {
    let mut columns = IndexMap::new();
    let group: Vec<i32> = (0..100_000).map(|x| x % 1000).collect();
    let values: Vec<f64> = (0..100_000).map(|x| (x as f64) * 0.5).collect();
    let bitmap: Vec<bool> = vec![true; group.len()];
    columns.insert(
        "group".to_string(),
        Series::I32("group".to_string(), group, bitmap.clone()),
    );
    columns.insert(
        "values".to_string(),
        Series::F64("values".to_string(), values, bitmap),
    );
    let df = DataFrame::new(columns);
    c.bench_function("groupby_sum_parallel", |b| {
        b.iter(|| {
            let _ = df.group_by(vec!["group".to_string()]).unwrap().agg_sum();
        })
    });
}

criterion_group!(benches, bench_groupby_sum);
criterion_main!(benches);