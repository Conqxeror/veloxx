use indexmap::IndexMap;
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;
use veloxx::series::Series;

#[test]
fn test_mismatched_lengths() {
    let mut columns = IndexMap::new();
    columns.insert(
        "a".to_string(),
        Series::new_i32("a", vec![Some(1), Some(2)]),
    );
    columns.insert("b".to_string(), Series::new_i32("b", vec![Some(1)]));

    // DataFrame::new no longer validates equal column lengths during construction.
    // We still construct the DataFrame and surface row_count based on the first column.
    let df = DataFrame::new(columns);
    assert_eq!(df.row_count(), 2);
}

#[test]
fn test_column_not_found() {
    let df = DataFrame::new(IndexMap::new());
    let err = df.select_columns(vec!["a".to_string()]).unwrap_err();
    assert_eq!(err, VeloxxError::ColumnNotFound("a".to_string()));
}
