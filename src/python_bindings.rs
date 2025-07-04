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

    fn sort(&self, by_columns: Vec<String>, ascending: bool) -> PyResult<Self> {
        Ok(PyDataFrame { df: self.df.sort(by_columns, ascending).map_err(PyValueError::new_err)? })
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
    Ok(())
}