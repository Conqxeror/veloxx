use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::window_functions::{WindowFunction, WindowSpec, RankingFunction};
use std::collections::BTreeMap;

#[test]
fn test_rank() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "sales".to_string(),
        Series::new_f64("sales", vec![Some(100.0), Some(200.0), Some(150.0), Some(300.0)]),
    );
    let df = DataFrame::new(columns).unwrap();
    let window_spec = WindowSpec::new().order_by(vec!["sales".to_string()]);
    let result = WindowFunction::apply_ranking(&df, &RankingFunction::Rank, &window_spec).unwrap();
    let rank_series = result.get_column("rank_rank").unwrap();
    assert_eq!(rank_series.get_value(0), Some(veloxx::types::Value::I32(1)));
    assert_eq!(rank_series.get_value(1), Some(veloxx::types::Value::I32(3)));
    assert_eq!(rank_series.get_value(2), Some(veloxx::types::Value::I32(2)));
    assert_eq!(rank_series.get_value(3), Some(veloxx::types::Value::I32(4)));
}

#[test]
fn test_dense_rank() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "sales".to_string(),
        Series::new_f64("sales", vec![Some(100.0), Some(200.0), Some(150.0), Some(300.0)]),
    );
    let df = DataFrame::new(columns).unwrap();
    let window_spec = WindowSpec::new().order_by(vec!["sales".to_string()]);
    let result = WindowFunction::apply_ranking(&df, &RankingFunction::DenseRank, &window_spec).unwrap();
    let rank_series = result.get_column("dense_rank_rank").unwrap();
    assert_eq!(rank_series.get_value(0), Some(veloxx::types::Value::I32(1)));
    assert_eq!(rank_series.get_value(1), Some(veloxx::types::Value::I32(3)));
    assert_eq!(rank_series.get_value(2), Some(veloxx::types::Value::I32(2)));
    assert_eq!(rank_series.get_value(3), Some(veloxx::types::Value::I32(4)));
}

#[test]
fn test_row_number() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "sales".to_string(),
        Series::new_f64("sales", vec![Some(100.0), Some(200.0), Some(150.0), Some(300.0)]),
    );
    let df = DataFrame::new(columns).unwrap();
    let window_spec = WindowSpec::new().order_by(vec!["sales".to_string()]);
    let result = WindowFunction::apply_ranking(&df, &RankingFunction::RowNumber, &window_spec).unwrap();
    let rank_series = result.get_column("row_number_rank").unwrap();
    assert_eq!(rank_series.get_value(0), Some(veloxx::types::Value::I32(1)));
    assert_eq!(rank_series.get_value(1), Some(veloxx::types::Value::I32(3)));
    assert_eq!(rank_series.get_value(2), Some(veloxx::types::Value::I32(2)));
    assert_eq!(rank_series.get_value(3), Some(veloxx::types::Value::I32(4)));
}

#[test]
fn test_lag() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "sales".to_string(),
        Series::new_f64("sales", vec![Some(100.0), Some(200.0), Some(150.0), Some(300.0)]),
    );
    let df = DataFrame::new(columns).unwrap();
    let window_spec = WindowSpec::new();
    let result = WindowFunction::apply_lag_lead(&df, "sales", 2, &window_spec).unwrap();
    let lag_series = result.get_column("lag_sales_2").unwrap();
    assert_eq!(lag_series.get_value(0), None);
    assert_eq!(lag_series.get_value(1), None);
    assert_eq!(lag_series.get_value(2), Some(veloxx::types::Value::F64(100.0)));
    assert_eq!(lag_series.get_value(3), Some(veloxx::types::Value::F64(200.0)));
}

#[test]
fn test_lead() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "sales".to_string(),
        Series::new_f64("sales", vec![Some(100.0), Some(200.0), Some(150.0), Some(300.0)]),
    );
    let df = DataFrame::new(columns).unwrap();
    let window_spec = WindowSpec::new();
    let result = WindowFunction::apply_lag_lead(&df, "sales", -2, &window_spec).unwrap();
    let lead_series = result.get_column("lead_sales_2").unwrap();
    assert_eq!(lead_series.get_value(0), Some(veloxx::types::Value::F64(150.0)));
    assert_eq!(lead_series.get_value(1), Some(veloxx::types::Value::F64(300.0)));
    assert_eq!(lead_series.get_value(2), None);
    assert_eq!(lead_series.get_value(3), None);
}
