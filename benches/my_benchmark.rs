use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::BTreeMap;
use veloxx::conditions::Condition;
use veloxx::dataframe::join::JoinType;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::types::{DataType, Value};

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

fn bench_series_sum(c: &mut Criterion) {
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
}

fn bench_series_min(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(Some).collect());
    let series_f64 = Series::new_f64("data", (0..1000).map(|i| Some(i as f64)).collect());

    c.bench_function("series_min_i32", |b| {
        b.iter(|| {
            series_i32.min().unwrap();
        });
    });

    c.bench_function("series_min_f64", |b| {
        b.iter(|| {
            series_f64.min().unwrap();
        });
    });
}

fn bench_series_max(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(Some).collect());
    let series_f64 = Series::new_f64("data", (0..1000).map(|i| Some(i as f64)).collect());

    c.bench_function("series_max_i32", |b| {
        b.iter(|| {
            series_i32.max().unwrap();
        });
    });

    c.bench_function("series_max_f64", |b| {
        b.iter(|| {
            series_f64.max().unwrap();
        });
    });
}

fn bench_series_mean(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(Some).collect());
    let series_f64 = Series::new_f64("data", (0..1000).map(|i| Some(i as f64)).collect());

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

fn bench_series_median(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(Some).collect());
    let series_f64 = Series::new_f64("data", (0..1000).map(|i| Some(i as f64)).collect());

    c.bench_function("series_median_i32", |b| {
        b.iter(|| {
            series_i32.median().unwrap();
        });
    });

    c.bench_function("series_median_f64", |b| {
        b.iter(|| {
            series_f64.median().unwrap();
        });
    });
}

fn bench_series_std_dev(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(Some).collect());
    let series_f64 = Series::new_f64("data", (0..1000).map(|i| Some(i as f64)).collect());

    c.bench_function("series_std_dev_i32", |b| {
        b.iter(|| {
            series_i32.std_dev().unwrap();
        });
    });

    c.bench_function("series_std_dev_f64", |b| {
        b.iter(|| {
            series_f64.std_dev().unwrap();
        });
    });
}

fn bench_series_to_vec_f64(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(Some).collect());
    let series_f64 = Series::new_f64("data", (0..1000).map(|i| Some(i as f64)).collect());

    c.bench_function("series_to_vec_f64_i32", |b| {
        b.iter(|| {
            series_i32.to_vec_f64().unwrap();
        });
    });

    c.bench_function("series_to_vec_f64_f64", |b| {
        b.iter(|| {
            series_f64.to_vec_f64().unwrap();
        });
    });
}

fn bench_dataframe_sort(c: &mut Criterion) {
    let mut columns = BTreeMap::new();
    columns.insert(
        "col1".to_string(),
        Series::new_i32("col1", (0..1000).rev().map(Some).collect()),
    );
    columns.insert(
        "col2".to_string(),
        Series::new_f64("col2", (0..1000).map(|i| Some(i as f64)).collect()),
    );
    let df = DataFrame::new(columns).unwrap();

    c.bench_function("dataframe_sort", |b| {
        b.iter(|| {
            df.sort(vec!["col1".to_string()], true).unwrap();
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

fn bench_series_apply(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(Some).collect());
    let series_f64 = Series::new_f64("data", (0..1000).map(|i| Some(i as f64)).collect());
    let series_string = Series::new_string(
        "data",
        (0..1000).map(|i| Some(format!("value{}", i))).collect(),
    );

    c.bench_function("series_apply_i32", |b| {
        b.iter(|| {
            series_i32.apply_i32(|v| v.map(|x| x * 2)).unwrap();
        });
    });

    c.bench_function("series_apply_f64", |b| {
        b.iter(|| {
            series_f64.apply_f64(|v| v.map(|x| x * 2.0)).unwrap();
        });
    });

    c.bench_function("series_apply_string", |b| {
        b.iter(|| {
            series_string
                .apply_string(|v| v.map(|s| format!("{}-suffix", s)))
                .unwrap();
        });
    });
}

fn bench_dataframe_from_vec_of_vec(c: &mut Criterion) {
    let data: Vec<Vec<String>> = (0..1000)
        .map(|i| {
            vec![
                i.to_string(),
                format!("name{}", i),
                (i % 2 == 0).to_string(),
            ]
        })
        .collect();
    let column_names = vec!["id".to_string(), "name".to_string(), "is_even".to_string()];

    c.bench_function("dataframe_from_vec_of_vec", |b| {
        b.iter(|| {
            DataFrame::from_vec_of_vec(data.clone(), column_names.clone()).unwrap();
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
            df.drop_nulls().unwrap();
        });
    });
}

fn bench_dataframe_fill_nulls(c: &mut Criterion) {
    let data_with_nulls_i32: Vec<Option<i32>> = (0..1000)
        .map(|i| if i % 10 == 0 { None } else { Some(i) })
        .collect();
    let data_with_nulls_f64: Vec<Option<f64>> = (0..1000)
        .map(|i| if i % 10 == 0 { None } else { Some(i as f64) })
        .collect();

    let mut columns_i32 = BTreeMap::new();
    columns_i32.insert(
        "col1".to_string(),
        Series::new_i32("col1", data_with_nulls_i32.clone()),
    );
    let df_i32 = DataFrame::new(columns_i32).unwrap();
    let fill_value_i32 = Value::I32(999);

    let mut columns_f64 = BTreeMap::new();
    columns_f64.insert(
        "col1".to_string(),
        Series::new_f64("col1", data_with_nulls_f64.clone()),
    );
    let df_f64 = DataFrame::new(columns_f64).unwrap();
    let fill_value_f64 = Value::F64(999.99);

    c.bench_function("dataframe_fill_nulls_i32", |b| {
        b.iter(|| {
            df_i32.fill_nulls(fill_value_i32.clone()).unwrap();
        });
    });

    c.bench_function("dataframe_fill_nulls_f64", |b| {
        b.iter(|| {
            df_f64.fill_nulls(fill_value_f64.clone()).unwrap();
        });
    });
}

fn bench_series_cast(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(Some).collect());
    let series_f64 = Series::new_f64("data", (0..1000).map(|i| Some(i as f64)).collect());
    let series_string_i32 =
        Series::new_string("data", (0..1000).map(|i| Some(i.to_string())).collect());
    let series_string_f64 = Series::new_string(
        "data",
        (0..1000).map(|i| Some(format!("{}.0", i))).collect(),
    );

    c.bench_function("series_cast_i32_to_f64", |b| {
        b.iter(|| {
            series_i32.cast(DataType::F64).unwrap();
        });
    });

    c.bench_function("series_cast_f64_to_i32", |b| {
        b.iter(|| {
            series_f64.cast(DataType::I32).unwrap();
        });
    });

    c.bench_function("series_cast_string_to_i32", |b| {
        b.iter(|| {
            series_string_i32.cast(DataType::I32).unwrap();
        });
    });

    c.bench_function("series_cast_string_to_f64", |b| {
        b.iter(|| {
            series_string_f64.cast(DataType::F64).unwrap();
        });
    });
}

fn bench_dataframe_with_column(c: &mut Criterion) {
    let mut columns = BTreeMap::new();
    columns.insert(
        "col1".to_string(),
        Series::new_i32("col1", (0..1000).map(Some).collect()),
    );
    columns.insert(
        "col2".to_string(),
        Series::new_i32("col2", (0..1000).map(Some).collect()),
    );
    let df = DataFrame::new(columns).unwrap();
    let expr = veloxx::expressions::Expr::Add(
        Box::new(veloxx::expressions::Expr::Column("col1".to_string())),
        Box::new(veloxx::expressions::Expr::Column("col2".to_string())),
    );

    c.bench_function("dataframe_with_column", |b| {
        b.iter(|| {
            df.with_column("new_col", &expr).unwrap();
        });
    });
}

fn bench_dataframe_append(c: &mut Criterion) {
    let mut columns1 = BTreeMap::new();
    columns1.insert(
        "col1".to_string(),
        Series::new_i32("col1", (0..500).map(Some).collect()),
    );
    columns1.insert(
        "col2".to_string(),
        Series::new_f64("col2", (0..500).map(|i| Some(i as f64)).collect()),
    );
    let df1 = DataFrame::new(columns1).unwrap();

    let mut columns2 = BTreeMap::new();
    columns2.insert(
        "col1".to_string(),
        Series::new_i32("col1", (500..1000).map(Some).collect()),
    );
    columns2.insert(
        "col2".to_string(),
        Series::new_f64("col2", (500..1000).map(|i| Some(i as f64)).collect()),
    );
    let df2 = DataFrame::new(columns2).unwrap();

    c.bench_function("dataframe_append", |b| {
        b.iter(|| {
            df1.append(&df2).unwrap();
        });
    });
}

fn bench_dataframe_describe(c: &mut Criterion) {
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

    c.bench_function("dataframe_describe", |b| {
        b.iter(|| {
            df.describe().unwrap();
        });
    });
}

fn bench_dataframe_correlation(c: &mut Criterion) {
    let mut columns = BTreeMap::new();
    columns.insert(
        "col1".to_string(),
        Series::new_i32("col1", (0..1000).map(Some).collect()),
    );
    columns.insert(
        "col2".to_string(),
        Series::new_i32("col2", (0..1000).map(|i| Some(i * 2)).collect()),
    );
    let df = DataFrame::new(columns).unwrap();

    c.bench_function("dataframe_correlation", |b| {
        b.iter(|| {
            df.correlation("col1", "col2").unwrap();
        });
    });
}

fn bench_dataframe_covariance(c: &mut Criterion) {
    let mut columns = BTreeMap::new();
    columns.insert(
        "col1".to_string(),
        Series::new_i32("col1", (0..1000).map(Some).collect()),
    );
    columns.insert(
        "col2".to_string(),
        Series::new_i32("col2", (0..1000).map(|i| Some(i * 2)).collect()),
    );
    let df = DataFrame::new(columns).unwrap();

    c.bench_function("dataframe_covariance", |b| {
        b.iter(|| {
            df.covariance("col1", "col2").unwrap();
        });
    });
}

fn bench_series_unique(c: &mut Criterion) {
    let series_i32 = Series::new_i32("data", (0..1000).map(|i| Some(i % 100)).collect());

    c.bench_function("series_unique", |b| {
        b.iter(|| {
            series_i32.unique().unwrap();
        });
    });
}

fn bench_series_interpolate_nulls(c: &mut Criterion) {
    let series_i32 = Series::new_i32(
        "data",
        (0..1000)
            .map(|i| if i % 5 == 0 { None } else { Some(i) })
            .collect(),
    );

    c.bench_function("series_interpolate_nulls_i32", |b| {
        b.iter(|| {
            series_i32.interpolate_nulls().unwrap();
        });
    });

    let series_f64 = Series::new_f64(
        "data",
        (0..1000)
            .map(|i| if i % 5 == 0 { None } else { Some(i as f64) })
            .collect(),
    );

    c.bench_function("series_interpolate_nulls_f64", |b| {
        b.iter(|| {
            series_f64.interpolate_nulls().unwrap();
        });
    });
}

fn bench_dataframe_from_json(c: &mut Criterion) {
    c.bench_function("dataframe_from_json", |b| {
        b.iter(|| {
            DataFrame::from_json("examples/test.json").unwrap();
        });
    });
}

fn bench_dataframe_to_csv(c: &mut Criterion) {
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

    c.bench_function("dataframe_to_csv", |b| {
        b.iter(|| {
            df.to_csv("target/test.csv").unwrap();
        });
    });
}

fn bench_dataframe_to_vec_of_vec(c: &mut Criterion) {
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

    c.bench_function("dataframe_to_vec_of_vec", |b| {
        b.iter(|| {
            df.to_vec_of_vec();
        });
    });
}

fn bench_dataframe_group_by_agg(c: &mut Criterion) {
    let mut columns = BTreeMap::new();
    columns.insert(
        "group".to_string(),
        Series::new_string(
            "group",
            (0..1000)
                .map(|i| Some(format!("group_{}", i % 10)))
                .collect(),
        ),
    );
    columns.insert(
        "value".to_string(),
        Series::new_i32("value", (0..1000).map(Some).collect()),
    );
    let df = DataFrame::new(columns).unwrap();

    c.bench_function("dataframe_group_by_agg", |b| {
        b.iter(|| {
            df.group_by(vec!["group".to_string()])
                .unwrap()
                .agg(vec![("value", "sum")])
                .unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_dataframe_creation,
    bench_dataframe_from_csv,
    bench_dataframe_filter,
    bench_series_sum,
    bench_series_min,
    bench_series_max,
    bench_series_mean,
    bench_series_median,
    bench_series_std_dev,
    bench_series_to_vec_f64,
    bench_dataframe_sort,
    bench_dataframe_join,
    bench_series_apply,
    bench_dataframe_from_vec_of_vec,
    bench_dataframe_drop_nulls,
    bench_dataframe_fill_nulls,
    bench_series_cast,
    bench_dataframe_with_column,
    bench_dataframe_append,
    bench_dataframe_describe,
    bench_dataframe_correlation,
    bench_dataframe_covariance,
    bench_series_unique,
    bench_series_interpolate_nulls,
    bench_dataframe_from_json,
    bench_dataframe_to_csv,
    bench_dataframe_to_vec_of_vec,
    bench_dataframe_group_by_agg
);
criterion_main!(benches);
