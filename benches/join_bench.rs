use criterion::{criterion_group, criterion_main, Criterion};

use veloxx::dataframe::join::JoinType;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

fn create_test_dataframe(size: usize) -> DataFrame {
    let ids: Vec<Option<i32>> = (0..size).map(|i| Some(i as i32)).collect();
    let values: Vec<Option<f64>> = (0..size).map(|i| Some(i as f64 * 1.5)).collect();

    let mut columns = indexmap::IndexMap::new();
    columns.insert("id".to_string(), Series::new_i32("id", ids));
    columns.insert("value".to_string(), Series::new_f64("value", values));

    DataFrame::new(columns)
}

fn join_benchmark(c: &mut Criterion) {
    // Create first dataframe (100K rows)
    let df1 = create_test_dataframe(100_000);

    // Create second dataframe with overlapping ids (50K rows)
    let ids: Vec<Option<i32>> = (50_000..150_000).map(Some).collect();
    let values2: Vec<Option<f64>> = (0..100_000).map(|i| Some(i as f64 * 2.5)).collect();

    let mut columns2 = indexmap::IndexMap::new();
    columns2.insert("id".to_string(), Series::new_i32("id", ids));
    columns2.insert("value2".to_string(), Series::new_f64("value2", values2));
    let df2 = DataFrame::new(columns2);

    c.bench_function("join_100k", |b| {
        b.iter(|| {
            let _joined = df1.join(&df2, "id", JoinType::Inner).unwrap();
        });
    });
}

criterion_group!(benches, join_benchmark);
criterion_main!(benches);
