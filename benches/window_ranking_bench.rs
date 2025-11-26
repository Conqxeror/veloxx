#[macro_use]
extern crate criterion;
use criterion::Criterion;

use indexmap::IndexMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::window_functions::{RankingFunction, WindowFunction, WindowSpec};

fn bench_window_ranking(c: &mut Criterion) {
    let mut columns = IndexMap::new();
    let data: Vec<f64> = (0..100_000).map(|x| (x as f64) * 0.5).collect();
    let bitmap: Vec<bool> = vec![true; data.len()];
    columns.insert(
        "values".to_string(),
        Series::F64("values".to_string(), data, bitmap),
    );
    let df = DataFrame::new(columns);
    let window_spec = WindowSpec::new().order_by(vec!["values".to_string()]);
    c.bench_function("window_ranking_parallel", |b| {
        b.iter(|| {
            let _ = WindowFunction::apply_ranking(&df, &RankingFunction::RowNumber, &window_spec);
        })
    });
}

criterion_group!(benches, bench_window_ranking);
criterion_main!(benches);
