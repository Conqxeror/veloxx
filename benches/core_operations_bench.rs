use criterion::{criterion_group, criterion_main, Criterion};

use veloxx::dataframe::join::JoinType;
use veloxx::dataframe::DataFrame;

use indexmap::IndexMap;
use veloxx::conditions::Condition;
use veloxx::series::Series;
use veloxx::types::Value;

fn filter_benchmark_small(c: &mut Criterion) {
    let mut columns: IndexMap<String, Series> = IndexMap::new();
    columns.insert(
        "id".to_string(),
        Series::new_i32("id", (0..10000).map(Some).collect()),
    );
    columns.insert(
        "value".to_string(),
        Series::new_f64("value", (0..10000).map(|i| Some(i as f64 * 1.5)).collect()),
    );
    columns.insert(
        "category".to_string(),
        Series::new_string(
            "category",
            (0..10000)
                .map(|i| Some(format!("category_{}", i % 10)))
                .collect(),
        ),
    );
    let df = DataFrame::new(columns);

    c.bench_function("filter_small_10k", |b| {
        b.iter(|| {
            let _filtered = df
                .filter(&Condition::Gt("id".to_string(), Value::I32(5000)))
                .unwrap();
        });
    });
}

fn filter_benchmark_medium(c: &mut Criterion) {
    let mut columns: IndexMap<String, Series> = IndexMap::new();
    columns.insert(
        "id".to_string(),
        Series::new_i32("id", (0..100000).map(Some).collect()),
    );
    columns.insert(
        "value".to_string(),
        Series::new_f64("value", (0..100000).map(|i| Some(i as f64 * 1.5)).collect()),
    );
    columns.insert(
        "category".to_string(),
        Series::new_string(
            "category",
            (0..100000)
                .map(|i| Some(format!("category_{}", i % 100)))
                .collect(),
        ),
    );
    let df = DataFrame::new(columns);

    c.bench_function("filter_medium_100k", |b| {
        b.iter(|| {
            let _filtered = df
                .filter(&Condition::Gt("id".to_string(), Value::I32(50000)))
                .unwrap();
        });
    });
}

fn aggregation_benchmark_small(c: &mut Criterion) {
    let mut columns: IndexMap<String, Series> = IndexMap::new();
    columns.insert(
        "category".to_string(),
        Series::new_string(
            "category",
            (0..10000)
                .map(|i| Some(format!("cat_{}", i % 100)))
                .collect(),
        ),
    );
    columns.insert(
        "value".to_string(),
        Series::new_i32("value", (0..10000).map(Some).collect()),
    );
    columns.insert(
        "amount".to_string(),
        Series::new_f64("amount", (0..10000).map(|i| Some(i as f64 * 2.5)).collect()),
    );
    let df = DataFrame::new(columns);

    c.bench_function("group_by_agg_small_10k", |b| {
        b.iter(|| {
            let grouped_df = df.group_by(vec!["category".to_string()]).unwrap();
            let _aggregated_df = grouped_df
                .agg(vec![("value", "sum"), ("amount", "mean")])
                .unwrap();
        });
    });
}

fn aggregation_benchmark_medium(c: &mut Criterion) {
    let mut columns: IndexMap<String, Series> = IndexMap::new();
    columns.insert(
        "category".to_string(),
        Series::new_string(
            "category",
            (0..100000)
                .map(|i| Some(format!("cat_{}", i % 1000)))
                .collect(),
        ),
    );
    columns.insert(
        "value".to_string(),
        Series::new_i32("value", (0..100000).map(Some).collect()),
    );
    columns.insert(
        "amount".to_string(),
        Series::new_f64(
            "amount",
            (0..100000).map(|i| Some(i as f64 * 2.5)).collect(),
        ),
    );
    let df = DataFrame::new(columns);

    c.bench_function("group_by_agg_medium_100k", |b| {
        b.iter(|| {
            let grouped_df = df.group_by(vec!["category".to_string()]).unwrap();
            let _aggregated_df = grouped_df
                .agg(vec![("value", "sum"), ("amount", "mean")])
                .unwrap();
        });
    });
}

fn join_benchmark_small(c: &mut Criterion) {
    // Create first dataframe
    let mut columns1: IndexMap<String, Series> = IndexMap::new();
    columns1.insert(
        "id".to_string(),
        Series::new_i32("id", (0..10000).map(Some).collect()),
    );
    columns1.insert(
        "value1".to_string(),
        Series::new_f64("value1", (0..10000).map(|i| Some(i as f64 * 1.5)).collect()),
    );
    let df1 = DataFrame::new(columns1);

    // Create second dataframe
    let mut columns2: IndexMap<String, Series> = IndexMap::new();
    columns2.insert(
        "id".to_string(),
        Series::new_i32("id", (0..10000).map(|i| Some(i + 5000)).collect()), // Overlapping ids
    );
    columns2.insert(
        "value2".to_string(),
        Series::new_f64("value2", (0..10000).map(|i| Some(i as f64 * 2.5)).collect()),
    );
    let df2 = DataFrame::new(columns2);

    c.bench_function("join_small_10k", |b| {
        b.iter(|| {
            let _joined = df1.join(&df2, "id", JoinType::Inner).unwrap();
        });
    });
}

fn join_benchmark_medium(c: &mut Criterion) {
    // Create first dataframe
    let mut columns1: IndexMap<String, Series> = IndexMap::new();
    columns1.insert(
        "id".to_string(),
        Series::new_i32("id", (0..100000).map(Some).collect()),
    );
    columns1.insert(
        "value1".to_string(),
        Series::new_f64(
            "value1",
            (0..100000).map(|i| Some(i as f64 * 1.5)).collect(),
        ),
    );
    let df1 = DataFrame::new(columns1);

    // Create second dataframe
    let mut columns2: IndexMap<String, Series> = IndexMap::new();
    columns2.insert(
        "id".to_string(),
        Series::new_i32("id", (0..100000).map(|i| Some(i + 50000)).collect()), // Overlapping ids
    );
    columns2.insert(
        "value2".to_string(),
        Series::new_f64(
            "value2",
            (0..100000).map(|i| Some(i as f64 * 2.5)).collect(),
        ),
    );
    let df2 = DataFrame::new(columns2);

    c.bench_function("join_medium_100k", |b| {
        b.iter(|| {
            let _joined = df1.join(&df2, "id", JoinType::Inner).unwrap();
        });
    });
}

criterion_group!(
    benches,
    filter_benchmark_small,
    filter_benchmark_medium,
    aggregation_benchmark_small,
    aggregation_benchmark_medium,
    join_benchmark_small,
    join_benchmark_medium
);
criterion_main!(benches);
