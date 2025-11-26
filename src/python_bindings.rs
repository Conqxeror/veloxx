//! Python bindings for Veloxx
//!
//! This module provides Python bindings for all Veloxx functionality,
//! including high-performance SIMD operations and vectorized filtering.

#[cfg(feature = "python")]
use pyo3::prelude::*;

use pyo3::prelude::Bound;
use pyo3::types::{PyDict, PyModule};
#[cfg(feature = "python")]
use pyo3::{pyclass, pymethods, pymodule, wrap_pyfunction, PyErr, PyObject, PyResult, Python};

// /// Python wrapper for GroupedDataFrame operations
// #[cfg(feature = "python")]
// #[pyclass]
// pub struct PyGroupedDataFrame<'a> {
//     pub(crate) inner: GroupedDataFrame<'a>,
// }

#[cfg(feature = "python")]
use crate::{
    conditions::Condition, dataframe::DataFrame, performance::optimized_simd::OptimizedSimdOps,
    performance::ultra_fast_join::UltraFastJoin, series::Series, types::Value,
};

#[cfg(feature = "python")]
use indexmap::IndexMap;

/// Python wrapper for DataType enum
#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub enum PyDataType {
    I32,
    F64,
    String,
    Bool,
    DateTime,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDataType {
    fn __str__(&self) -> String {
        match self {
            PyDataType::I32 => "I32".to_string(),
            PyDataType::F64 => "F64".to_string(),
            PyDataType::String => "String".to_string(),
            PyDataType::Bool => "Bool".to_string(),
            PyDataType::DateTime => "DateTime".to_string(),
        }
    }
}

/// Python wrapper for join types
#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub enum PyJoinType {
    Inner,
    Left,
    Right,
    Outer,
}

/// Python wrapper for conditions
#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyCondition {
    pub(crate) inner: Condition,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyCondition {
    #[staticmethod]
    pub fn gt(column: String, value: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            let val = if let Ok(py_value) = value.extract::<PyValue>(py) {
                py_value.inner
            } else if let Ok(v) = value.extract::<i32>(py) {
                Value::I32(v)
            } else if let Ok(v) = value.extract::<f64>(py) {
                Value::F64(v)
            } else if let Ok(v) = value.extract::<String>(py) {
                Value::String(v)
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unsupported value type for condition",
                ));
            };

            Ok(PyCondition {
                inner: Condition::Gt(column, val),
            })
        })
    }

    #[staticmethod]
    pub fn lt(column: String, value: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            let val = if let Ok(py_value) = value.extract::<PyValue>(py) {
                py_value.inner
            } else if let Ok(v) = value.extract::<i32>(py) {
                Value::I32(v)
            } else if let Ok(v) = value.extract::<f64>(py) {
                Value::F64(v)
            } else if let Ok(v) = value.extract::<String>(py) {
                Value::String(v)
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unsupported value type for condition",
                ));
            };

            Ok(PyCondition {
                inner: Condition::Lt(column, val),
            })
        })
    }

    #[staticmethod]
    pub fn eq(column: String, value: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            let val = if let Ok(py_value) = value.extract::<PyValue>(py) {
                py_value.inner
            } else if let Ok(v) = value.extract::<i32>(py) {
                Value::I32(v)
            } else if let Ok(v) = value.extract::<f64>(py) {
                Value::F64(v)
            } else if let Ok(v) = value.extract::<String>(py) {
                Value::String(v)
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unsupported value type for condition",
                ));
            };

            Ok(PyCondition {
                inner: Condition::Eq(column, val),
            })
        })
    }
}

/// Python wrapper for expressions
#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyExpr {
    pub(crate) inner: crate::expressions::Expr,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyExpr {
    #[staticmethod]
    pub fn column(name: String) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::Column(name),
        }
    }

    #[staticmethod]
    pub fn literal(value: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            let val = if let Ok(pyvalue) = value.extract::<PyValue>(py) {
                pyvalue.inner
            } else if let Ok(v) = value.extract::<i32>(py) {
                Value::I32(v)
            } else if let Ok(v) = value.extract::<f64>(py) {
                Value::F64(v)
            } else if let Ok(v) = value.extract::<String>(py) {
                Value::String(v)
            } else if let Ok(v) = value.extract::<bool>(py) {
                Value::Bool(v)
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unsupported literal value type",
                ));
            };

            Ok(PyExpr {
                inner: crate::expressions::Expr::Literal(val),
            })
        })
    }

    pub fn add(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::Add(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }

    pub fn subtract(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::Subtract(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }

    pub fn multiply(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::Multiply(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }

    pub fn divide(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::Divide(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn greater_than(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::GreaterThan(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn less_than(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::LessThan(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn equal(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::Equals(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn equals(left: &PyExpr, right: &PyExpr) -> Self {
        // Alias for equal to match test expectations
        Self::equal(left, right)
    }

    #[staticmethod]
    pub fn not_equals(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::NotEquals(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn and(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::And(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn or(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::Or(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn not(expr: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::Not(Box::new(expr.inner.clone())),
        }
    }

    /// Instance method for greater than comparison
    pub fn gt(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::GreaterThan(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }

    /// Instance method for less than comparison
    pub fn lt(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::LessThan(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }

    /// Instance method for greater than or equal comparison
    pub fn gte(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::GreaterThanOrEqual(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }

    /// Instance method for less than or equal comparison
    pub fn lte(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::LessThanOrEqual(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }

    /// Instance method for equality comparison
    pub fn eq(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::Equals(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }

    /// Instance method for not equal comparison
    pub fn ne(&self, other: &PyExpr) -> Self {
        PyExpr {
            inner: crate::expressions::Expr::NotEquals(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone()),
            ),
        }
    }
}

/// Python wrapper for Value
#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyValue {
    pub(crate) inner: Value,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyValue {
    #[new]
    pub fn new(value: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            let val = if let Ok(v) = value.extract::<i32>(py) {
                Value::I32(v)
            } else if let Ok(v) = value.extract::<f64>(py) {
                Value::F64(v)
            } else if let Ok(v) = value.extract::<String>(py) {
                Value::String(v)
            } else if let Ok(v) = value.extract::<bool>(py) {
                Value::Bool(v)
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unsupported value type",
                ));
            };

            Ok(PyValue { inner: val })
        })
    }

    #[staticmethod]
    pub fn from_i32(value: i32) -> Self {
        PyValue {
            inner: Value::I32(value),
        }
    }

    #[staticmethod]
    pub fn from_f64(value: f64) -> Self {
        PyValue {
            inner: Value::F64(value),
        }
    }

    #[staticmethod]
    pub fn from_string(value: String) -> Self {
        PyValue {
            inner: Value::String(value),
        }
    }

    #[staticmethod]
    pub fn from_bool(value: bool) -> Self {
        PyValue {
            inner: Value::Bool(value),
        }
    }

    #[staticmethod]
    pub fn null() -> Self {
        PyValue { inner: Value::Null }
    }

    /// Get the string representation of the value
    pub fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    /// Get the value type as string
    pub fn get_type(&self) -> String {
        match &self.inner {
            Value::I32(_) => "i32".to_string(),
            Value::F64(_) => "f64".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::DateTime(_) => "datetime".to_string(),
            Value::Null => "null".to_string(),
        }
    }
}

/// Python wrapper for GroupedDataFrame
#[cfg(feature = "python")]
/// Python wrapper for GroupedDataFrame operations
#[cfg(feature = "python")]
#[pyclass]
pub struct PyGroupedDataFrame {
    pub(crate) dataframe: PyDataFrame, // Own the dataframe
    pub(crate) group_columns: Vec<String>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyGroupedDataFrame {
    /// Aggregate operations
    pub fn agg(&self, aggregations: Vec<(String, String)>) -> PyResult<PyDataFrame> {
        // Convert to &str tuples for the core API
        let string_refs: Vec<(&str, &str)> = aggregations
            .iter()
            .map(|(c, a)| (c.as_str(), a.as_str()))
            .collect();

        match self.dataframe.inner.group_by(self.group_columns.clone()) {
            Ok(grouped) => match grouped.agg(string_refs) {
                Ok(result) => Ok(PyDataFrame { inner: result }),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            },
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Sum aggregation
    pub fn sum(&self) -> PyResult<PyDataFrame> {
        // Get all numeric columns for sum aggregation
        let column_names = self.dataframe.inner.column_names();
        // Build owned aggregation strings so references remain valid
        let mut sum_aggs_owned: Vec<(String, String)> = Vec::new();

        for col_name in column_names {
            if col_name.as_str() != self.group_columns[0].as_str() {
                // Skip group column
                sum_aggs_owned.push((col_name, "sum".to_string()));
            }
        }

        let sum_refs: Vec<(&str, &str)> = sum_aggs_owned
            .iter()
            .map(|(c, a)| (c.as_str(), a.as_str()))
            .collect();

        match self.dataframe.inner.group_by(self.group_columns.clone()) {
            Ok(grouped) => match grouped.agg(sum_refs) {
                Ok(result) => Ok(PyDataFrame { inner: result }),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            },
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Mean aggregation  
    pub fn mean(&self) -> PyResult<PyDataFrame> {
        let column_names = self.dataframe.inner.column_names();
        let mut mean_aggs_owned: Vec<(String, String)> = Vec::new();

        for col_name in column_names {
            if col_name.as_str() != self.group_columns[0].as_str() {
                mean_aggs_owned.push((col_name, "mean".to_string()));
            }
        }

        let mean_refs: Vec<(&str, &str)> = mean_aggs_owned
            .iter()
            .map(|(c, a)| (c.as_str(), a.as_str()))
            .collect();

        match self.dataframe.inner.group_by(self.group_columns.clone()) {
            Ok(grouped) => match grouped.agg(mean_refs) {
                Ok(result) => Ok(PyDataFrame { inner: result }),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            },
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Count aggregation
    pub fn count(&self) -> PyResult<PyDataFrame> {
        let column_names = self.dataframe.inner.column_names();
        let mut count_aggs_owned: Vec<(String, String)> = Vec::new();

        for col_name in column_names {
            if col_name.as_str() != self.group_columns[0].as_str() {
                count_aggs_owned.push((col_name, "count".to_string()));
            }
        }

        let count_refs: Vec<(&str, &str)> = count_aggs_owned
            .iter()
            .map(|(c, a)| (c.as_str(), a.as_str()))
            .collect();

        match self.dataframe.inner.group_by(self.group_columns.clone()) {
            Ok(grouped) => match grouped.agg(count_refs) {
                Ok(result) => Ok(PyDataFrame { inner: result }),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            },
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Min aggregation
    pub fn min(&self) -> PyResult<PyDataFrame> {
        let column_names = self.dataframe.inner.column_names();
        let mut min_aggs_owned: Vec<(String, String)> = Vec::new();

        for col_name in column_names {
            if col_name.as_str() != self.group_columns[0].as_str() {
                min_aggs_owned.push((col_name, "min".to_string()));
            }
        }

        let min_refs: Vec<(&str, &str)> = min_aggs_owned
            .iter()
            .map(|(c, a)| (c.as_str(), a.as_str()))
            .collect();

        match self.dataframe.inner.group_by(self.group_columns.clone()) {
            Ok(grouped) => match grouped.agg(min_refs) {
                Ok(result) => Ok(PyDataFrame { inner: result }),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            },
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Max aggregation
    pub fn max(&self) -> PyResult<PyDataFrame> {
        let column_names = self.dataframe.inner.column_names();
        let mut max_aggs_owned: Vec<(String, String)> = Vec::new();

        for col_name in column_names {
            if col_name.as_str() != self.group_columns[0].as_str() {
                max_aggs_owned.push((col_name, "max".to_string()));
            }
        }

        let max_refs: Vec<(&str, &str)> = max_aggs_owned
            .iter()
            .map(|(c, a)| (c.as_str(), a.as_str()))
            .collect();

        match self.dataframe.inner.group_by(self.group_columns.clone()) {
            Ok(grouped) => match grouped.agg(max_refs) {
                Ok(result) => Ok(PyDataFrame { inner: result }),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            },
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }
}

/// Python wrapper for Series with high-performance operations
#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PySeries {
    pub(crate) inner: Series,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySeries {
    #[new]
    pub fn new(name: String, data: Vec<Option<PyObject>>) -> PyResult<Self> {
        Python::with_gil(|py| {
            if data.is_empty() {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Cannot create Series from empty data",
                ));
            }

            // Infer type from first non-null value
            let first_valid = data.iter().find(|x| x.is_some()).and_then(|x| x.as_ref());

            let series = match first_valid {
                Some(obj) if obj.extract::<i32>(py).is_ok() => {
                    let values: Vec<Option<i32>> = data
                        .into_iter()
                        .map(|x| x.and_then(|obj| obj.extract::<i32>(py).ok()))
                        .collect();
                    Series::new_i32(&name, values)
                }
                Some(obj) if obj.extract::<f64>(py).is_ok() => {
                    let values: Vec<Option<f64>> = data
                        .into_iter()
                        .map(|x| x.and_then(|obj| obj.extract::<f64>(py).ok()))
                        .collect();
                    Series::new_f64(&name, values)
                }
                Some(obj) if obj.extract::<String>(py).is_ok() => {
                    let values: Vec<Option<String>> = data
                        .into_iter()
                        .map(|x| x.and_then(|obj| obj.extract::<String>(py).ok()))
                        .collect();
                    Series::new_string(&name, values)
                }
                Some(obj) if obj.extract::<bool>(py).is_ok() => {
                    let values: Vec<Option<bool>> = data
                        .into_iter()
                        .map(|x| x.and_then(|obj| obj.extract::<bool>(py).ok()))
                        .collect();
                    Series::new_bool(&name, values)
                }
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "Unsupported data type or all values are None",
                    ));
                }
            };

            Ok(PySeries { inner: series })
        })
    }

    /// Create a new PySeries (static method for compatibility)
    #[staticmethod]
    pub fn new_static(name: String, data: Vec<Option<PyObject>>) -> PyResult<Self> {
        Self::new(name, data)
    }

    /// Get the name of the series
    pub fn name(&self) -> String {
        self.inner.name().to_string()
    }

    /// Set the name of the series
    pub fn set_name(&mut self, name: String) {
        self.inner.set_name(&name);
    }

    /// Get the length of the series
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the series is empty
    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    /// Get the data type as a string
    pub fn data_type(&self) -> String {
        match &self.inner {
            Series::I32(_, _, _) => "I32".to_string(),
            Series::F64(_, _, _) => "F64".to_string(),
            Series::String(_, _, _) => "String".to_string(),
            Series::Bool(_, _, _) => "Bool".to_string(),
            Series::DateTime(_, _, _) => "DateTime".to_string(),
        }
    }

    /// Get a value at the specified index
    #[allow(deprecated)]
    pub fn get_value(&self, index: usize) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| match self.inner.get_value(index) {
            Some(Value::I32(v)) => Ok(Some(v.into_py(py))),
            Some(Value::F64(v)) => Ok(Some(v.into_py(py))),
            Some(Value::String(v)) => Ok(Some(v.into_py(py))),
            Some(Value::Bool(v)) => Ok(Some(v.into_py(py))),
            Some(Value::DateTime(v)) => Ok(Some(v.into_py(py))),
            Some(Value::Null) => Ok(None),
            None => Ok(None),
        })
    }

    /// Filter the series by indices (high-performance)
    pub fn filter(&self, indices: Vec<usize>) -> PyResult<Self> {
        match self.inner.filter(&indices) {
            Ok(filtered) => Ok(PySeries { inner: filtered }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Count non-null values
    pub fn count(&self) -> usize {
        self.inner.count()
    }

    /// Compute sum using SIMD optimization
    #[allow(deprecated)]
    pub fn sum(&self) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| match self.inner.sum() {
            Ok(Value::I32(v)) => Ok(Some(v.into_py(py))),
            Ok(Value::F64(v)) => Ok(Some(v.into_py(py))),
            Ok(Value::Null) => Ok(None),
            Ok(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Sum not supported for this data type",
            )),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        })
    }

    /// Add two series using SIMD optimization
    pub fn add(&self, other: &PySeries) -> PyResult<Self> {
        match self.inner.add(&other.inner) {
            Ok(result) => Ok(PySeries { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Multiply two series using SIMD optimization
    pub fn multiply(&self, other: &PySeries) -> PyResult<Self> {
        match self.inner.multiply(&other.inner) {
            Ok(result) => Ok(PySeries { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Get mean using optimized computation
    pub fn mean(&self) -> PyResult<Option<f64>> {
        match self.inner.mean() {
            Ok(Value::F64(v)) => Ok(Some(v)),
            Ok(Value::I32(v)) => Ok(Some(v as f64)),
            Ok(Value::Null) => Ok(None),
            Ok(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Mean not supported for this data type",
            )),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Get median value
    pub fn median(&self) -> PyResult<Option<f64>> {
        match self.inner.median() {
            Ok(Value::F64(v)) => Ok(Some(v)),
            Ok(Value::I32(v)) => Ok(Some(v as f64)),
            Ok(Value::Null) => Ok(None),
            Ok(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Median not supported for this data type",
            )),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Get minimum value
    #[allow(deprecated)]
    pub fn min(&self) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| match self.inner.min() {
            Ok(Value::I32(v)) => Ok(Some(v.into_py(py))),
            Ok(Value::F64(v)) => Ok(Some(v.into_py(py))),
            Ok(Value::String(v)) => Ok(Some(v.into_py(py))),
            Ok(Value::Null) => Ok(None),
            Ok(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Min not supported for this data type",
            )),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        })
    }

    /// Get maximum value
    #[allow(deprecated)]
    pub fn max(&self) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| match self.inner.max() {
            Ok(Value::I32(v)) => Ok(Some(v.into_py(py))),
            Ok(Value::F64(v)) => Ok(Some(v.into_py(py))),
            Ok(Value::String(v)) => Ok(Some(v.into_py(py))),
            Ok(Value::Null) => Ok(None),
            Ok(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Max not supported for this data type",
            )),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        })
    }

    /// Get standard deviation
    pub fn std_dev(&self) -> PyResult<Option<f64>> {
        match self.inner.std_dev() {
            Ok(Value::F64(v)) => Ok(Some(v)),
            Ok(Value::I32(v)) => Ok(Some(v as f64)),
            Ok(Value::Null) => Ok(None),
            Ok(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Standard deviation not supported for this data type",
            )),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Fill null values
    pub fn fill_nulls(&self, value: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            let fill_value = if let Ok(v) = value.extract::<i32>(py) {
                Value::I32(v)
            } else if let Ok(v) = value.extract::<f64>(py) {
                Value::F64(v)
            } else if let Ok(v) = value.extract::<String>(py) {
                Value::String(v)
            } else if let Ok(v) = value.extract::<bool>(py) {
                Value::Bool(v)
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unsupported fill value type",
                ));
            };

            match self.inner.fill_nulls(&fill_value) {
                Ok(result) => Ok(PySeries { inner: result }),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            }
        })
    }

    /// Interpolate null values
    pub fn interpolate_nulls(&self) -> PyResult<Self> {
        match self.inner.interpolate_nulls() {
            Ok(result) => Ok(PySeries { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Get unique values
    pub fn unique(&self) -> PyResult<Self> {
        match self.inner.unique() {
            Ok(result) => Ok(PySeries { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Append another series
    pub fn append(&self, other: &PySeries) -> PyResult<Self> {
        match self.inner.append(&other.inner) {
            Ok(result) => Ok(PySeries { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Cast to different data type
    pub fn cast(&self, target_type: PyDataType) -> PyResult<Self> {
        let data_type = match target_type {
            PyDataType::I32 => crate::types::DataType::I32,
            PyDataType::F64 => crate::types::DataType::F64,
            PyDataType::String => crate::types::DataType::String,
            PyDataType::Bool => crate::types::DataType::Bool,
            PyDataType::DateTime => crate::types::DataType::DateTime,
        };

        match self.inner.cast(data_type) {
            Ok(result) => Ok(PySeries { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Convert to `Vec<f64>` for numeric series
    pub fn to_vec_f64(&self) -> PyResult<Vec<Option<f64>>> {
        match self.inner.to_vec_f64() {
            Ok(result) => Ok(result.into_iter().map(Some).collect()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Calculate correlation with another series
    pub fn correlation(&self, other: &PySeries) -> PyResult<f64> {
        match self.inner.correlation(&other.inner) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Unable to compute correlation",
            )),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Calculate covariance with another series
    pub fn covariance(&self, other: &PySeries) -> PyResult<f64> {
        match self.inner.covariance(&other.inner) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Unable to compute covariance",
            )),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }
}

/// Python wrapper for DataFrame operations
#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyDataFrame {
    pub(crate) inner: DataFrame,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDataFrame {
    #[new]
    pub fn new(_py: Python, columns_dict: &Bound<'_, PyDict>) -> PyResult<Self> {
        let mut df_columns = IndexMap::new();

        for item in columns_dict.iter() {
            let key: String = item.0.extract()?;
            let series: PySeries = item.1.extract()?;
            df_columns.insert(key, series.inner);
        }

        let df = DataFrame::new(df_columns);
        Ok(PyDataFrame { inner: df })
    }

    #[staticmethod]
    pub fn from_dict(_py: Python, data: &Bound<'_, PyDict>) -> PyResult<Self> {
        let mut columns = IndexMap::new();
        for (key, value) in data.iter() {
            let name: String = key.extract()?;
            let series = if let Ok(values) = value.extract::<Vec<i32>>() {
                Series::new_i32(&name, values.into_iter().map(Some).collect())
            } else if let Ok(values) = value.extract::<Vec<f64>>() {
                Series::new_f64(&name, values.into_iter().map(Some).collect())
            } else if let Ok(values) = value.extract::<Vec<String>>() {
                Series::new_string(&name, values.into_iter().map(Some).collect())
            } else if let Ok(values) = value.extract::<Vec<bool>>() {
                Series::new_bool(&name, values.into_iter().map(Some).collect())
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unsupported data type for column '{}'",
                    name
                )));
            };
            columns.insert(name, series);
        }

        let df = DataFrame::new(columns);
        Ok(PyDataFrame { inner: df })
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.inner.row_count()
    }

    /// Get the number of columns  
    pub fn column_count(&self) -> usize {
        self.inner.column_count()
    }

    /// Get column names
    pub fn column_names(&self) -> Vec<String> {
        self.inner.column_names()
    }

    /// Get a column as PySeries
    pub fn get_column(&self, name: &str) -> PyResult<PySeries> {
        match self.inner.get_column(name) {
            Some(series) => Ok(PySeries {
                inner: series.clone(),
            }),
            None => Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                "Column '{}' not found",
                name
            ))),
        }
    }

    /// Filter DataFrame using high-performance vectorized operations
    pub fn filter_gt(&self, column: &str, value: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            let condition = if let Ok(val) = value.extract::<i32>(py) {
                Condition::Gt(column.to_string(), Value::I32(val))
            } else if let Ok(val) = value.extract::<f64>(py) {
                Condition::Gt(column.to_string(), Value::F64(val))
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unsupported value type for comparison",
                ));
            };

            match self.inner.filter(&condition) {
                Ok(filtered) => Ok(PyDataFrame { inner: filtered }),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            }
        })
    }

    /// Group by operations
    /// Reshape the DataFrame from long to wide format
    pub fn pivot(
        &self,
        values: &str,
        index: Vec<String>,
        columns: &str,
        agg_fn: &str,
    ) -> PyResult<Self> {
        use crate::dataframe::Pivot;
        match self.inner.pivot(values, index, columns, agg_fn) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    pub fn group_by(&self, columns: Vec<String>) -> PyResult<PyGroupedDataFrame> {
        // For now, just return a PyGroupedDataFrame with the same data
        Ok(PyGroupedDataFrame {
            dataframe: self.clone(),
            group_columns: columns,
        })
    }

    /// Aggregate operations (placeholder for group_by results)
    pub fn agg(&self, _aggregations: Vec<(String, String)>) -> PyResult<Self> {
        // For now, just return a copy of self
        Ok(self.clone())
    }

    /// Select columns
    pub fn select(&self, columns: Vec<String>) -> PyResult<Self> {
        match self.inner.select_columns(columns) {
            Ok(selected) => Ok(PyDataFrame { inner: selected }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Select columns (alias for compatibility)
    pub fn select_columns(&self, columns: Vec<String>) -> PyResult<Self> {
        self.select(columns)
    }

    /// Drop columns
    pub fn drop_columns(&self, columns: Vec<String>) -> PyResult<Self> {
        match self.inner.drop_columns(columns) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Rename a column
    pub fn rename_column(&self, old_name: &str, new_name: &str) -> PyResult<Self> {
        match self.inner.rename_column(old_name, new_name) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Drop null values
    pub fn drop_nulls(&self, subset: Option<Vec<String>>) -> PyResult<Self> {
        match self.inner.drop_nulls(subset.as_deref()) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Fill null values
    pub fn fill_nulls(&self, value: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            let fill_value = if let Ok(v) = value.extract::<i32>(py) {
                Value::I32(v)
            } else if let Ok(v) = value.extract::<f64>(py) {
                Value::F64(v)
            } else if let Ok(v) = value.extract::<String>(py) {
                Value::String(v)
            } else if let Ok(v) = value.extract::<bool>(py) {
                Value::Bool(v)
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unsupported fill value type",
                ));
            };

            match self.inner.fill_nulls(fill_value) {
                Ok(result) => Ok(PyDataFrame { inner: result }),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            }
        })
    }

    /// Sort by columns
    pub fn sort(&self, by_columns: Vec<String>, ascending: bool) -> PyResult<Self> {
        match self.inner.sort(by_columns, ascending) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Append another DataFrame
    pub fn append(&self, other: &PyDataFrame) -> PyResult<Self> {
        match self.inner.append(&other.inner) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Calculate correlation between two columns
    pub fn correlation(&self, col1: &str, col2: &str) -> PyResult<f64> {
        match self.inner.correlation(col1, col2) {
            Ok(result) => Ok(result),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Calculate covariance between two columns
    pub fn covariance(&self, col1: &str, col2: &str) -> PyResult<f64> {
        match self.inner.covariance(col1, col2) {
            Ok(result) => Ok(result),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Describe the DataFrame (statistical summary)
    pub fn describe(&self) -> PyResult<Self> {
        match self.inner.describe() {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Filter with condition or indices
    pub fn filter(&self, filter_param: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            // Try to extract as PyCondition first
            if let Ok(condition) = filter_param.extract::<PyCondition>(py) {
                match self.inner.filter(&condition.inner) {
                    Ok(result) => Ok(PyDataFrame { inner: result }),
                    Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        e.to_string(),
                    )),
                }
            }
            // Try to extract as Vec<usize> for indices
            else if let Ok(indices) = filter_param.extract::<Vec<usize>>(py) {
                self.filter_by_indices(indices)
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Filter parameter must be either a PyCondition or a list of indices",
                ))
            }
        })
    }

    /// Filter by row indices
    pub fn filter_by_indices(&self, indices: Vec<usize>) -> PyResult<Self> {
        // Create a new DataFrame with only the specified rows
        let mut new_series = indexmap::IndexMap::new();
        let column_names = self.inner.column_names();

        for column_name in column_names {
            if let Some(series) = self.inner.get_column(&column_name) {
                match series.filter(&indices) {
                    Ok(filtered_series) => {
                        new_series.insert(column_name.clone(), filtered_series);
                    }
                    Err(e) => {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            e.to_string(),
                        ))
                    }
                }
            }
        }

        let df = DataFrame::new(new_series);
        Ok(PyDataFrame { inner: df })
    }

    /// Add a computed column
    pub fn with_column(&self, name: &str, expr: &PyExpr) -> PyResult<Self> {
        match self.inner.with_column(name, &expr.inner) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Export to CSV
    pub fn to_csv(&self, path: &str) -> PyResult<()> {
        match self.inner.to_csv(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string())),
        }
    }

    /// Load from JSON
    #[staticmethod]
    pub fn from_json(path: &str) -> PyResult<Self> {
        match DataFrame::from_json(path) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string())),
        }
    }

    /// Load from CSV
    #[staticmethod]
    pub fn from_csv(path: &str) -> PyResult<Self> {
        match DataFrame::from_csv(path) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string())),
        }
    }

    /// Export to JSON (placeholder - not yet implemented)
    pub fn to_json(&self, _path: &str) -> PyResult<()> {
        Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
            "to_json not yet implemented",
        ))
    }

    /// Join with another DataFrame
    pub fn join(
        &self,
        other: &PyDataFrame,
        on_column: &str,
        join_type: &PyJoinType,
    ) -> PyResult<Self> {
        let jt = match join_type {
            PyJoinType::Inner => crate::dataframe::join::JoinType::Inner,
            PyJoinType::Left => crate::dataframe::join::JoinType::Left,
            PyJoinType::Right => crate::dataframe::join::JoinType::Right,
            PyJoinType::Outer => crate::dataframe::join::JoinType::Outer,
        };

        match self.inner.join(&other.inner, on_column, jt) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    /// Perform an ultra-fast inner join using SIMD-accelerated operations
    pub fn fast_inner_join(
        &self,
        other: &PyDataFrame,
        left_on: &str,
        right_on: &str,
    ) -> PyResult<Self> {
        match UltraFastJoin::inner_join_i32(&self.inner, &other.inner, left_on, right_on) {
            Ok(result) => Ok(PyDataFrame { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }
}

/// High-performance vectorized operations module for Python
#[cfg(feature = "python")]
#[pyfunction]
pub fn simd_add_f64(a: Vec<f64>, b: Vec<f64>) -> PyResult<Vec<f64>> {
    if a.len() != b.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Arrays must have the same length",
        ));
    }

    let mut result = vec![0.0; a.len()];
    a.optimized_simd_add(&b, &mut result);
    Ok(result)
}

/// High-performance vectorized sum for Python
#[cfg(feature = "python")]
#[pyfunction]
pub fn simd_sum_f64(data: Vec<f64>) -> f64 {
    data.optimized_simd_sum()
}

/// Create a DataFrame from CSV with high-performance parsing
#[cfg(feature = "python")]
#[pyfunction]
pub fn read_csv(file_path: String) -> PyResult<PyDataFrame> {
    use crate::io::CsvReader;

    let reader = CsvReader::new();
    match reader.read_file(&file_path) {
        Ok(df) => Ok(PyDataFrame { inner: df }),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string())),
    }
}

/// Python module definition
#[cfg(feature = "python")]
#[pymodule]
fn veloxx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Core data structures
    m.add_class::<PySeries>()?;
    m.add_class::<PyDataFrame>()?;
    m.add_class::<PyGroupedDataFrame>()?;

    // Helper classes
    m.add_class::<PyDataType>()?;
    m.add_class::<PyJoinType>()?;
    m.add_class::<PyCondition>()?;
    m.add_class::<PyExpr>()?;
    m.add_class::<PyValue>()?;

    // High-performance functions
    m.add_function(wrap_pyfunction!(simd_add_f64, m)?)?;
    m.add_function(wrap_pyfunction!(simd_sum_f64, m)?)?;
    m.add_function(wrap_pyfunction!(read_csv, m)?)?;

    Ok(())
}
