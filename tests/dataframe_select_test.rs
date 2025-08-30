use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
// use veloxx::expressions::Expr;
use std::collections::HashMap;

#[test]
fn test_select() {
    let mut columns = HashMap::new();
    columns.insert(
        "a".to_string(),
        Series::new_i32("a", vec![Some(1), Some(2), Some(3)]),
    );
    columns.insert(
        "b".to_string(),
        Series::new_f64("b", vec![Some(1.0), Some(2.0), Some(3.0)]),
    );
    let df = DataFrame::new(columns).unwrap();

    let selected_df = df
        .select_columns(vec!["a".to_string(), "b".to_string()])
        .unwrap();
    assert_eq!(selected_df.column_count(), 2);
    let column_names = selected_df.column_names();
    assert!(column_names.contains(&&"a".to_string()));
    assert!(column_names.contains(&&"b".to_string()));

    // let selected_df_with_lit = df.select_columns(vec!["a".to_string(), "c".to_string()]).unwrap();
    // assert_eq!(selected_df_with_lit.column_count(), 2);
    // assert_eq!(selected_df_with_lit.column_names(), vec!["a", "c"]);
    // let c_series = selected_df_with_lit.get_column("c").unwrap();
    // assert_eq!(c_series.len(), 3);
    // assert_eq!(c_series.get_value(0), Some(veloxx::types::Value::I32(10)));
}
