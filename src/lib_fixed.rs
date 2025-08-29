//! # Veloxx
//!
//! A high-performance, lightweight dataframe library for Rust, focusing on efficient
//! data manipulation with minimal overhead.

// Core exports
pub use crate::dataframe::DataFrame;
pub use crate::series::Series;
pub use crate::types::{DataType, Value};
pub use crate::conditions::Condition;

// WASM exports
#[cfg(feature = "wasm")]
pub use wasm::{WasmDataFrame, WasmSeries};

// Core modules
pub mod types;
pub mod errors;
pub mod dataframe;
pub mod series;
pub mod conditions;
pub mod memory;
pub mod performance;
pub mod statistics;

// Optional modules that only build for native targets
#[cfg(not(target_arch = "wasm32"))]
pub mod advanced_io;
#[cfg(not(target_arch = "wasm32"))]
pub mod distributed;
#[cfg(not(target_arch = "wasm32"))]
pub mod python_bindings;

// Re-export the main error type
pub use errors::VeloxxError;

// WASM Bindings
#[cfg(feature = "wasm")]
pub mod wasm {
    use crate::{DataFrame, Series, Value, DataType, Condition};
    use std::collections::HashMap;
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
            
            let mut columns = HashMap::new();
            
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
            let mut new_columns = HashMap::new();
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
                                Some(Value::I32(val)) => serde_json::Value::Number(serde_json::Number::from(val)),
                                Some(Value::F64(val)) => serde_json::Value::Number(serde_json::Number::from_f64(val).unwrap_or(serde_json::Number::from(0))),
                                Some(Value::String(val)) => serde_json::Value::String(val.clone()),
                                Some(Value::Bool(val)) => serde_json::Value::Bool(val),
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
                Some(Value::I32(i)) => JsValue::from_f64(i as f64),
                Some(Value::F64(f)) => JsValue::from_f64(f),
                Some(Value::String(s)) => JsValue::from_str(s),
                Some(Value::Bool(b)) => JsValue::from_bool(b),
                None => JsValue::NULL,
            }
        }
    }
}
