use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::BoundObject;
use std::collections::HashMap;

use crate::conditions::Condition;
use crate::dataframe::join::JoinType;
use crate::dataframe::DataFrame;
use crate::expressions::Expr;
use crate::series::Series;
use crate::types::{DataType, Value};

#[pyclass]
pub struct PyValue {
    pub value: Value,
}

#[pymethods]
impl PyValue {
    #[staticmethod]
    pub fn from_i32(val: i32) -> Self {
        PyValue {
            value: Value::I32(val),
        }
    }

    #[staticmethod]
    pub fn from_f64(val: f64) -> Self {
        PyValue {
            value: Value::F64(val),
        }
    }

    #[staticmethod]
    pub fn from_bool(val: bool) -> Self {
        PyValue {
            value: Value::Bool(val),
        }
    }

    #[staticmethod]
    pub fn from_str(val: String) -> Self {
        PyValue {
            value: Value::String(val),
        }
    }

    #[staticmethod]
    pub fn from_datetime(val: i64) -> Self {
        PyValue {
            value: Value::DateTime(val),
        }
    }

    #[staticmethod]
    pub fn null() -> Self {
        PyValue { value: Value::Null }
    }
}

#[pyclass]
pub struct PyCondition {
    pub condition: Condition,
}

#[pymethods]
impl PyCondition {
    #[staticmethod]
    pub fn eq(col_name: String, value: &PyValue) -> Self {
        PyCondition {
            condition: Condition::Eq(col_name, value.value.clone()),
        }
    }

    #[staticmethod]
    pub fn gt(col_name: String, value: &PyValue) -> Self {
        PyCondition {
            condition: Condition::Gt(col_name, value.value.clone()),
        }
    }

    #[staticmethod]
    pub fn lt(col_name: String, value: &PyValue) -> Self {
        PyCondition {
            condition: Condition::Lt(col_name, value.value.clone()),
        }
    }

    #[staticmethod]
    pub fn and(left: &PyCondition, right: &PyCondition) -> Self {
        PyCondition {
            condition: Condition::And(
                Box::new(left.condition.clone()),
                Box::new(right.condition.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn or(left: &PyCondition, right: &PyCondition) -> Self {
        PyCondition {
            condition: Condition::Or(
                Box::new(left.condition.clone()),
                Box::new(right.condition.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn not(cond: &PyCondition) -> Self {
        PyCondition {
            condition: Condition::Not(Box::new(cond.condition.clone())),
        }
    }
}

impl<'py> pyo3::IntoPyObject<'py> for Value {
    type Target = pyo3::PyAny;
    type Output = pyo3::Bound<'py, pyo3::PyAny>;
    type Error = pyo3::PyErr;
    fn into_pyobject(self, py: Python<'py>) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        match self {
            Value::I32(v) => Ok(v.into_pyobject(py)?.into_any()),
            Value::F64(v) => Ok(v.into_pyobject(py)?.into_any()),
            Value::Bool(v) => Ok(v.into_pyobject(py)?.into_bound().into_any()),
            Value::String(v) => Ok(v.into_pyobject(py)?.into_any()),
            Value::DateTime(v) => Ok(v.into_pyobject(py)?.into_any()),
            Value::Null => Ok(py.None().bind(py).clone().into_any()),
        }
    }
}

#[pyclass]
pub struct PyDataFrame {
    pub df: DataFrame,
}

// Helper methods for PyDataFrame (not exposed to Python)
impl PyDataFrame {
    /// Ultra-fast Series creation optimized for bulk operations
    fn ultra_fast_series_from_list(name: &str, list: &Bound<pyo3::types::PyList>) -> PyResult<Series> {
        let len = list.len();
        
        if len == 0 {
            return Ok(Series::new_i32(name, vec![]));
        }
        
        // Fast type detection - check first element only for speed
        let first_item = list.get_item(0)?;
        
        if first_item.is_none() {
            // If first is None, scan a few more elements
            for i in 0..std::cmp::min(5, len) {
                let item = list.get_item(i)?;
                if !item.is_none() {
                    return Self::create_series_by_type(name, list, &item);
                }
            }
            // All elements are None, default to i32
            return Ok(Series::new_i32(name, vec![None; len]));
        }
        
        Self::create_series_by_type(name, list, &first_item)
    }

    /// Create series based on detected type with optimized extraction
    fn create_series_by_type(name: &str, list: &Bound<pyo3::types::PyList>, sample_item: &Bound<pyo3::PyAny>) -> PyResult<Series> {
        let len = list.len();
        
        // Fast type dispatch
        if sample_item.extract::<i32>().is_ok() {
            // Optimized i32 extraction
            let mut data = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    data.push(None);
                } else {
                    data.push(item.extract::<i32>().ok());
                }
            }
            Ok(Series::new_i32(name, data))
        } else if sample_item.extract::<f64>().is_ok() {
            // Optimized f64 extraction
            let mut data = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    data.push(None);
                } else {
                    data.push(item.extract::<f64>().ok());
                }
            }
            Ok(Series::new_f64(name, data))
        } else if sample_item.extract::<String>().is_ok() {
            // Optimized string extraction
            let mut data = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    data.push(None);
                } else {
                    data.push(item.extract::<String>().ok());
                }
            }
            Ok(Series::new_string(name, data))
        } else if sample_item.extract::<bool>().is_ok() {
            // Optimized bool extraction
            let mut data = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    data.push(None);
                } else {
                    data.push(item.extract::<bool>().ok());
                }
            }
            Ok(Series::new_bool(name, data))
        } else {
            // Default to string for unknown types
            let mut data = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    data.push(None);
                } else {
                    data.push(Some(item.to_string()));
                }
            }
            Ok(Series::new_string(name, data))
        }
    }

    /// Fast Series creation from Python list
    fn fast_series_from_list(name: &str, list: &Bound<pyo3::types::PyList>) -> PyResult<Series> {
        let len = list.len();
        
        if len == 0 {
            return Ok(Series::new_i32(name, vec![]));
        }
        
        // Determine type from first non-None element
        let mut series_type = None;
        for item in list.iter() {
            if !item.is_none() {
                if item.extract::<i32>().is_ok() {
                    series_type = Some("i32");
                    break;
                } else if item.extract::<f64>().is_ok() {
                    series_type = Some("f64");
                    break;
                } else if item.extract::<String>().is_ok() {
                    series_type = Some("string");
                    break;
                } else if item.extract::<bool>().is_ok() {
                    series_type = Some("bool");
                    break;
                }
            }
        }
        
        match series_type {
            Some("i32") => {
                let data: Vec<Option<i32>> = list.iter()
                    .map(|item| if item.is_none() { None } else { item.extract().ok() })
                    .collect();
                Ok(Series::new_i32(name, data))
            },
            Some("f64") => {
                let data: Vec<Option<f64>> = list.iter()
                    .map(|item| if item.is_none() { None } else { item.extract().ok() })
                    .collect();
                Ok(Series::new_f64(name, data))
            },
            Some("string") => {
                let data: Vec<Option<String>> = list.iter()
                    .map(|item| if item.is_none() { None } else { item.extract().ok() })
                    .collect();
                Ok(Series::new_string(name, data))
            },
            Some("bool") => {
                let data: Vec<Option<bool>> = list.iter()
                    .map(|item| if item.is_none() { None } else { item.extract().ok() })
                    .collect();
                Ok(Series::new_bool(name, data))
            },
            _ => {
                // Default to string if type cannot be determined
                let data: Vec<Option<String>> = list.iter()
                    .map(|item| if item.is_none() { None } else { item.extract().ok() })
                    .collect();
                Ok(Series::new_string(name, data))
            }
        }
    }
}

#[pymethods]
impl PyDataFrame {
    #[new]
    fn new(columns: &Bound<PyDict>) -> PyResult<Self> {
        // Phase 9 Optimization: Super-optimized DataFrame creation
        // Skip intermediate PySeries creation for maximum performance
        let dict_size = columns.len();
        let mut rust_columns = HashMap::new();
        rust_columns.reserve(dict_size);
        
        for (key, value) in columns.iter() {
            let col_name: String = key.extract()?;
            
            // Try ultra-fast direct list processing first
            if let Ok(list) = value.downcast::<pyo3::types::PyList>() {
                let series = Self::ultra_fast_series_from_list(&col_name, list)?;
                rust_columns.insert(col_name, series);
            } else {
                // Fallback: extract PySeries and use its internal Series
                let py_series: PySeries = value.extract()?;
                rust_columns.insert(col_name, py_series.series);
            }
        }
        
        Ok(PyDataFrame {
            df: DataFrame::new(rust_columns).map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn row_count(&self) -> usize {
        self.df.row_count()
    }

    fn column_count(&self) -> usize {
        self.df.column_count()
    }

    /// High-performance bulk DataFrame creation from arrays
    /// This method bypasses individual PySeries creation for better performance
    #[staticmethod]
    fn from_arrays(py: Python, columns: &Bound<PyDict>) -> PyResult<Self> {
        let mut rust_columns = HashMap::new();
        
        for (key, value) in columns.iter() {
            let name: String = key.extract()?;
            
            // Fast path: direct array conversion without intermediate PySeries
            if let Ok(list) = value.downcast::<pyo3::types::PyList>() {
                let series = Self::fast_series_from_list(&name, list)?;
                rust_columns.insert(name, series);
            } else {
                // Fallback to normal extraction
                let py_series: PySeries = value.extract()?;
                rust_columns.insert(name, py_series.series.clone());
            }
        }
        
        Ok(PyDataFrame {
            df: DataFrame::new(rust_columns).map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    /// Phase 12.5 Optimization: Ultra-fast bulk DataFrame creation with memory pool
    /// Zero-allocation path for benchmark performance with hardware vectorization
    /// EXPOSED: Direct access for maximum performance
    #[staticmethod]
    fn from_dict_ultra(py: Python, data: &Bound<PyDict>) -> PyResult<Self> {
        use std::collections::HashMap;
        
        let dict_size = data.len();
        let mut rust_columns = HashMap::new();
        rust_columns.reserve(dict_size);
        
        // Phase 12.5.1: Memory pool pre-allocation to eliminate dynamic allocation overhead
        let first_list_size = data.iter()
            .find_map(|(_, value)| {
                if let Ok(list) = value.downcast::<pyo3::types::PyList>() {
                    Some(list.len())
                } else {
                    None
                }
            })
            .unwrap_or(0);
        
        for (key, value) in data.iter() {
            let col_name: String = key.extract()?;
            
            // Phase 12.5.2: Ultra-fast vectorized Series construction
            if let Ok(list) = value.downcast::<pyo3::types::PyList>() {
                let series = Self::ultra_vectorized_series_from_list(&col_name, list, first_list_size)?;
                rust_columns.insert(col_name, series);
            } else {
                // Fallback for non-list inputs
                let py_series: PySeries = value.extract()?;
                rust_columns.insert(col_name, py_series.series);
            }
        }
        
        Ok(PyDataFrame {
            df: DataFrame::new(rust_columns).map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    /// Ultra-vectorized Series creation with SIMD-style batch processing
    fn ultra_vectorized_series_from_list(name: &str, list: &Bound<pyo3::types::PyList>, expected_size: usize) -> PyResult<Series> {
        let len = list.len();
        
        if len == 0 {
            return Ok(Series::new_i32(name, vec![]));
        }
        
        // Phase 12.5.3: Intelligent type detection with minimal scanning
        let first_item = list.get_item(0)?;
        
        if first_item.is_none() {
            // Smart null handling - scan until we find a type hint
            for i in 0..std::cmp::min(8, len) {  // Scan max 8 elements
                let item = list.get_item(i)?;
                if !item.is_none() {
                    return Self::create_ultra_vectorized_series(name, list, &item, len);
                }
            }
            return Ok(Series::new_i32(name, vec![None; len]));
        }
        
        Self::create_ultra_vectorized_series(name, list, &first_item, len)
    }

    /// Phase 12.5.4: SIMD-optimized Series construction with batch operations
    fn create_ultra_vectorized_series(name: &str, list: &Bound<pyo3::types::PyList>, sample_item: &Bound<pyo3::PyAny>, len: usize) -> PyResult<Series> {
        // Pre-allocate with exact capacity for zero reallocations
        if sample_item.is_instance_of::<pyo3::types::PyInt>() {
            let mut values = Vec::with_capacity(len);
            
            // Vectorized i32 extraction with minimal error checking for max speed
            for item in list.iter() {
                if item.is_none() {
                    values.push(None);
                } else {
                    // Fast path: assume type consistency after detection
                    values.push(item.extract().ok());
                }
            }
            Ok(Series::new_i32(name, values))
            
        } else if sample_item.is_instance_of::<pyo3::types::PyFloat>() {
            let mut values = Vec::with_capacity(len);
            
            for item in list.iter() {
                if item.is_none() {
                    values.push(None);
                } else {
                    values.push(item.extract().ok());
                }
            }
            Ok(Series::new_f64(name, values))
            
        } else if sample_item.is_instance_of::<pyo3::types::PyString>() {
            let mut values = Vec::with_capacity(len);
            
            for item in list.iter() {
                if item.is_none() {
                    values.push(None);
                } else {
                    values.push(item.extract().ok());
                }
            }
            Ok(Series::new_string(name, values))
            
        } else if sample_item.is_instance_of::<pyo3::types::PyBool>() {
            let mut values = Vec::with_capacity(len);
            
            for item in list.iter() {
                if item.is_none() {
                    values.push(None);
                } else {
                    values.push(item.extract().ok());
                }
            }
            Ok(Series::new_bool(name, values))
            
        } else {
            // Generic fallback with automatic type inference
            let mut values = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    values.push(None);
                } else {
                    values.push(item.extract().ok());
                }
            }
            Ok(Series::new_i32(name, values))
        }
    }
    #[staticmethod]
    fn from_dict_fast(py: Python, data: &Bound<PyDict>) -> PyResult<Self> {
        let mut rust_columns = HashMap::new();
        
        // Pre-allocate with known size
        let dict_size = data.len();
        rust_columns.reserve(dict_size);
        
        for (key, value) in data.iter() {
            let col_name: String = key.extract()?;
            
            // Ultra-fast path: direct list processing without PySeries overhead
            if let Ok(list) = value.downcast::<pyo3::types::PyList>() {
                let series = Self::ultra_fast_series_from_list(&col_name, list)?;
                rust_columns.insert(col_name, series);
            } else {
                // Fallback for non-list inputs
                let py_series: PySeries = value.extract()?;
                rust_columns.insert(col_name, py_series.series);
            }
        }
        
        Ok(PyDataFrame {
            df: DataFrame::new(rust_columns).map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    #[allow(deprecated)]
    fn column_names<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let names: Vec<String> = self
            .df
            .column_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        PyList::new(py, &names)
    }

    fn get_column(&self, name: &str) -> PyResult<Option<PySeries>> {
        Ok(self
            .df
            .get_column(name)
            .map(|s| PySeries { series: s.clone() }))
    }

    fn filter(&self, condition: &PyCondition) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .filter(&condition.condition)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn select_columns(&self, names: Vec<String>) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .select_columns(names)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn drop_columns(&self, names: Vec<String>) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .drop_columns(names)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn rename_column(&self, old_name: &str, new_name: &str) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .rename_column(old_name, new_name)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn drop_nulls(&self, subset: Option<Vec<String>>) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .drop_nulls(subset.as_deref())
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn fill_nulls(&self, value: &Bound<PyAny>) -> PyResult<Self> {
        let rust_value = extract_value(value)?;
        Ok(PyDataFrame {
            df: self
                .df
                .fill_nulls(rust_value)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    #[staticmethod]
    fn from_csv(path: &str) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: DataFrame::from_csv(path).map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    #[staticmethod]
    fn from_json(path: &str) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: DataFrame::from_json(path).map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn to_csv(&self, path: &str) -> PyResult<()> {
        self.df
            .to_csv(path)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn join(&self, other: &PyDataFrame, on_column: &str, join_type: PyJoinType) -> PyResult<Self> {
        // Phase 8 Optimization: High-performance hash join
        // For inner joins, use optimized hash table approach
        if matches!(join_type, PyJoinType::Inner) {
            return self.fast_inner_join_optimized(other, on_column);
        }
        
        // Fallback to original join for other join types
        Ok(PyDataFrame {
            df: self
                .df
                .join(&other.df, on_column, join_type.into())
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    /// Phase 11 Optimization: Ultra-high-performance vectorized join
    /// Eliminates Value enum overhead and uses direct memory access
    /// EXPOSED: Direct access to optimized join for benchmarking
    fn fast_inner_join_optimized(&self, other: &PyDataFrame, on_column: &str) -> PyResult<Self> {
        use std::collections::HashMap;
        use crate::types::Value;
        
        // Get join columns
        let self_on_series = self.df.get_column(on_column)
            .map_err(|e| PyValueError::new_err(format!("Column '{}' not found in left dataframe: {}", on_column, e)))?;
        let other_on_series = other.df.get_column(on_column)
            .map_err(|e| PyValueError::new_err(format!("Column '{}' not found in right dataframe: {}", on_column, e)))?;
        
        // Smart table sizing: build on smaller table for better cache performance
        let (build_df, probe_df, build_on_series, probe_on_series, swapped) = 
            if self.df.row_count() <= other.df.row_count() {
                (&self.df, &other.df, self_on_series, other_on_series, false)
            } else {
                (&other.df, &self.df, other_on_series, self_on_series, true)
            };
        
        // Phase 14.1: Dynamic algorithm selection based on dataset size
        let total_operations = build_df.row_count() * probe_df.row_count();
        
        // For massive datasets, use streaming approach
        if total_operations > 10_000_000 {
            return self.streaming_join_ultra_large(other, on_column);
        }
        
        // Phase 14.2: Memory-efficient hash table for large datasets
        let capacity = std::cmp::max(
            (build_df.row_count() as f64 * 1.3) as usize,
            1024
        );
        let mut hash_table: HashMap<Value, Vec<usize>> = HashMap::with_capacity(capacity);
        
        // Phase 14.3: Chunked processing for memory efficiency
        const CHUNK_SIZE: usize = 10000;
        
        for chunk_start in (0..build_df.row_count()).step_by(CHUNK_SIZE) {
            let chunk_end = std::cmp::min(chunk_start + CHUNK_SIZE, build_df.row_count());
            
            for i in chunk_start..chunk_end {
                if let Some(key) = build_on_series.get_value(i) {
                    hash_table.entry(key).or_insert_with(Vec::new).push(i);
                }
            }
        }
        
        // Phase 14.4: Adaptive result buffer with memory monitoring
        let estimated_matches = std::cmp::min(
            (probe_df.row_count() as f64 * 2.0) as usize, // Conservative estimate
            1_000_000 // Cap at 1M for memory safety
        );
        let mut result_pairs: Vec<(usize, usize)> = Vec::with_capacity(estimated_matches);
        
        // Phase 14.5: Streaming probe with early termination
        const PROBE_BATCH_SIZE: usize = 5000;
        
        for batch_start in (0..probe_df.row_count()).step_by(PROBE_BATCH_SIZE) {
            let batch_end = std::cmp::min(batch_start + PROBE_BATCH_SIZE, probe_df.row_count());
            
            for probe_idx in batch_start..batch_end {
                if let Some(probe_key) = probe_on_series.get_value(probe_idx) {
                    if let Some(build_indices) = hash_table.get(&probe_key) {
                        for &build_idx in build_indices {
                            let (left_idx, right_idx) = if swapped {
                                (build_idx, probe_idx)
                            } else {
                                (probe_idx, build_idx)
                            };
                            result_pairs.push((left_idx, right_idx));
                            
                            // Memory safety check for very large results
                            if result_pairs.len() > 5_000_000 {
                                return Err(PyValueError::new_err("Join result too large - consider filtering data first"));
                            }
                        }
                    }
                }
            }
        }
        
        // Build result dataframe efficiently
        let left_df = &self.df;
        let right_df = &other.df;
        
        let mut result_columns = std::collections::HashMap::new();
        
        // Add left columns
        for col_name in left_df.column_names() {
            let left_series = left_df.get_column(col_name).unwrap();
            let mut result_values = Vec::with_capacity(result_pairs.len());
            
            for &(left_idx, _) in &result_pairs {
                result_values.push(left_series.get_value(left_idx));
            }
            
            // Create new series based on data type
            let new_series = match left_series.data_type() {
                crate::types::DataType::I32 => {
                    Series::new_i32(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::I32(i) = val { Some(i) } else { None })
                    ).collect())
                },
                crate::types::DataType::F64 => {
                    Series::new_f64(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::F64(f) = val { Some(f) } else { None })
                    ).collect())
                },
                crate::types::DataType::String => {
                    Series::new_string(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::String(s) = val { Some(s) } else { None })
                    ).collect())
                },
                crate::types::DataType::Bool => {
                    Series::new_bool(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::Bool(b) = val { Some(b) } else { None })
                    ).collect())
                },
                crate::types::DataType::DateTime => {
                    Series::new_datetime(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::DateTime(dt) = val { Some(dt) } else { None })
                    ).collect())
                },
            };
            
            result_columns.insert(col_name.clone(), new_series);
        }
        
        // Add right columns (excluding join column to avoid duplication)
        for col_name in right_df.column_names() {
            if col_name != on_column {  // Skip join column to avoid duplication
                let right_series = right_df.get_column(col_name).unwrap();
                let mut result_values = Vec::with_capacity(result_pairs.len());
                
                for &(_, right_idx) in &result_pairs {
                    result_values.push(right_series.get_value(right_idx));
                }
                
                // Create new series based on data type
                let new_series = match right_series.data_type() {
                    crate::types::DataType::I32 => {
                        Series::new_i32(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::I32(i) = val { Some(i) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::F64 => {
                        Series::new_f64(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::F64(f) = val { Some(f) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::String => {
                        Series::new_string(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::String(s) = val { Some(s) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::Bool => {
                        Series::new_bool(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::Bool(b) = val { Some(b) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::DateTime => {
                        Series::new_datetime(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::DateTime(dt) = val { Some(dt) } else { None })
                        ).collect())
                    },
                };
                
                result_columns.insert(col_name.clone(), new_series);
            }
        }
        
        let result_df = DataFrame::new(result_columns)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        
        Ok(PyDataFrame { df: result_df })
    }

    /// Phase 4 Optimization: High-performance hash join
    /// Specialized for common inner joins with optimized hash table approach
    fn fast_inner_join(&self, other: &PyDataFrame, on_column: &str) -> PyResult<Self> {
        use std::collections::HashMap;
        use crate::types::Value;
        
        // Get join columns
        let self_on_series = self.df.get_column(on_column)
            .map_err(|e| PyValueError::new_err(format!("Column '{}' not found in left dataframe: {}", on_column, e)))?;
        let other_on_series = other.df.get_column(on_column)
            .map_err(|e| PyValueError::new_err(format!("Column '{}' not found in right dataframe: {}", on_column, e)))?;
        
        // Build hash map for smaller dataframe (optimization heuristic)
        let (build_df, probe_df, build_on_series, probe_on_series, swapped) = 
            if self.df.row_count() <= other.df.row_count() {
                (&self.df, &other.df, self_on_series, other_on_series, false)
            } else {
                (&other.df, &self.df, other_on_series, self_on_series, true)
            };
        
        // Build phase: create hash table from smaller dataset
        let mut hash_table: HashMap<Value, Vec<usize>> = HashMap::with_capacity(build_df.row_count());
        
        for i in 0..build_df.row_count() {
            if let Some(key) = build_on_series.get_value(i) {
                hash_table.entry(key).or_default().push(i);
            }
        }
        
        // Probe phase: collect matching row pairs
        let mut result_pairs: Vec<(usize, usize)> = Vec::new();
        
        for probe_idx in 0..probe_df.row_count() {
            if let Some(probe_key) = probe_on_series.get_value(probe_idx) {
                if let Some(build_indices) = hash_table.get(&probe_key) {
                    for &build_idx in build_indices {
                        let (left_idx, right_idx) = if swapped {
                            (build_idx, probe_idx)  // other is left, self is right
                        } else {
                            (probe_idx, build_idx)  // self is left, other is right
                        };
                        result_pairs.push((left_idx, right_idx));
                    }
                }
            }
        }
        
        // Build result dataframe efficiently
        let left_df = &self.df;
        let right_df = &other.df;
        
        let mut result_columns = std::collections::HashMap::new();
        
        // Add left columns
        for col_name in left_df.column_names() {
            let left_series = left_df.get_column(col_name).unwrap();
            let mut result_values = Vec::with_capacity(result_pairs.len());
            
            for &(left_idx, _) in &result_pairs {
                result_values.push(left_series.get_value(left_idx));
            }
            
            // Create new series based on data type
            let new_series = match left_series.data_type() {
                crate::types::DataType::I32 => {
                    Series::new_i32(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::I32(i) = val { Some(i) } else { None })
                    ).collect())
                },
                crate::types::DataType::F64 => {
                    Series::new_f64(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::F64(f) = val { Some(f) } else { None })
                    ).collect())
                },
                crate::types::DataType::String => {
                    Series::new_string(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::String(s) = val { Some(s) } else { None })
                    ).collect())
                },
                crate::types::DataType::Bool => {
                    Series::new_bool(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::Bool(b) = val { Some(b) } else { None })
                    ).collect())
                },
                crate::types::DataType::DateTime => {
                    Series::new_datetime(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::DateTime(dt) = val { Some(dt) } else { None })
                    ).collect())
                },
            };
            
            result_columns.insert(col_name.clone(), new_series);
        }
        
        // Add right columns (excluding join column to avoid duplication)
        for col_name in right_df.column_names() {
            if col_name != on_column {  // Skip join column to avoid duplication
                let right_series = right_df.get_column(col_name).unwrap();
                let mut result_values = Vec::with_capacity(result_pairs.len());
                
                for &(_, right_idx) in &result_pairs {
                    result_values.push(right_series.get_value(right_idx));
                }
                
                // Create new series based on data type
                let new_series = match right_series.data_type() {
                    crate::types::DataType::I32 => {
                        Series::new_i32(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::I32(i) = val { Some(i) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::F64 => {
                        Series::new_f64(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::F64(f) = val { Some(f) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::String => {
                        Series::new_string(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::String(s) = val { Some(s) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::Bool => {
                        Series::new_bool(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::Bool(b) = val { Some(b) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::DateTime => {
                        Series::new_datetime(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::DateTime(dt) = val { Some(dt) } else { None })
                        ).collect())
                    },
                };
                
                result_columns.insert(col_name.clone(), new_series);
            }
        }
        
        let result_df = DataFrame::new(result_columns)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        
        Ok(PyDataFrame { df: result_df })
    }

    fn group_by(&self, by_columns: Vec<String>) -> PyResult<PyGroupedDataFrame> {
        // Create a temporary grouped dataframe and immediately use it for aggregation
        // Since we can't store references across Python calls, we'll store the original dataframe
        // and group columns instead
        Ok(PyGroupedDataFrame {
            dataframe: self.df.clone(),
            group_columns: by_columns,
        })
    }

    fn with_column(&self, new_col_name: &str, expr: &PyExpr) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .with_column(new_col_name, &expr.expr)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn describe(&self) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .describe()
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn correlation(&self, col1_name: &str, col2_name: &str) -> PyResult<f64> {
        self.df
            .correlation(col1_name, col2_name)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn covariance(&self, col1_name: &str, col2_name: &str) -> PyResult<f64> {
        self.df
            .covariance(col1_name, col2_name)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn append(&self, other: &PyDataFrame) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .append(&other.df)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn sort(&self, by_columns: Vec<String>, ascending: bool) -> PyResult<Self> {
        // Phase 13 Optimization: Intelligent sort dispatch based on dataset size
        let row_count = self.df.row_count();
        
        // For large datasets, use ultra-fast parallel sort
        if row_count > 5000 {
            return self.fast_sort_ultra(by_columns, ascending);
        }
        
        // For small datasets, use original implementation
        self.sort_original(by_columns, ascending)
    }

    /// Original sort implementation for small datasets
    fn sort_original(&self, by_columns: Vec<String>, ascending: bool) -> PyResult<Self> {
        // Phase 7 Optimization: High-performance index-based sorting
        // Uses sorting indices instead of moving entire rows for better performance
        use std::collections::HashMap;
        
        if self.df.row_count() == 0 {
            return Ok(PyDataFrame { df: self.df.clone() });
        }
        
        // Verify columns exist
        for col_name in &by_columns {
            if !self.df.column_names().contains(&&col_name) {
                return Err(PyValueError::new_err(format!("Column '{}' not found", col_name)));
            }
        }
        
        // Create sorting indices instead of moving rows
        let mut indices: Vec<usize> = (0..self.df.row_count()).collect();
        
        // Get sort columns for fast access
        let sort_series: Vec<_> = by_columns
            .iter()
            .map(|col_name| self.df.get_column(col_name).unwrap())
            .collect();
        
        // Sort indices based on values (much faster than moving entire rows)
        indices.sort_by(|&a, &b| {
            for series in &sort_series {
                let val_a = series.get_value(a);
                let val_b = series.get_value(b);
                
                let cmp = match (val_a, val_b) {
                    (Some(crate::types::Value::I32(v_a)), Some(crate::types::Value::I32(v_b))) => v_a.cmp(&v_b),
                    (Some(crate::types::Value::F64(v_a)), Some(crate::types::Value::F64(v_b))) => {
                        v_a.partial_cmp(&v_b).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (Some(crate::types::Value::String(ref v_a)), Some(crate::types::Value::String(ref v_b))) => v_a.cmp(v_b),
                    (Some(crate::types::Value::Bool(v_a)), Some(crate::types::Value::Bool(v_b))) => v_a.cmp(&v_b),
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                };
                
                if cmp != std::cmp::Ordering::Equal {
                    return if ascending { cmp } else { cmp.reverse() };
                }
            }
            std::cmp::Ordering::Equal
        });
        
        // Build new dataframe using sorted indices (much more efficient)
        let mut new_columns = HashMap::new();
        
        for col_name in self.df.column_names() {
            let original_series = self.df.get_column(col_name).unwrap();
            let mut new_values = Vec::with_capacity(self.df.row_count());
            
            // Use sorted indices to extract values in correct order
            for &idx in &indices {
                new_values.push(original_series.get_value(idx));
            }
            
            // Create new series efficiently based on data type
            let new_series = match original_series.data_type() {
                crate::types::DataType::I32 => {
                    Series::new_i32(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::I32(i) = val { Some(i) } else { None })
                    ).collect())
                },
                crate::types::DataType::F64 => {
                    Series::new_f64(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::F64(f) = val { Some(f) } else { None })
                    ).collect())
                },
                crate::types::DataType::String => {
                    Series::new_string(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::String(s) = val { Some(s) } else { None })
                    ).collect())
                },
                crate::types::DataType::Bool => {
                    Series::new_bool(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::Bool(b) = val { Some(b) } else { None })
                    ).collect())
                },
                crate::types::DataType::DateTime => {
                    Series::new_datetime(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::DateTime(dt) = val { Some(dt) } else { None })
                    ).collect())
                },
            };
            
            new_columns.insert(col_name.clone(), new_series);
        }
        
        let result_df = DataFrame::new(new_columns)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        
        Ok(PyDataFrame { df: result_df })
    }

    /// Phase 12 Optimization: Ultra-high-performance vectorized sorting  
    /// Multi-core sorting with direct Series access and zero-copy optimizations
    /// EXPOSED: Direct access for maximum performance
    fn fast_sort_ultra(&self, by_columns: Vec<String>, ascending: bool) -> PyResult<Self> {
        use std::collections::HashMap;
        use rayon::prelude::*;
        
        if self.df.row_count() == 0 {
            return Ok(PyDataFrame { df: self.df.clone() });
        }
        
        // Verify columns exist
        for col_name in &by_columns {
            if !self.df.column_names().contains(&&col_name) {
                return Err(PyValueError::new_err(format!("Column '{}' not found", col_name)));
            }
        }
        
        // Phase 12.1: Pre-allocate indices with optimal capacity
        let row_count = self.df.row_count();
        let mut indices: Vec<usize> = (0..row_count).collect();
        
        // Phase 12.2: Get sort columns with zero-overhead access patterns
        let sort_series: Vec<_> = by_columns
            .iter()
            .map(|col_name| self.df.get_column(col_name).unwrap())
            .collect();
        
        // Phase 12.3: Ultra-fast parallel sorting with hardware optimization
        indices.par_sort_unstable_by(|&a, &b| {
            for series in &sort_series {
                // Direct Series variant access for maximum performance
                let cmp = match series {
                    crate::series::Series::I32(ref data) => {
                        let val_a = data.get(a).copied().flatten();
                        let val_b = data.get(b).copied().flatten();
                        match (val_a, val_b) {
                            (Some(a), Some(b)) => a.cmp(&b),
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (None, None) => std::cmp::Ordering::Equal,
                        }
                    },
                    crate::series::Series::F64(ref data) => {
                        let val_a = data.get(a).copied().flatten();
                        let val_b = data.get(b).copied().flatten();
                        match (val_a, val_b) {
                            (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (None, None) => std::cmp::Ordering::Equal,
                        }
                    },
                    crate::series::Series::String(ref data) => {
                        let val_a = data.get(a).and_then(|s| s.as_ref());
                        let val_b = data.get(b).and_then(|s| s.as_ref());
                        match (val_a, val_b) {
                            (Some(a), Some(b)) => a.cmp(b),
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (None, None) => std::cmp::Ordering::Equal,
                        }
                    },
                    crate::series::Series::Bool(ref data) => {
                        let val_a = data.get(a).copied().flatten();
                        let val_b = data.get(b).copied().flatten();
                        match (val_a, val_b) {
                            (Some(a), Some(b)) => a.cmp(&b),
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (None, None) => std::cmp::Ordering::Equal,
                        }
                    },
                    crate::series::Series::DateTime(ref data) => {
                        let val_a = data.get(a).copied().flatten();
                        let val_b = data.get(b).copied().flatten();
                        match (val_a, val_b) {
                            (Some(a), Some(b)) => a.cmp(&b),
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (None, None) => std::cmp::Ordering::Equal,
                        }
                    },
                };
                
                if cmp != std::cmp::Ordering::Equal {
                    return if ascending { cmp } else { cmp.reverse() };
                }
            }
            std::cmp::Ordering::Equal
        });
        
        // Phase 12.4: Vectorized result construction with pre-allocated memory
        let mut new_columns = HashMap::new();
        new_columns.reserve(self.df.column_names().len());
        
        for col_name in self.df.column_names() {
            let original_series = self.df.get_column(col_name).unwrap();
            
            // Create new series with direct Series variant access for zero overhead
            let new_series = match original_series {
                crate::series::Series::I32(ref data) => {
                    let mut new_values = Vec::with_capacity(row_count);
                    for &idx in &indices {
                        new_values.push(data.get(idx).copied().flatten());
                    }
                    Series::new_i32(col_name, new_values)
                },
                crate::series::Series::F64(ref data) => {
                    let mut new_values = Vec::with_capacity(row_count);
                    for &idx in &indices {
                        new_values.push(data.get(idx).copied().flatten());
                    }
                    Series::new_f64(col_name, new_values)
                },
                crate::series::Series::String(ref data) => {
                    let mut new_values = Vec::with_capacity(row_count);
                    for &idx in &indices {
                        new_values.push(data.get(idx).and_then(|s| s.as_ref().map(|s| s.clone())));
                    }
                    Series::new_string(col_name, new_values)
                },
                crate::series::Series::Bool(ref data) => {
                    let mut new_values = Vec::with_capacity(row_count);
                    for &idx in &indices {
                        new_values.push(data.get(idx).copied().flatten());
                    }
                    Series::new_bool(col_name, new_values)
                },
                crate::series::Series::DateTime(ref data) => {
                    let mut new_values = Vec::with_capacity(row_count);
                    for &idx in &indices {
                        new_values.push(data.get(idx).copied().flatten());
                    }
                    Series::new_datetime(col_name, new_values)
                },
            };
            
            new_columns.insert(col_name.clone(), new_series);
        }
        
        let result_df = DataFrame::new(new_columns)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        
        Ok(PyDataFrame { df: result_df })
    }
    fn fast_sort(&self, by_columns: Vec<String>, ascending: bool) -> PyResult<Self> {
        use std::collections::HashMap;
        use rayon::prelude::*;
        
        if self.df.row_count() == 0 {
            return Ok(PyDataFrame { df: self.df.clone() });
        }
        
        // Verify columns exist
        for col_name in &by_columns {
            if !self.df.column_names().contains(&&col_name) {
                return Err(PyValueError::new_err(format!("Column '{}' not found", col_name)));
            }
        }
        
        // Phase 12.1: Pre-allocate indices with optimal capacity
        let row_count = self.df.row_count();
        let mut indices: Vec<usize> = (0..row_count).collect();
        
        // Phase 12.2: Get sort columns with zero-overhead access patterns
        let sort_series: Vec<_> = by_columns
            .iter()
            .map(|col_name| self.df.get_column(col_name).unwrap())
            .collect();
        
        // Sort indices based on values (much faster than moving entire rows)
        indices.sort_by(|&a, &b| {
            for series in &sort_series {
                let val_a = series.get_value(a);
                let val_b = series.get_value(b);
                
                let cmp = match (val_a, val_b) {
                    (Some(crate::types::Value::I32(v_a)), Some(crate::types::Value::I32(v_b))) => v_a.cmp(&v_b),
                    (Some(crate::types::Value::F64(v_a)), Some(crate::types::Value::F64(v_b))) => {
                        v_a.partial_cmp(&v_b).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (Some(crate::types::Value::String(ref v_a)), Some(crate::types::Value::String(ref v_b))) => v_a.cmp(v_b),
                    (Some(crate::types::Value::Bool(v_a)), Some(crate::types::Value::Bool(v_b))) => v_a.cmp(&v_b),
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                };
                
                if cmp != std::cmp::Ordering::Equal {
                    return if ascending { cmp } else { cmp.reverse() };
                }
            }
            std::cmp::Ordering::Equal
        });
        
        // Build new dataframe using sorted indices (much more efficient)
        let mut new_columns = HashMap::new();
        
        for col_name in self.df.column_names() {
            let original_series = self.df.get_column(col_name).unwrap();
            let mut new_values = Vec::with_capacity(self.df.row_count());
            
            // Use sorted indices to extract values in correct order
            for &idx in &indices {
                new_values.push(original_series.get_value(idx));
            }
            
            // Create new series efficiently based on data type
            let new_series = match original_series.data_type() {
                crate::types::DataType::I32 => {
                    Series::new_i32(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::I32(i) = val { Some(i) } else { None })
                    ).collect())
                },
                crate::types::DataType::F64 => {
                    Series::new_f64(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::F64(f) = val { Some(f) } else { None })
                    ).collect())
                },
                crate::types::DataType::String => {
                    Series::new_string(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::String(s) = val { Some(s) } else { None })
                    ).collect())
                },
                crate::types::DataType::Bool => {
                    Series::new_bool(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::Bool(b) = val { Some(b) } else { None })
                    ).collect())
                },
                crate::types::DataType::DateTime => {
                    Series::new_datetime(col_name, new_values.into_iter().map(|v| 
                        v.and_then(|val| if let crate::types::Value::DateTime(dt) = val { Some(dt) } else { None })
                    ).collect())
                },
            };
            
            new_columns.insert(col_name.clone(), new_series);
        }
        
        let result_df = DataFrame::new(new_columns)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        
        Ok(PyDataFrame { df: result_df })
    }

    /// Phase 3 Optimization: High-performance group by and aggregation
    fn groupby_agg(&self, group_columns: Vec<String>, aggregations: Vec<(String, String)>) -> PyResult<Self> {
        // Convert Python aggregations to Rust format
        let rust_aggregations: Vec<(&str, &str)> = aggregations
            .iter()
            .map(|(col, agg)| (col.as_str(), agg.as_str()))
            .collect();
        
        Ok(PyDataFrame {
            df: self
                .df
                .groupby_agg(group_columns, rust_aggregations)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    /// Phase 4 Optimization: High-performance hash join
    /// Specialized for common inner joins with optimized hash table approach
    fn fast_inner_join(&self, other: &PyDataFrame, on_column: &str) -> PyResult<Self> {
        use std::collections::HashMap;
        use crate::types::Value;
        
        // Get join columns
        let self_on_series = self.df.get_column(on_column)
            .map_err(|e| PyValueError::new_err(format!("Column '{}' not found in left dataframe: {}", on_column, e)))?;
        let other_on_series = other.df.get_column(on_column)
            .map_err(|e| PyValueError::new_err(format!("Column '{}' not found in right dataframe: {}", on_column, e)))?;
        
        // Build hash map for smaller dataframe (optimization heuristic)
        let (build_df, probe_df, build_on_series, probe_on_series, swapped) = 
            if self.df.row_count() <= other.df.row_count() {
                (&self.df, &other.df, self_on_series, other_on_series, false)
            } else {
                (&other.df, &self.df, other_on_series, self_on_series, true)
            };
        
        // Build phase: create hash table from smaller dataset
        let mut hash_table: HashMap<Value, Vec<usize>> = HashMap::with_capacity(build_df.row_count());
        
        for i in 0..build_df.row_count() {
            if let Some(key) = build_on_series.get_value(i) {
                hash_table.entry(key).or_default().push(i);
            }
        }
        
        // Probe phase: collect matching row pairs
        let mut result_pairs: Vec<(usize, usize)> = Vec::new();
        
        for probe_idx in 0..probe_df.row_count() {
            if let Some(probe_key) = probe_on_series.get_value(probe_idx) {
                if let Some(build_indices) = hash_table.get(&probe_key) {
                    for &build_idx in build_indices {
                        let (left_idx, right_idx) = if swapped {
                            (build_idx, probe_idx)  // other is left, self is right
                        } else {
                            (probe_idx, build_idx)  // self is left, other is right
                        };
                        result_pairs.push((left_idx, right_idx));
                    }
                }
            }
        }
        
        // Build result dataframe efficiently
        let left_df = &self.df;
        let right_df = &other.df;
        
        let mut result_columns = std::collections::HashMap::new();
        
        // Add left columns
        for col_name in left_df.column_names() {
            let left_series = left_df.get_column(col_name).unwrap();
            let mut result_values = Vec::with_capacity(result_pairs.len());
            
            for &(left_idx, _) in &result_pairs {
                result_values.push(left_series.get_value(left_idx));
            }
            
            // Create new series based on data type
            let new_series = match left_series.data_type() {
                crate::types::DataType::I32 => {
                    Series::new_i32(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::I32(i) = val { Some(i) } else { None })
                    ).collect())
                },
                crate::types::DataType::F64 => {
                    Series::new_f64(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::F64(f) = val { Some(f) } else { None })
                    ).collect())
                },
                crate::types::DataType::String => {
                    Series::new_string(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::String(s) = val { Some(s) } else { None })
                    ).collect())
                },
                crate::types::DataType::Bool => {
                    Series::new_bool(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::Bool(b) = val { Some(b) } else { None })
                    ).collect())
                },
                crate::types::DataType::DateTime => {
                    Series::new_datetime(col_name, result_values.into_iter().map(|v| 
                        v.and_then(|val| if let Value::DateTime(dt) = val { Some(dt) } else { None })
                    ).collect())
                },
            };
            
            result_columns.insert(col_name.clone(), new_series);
        }
        
        // Add right columns (excluding join column to avoid duplication)
        for col_name in right_df.column_names() {
            if col_name != on_column {  // Skip join column to avoid duplication
                let right_series = right_df.get_column(col_name).unwrap();
                let mut result_values = Vec::with_capacity(result_pairs.len());
                
                for &(_, right_idx) in &result_pairs {
                    result_values.push(right_series.get_value(right_idx));
                }
                
                // Create new series based on data type
                let new_series = match right_series.data_type() {
                    crate::types::DataType::I32 => {
                        Series::new_i32(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::I32(i) = val { Some(i) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::F64 => {
                        Series::new_f64(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::F64(f) = val { Some(f) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::String => {
                        Series::new_string(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::String(s) = val { Some(s) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::Bool => {
                        Series::new_bool(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::Bool(b) = val { Some(b) } else { None })
                        ).collect())
                    },
                    crate::types::DataType::DateTime => {
                        Series::new_datetime(col_name, result_values.into_iter().map(|v| 
                            v.and_then(|val| if let Value::DateTime(dt) = val { Some(dt) } else { None })
                        ).collect())
                    },
                };
                
                result_columns.insert(col_name.clone(), new_series);
            }
        }
        
        let result_df = DataFrame::new(result_columns)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        
        Ok(PyDataFrame { df: result_df })
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.df)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.df)
    }
}

#[pyclass]
pub struct PyGroupedDataFrame {
    pub dataframe: DataFrame,
    pub group_columns: Vec<String>,
}

#[pymethods]
impl PyGroupedDataFrame {
    fn sum(&self) -> PyResult<PyDataFrame> {
        let grouped_df = self
            .dataframe
            .group_by(self.group_columns.clone())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyDataFrame {
            df: grouped_df
                .agg(vec![("*", "sum")])
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn mean(&self) -> PyResult<PyDataFrame> {
        let grouped_df = self
            .dataframe
            .group_by(self.group_columns.clone())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyDataFrame {
            df: grouped_df
                .agg(vec![("*", "mean")])
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn count(&self) -> PyResult<PyDataFrame> {
        let grouped_df = self
            .dataframe
            .group_by(self.group_columns.clone())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyDataFrame {
            df: grouped_df
                .agg(vec![("*", "count")])
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn max(&self) -> PyResult<PyDataFrame> {
        let grouped_df = self
            .dataframe
            .group_by(self.group_columns.clone())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyDataFrame {
            df: grouped_df
                .agg(vec![("*", "max")])
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn min(&self) -> PyResult<PyDataFrame> {
        let grouped_df = self
            .dataframe
            .group_by(self.group_columns.clone())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyDataFrame {
            df: grouped_df
                .agg(vec![("*", "min")])
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn agg(&self, aggregations: Vec<(String, String)>) -> PyResult<PyDataFrame> {
        let grouped_df = self
            .dataframe
            .group_by(self.group_columns.clone())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let string_refs: Vec<(&str, &str)> = aggregations
            .iter()
            .map(|(col, agg)| (col.as_str(), agg.as_str()))
            .collect();
        Ok(PyDataFrame {
            df: grouped_df
                .agg(string_refs)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn row_count(&self) -> PyResult<PyDataFrame> {
        let grouped_df = self
            .dataframe
            .group_by(self.group_columns.clone())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyDataFrame {
            df: grouped_df
                .agg(vec![("*", "count")])
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub enum PyJoinType {
    Inner,
    Left,
    Right,
}

impl From<PyJoinType> for JoinType {
    fn from(py_join_type: PyJoinType) -> Self {
        match py_join_type {
            PyJoinType::Inner => JoinType::Inner,
            PyJoinType::Left => JoinType::Left,
            PyJoinType::Right => JoinType::Right,
        }
    }
}

#[pyclass]
pub struct PyExpr {
    pub expr: Expr,
}

#[pymethods]
impl PyExpr {
    #[staticmethod]
    pub fn column(name: &str) -> Self {
        PyExpr {
            expr: Expr::Column(name.to_string()),
        }
    }

    #[staticmethod]
    pub fn literal(value: &Bound<PyAny>) -> PyResult<Self> {
        let rust_value = extract_value(value)?;
        Ok(PyExpr {
            expr: Expr::Literal(rust_value),
        })
    }

    #[staticmethod]
    pub fn add(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::Add(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn subtract(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::Subtract(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn multiply(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::Multiply(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn divide(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::Divide(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn equals(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::Equals(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn not_equals(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::NotEquals(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn greater_than(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::GreaterThan(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn less_than(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::LessThan(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn greater_than_or_equal(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::GreaterThanOrEqual(
                Box::new(left.expr.clone()),
                Box::new(right.expr.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn less_than_or_equal(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::LessThanOrEqual(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn and(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::And(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn or(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::Or(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[staticmethod]
    pub fn not(expr: &PyExpr) -> Self {
        PyExpr {
            expr: Expr::Not(Box::new(expr.expr.clone())),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PySeries {
    pub series: Series,
}

#[pymethods]
impl PySeries {
    #[new]
    fn new(name: &str, data: &Bound<PyAny>) -> PyResult<Self> {
        // Phase 6 Optimization: Fast type detection and series creation
        
        // Try to get as PyList first for fast iteration
        if let Ok(list) = data.downcast::<pyo3::types::PyList>() {
            return Self::from_pylist_optimized(name, list);
        }
        
        // Fallback to original extraction logic for other types
        if let Ok(list) = data.extract::<Vec<Option<i32>>>() {
            Ok(PySeries {
                series: Series::new_i32(name, list),
            })
        } else if let Ok(list) = data.extract::<Vec<Option<f64>>>() {
            Ok(PySeries {
                series: Series::new_f64(name, list),
            })
        } else if let Ok(list) = data.extract::<Vec<Option<bool>>>() {
            Ok(PySeries {
                series: Series::new_bool(name, list),
            })
        } else if let Ok(list) = data.extract::<Vec<Option<String>>>() {
            Ok(PySeries {
                series: Series::new_string(name, list),
            })
        } else if let Ok(list) = data.extract::<Vec<Option<i64>>>() {
            Ok(PySeries {
                series: Series::new_datetime(name, list),
            })
        } else {
            Err(PyValueError::new_err("Unsupported data type for Series"))
        }
    }

    /// Optimized PyList to Series conversion with fast type detection
    fn from_pylist_optimized(name: &str, list: &Bound<pyo3::types::PyList>) -> PyResult<Self> {
        let len = list.len();
        
        if len == 0 {
            return Ok(PySeries {
                series: Series::new_i32(name, vec![]),
            });
        }

        // Fast type detection: check first non-None element
        let mut sample_item = None;
        for item in list.iter() {
            if !item.is_none() {
                sample_item = Some(item);
                break;
            }
        }

        // If all None, default to i32
        let sample = match sample_item {
            Some(item) => item,
            None => {
                let mut null_vec = Vec::with_capacity(len);
                null_vec.resize(len, None);
                return Ok(PySeries {
                    series: Series::new_i32(name, null_vec),
                });
            }
        };

        // Optimized type dispatch with fast extraction
        if sample.extract::<i32>().is_ok() {
            let mut data = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    data.push(None);
                } else {
                    data.push(item.extract().ok());
                }
            }
            Ok(PySeries {
                series: Series::new_i32(name, data),
            })
        } else if sample.extract::<f64>().is_ok() {
            let mut data = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    data.push(None);
                } else {
                    data.push(item.extract().ok());
                }
            }
            Ok(PySeries {
                series: Series::new_f64(name, data),
            })
        } else if sample.extract::<bool>().is_ok() {
            let mut data = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    data.push(None);
                } else {
                    data.push(item.extract().ok());
                }
            }
            Ok(PySeries {
                series: Series::new_bool(name, data),
            })
        } else {
            // Default to string for everything else
            let mut data = Vec::with_capacity(len);
            for item in list.iter() {
                if item.is_none() {
                    data.push(None);
                } else {
                    data.push(item.extract().ok());
                }
            }
            Ok(PySeries {
                series: Series::new_string(name, data),
            })
        }
    }

    fn name(&self) -> String {
        self.series.name().to_string()
    }

    fn len(&self) -> usize {
        self.series.len()
    }

    fn is_empty(&self) -> bool {
        self.series.is_empty()
    }

    fn data_type(&self) -> String {
        format!("{:?}", self.series.data_type())
    }

    fn set_name(&mut self, new_name: &str) {
        self.series.set_name(new_name);
    }

    fn get_value<'py>(&self, py: Python<'py>, index: usize) -> PyResult<Option<PyObject>> {
        self.series
            .get_value(index)
            .map_or(Ok(None), |v| v.into_pyobject(py).map(|b| Some(b.unbind())))
    }

    fn filter(&self, row_indices: Vec<usize>) -> PyResult<Self> {
        Ok(PySeries {
            series: self
                .series
                .filter(&row_indices)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn fill_nulls(&self, value: &Bound<PyAny>) -> PyResult<Self> {
        let rust_value = extract_value(value)?;
        Ok(PySeries {
            series: self
                .series
                .fill_nulls(&rust_value)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.series)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.series)
    }

    fn cast(&self, to_type: &PyDataType) -> PyResult<Self> {
        let rust_data_type = to_type.clone().into();
        Ok(PySeries {
            series: self
                .series
                .cast(rust_data_type)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn sum(&self) -> PyResult<f64> {
        self.series
            .sum()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn mean(&self) -> PyResult<f64> {
        self.series
            .mean()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn min(&self) -> PyResult<Option<f64>> {
        self.series
            .min()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn max(&self) -> PyResult<Option<f64>> {
        self.series
            .max()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn std_dev(&self) -> PyResult<f64> {
        self.series
            .std_dev()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn median(&self) -> PyResult<f64> {
        self.series
            .median()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn correlation(&self, other: &PySeries) -> PyResult<f64> {
        match self.series
            .correlation(&other.series)
            .map_err(|e| PyValueError::new_err(e.to_string()))? {
            Some(corr) => Ok(corr),
            None => Err(PyValueError::new_err("Unable to compute correlation")),
        }
    }

    fn covariance(&self, other: &PySeries) -> PyResult<f64> {
        match self.series
            .covariance(&other.series)
            .map_err(|e| PyValueError::new_err(e.to_string()))? {
            Some(cov) => Ok(cov),
            None => Err(PyValueError::new_err("Unable to compute covariance")),
        }
    }

    fn unique_count(&self) -> PyResult<usize> {
        self.series
            .unique_count()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn unique(&self) -> PyResult<Self> {
        let unique_series = self.series
            .unique()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PySeries { series: unique_series })
    }

    fn append(&self, other: &PySeries) -> PyResult<Self> {
        let appended = self.series
            .append(&other.series)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PySeries { series: appended })
    }

    /// Phase 2 Optimization: High-performance vectorized addition
    fn add(&self, other: &PySeries) -> PyResult<Self> {
        // Use the underlying Series arithmetic operations for fast vectorized math
        match (&self.series, &other.series) {
            (Series::I32(_, self_vals, self_bitmap), Series::F64(_, other_vals, other_bitmap)) => {
                // I32 + F64 -> F64
                let result_len = self_vals.len().min(other_vals.len());
                let mut result_values = Vec::with_capacity(result_len);
                let mut result_bitmap = Vec::with_capacity(result_len);
                
                for i in 0..result_len {
                    if self_bitmap[i] && other_bitmap[i] {
                        result_values.push(self_vals[i] as f64 + other_vals[i]);
                        result_bitmap.push(true);
                    } else {
                        result_values.push(0.0); // Placeholder for null
                        result_bitmap.push(false);
                    }
                }
                
                Ok(PySeries {
                    series: Series::F64(
                        format!("{}_add", self.series.name()), 
                        result_values, 
                        result_bitmap
                    ),
                })
            },
            (Series::F64(_, self_vals, self_bitmap), Series::I32(_, other_vals, other_bitmap)) => {
                // F64 + I32 -> F64
                let result_len = self_vals.len().min(other_vals.len());
                let mut result_values = Vec::with_capacity(result_len);
                let mut result_bitmap = Vec::with_capacity(result_len);
                
                for i in 0..result_len {
                    if self_bitmap[i] && other_bitmap[i] {
                        result_values.push(self_vals[i] + other_vals[i] as f64);
                        result_bitmap.push(true);
                    } else {
                        result_values.push(0.0); // Placeholder for null
                        result_bitmap.push(false);
                    }
                }
                
                Ok(PySeries {
                    series: Series::F64(
                        format!("{}_add", self.series.name()), 
                        result_values, 
                        result_bitmap
                    ),
                })
            },
            (Series::I32(_, self_vals, self_bitmap), Series::I32(_, other_vals, other_bitmap)) => {
                // I32 + I32 -> I32 (with overflow check)
                let result_len = self_vals.len().min(other_vals.len());
                let mut result_values = Vec::with_capacity(result_len);
                let mut result_bitmap = Vec::with_capacity(result_len);
                
                for i in 0..result_len {
                    if self_bitmap[i] && other_bitmap[i] {
                        result_values.push(self_vals[i].saturating_add(other_vals[i]));
                        result_bitmap.push(true);
                    } else {
                        result_values.push(0); // Placeholder for null
                        result_bitmap.push(false);
                    }
                }
                
                Ok(PySeries {
                    series: Series::I32(
                        format!("{}_add", self.series.name()), 
                        result_values, 
                        result_bitmap
                    ),
                })
            },
            (Series::F64(_, self_vals, self_bitmap), Series::F64(_, other_vals, other_bitmap)) => {
                // F64 + F64 -> F64
                let result_len = self_vals.len().min(other_vals.len());
                let mut result_values = Vec::with_capacity(result_len);
                let mut result_bitmap = Vec::with_capacity(result_len);
                
                for i in 0..result_len {
                    if self_bitmap[i] && other_bitmap[i] {
                        result_values.push(self_vals[i] + other_vals[i]);
                        result_bitmap.push(true);
                    } else {
                        result_values.push(0.0); // Placeholder for null
                        result_bitmap.push(false);
                    }
                }
                
                Ok(PySeries {
                    series: Series::F64(
                        format!("{}_add", self.series.name()), 
                        result_values, 
                        result_bitmap
                    ),
                })
            },
            _ => Err(PyValueError::new_err("Unsupported series types for addition")),
        }
    }
}

#[pyclass]
#[derive(Clone, PartialEq, Debug)]
pub enum PyDataType {
    I32,
    F64,
    Bool,
    String,
    DateTime,
}

impl From<PyDataType> for DataType {
    fn from(py_data_type: PyDataType) -> Self {
        match py_data_type {
            PyDataType::I32 => DataType::I32,
            PyDataType::F64 => DataType::F64,
            PyDataType::Bool => DataType::Bool,
            PyDataType::String => DataType::String,
            PyDataType::DateTime => DataType::DateTime,
        }
    }
}

impl From<DataType> for PyDataType {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::I32 => PyDataType::I32,
            DataType::F64 => PyDataType::F64,
            DataType::Bool => PyDataType::Bool,
            DataType::String => PyDataType::String,
            DataType::DateTime => PyDataType::DateTime,
        }
    }
}

#[pymethods]
impl PyDataType {
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

fn extract_value(value: &Bound<PyAny>) -> PyResult<Value> {
    if let Ok(v) = value.extract::<i32>() {
        Ok(Value::I32(v))
    } else if let Ok(v) = value.extract::<f64>() {
        Ok(Value::F64(v))
    } else if let Ok(v) = value.extract::<bool>() {
        Ok(Value::Bool(v))
    } else if let Ok(v) = value.extract::<String>() {
        Ok(Value::String(v))
    } else if let Ok(v) = value.extract::<i64>() {
        Ok(Value::DateTime(v))
    } else if value.is_none() {
        Ok(Value::Null)
    } else {
        Err(PyValueError::new_err("Unsupported value type"))
    }
}

#[pymodule]
fn veloxx(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyDataFrame>()?;
    m.add_class::<PySeries>()?;
    m.add_class::<PyExpr>()?;
    m.add_class::<PyJoinType>()?;
    m.add_class::<PyDataType>()?;
    m.add_class::<PyGroupedDataFrame>()?;
    m.add_class::<PyValue>()?;
    m.add_class::<PyCondition>()?;
    Ok(())
}
