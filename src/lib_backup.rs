#![allow(clippy::uninlined_format_args)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/Conqxeror/veloxx/main/docs/veloxx_logo.png"
)]
//! Veloxx is a lightweight Rust library for in-memory data processing and analytics.
//! It provides core data structures like `DataFrame` and `Serie// Re-export main types for easy access
pub use crate::dataframe::DataFrame;
pub use crate::series::Series;
pub use crate::types::{DataType, Value};

// WASM exports
#[cfg(feature = "wasm")]
pub use wasm::{WasmDataFrame, WasmSeries};

//! # Veloxx
//!
//! A high-performance, lightweight dataframe library for Rust, focusing on efficient
//! data manipulation with minimal overhead. Veloxx provides a DataFrame structure
//! that works with columnar data storage and provides a performant API along with a suite
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
//! use indexmap::IndexMap;
//!
//!
//! let mut columns = IndexMap::new();
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
//! use indexmap::IndexMap;
//!
//!
//! let mut columns = IndexMap::new();
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
//! use indexmap::IndexMap;
//!
//!
//! let mut columns = IndexMap::new();
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
#[cfg(all(feature = "data_quality", not(target_arch = "wasm32")))]
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
#[cfg(not(target_arch = "wasm32"))]
pub mod io;
/// Lazy evaluation module for query optimization
pub mod lazy;
/// Machine learning integration module
#[cfg(feature = "ml")]
pub mod ml;
/// Performance optimization module for high-performance data operations
#[cfg(not(target_arch = "wasm32"))]
pub mod performance;
/// Minimal performance module for WASM (without problematic dependencies)
#[cfg(target_arch = "wasm32")]
pub mod performance {
    use crate::VeloxxError;
    
    pub mod optimized_simd {
        use super::VeloxxError;
        
        pub struct OptimizedSimdOps;
        impl OptimizedSimdOps {
            pub fn new() -> Self { Self }
            pub fn simd_add_f64(&self, _a: &[f64], _b: &[f64]) -> Vec<f64> {
                vec![] // Stub implementation
            }
            pub fn simd_multiply_f64(&self, _a: &[f64], _b: &[f64]) -> Vec<f64> {
                vec![] // Stub implementation  
            }
        }
        
        // Add trait extensions for optimized SIMD operations in WASM
        pub trait OptimizedSimdExtension<T> {
            fn optimized_simd_add(&self, other: &[T], result: &mut Vec<T>);
            fn optimized_simd_mul(&self, other: &[T], result: &mut Vec<T>);
        }
        
        impl OptimizedSimdExtension<f64> for Vec<f64> {
            fn optimized_simd_add(&self, other: &[f64], result: &mut Vec<f64>) {
                result.clear();
                for (a, b) in self.iter().zip(other.iter()) {
                    result.push(a + b);
                }
            }
            
            fn optimized_simd_mul(&self, other: &[f64], result: &mut Vec<f64>) {
                result.clear();
                for (a, b) in self.iter().zip(other.iter()) {
                    result.push(a * b);
                }
            }
        }
        
        impl OptimizedSimdExtension<i32> for Vec<i32> {
            fn optimized_simd_add(&self, other: &[i32], result: &mut Vec<i32>) {
                result.clear();
                for (a, b) in self.iter().zip(other.iter()) {
                    result.push(a + b);
                }
            }
            
            fn optimized_simd_mul(&self, other: &[i32], result: &mut Vec<i32>) {
                result.clear();
                for (a, b) in self.iter().zip(other.iter()) {
                    result.push(a * b);
                }
            }
        }
    }
    
    pub mod simd {
        use super::VeloxxError;
        
        pub struct SimdOps;
        impl SimdOps {
            pub fn new() -> Self { Self }
        }
        
        // Add trait extensions for WASM
        pub trait SimdSumExtension<T> {
            fn simd_sum(&self) -> T;
        }
        
        impl SimdSumExtension<i32> for Vec<i32> {
            fn simd_sum(&self) -> i32 {
                self.iter().sum()
            }
        }
        
        impl SimdSumExtension<f64> for Vec<f64> {
            fn simd_sum(&self) -> f64 {
                self.iter().sum()
            }
        }
        
        pub mod optimized {
            pub fn simd_sum_optimized<T: std::iter::Sum + Copy>(values: &[T]) -> T {
                values.iter().copied().sum()
            }
        }
    }
    
    pub mod simd_std {
        use super::VeloxxError;
        
        pub struct StdSimdOps;
        
        pub trait StdSimdMeanExtension<T> {
            fn std_simd_mean(&self) -> Result<Option<T>, VeloxxError>;
        }
        
        impl StdSimdMeanExtension<i32> for Vec<i32> {
            fn std_simd_mean(&self) -> Result<Option<i32>, VeloxxError> {
                if self.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(self.iter().sum::<i32>() / self.len() as i32))
                }
            }
        }
        
        impl StdSimdMeanExtension<f64> for Vec<f64> {
            fn std_simd_mean(&self) -> Result<Option<f64>, VeloxxError> {
                if self.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(self.iter().sum::<f64>() / self.len() as f64))
                }
            }
        }
    }
    
    pub mod vectorized_filter {
        use crate::error::VeloxxError;
        use crate::types::Value;
        
        pub enum ComparisonOp {
            GreaterThan,
            LessThan,
            Equal,
            Gt,
            Lt,
            Eq,
        }
        
        pub struct VectorizedFilter;
        impl VectorizedFilter {
            pub fn new() -> Self { Self }
            
            pub fn fast_filter_single_column(_series: &crate::series::Series, _comparison_value: &Value, _op: &ComparisonOp) -> Result<Vec<bool>, VeloxxError> {
                Err(VeloxxError::Unsupported("WASM filtering not implemented".to_string()))
            }
            
            pub fn filter_series_with_mask(_series: &crate::series::Series, _mask: &[bool]) -> Result<crate::series::Series, VeloxxError> {
                Err(VeloxxError::Unsupported("WASM filtering not implemented".to_string()))
            }
        }
    }
}
/// Core Series (column) data structure and its associated operations, including
/// type casting, aggregation, and statistical calculations.
pub mod series;
/// Advanced query engine with SIMD-accelerated predicate evaluation
pub mod query;
/// Defines the fundamental data types (`DataType`) and value (`Value`) enums
/// used to represent data within Series and DataFrames.
pub mod types;
/// Data visualization and plotting module
#[cfg(feature = "visualization")]
pub mod visualization;
/// Window functions and advanced analytics module
#[cfg(feature = "window_functions")]
pub mod window_functions;
/// Apache Arrow integration module for improved performance (not available in WASM)
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
pub mod arrow;

/// Python bindings module for high-performance data operations
#[cfg(feature = "python")]
pub mod python_bindings;

// Re-export main types for easy access
pub use crate::dataframe::DataFrame;
pub use crate::series::{DataType, Series};
pub use crate::types::Value;

// WASM exports
#[cfg(feature = "wasm")]
pub use wasm::{WasmDataFrame, WasmSeries};

/// WASM bindings module for web deployment
#[cfg(feature = "wasm")]
pub mod wasm_bindings;
#[cfg(feature = "wasm")]
pub use wasm_bindings::*;

#[cfg(test)]
mod tests {
    use crate::conditions::Condition;
    use crate::dataframe::DataFrame;
    use crate::error::VeloxxError;
    use indexmap::IndexMap;

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
        let mut columns = IndexMap::new();
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
        let mut columns = IndexMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2)]),
        );
        let df = DataFrame::new(columns).unwrap();

        let col1 = df.get_column("col1").unwrap();
        match col1 {
            Series::I32(_, v, b) => {
                let expected_values: Vec<i32> = vec![1, 2];
                let expected_bitmap: Vec<bool> = vec![true, true];
                assert_eq!(*v, expected_values);
                assert_eq!(*b, expected_bitmap);
            }
            _ => panic!("Unexpected series type"),
        }

        assert!(df.get_column("non_existent").is_none());
    }

    #[test]
    fn test_dataframe_display() {
        let mut columns = IndexMap::new();
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
        let expected_output = "col1           col2           col3           
--------------- --------------- --------------- 
1              a              1.1            
0              b              2.2            
3                             3.3            
";
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
            Series::I32(_, v, b) => {
                let expected_values: Vec<i32> = vec![1, 4, 7, 0];
                let expected_bitmap: Vec<bool> = vec![true, true, true, false];
                assert_eq!(*v, expected_values);
                assert_eq!(*b, expected_bitmap);
            }
            _ => panic!("Expected I32 series"),
        }

        let col_f64 = df.get_column("col_f64").unwrap();
        match col_f64 {
            Series::F64(_, v, b) => {
                let expected_values: Vec<f64> = vec![2.0, 5.0, 8.0, 0.0];
                let expected_bitmap: Vec<bool> = vec![true, true, true, false];
                assert_eq!(*v, expected_values);
                assert_eq!(*b, expected_bitmap);
            }
            _ => panic!("Expected F64 series"),
        }

        let col_bool = df.get_column("col_bool").unwrap();
        match col_bool {
            Series::Bool(_, v, b) => {
                let expected_values: Vec<bool> = vec![true, false, false, true];
                let expected_bitmap: Vec<bool> = vec![true, true, false, true];
                assert_eq!(*v, expected_values);
                assert_eq!(*b, expected_bitmap);
            }
            _ => panic!("Expected Bool series"),
        }

        let col_string = df.get_column("col_string").unwrap();
        match col_string {
            Series::String(_, v, b) => {
                let expected_values: Vec<String> = vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "rust".to_string(),
                    String::new(),
                ];
                let expected_bitmap: Vec<bool> = vec![true, true, true, false];
                assert_eq!(*v, expected_values);
                assert_eq!(*b, expected_bitmap);
            }
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

// WASM Bindings
#[cfg(feature = "wasm")]
pub mod wasm {
    use crate::{DataFrame, Series, Value, DataType, Condition};
    use indexmap::IndexMap;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct WasmDataFrame {
        inner: DataFrame,
    }

    #[wasm_bindgen]
    impl WasmDataFrame {
        #[wasm_bindgen(constructor)]
        pub fn new(data: JsValue) -> Result<WasmDataFrame, JsValue> {
            let data: serde_json::Value = serde_wasm_bindgen::from_value(data)
                .map_err(|e| JsValue::from_str(&format!("Failed to parse data: {}", e)))?;
            
            let mut columns = IndexMap::new();
            
            if let Some(obj) = data.as_object() {
                for (name, values) in obj {
                    if let Some(array) = values.as_array() {
                        let series = Self::json_array_to_series(name, array)?;
                        columns.insert(name.clone(), series);
                    }
                }
            }
            
            let df = DataFrame::new(columns)
                .map_err(|e| JsValue::from_str(&format!("Failed to create DataFrame: {}", e)))?;
            
            Ok(WasmDataFrame { inner: df })
        }

        #[wasm_bindgen]
        pub fn row_count(&self) -> usize {
            self.inner.row_count()
        }

        #[wasm_bindgen]
        pub fn column_count(&self) -> usize {
            self.inner.column_count()
        }

        #[wasm_bindgen]
        pub fn column_names(&self) -> Vec<String> {
            self.inner.column_names().iter().map(|s| s.to_string()).collect()
        }

        #[wasm_bindgen]
        pub fn get_column(&self, name: &str) -> Option<WasmSeries> {
            self.inner.get_column(name).map(|s| WasmSeries { inner: s.clone() })
        }

        #[wasm_bindgen]
        pub fn filter_gt(&self, column: &str, value: f64) -> Result<WasmDataFrame, JsValue> {
            let condition = Condition::Gt(column.to_string(), Value::F64(value));
            let filtered = self.inner.filter(&condition)
                .map_err(|e| JsValue::from_str(&format!("Filter error: {}", e)))?;
            Ok(WasmDataFrame { inner: filtered })
        }

        #[wasm_bindgen]
        pub fn select(&self, columns: Vec<String>) -> Result<WasmDataFrame, JsValue> {
            // For now, create a new DataFrame with selected columns
            let mut new_columns = IndexMap::new();
            for col_name in columns {
                if let Some(column) = self.inner.get_column(&col_name) {
                    new_columns.insert(col_name, column.clone());
                }
            }
            let selected = DataFrame::new(new_columns)
                .map_err(|e| JsValue::from_str(&format!("Failed to select columns: {}", e)))?;
            Ok(WasmDataFrame { inner: selected })
        }

        #[wasm_bindgen]
        pub fn to_json(&self) -> Result<String, JsValue> {
            // Convert DataFrame to a simple JSON representation
            let mut result = serde_json::Map::new();
            for col_name in self.inner.column_names() {
                if let Some(series) = self.inner.get_column(col_name) {
                    let values: Vec<serde_json::Value> = (0..series.len())
                        .map(|i| {
                            match series.get_value(i) {
                                Some(Value::I32(val)) => serde_json::Value::Number(serde_json::Number::from(*val)),
                                Some(Value::F64(val)) => serde_json::Value::Number(serde_json::Number::from_f64(*val).unwrap_or(serde_json::Number::from(0))),
                                Some(Value::String(val)) => serde_json::Value::String(val.clone()),
                                Some(Value::Bool(val)) => serde_json::Value::Bool(*val),
                                None => serde_json::Value::Null,
                            }
                        })
                        .collect();
                    result.insert(col_name.clone(), serde_json::Value::Array(values));
                }
            }
            serde_json::to_string(&result)
                .map_err(|e| JsValue::from_str(&format!("Failed to serialize to JSON: {}", e)))
        }

        fn json_array_to_series(name: &str, array: &[serde_json::Value]) -> Result<Series, JsValue> {
            if array.is_empty() {
                return Ok(Series::new_string(name, vec![]));
            }

            // Detect type from first non-null value
            let first_val = array.iter().find(|v| !v.is_null());
            
            match first_val {
                Some(serde_json::Value::Number(n)) => {
                    if n.is_f64() {
                        let values: Vec<Option<f64>> = array.iter()
                            .map(|v| v.as_f64())
                            .collect();
                        Ok(Series::new_f64(name, values))
                    } else {
                        let values: Vec<Option<i32>> = array.iter()
                            .map(|v| v.as_i64().map(|i| i as i32))
                            .collect();
                        Ok(Series::new_i32(name, values))
                    }
                },
                Some(serde_json::Value::String(_)) => {
                    let values: Vec<Option<String>> = array.iter()
                        .map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    Ok(Series::new_string(name, values))
                },
                Some(serde_json::Value::Bool(_)) => {
                    let values: Vec<Option<bool>> = array.iter()
                        .map(|v| v.as_bool())
                        .collect();
                    Ok(Series::new_bool(name, values))
                },
                _ => {
                    // Default to string if type is unclear
                    let values: Vec<Option<String>> = array.iter()
                        .map(|v| if v.is_null() { None } else { Some(v.to_string()) })
                        .collect();
                    Ok(Series::new_string(name, values))
                }
            }
        }
    }

    #[wasm_bindgen]
    pub struct WasmSeries {
        inner: Series,
    }

    #[wasm_bindgen]
    impl WasmSeries {
        #[wasm_bindgen(constructor)]
        pub fn new(name: &str, data: JsValue) -> Result<WasmSeries, JsValue> {
            let data: serde_json::Value = serde_wasm_bindgen::from_value(data)
                .map_err(|e| JsValue::from_str(&format!("Failed to parse data: {}", e)))?;
            
            if let Some(array) = data.as_array() {
                let series = WasmDataFrame::json_array_to_series(name, array)?;
                Ok(WasmSeries { inner: series })
            } else {
                Err(JsValue::from_str("Data must be an array"))
            }
        }

        #[wasm_bindgen]
        pub fn count(&self) -> usize {
            self.inner.len()
        }

        #[wasm_bindgen]
        pub fn is_empty(&self) -> bool {
            self.inner.is_empty()
        }

        #[wasm_bindgen]
        pub fn data_type(&self) -> String {
            format!("{:?}", self.inner.data_type())
        }

        #[wasm_bindgen]
        pub fn get_value(&self, index: usize) -> JsValue {
            match self.inner.get_value(index) {
                Some(Value::I32(i)) => JsValue::from_f64(*i as f64),
                Some(Value::F64(f)) => JsValue::from_f64(*f),
                Some(Value::String(s)) => JsValue::from_str(s),
                Some(Value::Bool(b)) => JsValue::from_bool(*b),
                None => JsValue::NULL,
            }
        }
    }
}
