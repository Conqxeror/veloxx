use indexmap::IndexMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::visualization::{ChartType, Plot};

#[test]
fn test_save_histogram() {
    let mut columns = IndexMap::new();
    columns.insert(
        "a".to_string(),
        Series::new_i32("a", vec![Some(1), Some(2), Some(2), Some(3)]),
    );
    let df = DataFrame::new(columns);
    let plot = Plot::new(&df, ChartType::Histogram).with_columns("a", "");
    assert!(plot.save("test_histogram.svg").is_ok());
}

#[test]
fn test_save_scatter_plot() {
    let mut columns = IndexMap::new();
    columns.insert(
        "a".to_string(),
        Series::new_i32("a", vec![Some(1), Some(2), Some(3), Some(4)]),
    );
    columns.insert(
        "b".to_string(),
        Series::new_i32("b", vec![Some(1), Some(2), Some(3), Some(4)]),
    );
    let df = DataFrame::new(columns);
    let plot = Plot::new(&df, ChartType::Scatter).with_columns("a", "b");
    assert!(plot.save("test_scatter.svg").is_ok());
}
