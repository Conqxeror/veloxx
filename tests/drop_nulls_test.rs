use std::collections::BTreeMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

#[test]
fn test_drop_nulls_with_mixed_data() {
    let series1 = Series::new_i32("col1", vec![Some(1), Some(2), None, Some(4)]);
    let series2 = Series::new_string(
        "col2",
        vec![
            Some("a".to_string()),
            None,
            Some("c".to_string()),
            Some("d".to_string()),
        ],
    );
    let mut columns = BTreeMap::new();
    columns.insert("col1".to_string(), series1);
    columns.insert("col2".to_string(), series2);
    let df = DataFrame::new(columns).unwrap();

    let cleaned_df = df.drop_nulls(None).unwrap();

    assert_eq!(cleaned_df.row_count(), 2);
    assert_eq!(cleaned_df.column_count(), 2);

    let col1_values: Vec<Option<String>> = match cleaned_df.get_column("col1").unwrap() {
        Series::I32(_, data) => data.iter().map(|v| v.map(|x| x.to_string())).collect(),
        _ => panic!("Wrong type"),
    };
    assert_eq!(
        col1_values,
        vec![Some("1".to_string()), Some("4".to_string())]
    );

    let col2_values: Vec<Option<String>> = match cleaned_df.get_column("col2").unwrap() {
        Series::String(_, data) => data.iter().map(|v| v.clone()).collect(),
        _ => panic!("Wrong type"),
    };
    assert_eq!(
        col2_values,
        vec![Some("a".to_string()), Some("d".to_string())]
    );
}
