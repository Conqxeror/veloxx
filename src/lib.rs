#![allow(clippy::uninlined_format_args)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/Conqxeror/veloxx/main/docs/veloxx_logo.png"
)]
//! Veloxx is a lightweight Rust library for in-memory data processing and analytics.
//! It provides core data structures like `DataFrame` and `Series`, along with a suite
//! of operations for data manipulation, cleaning, aggregation, and basic statistics.
//!
//! The library prioritizes minimal dependencies, optimal memory footprint, and
//! compile-time guarantees, making it suitable for high-performance and
//! resource-constrained environments.
//!
//! # Getting Started
//!
//! Add `veloxx` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! veloxx = "0.2"
//! ```
//!
//! # Examples
//!
//! ## Creating a DataFrame
//!
//! ```rust
//! use veloxx::dataframe::DataFrame;
//! use veloxx::series::Series;
//!
//!
//! let mut columns = BTreeMap::new();
//! columns.insert(
//!     "name".to_string(),
//!     Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string())]),
//! );
//! columns.insert(
//!     "age".to_string(),
//!     Series::new_i32("age", vec![Some(30), Some(24)]),
//! );
//!
//! let df = DataFrame::new(columns).unwrap();
//! println!("Initial DataFrame:\n{}", df);
//! ```
//!
//! ## Filtering a DataFrame
//!
//! ```rust
//! use veloxx::dataframe::DataFrame;
//! use veloxx::series::Series;
//! use veloxx::conditions::Condition;
//! use veloxx::types::Value;
//!
//!
//! let mut columns = BTreeMap::new();
//! columns.insert(
//!     "name".to_string(),
//!     Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]),
//! );
//! columns.insert(
//!     "age".to_string(),
//!     Series::new_i32("age", vec![Some(30), Some(24), Some(35)]),
//! );
//!
//! let df = DataFrame::new(columns).unwrap();
//!
//! let condition = Condition::Gt("age".to_string(), Value::I32(30));
//! let filtered_df = df.filter(&condition).unwrap();
//! println!("Filtered DataFrame (age > 30):\n{}", filtered_df);
//! ```
//!
//! ## Performing Aggregations
//!
//! ```rust
//! use veloxx::dataframe::DataFrame;
//! use veloxx::series::Series;
//!
//!
//! let mut columns = BTreeMap::new();
//! columns.insert(
//!     "city".to_string(),
//!     Series::new_string("city", vec![Some("New York".to_string()), Some("London".to_string()), Some("New York".to_string())]),
//! );
//! columns.insert(
//!     "sales".to_string(),
//!     Series::new_f64("sales", vec![Some(100.0), Some(150.0), Some(200.0)]),
//! );
//!
//! let df = DataFrame::new(columns).unwrap();
//!
//! let grouped_df = df.group_by(vec!["city".to_string()]).unwrap();
//! let aggregated_df = grouped_df.agg(vec![("sales", "sum")]).unwrap();
//! println!("Aggregated Sales by City:\n{}", aggregated_df);
//! ```
//!
//! ![Veloxx Logo](https://raw.githubusercontent.com/Conqxeror/veloxx/main/docs/veloxx_logo.png)

/// Advanced I/O operations module
#[cfg(feature = "advanced_io")]
pub mod advanced_io;
/// Audit trail generation for data quality checks
pub mod audit;
/// Defines conditions used for filtering DataFrames, supporting various comparison
/// and logical operations.
pub mod conditions;
/// Data quality and validation module
#[cfg(feature = "data_quality")]
pub mod data_quality;
/// Core DataFrame and its associated operations, including data ingestion, manipulation,
/// cleaning, joining, grouping, and display.
pub mod dataframe;
/// Distributed computing support module
#[cfg(feature = "distributed")]
pub mod distributed;
/// Defines the custom error type `VeloxxError` for unified error handling.
pub mod error;
/// Defines expressions that can be used to create new columns or perform calculations
/// based on existing data within a DataFrame.
pub mod expressions;
/// I/O operations for reading and writing data
pub mod io;
/// Machine learning integration module
#[cfg(feature = "ml")]
pub mod ml;
/// Performance optimization module for high-performance data operations
/// Core Series (column) data structure and its associated operations, including
/// type casting, aggregation, and statistical calculations.
pub mod series;
/// Defines the fundamental data types (`DataType`) and value (`Value`) enums
/// used to represent data within Series and DataFrames.
pub mod types;
/// Data visualization and plotting module
#[cfg(feature = "visualization")]
pub mod visualization;
/// Window functions and advanced analytics module
#[cfg(feature = "window_functions")]
pub mod window_functions;

#[cfg(feature = "python")]
#[path = "../bindings/python/mod.rs"]
mod python_bindings;

#[cfg(feature = "wasm")]
pub mod wasm_bindings;
#[cfg(feature = "wasm")]
pub use wasm_bindings::*;

#[cfg(test)]
mod tests {
    use crate::conditions::Condition;
    use crate::dataframe::DataFrame;
    use crate::error::VeloxxError;
    use std::collections::BTreeMap;

    use crate::series::Series;
    use crate::types::Value;

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
        let columns = BTreeMap::new();
        let df = DataFrame::new(columns).unwrap();
        assert_eq!(df.row_count(), 0);
        assert_eq!(df.column_count(), 0);
    }

    #[test]
    fn test_dataframe_new_mismatched_lengths() {
        let mut columns = BTreeMap::new();
        columns.insert("col1".to_string(), Series::new_i32("col1", vec![Some(1)]));
        columns.insert(
            "col2".to_string(),
            Series::new_f64("col2", vec![Some(1.0), Some(2.0)]),
        );

        let err = DataFrame::new(columns).unwrap_err();
        assert_eq!(
            err,
            VeloxxError::InvalidOperation(
                "All series in a DataFrame must have the same length.".to_string()
            )
        );
    }

    #[test]
    fn test_dataframe_get_column() {
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
            VeloxxError::InvalidOperation(
                "Number of columns in data does not match number of column names.".to_string()
            )
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
        assert_eq!(err, VeloxxError::ColumnNotFound("non_existent".to_string()));

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
        assert_eq!(err, VeloxxError::ColumnNotFound("non_existent".to_string()));

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
        assert_eq!(err, VeloxxError::ColumnNotFound("non_existent".to_string()));

        // Try to rename to an existing column name
        let err = df.rename_column("col1", "col2").unwrap_err();
        assert_eq!(
            err,
            VeloxxError::InvalidOperation(
                "Column with new name 'col2' already exists.".to_string()
            )
        );
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
    }
}
