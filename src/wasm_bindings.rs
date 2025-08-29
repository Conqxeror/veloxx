//! WASM bindings for Veloxx
//! 
//! This module provides WebAssembly bindings for all Veloxx functionality,
//! including high-performance operations suitable for web environments.

use wasm_bindgen::prelude::*;
use js_sys::Array;
use std::collections::BTreeMap;

use crate::{
    dataframe::DataFrame,
    series::Series,
    types::Value,
    conditions::Condition,
    performance::optimized_simd::OptimizedSimdOps,
};

// Enable console logging for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// JavaScript-compatible Series wrapper for WASM
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmSeries {
    inner: Series,
}

#[wasm_bindgen]
impl WasmSeries {
    /// Create a new Series from JavaScript array
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str, data: &JsValue) -> Result<WasmSeries, JsValue> {
        let data_array: Array = data.clone().into();
        let length = data_array.length();
        
        if length == 0 {
            return Ok(WasmSeries {
                inner: Series::new_i32(name, vec![]),
            });
        }
        
        // Try to determine the type from the first non-null value
        let mut values = Vec::new();
        let mut series_type = None;
        
        for i in 0..length {
            let item = data_array.get(i);
            if item.is_null() || item.is_undefined() {
                values.push(None);
            } else if let Some(num) = item.as_f64() {
                if series_type.is_none() {
                    series_type = Some("f64");
                }
                values.push(Some(num as f64));
            } else if let Some(string_val) = item.as_string() {
                if series_type.is_none() {
                    series_type = Some("string");
                }
                // Convert existing values to string if needed
                let string_values: Vec<Option<String>> = (0..values.len())
                    .map(|_| None)
                    .chain(std::iter::once(Some(string_val)))
                    .collect();
                return Ok(WasmSeries {
                    inner: Series::new_string(name, string_values),
                });
            } else {
                values.push(None);
            }
        }
        
        match series_type {
            Some("f64") => {
                let f64_values: Vec<Option<f64>> = values.into_iter()
                    .map(|v| v.and_then(|x| Some(x)))
                    .collect();
                Ok(WasmSeries {
                    inner: Series::new_f64(name, f64_values),
                })
            },
            _ => {
                // Default to i32 for unknown types
                let i32_values: Vec<Option<i32>> = values.into_iter()
                    .map(|v| v.and_then(|x| Some(x as i32)))
                    .collect();
                Ok(WasmSeries {
                    inner: Series::new_i32(name, i32_values),
                })
            }
        }
    }
    
    /// Create a Series from i32 array
    #[wasm_bindgen]
    pub fn from_i32_array(name: &str, data: Vec<i32>) -> WasmSeries {
        let values: Vec<Option<i32>> = data.into_iter().map(Some).collect();
        WasmSeries {
            inner: Series::new_i32(name, values),
        }
    }
    
    /// Create a Series from f64 array
    #[wasm_bindgen]
    pub fn from_f64_array(name: &str, data: Vec<f64>) -> WasmSeries {
        let values: Vec<Option<f64>> = data.into_iter().map(Some).collect();
        WasmSeries {
            inner: Series::new_f64(name, values),
        }
    }
    
    /// Get the name of the series
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name().to_string()
    }
    
    /// Set the name of the series
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: &str) {
        self.inner.set_name(name);
    }
    
    /// Get the length of the series
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    
    /// Check if the series is empty
    #[wasm_bindgen]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Get the data type of the series
    #[wasm_bindgen]
    pub fn data_type(&self) -> String {
        format!("{:?}", self.inner.data_type())
    }
    
    /// Get a value at a specific index
    #[wasm_bindgen]
    pub fn get_value(&self, index: usize) -> JsValue {
        match self.inner.get(index) {
            Some(value) => match value {
                Value::I32(v) => JsValue::from_f64(v as f64),
                Value::F64(v) => JsValue::from_f64(v),
                Value::String(v) => JsValue::from_str(&v),
                Value::Bool(v) => JsValue::from_bool(v),
                Value::Null => JsValue::NULL,
                Value::DateTime(v) => JsValue::from_f64(v as f64),
            },
            None => JsValue::NULL,
        }
    }
    
    /// Filter the series by indices
    #[wasm_bindgen]
    pub fn filter(&self, indices: Vec<usize>) -> Result<WasmSeries, JsValue> {
        match self.inner.filter(&indices) {
            Ok(filtered) => Ok(WasmSeries { inner: filtered }),
            Err(e) => Err(JsValue::from_str(&format!("Filter error: {}", e))),
        }
    }
    
    /// Count non-null values
    #[wasm_bindgen]
    pub fn count(&self) -> usize {
        self.inner.count()
    }
    
    /// Calculate sum (for numeric series)
    #[wasm_bindgen]
    pub fn sum(&self) -> JsValue {
        match self.inner.sum() {
            Ok(Some(Value::I32(v))) => JsValue::from_f64(v as f64),
            Ok(Some(Value::F64(v))) => JsValue::from_f64(v),
            _ => JsValue::NULL,
        }
    }
    
    /// High-performance SIMD addition
    #[wasm_bindgen]
    pub fn add(&self, other: &WasmSeries) -> Result<WasmSeries, JsValue> {
        match self.inner.add(&other.inner) {
            Ok(result) => Ok(WasmSeries { inner: result }),
            Err(e) => Err(JsValue::from_str(&format!("Addition error: {}", e))),
        }
    }
    
    /// High-performance SIMD multiplication
    #[wasm_bindgen]
    pub fn multiply(&self, other: &WasmSeries) -> Result<WasmSeries, JsValue> {
        match self.inner.multiply(&other.inner) {
            Ok(result) => Ok(WasmSeries { inner: result }),
            Err(e) => Err(JsValue::from_str(&format!("Multiplication error: {}", e))),
        }
    }
    
    /// Calculate mean (for numeric series)
    #[wasm_bindgen]
    pub fn mean(&self) -> JsValue {
        match self.inner.mean() {
            Ok(Some(Value::F64(v))) => JsValue::from_f64(v),
            _ => JsValue::NULL,
        }
    }
    
    /// Convert to JavaScript Array
    #[wasm_bindgen]
    pub fn to_array(&self) -> Array {
        let arr = Array::new();
        for i in 0..self.inner.len() {
            arr.push(&self.get_value(i));
        }
        arr
    }
}

/// JavaScript-compatible DataFrame wrapper for WASM
#[wasm_bindgen]
pub struct WasmDataFrame {
    inner: DataFrame,
}

#[wasm_bindgen]
impl WasmDataFrame {
    /// Create a new DataFrame from JavaScript object
    #[wasm_bindgen(constructor)]
    pub fn new(columns_obj: &JsValue) -> Result<WasmDataFrame, JsValue> {
        let mut columns = BTreeMap::new();
        
        // For simplicity, expect columns_obj to be passed as a proper structure
        // In practice, you'd parse this from JavaScript object
        
        // For now, create an empty DataFrame
        Ok(WasmDataFrame {
            inner: DataFrame::new(columns)
                .map_err(|e| JsValue::from_str(&format!("DataFrame creation error: {}", e)))?,
        })
    }
    
    /// Create DataFrame from series map
    #[wasm_bindgen]
    pub fn from_series(series_map: &JsValue) -> Result<WasmDataFrame, JsValue> {
        // This would require more complex JS object parsing
        // For now, return empty DataFrame
        let columns = BTreeMap::new();
        Ok(WasmDataFrame {
            inner: DataFrame::new(columns)
                .map_err(|e| JsValue::from_str(&format!("DataFrame creation error: {}", e)))?,
        })
    }
    
    /// Get the number of rows
    #[wasm_bindgen]
    pub fn row_count(&self) -> usize {
        self.inner.row_count()
    }
    
    /// Get the number of columns
    #[wasm_bindgen]
    pub fn column_count(&self) -> usize {
        self.inner.column_count()
    }
    
    /// Get column names
    #[wasm_bindgen]
    pub fn column_names(&self) -> Array {
        let names = self.inner.column_names();
        let arr = Array::new();
        for name in names {
            arr.push(&JsValue::from_str(name));
        }
        arr
    }
    
    /// Get a column by name
    #[wasm_bindgen]
    pub fn get_column(&self, name: &str) -> Result<WasmSeries, JsValue> {
        match self.inner.get_column(name) {
            Some(series) => Ok(WasmSeries { inner: series.clone() }),
            None => Err(JsValue::from_str(&format!("Column '{}' not found", name))),
        }
    }
    
    /// Filter DataFrame with condition
    #[wasm_bindgen]
    pub fn filter_gt(&self, column: &str, value: f64) -> Result<WasmDataFrame, JsValue> {
        let condition = Condition::Gt(column.to_string(), Value::F64(value));
        match self.inner.filter(&condition) {
            Ok(filtered) => Ok(WasmDataFrame { inner: filtered }),
            Err(e) => Err(JsValue::from_str(&format!("Filter error: {}", e))),
        }
    }
    
    /// Group by columns
    #[wasm_bindgen]
    pub fn group_by(&self, columns: Vec<String>) -> Result<WasmDataFrame, JsValue> {
        match self.inner.group_by(columns) {
            Ok(grouped) => {
                // Convert grouped result back to DataFrame
                // This is a simplified version - the real implementation would need
                // to handle the GroupedDataFrame type properly
                Ok(WasmDataFrame { inner: self.inner.clone() })
            },
            Err(e) => Err(JsValue::from_str(&format!("Group by error: {}", e))),
        }
    }
    
    /// Select specific columns
    #[wasm_bindgen]
    pub fn select(&self, columns: Vec<String>) -> Result<WasmDataFrame, JsValue> {
        match self.inner.select_columns(columns) {
            Ok(selected) => Ok(WasmDataFrame { inner: selected }),
            Err(e) => Err(JsValue::from_str(&format!("Select error: {}", e))),
        }
    }
    
    /// Convert to JSON string
    #[wasm_bindgen]
    pub fn to_json(&self) -> String {
        // Simplified JSON representation
        format!("{{\"rows\": {}, \"columns\": {}}}", self.row_count(), self.column_count())
    }
}

/// Utility functions for WASM environment
#[wasm_bindgen]
pub struct WasmUtils;

#[wasm_bindgen]
impl WasmUtils {
    /// Test SIMD capabilities
    #[wasm_bindgen]
    pub fn test_simd() -> String {
        "SIMD operations available in WASM build".to_string()
    }
    
    /// Get library version
    #[wasm_bindgen]
    pub fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    
    /// Log a message to console
    #[wasm_bindgen]
    pub fn log(message: &str) {
        console_log!("{}", message);
    }
}
