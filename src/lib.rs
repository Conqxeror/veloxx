//! Velox is a lightweight Rust library for in-memory data processing and analytics.
//! It provides core data structures like DataFrame and Series, along with a suite
//! of operations for data manipulation, cleaning, aggregation, and basic statistics.
//!
//! The library prioritizes minimal dependencies, optimal memory footprint, and
//! compile-time guarantees, making it suitable for high-performance and
//! resource-constrained environments.

/// Defines conditions used for filtering DataFrames, supporting various comparison
/// and logical operations.
pub mod conditions;
/// Core DataFrame and its associated operations, including data ingestion, manipulation,
/// cleaning, joining, grouping, and display.
pub mod dataframe;
/// Defines expressions that can be used to create new columns or perform calculations
/// based on existing data within a DataFrame.
pub mod expressions;
/// Core Series (column) data structure and its associated operations, including
/// type casting, aggregation, and statistical calculations.
pub mod series;
/// Defines the fundamental data types (`DataType`) and value (`Value`) enums
/// used to represent data within Series and DataFrames.
pub mod types;

#[cfg(test)]
mod tests {
    use crate::conditions::Condition;
    use crate::dataframe::DataFrame;
    use crate::series::Series;
    use crate::types::Value;
    use std::collections::BTreeMap;

    #[test]
    fn test_dataframe_new() {
        let mut columns = std::collections::BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2)]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_f64("col2", vec![Some(1.0), Some(2.0)]),
        );

        let df = DataFrame::new(columns).unwrap();
        assert_eq!(df.row_count(), 2);
        assert_eq!(df.column_count(), 2);
        assert!(df.column_names().contains(&&"col1".to_string()));
        assert!(df.column_names().contains(&&"col2".to_string()));
    }

    #[test]
    fn test_dataframe_new_empty() {
        use std::collections::BTreeMap;
        let columns = BTreeMap::new();
        let df = DataFrame::new(columns).unwrap();
        assert_eq!(df.row_count(), 0);
        assert_eq!(df.column_count(), 0);
    }

    #[test]
    fn test_dataframe_new_mismatched_lengths() {
        use std::collections::BTreeMap;
        let mut columns = BTreeMap::new();
        columns.insert("col1".to_string(), Series::new_i32("col1", vec![Some(1)]));
        columns.insert(
            "col2".to_string(),
            Series::new_f64("col2", vec![Some(1.0), Some(2.0)]),
        );

        let err = DataFrame::new(columns).unwrap_err();
        assert_eq!(err, "All series in a DataFrame must have the same length.");
    }

    #[test]
    fn test_dataframe_get_column() {
        use std::collections::BTreeMap;
        let mut columns = BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2)]),
        );
        let df = DataFrame::new(columns).unwrap();

        let col1 = df.get_column("col1").unwrap();
        match col1 {
            Series::I32(_, v) => assert_eq!(*v, vec![Some(1), Some(2)]),
            _ => panic!("Unexpected series type"),
        }

        assert!(df.get_column("non_existent").is_none());
    }

    #[test]
    fn test_dataframe_display() {
        use std::collections::BTreeMap;
        let mut columns = BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), None, Some(3)]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_string(
                "col2",
                vec![Some("a".to_string()), Some("b".to_string()), None],
            ),
        );
        columns.insert(
            "col3".to_string(),
            Series::new_f64("col3", vec![Some(1.1), Some(2.2), Some(3.3)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let expected_output = "col1           col2           col3           \n--------------- --------------- --------------- \n1              a              1.10           \nnull           b              2.20           \n3              null           3.30           \n";
        assert_eq!(format!("{df}"), expected_output);
    }

    #[test]
    fn test_dataframe_from_vec_of_vec() {
        let data = vec![
            vec![
                "1".to_string(),
                "2.0".to_string(),
                "true".to_string(),
                "hello".to_string(),
            ],
            vec![
                "4".to_string(),
                "5.0".to_string(),
                "false".to_string(),
                "world".to_string(),
            ],
            vec![
                "7".to_string(),
                "8.0".to_string(),
                "".to_string(),
                "rust".to_string(),
            ],
            vec![
                "".to_string(),
                "".to_string(),
                "true".to_string(),
                "".to_string(),
            ],
        ];
        let column_names = vec![
            "col_i32".to_string(),
            "col_f64".to_string(),
            "col_bool".to_string(),
            "col_string".to_string(),
        ];

        let df = DataFrame::from_vec_of_vec(data, column_names).unwrap();

        assert_eq!(df.row_count(), 4);
        assert_eq!(df.column_count(), 4);

        let col_i32 = df.get_column("col_i32").unwrap();
        match col_i32 {
            Series::I32(_, v) => assert_eq!(*v, vec![Some(1), Some(4), Some(7), None]),
            _ => panic!("Expected I32 series"),
        }

        let col_f64 = df.get_column("col_f64").unwrap();
        match col_f64 {
            Series::F64(_, v) => assert_eq!(*v, vec![Some(2.0), Some(5.0), Some(8.0), None]),
            _ => panic!("Expected F64 series"),
        }

        let col_bool = df.get_column("col_bool").unwrap();
        match col_bool {
            Series::Bool(_, v) => assert_eq!(*v, vec![Some(true), Some(false), None, Some(true)]),
            _ => panic!("Expected Bool series"),
        }

        let col_string = df.get_column("col_string").unwrap();
        match col_string {
            Series::String(_, v) => assert_eq!(
                *v,
                vec![
                    Some("hello".to_string()),
                    Some("world".to_string()),
                    Some("rust".to_string()),
                    None
                ]
            ),
            _ => panic!("Expected String series"),
        }

        // Test with empty data
        let empty_data: Vec<Vec<String>> = vec![];
        let empty_column_names = vec!["col1".to_string()];
        let empty_df = DataFrame::from_vec_of_vec(empty_data, empty_column_names).unwrap();
        assert_eq!(empty_df.row_count(), 0);
        assert_eq!(empty_df.column_count(), 0);

        // Test with mismatched column count
        let mismatched_data = vec![vec!["1".to_string()]];
        let mismatched_column_names = vec!["col1".to_string(), "col2".to_string()];
        let err = DataFrame::from_vec_of_vec(mismatched_data, mismatched_column_names).unwrap_err();
        assert_eq!(
            err,
            "Number of columns in data does not match number of column names."
        );
    }

    #[test]
    fn test_dataframe_select_columns() {
        let mut columns = std::collections::BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2)]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_f64("col2", vec![Some(1.0), Some(2.0)]),
        );
        columns.insert(
            "col3".to_string(),
            Series::new_string("col3", vec![Some("a".to_string()), Some("b".to_string())]),
        );

        let df = DataFrame::new(columns).unwrap();

        // Select a subset of columns
        let selected_df = df
            .select_columns(vec!["col1".to_string(), "col3".to_string()])
            .unwrap();
        assert_eq!(selected_df.column_count(), 2);
        assert!(selected_df.column_names().contains(&&"col1".to_string()));
        assert!(selected_df.column_names().contains(&&"col3".to_string()));
        assert_eq!(selected_df.row_count(), 2);

        // Try to select a non-existent column
        let err = df
            .select_columns(vec!["col1".to_string(), "non_existent".to_string()])
            .unwrap_err();
        assert_eq!(err, "Column 'non_existent' not found.");

        // Select all columns
        let all_columns_df = df
            .select_columns(vec![
                "col1".to_string(),
                "col2".to_string(),
                "col3".to_string(),
            ])
            .unwrap();
        assert_eq!(all_columns_df.column_count(), 3);
    }

    #[test]
    fn test_dataframe_drop_columns() {
        let mut columns = std::collections::BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2)]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_f64("col2", vec![Some(1.0), Some(2.0)]),
        );
        columns.insert(
            "col3".to_string(),
            Series::new_string("col3", vec![Some("a".to_string()), Some("b".to_string())]),
        );

        let df = DataFrame::new(columns).unwrap();

        // Drop a subset of columns
        let dropped_df = df.drop_columns(vec!["col1".to_string()]).unwrap();
        assert_eq!(dropped_df.column_count(), 2);
        assert!(dropped_df.column_names().contains(&&"col2".to_string()));
        assert!(dropped_df.column_names().contains(&&"col3".to_string()));
        assert_eq!(dropped_df.row_count(), 2);

        // Try to drop a non-existent column
        let err = df
            .drop_columns(vec!["col1".to_string(), "non_existent".to_string()])
            .unwrap_err();
        assert_eq!(err, "Column 'non_existent' not found.");

        // Drop all columns
        let empty_df = df
            .drop_columns(vec![
                "col1".to_string(),
                "col2".to_string(),
                "col3".to_string(),
            ])
            .unwrap();
        assert_eq!(empty_df.column_count(), 0);
        assert_eq!(empty_df.row_count(), 0);
    }

    #[test]
    fn test_dataframe_rename_column() {
        let mut columns = std::collections::BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2)]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_f64("col2", vec![Some(1.0), Some(2.0)]),
        );
        let df = DataFrame::new(columns).unwrap();

        // Rename an existing column
        let renamed_df = df.rename_column("col1", "new_col1").unwrap();
        assert!(renamed_df.column_names().contains(&&"new_col1".to_string()));
        assert!(!renamed_df.column_names().contains(&&"col1".to_string()));
        assert_eq!(renamed_df.column_count(), 2);

        // Try to rename a non-existent column
        let err = df.rename_column("non_existent", "new_name").unwrap_err();
        assert_eq!(err, "Column 'non_existent' not found.");

        // Try to rename to an existing column name
        let err = df.rename_column("col1", "col2").unwrap_err();
        assert_eq!(err, "Column with new name 'col2' already exists.");
    }

    #[test]
    fn test_dataframe_filter() {
        let mut columns = std::collections::BTreeMap::new();
        columns.insert(
            "age".to_string(),
            Series::new_i32("age", vec![Some(10), Some(20), Some(30), Some(40)]),
        );
        columns.insert(
            "city".to_string(),
            Series::new_string(
                "city",
                vec![
                    Some("London".to_string()),
                    Some("Paris".to_string()),
                    Some("London".to_string()),
                    Some("New York".to_string()),
                ],
            ),
        );
        let df = DataFrame::new(columns).unwrap();

        // Filter by age > 20
        let condition = Condition::Gt("age".to_string(), Value::I32(20));
        let filtered_df = df.filter(&condition).unwrap();
        assert_eq!(filtered_df.row_count(), 2);
        assert_eq!(
            filtered_df.get_column("age").unwrap().get_value(0),
            Some(Value::I32(30))
        );
        assert_eq!(
            filtered_df.get_column("age").unwrap().get_value(1),
            Some(Value::I32(40))
        );

        // Filter by city == "London"
        let condition = Condition::Eq("city".to_string(), Value::String("London".to_string()));
        let filtered_df = df.filter(&condition).unwrap();
        assert_eq!(filtered_df.row_count(), 2);
        assert_eq!(
            filtered_df.get_column("city").unwrap().get_value(0),
            Some(Value::String("London".to_string()))
        );
        assert_eq!(
            filtered_df.get_column("city").unwrap().get_value(1),
            Some(Value::String("London".to_string()))
        );

        // Filter by age > 20 AND city == "London"
        let condition = Condition::And(
            Box::new(Condition::Gt("age".to_string(), Value::I32(20))),
            Box::new(Condition::Eq(
                "city".to_string(),
                Value::String("London".to_string()),
            )),
        );
        let filtered_df = df.filter(&condition).unwrap();
        assert_eq!(filtered_df.row_count(), 1);
        assert_eq!(
            filtered_df.get_column("age").unwrap().get_value(0),
            Some(Value::I32(30))
        );

        // Filter by age > 30 OR city == "Paris"
        let condition = Condition::Or(
            Box::new(Condition::Gt("age".to_string(), Value::I32(30))),
            Box::new(Condition::Eq(
                "city".to_string(),
                Value::String("Paris".to_string()),
            )),
        );
        let filtered_df = df.filter(&condition).unwrap();
        assert_eq!(filtered_df.row_count(), 2);
        assert_eq!(
            filtered_df.get_column("age").unwrap().get_value(0),
            Some(Value::I32(20))
        );
        assert_eq!(
            filtered_df.get_column("age").unwrap().get_value(1),
            Some(Value::I32(40))
        );

        // Filter by NOT (age > 20)
        let condition = Condition::Not(Box::new(Condition::Gt("age".to_string(), Value::I32(20))));
        let filtered_df = df.filter(&condition).unwrap();
        assert_eq!(filtered_df.row_count(), 2);
        assert_eq!(
            filtered_df.get_column("age").unwrap().get_value(0),
            Some(Value::I32(10))
        );
        assert_eq!(
            filtered_df.get_column("age").unwrap().get_value(1),
            Some(Value::I32(20))
        );

        // Test with non-existent column in condition
        let condition = Condition::Eq("non_existent".to_string(), Value::I32(10));
        let err = df.filter(&condition).unwrap_err();
        assert_eq!(err, "Column 'non_existent' not found.");
    }

    #[test]
    fn test_dataframe_drop_nulls() {
        let mut columns = std::collections::BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), None, Some(3)]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_string(
                "col2",
                vec![Some("a".to_string()), Some("b".to_string()), None],
            ),
        );
        columns.insert(
            "col3".to_string(),
            Series::new_f64("col3", vec![Some(1.1), Some(2.2), Some(3.3)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let dropped_df = df.drop_nulls().unwrap();

        assert_eq!(dropped_df.row_count(), 1);
        assert_eq!(
            dropped_df.get_column("col1").unwrap().get_value(0),
            Some(Value::I32(1))
        );
        assert_eq!(
            dropped_df.get_column("col2").unwrap().get_value(0),
            Some(Value::String("a".to_string()))
        );
        assert_eq!(
            dropped_df.get_column("col3").unwrap().get_value(0),
            Some(Value::F64(1.1))
        );

        let mut columns_no_nulls = std::collections::BTreeMap::new();
        columns_no_nulls.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2)]),
        );
        let df_no_nulls = DataFrame::new(columns_no_nulls).unwrap();
        let dropped_df_no_nulls = df_no_nulls.drop_nulls().unwrap();
        assert_eq!(dropped_df_no_nulls.row_count(), 2);
    }

    #[test]
    fn test_dataframe_fill_nulls() {
        let mut columns = std::collections::BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), None, Some(3)]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_string(
                "col2",
                vec![Some("a".to_string()), Some("b".to_string()), None],
            ),
        );
        columns.insert(
            "col3".to_string(),
            Series::new_f64("col3", vec![Some(1.1), Some(2.2), None]),
        );

        let df = DataFrame::new(columns).unwrap();

        // Fill i32 column
        let filled_df_i32 = df.fill_nulls(Value::I32(99)).unwrap();
        assert_eq!(
            filled_df_i32.get_column("col1").unwrap().get_value(1),
            Some(Value::I32(99))
        );

        // Fill string column
        let filled_df_string = df.fill_nulls(Value::String("missing".to_string())).unwrap();
        assert_eq!(
            filled_df_string.get_column("col2").unwrap().get_value(2),
            Some(Value::String("missing".to_string()))
        );

        // Fill f64 column
        let filled_df_f64 = df.fill_nulls(Value::F64(99.9)).unwrap();
        assert_eq!(
            filled_df_f64.get_column("col3").unwrap().get_value(2),
            Some(Value::F64(99.9))
        );

        // Test type mismatch for i32 column
        let filled_df_mismatch_i32 = df
            .fill_nulls(Value::String("wrong_type".to_string()))
            .unwrap();
        assert_eq!(
            filled_df_mismatch_i32
                .get_column("col1")
                .unwrap()
                .get_value(1),
            None
        ); // Should remain None

        // Test type mismatch for string column
        let filled_df_mismatch_string = df.fill_nulls(Value::I32(123)).unwrap();
        assert_eq!(
            filled_df_mismatch_string
                .get_column("col2")
                .unwrap()
                .get_value(2),
            None
        ); // Should remain None

        // Test type mismatch for f64 column
        let filled_df_mismatch_f64 = df.fill_nulls(Value::Bool(true)).unwrap();
        assert_eq!(
            filled_df_mismatch_f64
                .get_column("col3")
                .unwrap()
                .get_value(2),
            None
        ); // Should remain None
    }

    #[test]
    fn test_series_cast() {
        // Test i32 to f64
        let series_i32 = Series::new_i32("int_col", vec![Some(1), Some(2), None]);
        let casted_f64 = series_i32.cast(crate::types::DataType::F64).unwrap();
        match casted_f64 {
            Series::F64(name, data) => {
                assert_eq!(name, "int_col");
                assert_eq!(data, vec![Some(1.0), Some(2.0), None]);
            }
            _ => panic!("Expected F64 series"),
        }

        // Test f64 to i32
        let series_f64 = Series::new_f64("float_col", vec![Some(1.1), Some(2.9), None]);
        let casted_i32 = series_f64.cast(crate::types::DataType::I32).unwrap();
        match casted_i32 {
            Series::I32(name, data) => {
                assert_eq!(name, "float_col");
                assert_eq!(data, vec![Some(1), Some(2), None]);
            }
            _ => panic!("Expected I32 series"),
        }

        // Test string to i32
        let series_string_i32 = Series::new_string(
            "str_int_col",
            vec![Some("10".to_string()), Some("abc".to_string()), None],
        );
        let casted_i32_from_string = series_string_i32.cast(crate::types::DataType::I32).unwrap();
        match casted_i32_from_string {
            Series::I32(name, data) => {
                assert_eq!(name, "str_int_col");
                assert_eq!(data, vec![Some(10), None, None]);
            }
            _ => panic!("Expected I32 series"),
        }

        // Test string to f64
        let series_string_f64 = Series::new_string(
            "str_float_col",
            vec![Some("10.5".to_string()), Some("xyz".to_string()), None],
        );
        let casted_f64_from_string = series_string_f64.cast(crate::types::DataType::F64).unwrap();
        match casted_f64_from_string {
            Series::F64(name, data) => {
                assert_eq!(name, "str_float_col");
                assert_eq!(data, vec![Some(10.5), None, None]);
            }
            _ => panic!("Expected F64 series"),
        }

        // Test string to bool
        let series_string_bool = Series::new_string(
            "str_bool_col",
            vec![
                Some("true".to_string()),
                Some("false".to_string()),
                Some("invalid".to_string()),
                None,
            ],
        );
        let casted_bool_from_string = series_string_bool
            .cast(crate::types::DataType::Bool)
            .unwrap();
        match casted_bool_from_string {
            Series::Bool(name, data) => {
                assert_eq!(name, "str_bool_col");
                assert_eq!(data, vec![Some(true), Some(false), None, None]);
            }
            _ => panic!("Expected Bool series"),
        }

        // Test unsupported cast
        let series_i32_unsupported = Series::new_i32("int_col", vec![Some(1)]);
        let err = series_i32_unsupported
            .cast(crate::types::DataType::String)
            .unwrap_err();
        assert!(err.contains("Unsupported cast from I32 to String"));

        // Test casting to same type
        let series_i32_same_type = Series::new_i32("int_col", vec![Some(1), Some(2)]);
        let casted_i32_same_type = series_i32_same_type
            .cast(crate::types::DataType::I32)
            .unwrap();
        assert_eq!(series_i32_same_type, casted_i32_same_type);
    }

    #[test]
    fn test_dataframe_sort() {
        let mut columns = std::collections::BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(3), Some(1), Some(2)]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_string(
                "col2",
                vec![
                    Some("c".to_string()),
                    Some("a".to_string()),
                    Some("b".to_string()),
                ],
            ),
        );
        let df = DataFrame::new(columns).unwrap();

        // Sort by col1 ascending
        let sorted_df = df.sort(vec!["col1".to_string()], true).unwrap();
        assert_eq!(
            sorted_df.get_column("col1").unwrap().get_value(0),
            Some(Value::I32(1))
        );
        assert_eq!(
            sorted_df.get_column("col1").unwrap().get_value(1),
            Some(Value::I32(2))
        );
        assert_eq!(
            sorted_df.get_column("col1").unwrap().get_value(2),
            Some(Value::I32(3))
        );

        // Sort by col1 descending
        let sorted_df_desc = df.sort(vec!["col1".to_string()], false).unwrap();
        assert_eq!(
            sorted_df_desc.get_column("col1").unwrap().get_value(0),
            Some(Value::I32(3))
        );
        assert_eq!(
            sorted_df_desc.get_column("col1").unwrap().get_value(1),
            Some(Value::I32(2))
        );
        assert_eq!(
            sorted_df_desc.get_column("col1").unwrap().get_value(2),
            Some(Value::I32(1))
        );

        // Sort by col2 ascending
        let sorted_df_str = df.sort(vec!["col2".to_string()], true).unwrap();
        assert_eq!(
            sorted_df_str.get_column("col2").unwrap().get_value(0),
            Some(Value::String("a".to_string()))
        );
        assert_eq!(
            sorted_df_str.get_column("col2").unwrap().get_value(1),
            Some(Value::String("b".to_string()))
        );
        assert_eq!(
            sorted_df_str.get_column("col2").unwrap().get_value(2),
            Some(Value::String("c".to_string()))
        );

        // Test with non-existent column
        let err = df.sort(vec!["non_existent".to_string()], true).unwrap_err();
        assert_eq!(err, "Column 'non_existent' not found for sorting.");

        // Test with empty DataFrame
        let empty_df = DataFrame::new(BTreeMap::new()).unwrap();
        let sorted_empty_df = empty_df.sort(vec!["col1".to_string()], true).unwrap();
        assert_eq!(sorted_empty_df.row_count(), 0);
    }
}
