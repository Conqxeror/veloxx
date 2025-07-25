use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::BTreeMap;
use veloxx::conditions::Condition;
use veloxx::dataframe::join::JoinType;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::types::Value;

fn bench_dataframe_creation(c: &mut Criterion) {
    c.bench_function("dataframe_creation", |b| {
        b.iter(|| {
            let mut columns = BTreeMap::new();
            columns.insert(
                "col1".to_string(),
                Series::new_i32("col1", vec![Some(1); 1000]),
            );
            columns.insert(
                "col2".to_string(),
                Series::new_f64("col2", vec![Some(1.0); 1000]),
            );
            DataFrame::new(columns).unwrap();
        });
    });
}

fn bench_dataframe_from_csv(c: &mut Criterion) {
    c.bench_function("dataframe_from_csv", |b| {
        b.iter(|| {
            DataFrame::from_csv("examples/test.csv").unwrap();
        });
    });
}

fn bench_dataframe_filter(c: &mut Criterion) {
    let mut columns = BTreeMap::new();
    columns.insert(
        "col1".to_string(),
        Series::new_i32("col1", (0..1000).map(Some).collect()),
    );
    columns.insert(
        "col2".to_string(),
        Series::new_f64("col2", (0..1000).map(|i| Some(i as f64)).collect()),
    );
    let df = DataFrame::new(columns).unwrap();
    let condition = Condition::Gt("col1".to_string(), Value::I32(500));

    c.bench_function("dataframe_filter", |b| {
        b.iter(|| {
            df.filter(&condition).unwrap();
        });
    });
}

fn bench_series_operations(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(Some).collect());
    let series_f64 = Series::new_f64("data", (0..1000).map(|i| Some(i as f64)).collect());

    c.bench_function("series_sum_i32", |b| {
        b.iter(|| {
            series_i32.sum().unwrap();
        });
    });

    c.bench_function("series_sum_f64", |b| {
        b.iter(|| {
            series_f64.sum().unwrap();
        });
    });

    c.bench_function("series_mean_i32", |b| {
        b.iter(|| {
            series_i32.mean().unwrap();
        });
    });

    c.bench_function("series_mean_f64", |b| {
        b.iter(|| {
            series_f64.mean().unwrap();
        });
    });
}

fn bench_dataframe_join(c: &mut Criterion) {
    let mut columns1 = BTreeMap::new();
    columns1.insert(
        "id".to_string(),
        Series::new_i32("id", (0..1000).map(Some).collect()),
    );
    columns1.insert(
        "value1".to_string(),
        Series::new_i32("value1", (0..1000).map(Some).collect()),
    );
    let df1 = DataFrame::new(columns1).unwrap();

    let mut columns2 = BTreeMap::new();
    columns2.insert(
        "id".to_string(),
        Series::new_i32("id", (0..1000).map(Some).collect()),
    );
    columns2.insert(
        "value2".to_string(),
        Series::new_i32("value2", (0..1000).map(Some).collect()),
    );
    let df2 = DataFrame::new(columns2).unwrap();

    c.bench_function("dataframe_join_inner", |b| {
        b.iter(|| {
            df1.join(&df2, "id", JoinType::Inner).unwrap();
        });
    });
}

fn bench_dataframe_drop_nulls(c: &mut Criterion) {
    let mut columns = BTreeMap::new();
    columns.insert(
        "col1".to_string(),
        Series::new_i32(
            "col1",
            (0..1000)
                .map(|i| if i % 10 == 0 { None } else { Some(i) })
                .collect(),
        ),
    );
    columns.insert(
        "col2".to_string(),
        Series::new_f64(
            "col2",
            (0..1000)
                .map(|i| if i % 10 == 0 { None } else { Some(i as f64) })
                .collect(),
        ),
    );
    let df = DataFrame::new(columns).unwrap();

    c.bench_function("dataframe_drop_nulls", |b| {
        b.iter(|| {
            df.drop_nulls(None).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_dataframe_creation,
    bench_dataframe_from_csv,
    bench_dataframe_filter,
    bench_series_operations,
    bench_dataframe_join,
    bench_dataframe_drop_nulls
);

criterion_main!(benches);
