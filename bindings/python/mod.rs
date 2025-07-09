use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyList, PyDict};
use pyo3::conversion::{FromPyObject, IntoPy};

use crate::dataframe::DataFrame;
use crate::series::Series;
use crate::types::{DataType, Value};
use std::collections::BTreeMap;

// --- PyDataFrame ---
#[pyclass(unsendable)]
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
            df: DataFrame::new(rust_columns).map_err(PyValueError::new_err)?,
        })
    }

    fn row_count(&self) -> usize {
        self.df.row_count()
    }

    fn column_count(&self) -> usize {
        self.df.column_count()
    }

    fn column_names<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let names: Vec<String> = self.df.column_names().into_iter().map(|s| s.to_string()).collect();
        Ok(PyList::new_bound(py, names))
    }

    fn get_column(&self, name: &str) -> PyResult<Option<PySeries>> {
        Ok(self.df.get_column(name).map(|s| PySeries { series: s.clone() }))
    }

    fn filter(&self, row_indices: Vec<usize>) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.filter_by_indices(&row_indices).map_err(PyValueError::new_err)? })
    }

    fn select_columns(&self, names: Vec<String>) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.select_columns(names).map_err(PyValueError::new_err)? })
    }

    fn drop_columns(&self, names: Vec<String>) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.drop_columns(names).map_err(PyValueError::new_err)? })
    }

    fn rename_column(&self, old_name: &str, new_name: &str) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.rename_column(old_name, new_name).map_err(PyValueError::new_err)? })
    }

    fn drop_nulls(&self) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.drop_nulls().map_err(PyValueError::new_err)? })
    }

    fn fill_nulls(&self, value: &PyAny) -> PyResult<Self> {
        let rust_value = value.extract::<Value>()?;
        Ok(PyDataFrame { df: self.df.fill_nulls(rust_value).map_err(PyValueError::new_err)? })
    }

    #[staticmethod]
    fn from_csv(path: &str) -> PyResult<Self> {
        Ok(PyDataFrame { df: DataFrame::from_csv(path).map_err(PyValueError::new_err)? })
    }

    #[staticmethod]
    fn from_json(path: &str) -> PyResult<Self> {
        Ok(PyDataFrame { df: DataFrame::from_json(path).map_err(PyValueError::new_err)? })
    }

    fn to_csv(&self, path: &str) -> PyResult<()> {
        self.df.to_csv(path).map_err(PyValueError::new_err)
    }

    fn join(&self, other: &PyDataFrame, on_column: &str, join_type: PyJoinType) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.join(&other.df, on_column, join_type.into()).map_err(PyValueError::new_err)? })
    }

    fn group_by(&self, by_columns: Vec<String>) -> PyResult<PyGroupedDataFrame> {
        Ok(PyGroupedDataFrame { grouped_df: self.df.group_by(by_columns).map_err(PyValueError::new_err)? })
    }

    fn with_column(&self, new_col_name: &str, expr: &PyExpr) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.with_column(new_col_name, &expr.expr).map_err(PyValueError::new_err)? })
    }

    fn describe(&self) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.describe().map_err(PyValueError::new_err)? })
    }

    fn correlation(&self, col1_name: &str, col2_name: &str) -> PyResult<f64> {
        self.df.correlation(col1_name, col2_name).map_err(PyValueError::new_err)
    }

    fn covariance(&self, col1_name: &str, col2_name: &str) -> PyResult<f64> {
        self.df.covariance(col1_name, col2_name).map_err(PyValueError::new_err)
    }

    fn append(&self, other: &PyDataFrame) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.append(&other.df).map_err(PyValueError::new_err)? })
    }
}

#[pyclass(unsendable)]
pub struct PyGroupedDataFrame {
    pub grouped_df: GroupedDataFrame<'static>,
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
        PyExpr { expr: Expr::Column(name.to_string()) }
    }

    #[staticmethod]
    pub fn literal(value: &PyAny) -> PyResult<Self> {
        let rust_value = value.extract::<Value>()?;
        Ok(PyExpr { expr: Expr::Literal(rust_value) })
    }

    #[staticmethod]
    pub fn add(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::Add(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn subtract(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::Subtract(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn multiply(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::Multiply(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn divide(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::Divide(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn equals(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::Equals(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn not_equals(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::NotEquals(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn greater_than(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::GreaterThan(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn less_than(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::LessThan(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn greater_than_or_equal(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::GreaterThanOrEqual(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn less_than_or_equal(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::LessThanOrEqual(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn and(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::And(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn or(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr { expr: Expr::Or(Box::new(left.expr.clone()), Box::new(right.expr.clone())) }
    }

    #[staticmethod]
    pub fn not(expr: &PyExpr) -> Self {
        PyExpr { expr: Expr::Not(Box::new(expr.expr.clone())) }
    }
}

// --- PySeries ---
#[pyclass(unsendable)]
#[derive(Clone)]
pub struct PySeries {
    pub series: Series,
}

#[pymethods]
impl PySeries {
    #[new]
    fn new(name: &str, data: &Bound<PyAny>) -> PyResult<Self> {
        if let Ok(list) = data.extract::<Vec<Option<i32>>>() {
            Ok(PySeries { series: Series::new_i32(name, list) })
        } else if let Ok(list) = data.extract::<Vec<Option<f64>>>() {
            Ok(PySeries { series: Series::new_f64(name, list) })
        } else if let Ok(list) = data.extract::<Vec<Option<bool>>>() {
            Ok(PySeries { series: Series::new_bool(name, list) })
        } else if let Ok(list) = data.extract::<Vec<Option<String>>>() {
            Ok(PySeries { series: Series::new_string(name, list) })
        } else if let Ok(list) = data.extract::<Vec<Option<i64>>>() {
            Ok(PySeries { series: Series::new_datetime(name, list) })
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
        Ok(self.series.get_value(index).map(|v| v.into_py(py)))
    }

    fn filter(&self, row_indices: Vec<usize>) -> PyResult<Self> {
        Ok(PySeries { series: self.series.filter(&row_indices).map_err(PyValueError::new_err)? })
    }

    fn fill_nulls(&self, value: &PyAny) -> PyResult<Self> {
        let rust_value = value.extract::<Value>()?;
        Ok(PySeries { series: self.series.fill_nulls(&rust_value).map_err(PyValueError::new_err)? })
    }

    fn cast(&self, to_type: PyDataType) -> PyResult<Self> {
        Ok(PySeries { series: self.series.cast(to_type.into()).map_err(PyValueError::new_err)? })
    }

    fn append(&self, other: &PySeries) -> PyResult<Self> {
        Ok(PySeries { series: self.series.append(&other.series).map_err(PyValueError::new_err)? })
    }

    fn sum<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self.series.sum().map_err(PyValueError::new_err)?.map(|v| v.into_py(py)))
    }

    fn count(&self) -> usize {
        self.series.count()
    }

    fn min<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self.series.min().map_err(PyValueError::new_err)?.map(|v| v.into_py(py)))
    }
    
    fn max<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self.series.max().map_err(PyValueError::new_err)?.map(|v| v.into_py(py)))
    }

    fn mean<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self.series.mean().map_err(PyValueError::new_err)?.map(|v| v.into_py(py)))
    }

    fn median<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self.series.median().map_err(PyValueError::new_err)?.map(|v| v.into_py(py)))
    }

    fn std_dev<'py>(&self, py: Python<'py>) -> PyResult<Option<PyObject>> {
        Ok(self.series.std_dev().map_err(PyValueError::new_err)?.map(|v| v.into_py(py)))
    }

    fn correlation<'py>(&self, py: Python<'py>, other: &PySeries) -> PyResult<Option<PyObject>> {
        Ok(self.series.correlation(&other.series).map_err(PyValueError::new_err)?.map(|v| v.into_py(py)))
    }

    fn covariance<'py>(&self, py: Python<'py>, other: &PySeries) -> PyResult<Option<PyObject>> {
        Ok(self.series.covariance(&other.series).map_err(PyValueError::new_err)?.map(|v| v.into_py(py)))
    }

    fn unique(&self) -> PyResult<Self> {
        Ok(PySeries { series: self.series.unique().map_err(PyValueError::new_err)? })
    }

    fn to_vec_f64<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let vec_f64 = self.series.to_vec_f64().map_err(PyValueError::new_err)?;
        Ok(PyList::new_bound(py, vec_f64))
    }

    fn interpolate_nulls(&self) -> PyResult<Self> {
        Ok(PySeries { series: self.series.interpolate_nulls().map_err(PyValueError::new_err)? })
    }

    fn apply_i32(&self, py_callable: &PyAny) -> PyResult<Self> {
        let mut new_data: Vec<Option<i32>> = Vec::with_capacity(self.series.len());
        match &self.series {
            Series::I32(_, data) => {
                for item in data.iter() {
                    let py_arg = item.map_or(pyo3::Python::current().None(), |v| v.into_py(pyo3::Python::current()));
                    let result = py_callable.call1((py_arg,))?;
                    new_data.push(result.extract()?);
                }
                Ok(PySeries { series: Series::new_i32(self.series.name(), new_data) })
            },
            _ => Err(PyValueError::new_err("apply_i32 only supported for I32 series"))
        }
    }

    fn apply_f64(&self, py_callable: &PyAny) -> PyResult<Self> {
        let mut new_data: Vec<Option<f64>> = Vec::with_capacity(self.series.len());
        match &self.series {
            Series::F64(_, data) => {
                for item in data.iter() {
                    let py_arg = item.map_or(pyo3::Python::current().None(), |v| v.into_py(pyo3::Python::current()));
                    let result = py_callable.call1((py_arg,))?;
                    new_data.push(result.extract()?);
                }
                Ok(PySeries { series: Series::new_f64(self.series.name(), new_data) })
            },
            _ => Err(PyValueError::new_err("apply_f64 only supported for F64 series"))
        }
    }

    fn apply_bool(&self, py_callable: &PyAny) -> PyResult<Self> {
        let mut new_data: Vec<Option<bool>> = Vec::with_capacity(self.series.len());
        match &self.series {
            Series::Bool(_, data) => {
                for item in data.iter() {
                    let py_arg = item.map_or(pyo3::Python::current().None(), |v| v.into_py(pyo3::Python::current()));
                    let result = py_callable.call1((py_arg,))?;
                    new_data.push(result.extract()?);
                }
                Ok(PySeries { series: Series::new_bool(self.series.name(), new_data) })
            },
            _ => Err(PyValueError::new_err("apply_bool only supported for Bool series"))
        }
    }

    fn apply_string(&self, py_callable: &PyAny) -> PyResult<Self> {
        let mut new_data: Vec<Option<String>> = Vec::with_capacity(self.series.len());
        match &self.series {
            Series::String(_, data) => {
                for item in data.iter() {
                    let py_arg = item.as_ref().map_or(pyo3::Python::current().None(), |v| v.into_py(pyo3::Python::current()));
                    let result = py_callable.call1((py_arg,))?;
                    new_data.push(result.extract()?);
                }
                Ok(PySeries { series: Series::new_string(self.series.name(), new_data) })
            },
            _ => Err(PyValueError::new_err("apply_string only supported for String series"))
        }
    }

    fn apply_datetime(&self, py_callable: &PyAny) -> PyResult<Self> {
        let mut new_data: Vec<Option<i64>> = Vec::with_capacity(self.series.len());
        match &self.series {
            Series::DateTime(_, data) => {
                for item in data.iter() {
                    let py_arg = item.map_or(pyo3::Python::current().None(), |v| v.into_py(pyo3::Python::current()));
                    let result = py_callable.call1((py_arg,))?;
                    new_data.push(result.extract()?);
                }
                Ok(PySeries { series: Series::new_datetime(self.series.name(), new_data) })
            },
            _ => Err(PyValueError::new_err("apply_datetime only supported for DateTime series"))
        }
    }
}

// --- PyDataType ---
#[pyclass]
#[derive(Clone)]
pub enum PyDataType {
    I32,
    F64,
    Bool,
    String,
    DateTime,
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

// --- Value (Rust enum) conversions to/from Python objects ---
impl IntoPy<PyObject> for Value {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Value::I32(v) => v.into_py(py),
            Value::F64(v) => v.into_py(py),
            Value::Bool(v) => v.into_py(py),
            Value::String(v) => v.into_py(py),
            Value::DateTime(v) => v.into_py(py),
            Value::Null => py.None(),
        }
    }
}

impl <'py> FromPyObject<'py> for Value {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        if let Ok(v) = ob.extract::<i32>() {
            Ok(Value::I32(v))
        } else if let Ok(v) = ob.extract::<f64>() {
            Ok(Value::F64(v))
        } else if let Ok(v) = ob.extract::<bool>() {
            Ok(Value::Bool(v))
        } else if let Ok(v) = ob.extract::<String>() {
            Ok(Value::String(v))
        } else if let Ok(v) = ob.extract::<i64>() {
            Ok(Value::DateTime(v))
        } else if ob.is_none() {
            Ok(Value::Null)
        } else {
            Err(PyValueError::new_err("Unsupported Python type for Value conversion"))
        }
    }
}

// --- Module definition ---
#[pymodule]
fn veloxx(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyDataFrame>()?;
    m.add_class::<PySeries>()?;
    m.add_class::<PyDataType>()?;
    m.add_class::<PyGroupedDataFrame>()?;
    m.add_class::<PyExpr>()?;
    Ok(())
}