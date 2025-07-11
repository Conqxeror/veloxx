use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::BTreeMap;

use crate::dataframe::join::JoinType;
use crate::dataframe::DataFrame;
use crate::expressions::Expr;
use crate::series::Series;
use crate::types::Value;

#[pyclass]
pub struct PyDataFrame {
    pub df: DataFrame,
}

#[pymethods]
impl PyDataFrame {
    #[new]
    fn new(columns: &Bound<PyDict>) -> PyResult<Self> {
        let mut rust_columns = BTreeMap::new();
        for (key, value) in columns.iter() {
            let name: String = key.extract()?;
            let py_series: PySeries = value.extract()?;
            rust_columns.insert(name, py_series.series.clone());
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

    fn column_names<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let names: Vec<String> = self
            .df
            .column_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        Ok(PyList::new_bound(py, names))
    }

    fn get_column(&self, name: &str) -> PyResult<Option<PySeries>> {
        Ok(self
            .df
            .get_column(name)
            .map(|s| PySeries { series: s.clone() }))
    }

    fn filter(&self, row_indices: Vec<usize>) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .filter_by_indices(&row_indices)
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

    fn drop_nulls(&self) -> PyResult<Self> {
        Ok(PyDataFrame {
            df: self
                .df
                .drop_nulls()
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
        Ok(PyDataFrame {
            df: self
                .df
                .join(&other.df, on_column, join_type.into())
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
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
        Ok(PyDataFrame {
            df: self
                .df
                .sort(by_columns, ascending)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn __repr__(&self) -> String {
        format!("{}", self.df)
    }

    fn __str__(&self) -> String {
        format!("{}", self.df)
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

    fn name(&self) -> String {
        self.series.name().to_string()
    }

    fn len(&self) -> usize {
        self.series.len()
    }

    fn is_empty(&self) -> bool {
        self.series.is_empty()
    }

    fn data_type(&self) -> PyDataType {
        self.series.data_type().into()
    }

    fn set_name(&mut self, new_name: &str) {
        self.series.set_name(new_name);
    }

    fn get_value<'py>(&self, py: Python<'py>, index: usize) -> PyResult<Option<PyObject>> {
        Ok(self.series.get_value(index).map(|v| value_to_py(py, v)))
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

    fn cast(&self, to_type: PyDataType) -> PyResult<Self> {
        Ok(PySeries {
            series: self
                .series
                .cast(to_type.into())
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn append(&self, other: &PySeries) -> PyResult<Self> {
        Ok(PySeries {
            series: self
                .series
                .append(&other.series)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn sum<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self
            .series
            .sum()
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .map(|v| value_to_py(py, v)))
    }

    fn count(&self) -> usize {
        self.series.count()
    }

    fn min<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self
            .series
            .min()
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .map(|v| value_to_py(py, v)))
    }

    fn max<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self
            .series
            .max()
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .map(|v| value_to_py(py, v)))
    }

    fn mean<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self
            .series
            .mean()
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .map(|v| value_to_py(py, v)))
    }

    fn median<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self
            .series
            .median()
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .map(|v| value_to_py(py, v)))
    }

    fn std_dev<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self
            .series
            .std_dev()
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .map(|v| value_to_py(py, v)))
    }

    fn correlation<'py>(&self, py: Python<'py>, other: &PySeries) -> PyResult<Option<PyObject>> {
        Ok(self
            .series
            .correlation(&other.series)
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .map(|v| value_to_py(py, v)))
    }

    fn covariance<'py>(&self, py: Python<'py>, other: &PySeries) -> PyResult<Option<PyObject>> {
        Ok(self
            .series
            .covariance(&other.series)
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .map(|v| value_to_py(py, v)))
    }

    fn unique(&self) -> PyResult<Self> {
        Ok(PySeries {
            series: self
                .series
                .unique()
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn to_vec_f64<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let vec_f64 = self
            .series
            .to_vec_f64()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyList::new_bound(py, vec_f64))
    }

    fn interpolate_nulls(&self) -> PyResult<Self> {
        Ok(PySeries {
            series: self
                .series
                .interpolate_nulls()
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.series)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.series)
    }
}

#[pyclass]
#[derive(Clone)]
pub enum PyDataType {
    I32,
    F64,
    Bool,
    String,
    DateTime,
}

impl From<crate::types::DataType> for PyDataType {
    fn from(data_type: crate::types::DataType) -> Self {
        match data_type {
            crate::types::DataType::I32 => PyDataType::I32,
            crate::types::DataType::F64 => PyDataType::F64,
            crate::types::DataType::Bool => PyDataType::Bool,
            crate::types::DataType::String => PyDataType::String,
            crate::types::DataType::DateTime => PyDataType::DateTime,
        }
    }
}

impl From<PyDataType> for crate::types::DataType {
    fn from(py_data_type: PyDataType) -> Self {
        match py_data_type {
            PyDataType::I32 => crate::types::DataType::I32,
            PyDataType::F64 => crate::types::DataType::F64,
            PyDataType::Bool => crate::types::DataType::Bool,
            PyDataType::String => crate::types::DataType::String,
            PyDataType::DateTime => crate::types::DataType::DateTime,
        }
    }
}

// Helper functions for Value conversion
fn extract_value(py_value: &Bound<PyAny>) -> PyResult<Value> {
    if let Ok(v) = py_value.extract::<i32>() {
        Ok(Value::I32(v))
    } else if let Ok(v) = py_value.extract::<f64>() {
        Ok(Value::F64(v))
    } else if let Ok(v) = py_value.extract::<bool>() {
        Ok(Value::Bool(v))
    } else if let Ok(v) = py_value.extract::<String>() {
        Ok(Value::String(v))
    } else if let Ok(v) = py_value.extract::<i64>() {
        Ok(Value::DateTime(v))
    } else if py_value.is_none() {
        Ok(Value::Null)
    } else {
        Err(PyValueError::new_err(
            "Unsupported Python type for Value conversion",
        ))
    }
}

fn value_to_py(py: Python, value: Value) -> PyObject {
    match value {
        Value::I32(v) => v.into_py(py),
        Value::F64(v) => v.into_py(py),
        Value::Bool(v) => v.into_py(py),
        Value::String(v) => v.into_py(py),
        Value::DateTime(v) => v.into_py(py),
        Value::Null => py.None(),
    }
}

#[pymodule]
fn veloxx(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyDataFrame>()?;
    m.add_class::<PySeries>()?;
    m.add_class::<PyDataType>()?;
    m.add_class::<PyGroupedDataFrame>()?;
    m.add_class::<PyExpr>()?;
    m.add_class::<PyJoinType>()?;
    Ok(())
}
