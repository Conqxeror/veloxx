use crate::types::{DataType, Value};
use std::collections::HashSet;
use crate::error::VeloxxError;

/// Represents a single-typed, named column of data within a DataFrame.
///
/// Supports various data types including integers, floats, booleans, and strings.
#[derive(Debug, PartialEq, Clone)]
pub enum Series {
    /// A series containing 32-bit signed integers.
    I32(String, Vec<Option<i32>>),
    /// A series containing 64-bit floating-point numbers.
    F64(String, Vec<Option<f64>>),
    /// A series containing boolean values.
    Bool(String, Vec<Option<bool>>),
    /// A series containing string values.
    String(String, Vec<Option<String>>),
    /// A series containing DateTime values (Unix timestamp i64).
    DateTime(String, Vec<Option<i64>>),
}

impl Series {
    /// Creates a new `Series` of 32-bit signed integers.
    pub fn new_i32(name: &str, data: Vec<Option<i32>>) -> Self {
        Series::I32(name.to_string(), data)
    }

    /// Creates a new `Series` of 64-bit floating-point numbers.
    pub fn new_f64(name: &str, data: Vec<Option<f64>>) -> Self {
        Series::F64(name.to_string(), data)
    }

    /// Creates a new `Series` of boolean values.
    pub fn new_bool(name: &str, data: Vec<Option<bool>>) -> Self {
        Series::Bool(name.to_string(), data)
    }

    /// Creates a new `Series` of string values.
    pub fn new_string(name: &str, data: Vec<Option<String>>) -> Self {
        Series::String(name.to_string(), data)
    }

    /// Creates a new `Series` of DateTime values.
    pub fn new_datetime(name: &str, data: Vec<Option<i64>>) -> Self {
        Series::DateTime(name.to_string(), data)
    }

    /// Returns the name of the series.
    pub fn name(&self) -> &str {
        match self {
            Series::I32(name, _) => name,
            Series::F64(name, _) => name,
            Series::Bool(name, _) => name,
            Series::String(name, _) => name,
            Series::DateTime(name, _) => name,
        }
    }

    /// Returns the number of elements in the series.
    pub fn len(&self) -> usize {
        match self {
            Series::I32(_, v) => v.len(),
            Series::F64(_, v) => v.len(),
            Series::Bool(_, v) => v.len(),
            Series::String(_, v) => v.len(),
            Series::DateTime(_, v) => v.len(),
        }
    }

    /// Returns `true` if the series contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the `DataType` of the series.
    pub fn data_type(&self) -> DataType {
        match self {
            Series::I32(_, _) => DataType::I32,
            Series::F64(_, _) => DataType::F64,
            Series::Bool(_, _) => DataType::Bool,
            Series::String(_, _) => DataType::String,
            Series::DateTime(_, _) => DataType::DateTime,
        }
    }

    /// Sets the name of the series.
    pub fn set_name(&mut self, new_name: &str) {
        match self {
            Series::I32(name, _) => *name = new_name.to_string(),
            Series::F64(name, _) => *name = new_name.to_string(),
            Series::Bool(name, _) => *name = new_name.to_string(),
            Series::String(name, _) => *name = new_name.to_string(),
            Series::DateTime(name, _) => *name = new_name.to_string(),
        }
    }

    /// Returns the `Value` at the given index, if it exists.
    pub fn get_value(&self, index: usize) -> Option<Value> {
        match self {
            Series::I32(_, v) => v.get(index).and_then(|&val| val.map(Value::I32)),
            Series::F64(_, v) => v.get(index).and_then(|&val| val.map(Value::F64)),
            Series::Bool(_, v) => v.get(index).and_then(|&val| val.map(Value::Bool)),
            Series::String(_, v) => v
                .get(index)
                .and_then(|val| val.as_ref().map(|s| Value::String(s.clone()))),
            Series::DateTime(_, v) => v.get(index).and_then(|&val| val.map(Value::DateTime)),
        }
    }

    /// Filters the series based on the provided row indices.
    pub fn filter(&self, row_indices: &[usize]) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                let filtered_data: Vec<Option<i32>> =
                    row_indices.iter().map(|&i| data[i]).collect();
                Ok(Series::I32(name, filtered_data))
            }
            Series::F64(_, data) => {
                let filtered_data: Vec<Option<f64>> =
                    row_indices.iter().map(|&i| data[i]).collect();
                Ok(Series::F64(name, filtered_data))
            }
            Series::Bool(_, data) => {
                let filtered_data: Vec<Option<bool>> =
                    row_indices.iter().map(|&i| data[i]).collect();
                Ok(Series::Bool(name, filtered_data))
            }
            Series::String(_, data) => {
                let filtered_data: Vec<Option<String>> =
                    row_indices.iter().map(|&i| data[i].clone()).collect();
                Ok(Series::String(name, filtered_data))
            }
            Series::DateTime(_, data) => {
                let filtered_data: Vec<Option<i64>> =
                    row_indices.iter().map(|&i| data[i]).collect();
                Ok(Series::DateTime(name, filtered_data))
            }
        }
    }

    /// Fills null values in the series with a specified value.
    ///
    /// Returns an error if the fill value's type does not match the series' data type.
    pub fn fill_nulls(&self, value: &Value) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                if let Value::I32(fill_val) = value {
                    let filled_data: Vec<Option<i32>> =
                        data.iter().map(|&x| x.or(Some(*fill_val))).collect();
                    Ok(Series::I32(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill I32 series with {value:?}"
                    )))
                }
            }
            Series::F64(_, data) => {
                if let Value::F64(fill_val) = value {
                    let filled_data: Vec<Option<f64>> =
                        data.iter().map(|&x| x.or(Some(*fill_val))).collect();
                    Ok(Series::F64(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill F64 series with {value:?}"
                    )))
                }
            }
            Series::Bool(_, data) => {
                if let Value::Bool(fill_val) = value {
                    let filled_data: Vec<Option<bool>> =
                        data.iter().map(|&x| x.or(Some(*fill_val))).collect();
                    Ok(Series::Bool(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill Bool series with {value:?}"
                    )))
                }
            }
            Series::String(_, data) => {
                if let Value::String(fill_val) = value {
                    let filled_data: Vec<Option<String>> = data
                        .iter()
                        .map(|x| x.clone().or(Some(fill_val.clone())))
                        .collect();
                    Ok(Series::String(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill String series with {value:?}"
                    )))
                }
            }
            Series::DateTime(_, data) => {
                if let Value::DateTime(fill_val) = value {
                    let filled_data: Vec<Option<i64>> =
                        data.iter().map(|&x| x.or(Some(*fill_val))).collect();
                    Ok(Series::DateTime(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill DateTime series with {value:?}"
                    )))
                }
            }
        }
    }

    /// Casts the series to a new data type.
    ///
    /// Returns an error if the cast is not supported.
    pub fn cast(&self, to_type: DataType) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match (self, to_type) {
            (Series::I32(_, data), DataType::F64) => Ok(Series::F64(
                name,
                data.iter().map(|&x| x.map(|val| val as f64)).collect(),
            )),
            (Series::F64(_, data), DataType::I32) => Ok(Series::I32(
                name,
                data.iter().map(|&x| x.map(|val| val as i32)).collect(),
            )),
            (Series::String(_, data), DataType::I32) => Ok(Series::I32(
                name,
                data.iter()
                    .map(|x| x.as_ref().and_then(|s| s.parse::<i32>().ok()))
                    .collect::<Vec<Option<i32>>>(),
            )),
            (Series::String(_, data), DataType::F64) => Ok(Series::F64(
                name,
                data.iter()
                    .map(|x| x.as_ref().and_then(|s| s.parse::<f64>().ok()))
                    .collect::<Vec<Option<f64>>>(),
            )),
            (Series::String(_, data), DataType::Bool) => Ok(Series::Bool(
                name,
                data.iter()
                    .map(|x| x.as_ref().and_then(|s| s.parse::<bool>().ok()))
                    .collect(),
            )),
            (Series::I32(_, data), DataType::DateTime) => Ok(Series::DateTime(
                name,
                data.iter().map(|&x| x.map(|val| val as i64)).collect(),
            )),
            (Series::String(_, data), DataType::DateTime) => Ok(Series::DateTime(
                name,
                data.iter()
                    .map(|x| x.as_ref().and_then(|s| s.parse::<i64>().ok()))
                    .collect(),
            )),
            (Series::DateTime(_, data), DataType::String) => Ok(Series::String(
                name,
                data.iter().map(|&x| x.map(|val| val.to_string())).collect(),
            )),
            (s, t) if s.data_type() == t => Ok(s.clone()),
            (_, to_type) => Err(VeloxxError::Unsupported(format!(
                "Unsupported cast from {:?} to {:?}",
                self.data_type(),
                to_type
            ))),
        }
    }

    /// Appends another series to the end of this series.
    ///
    /// Returns an error if the series have different data types.
    pub fn append(&self, other: &Series) -> Result<Self, VeloxxError> {
        if self.data_type() != other.data_type() {
            return Err(VeloxxError::DataTypeMismatch(format!(
                "Cannot append Series of different types: {:?} and {:?}",
                self.data_type(),
                other.data_type()
            )));
        }
        let new_name = self.name().to_string();
        match (self, other) {
            (Series::I32(_, data1), Series::I32(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::I32(new_name, new_data))
            }
            (Series::F64(_, data1), Series::F64(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::F64(new_name, new_data))
            }
            (Series::Bool(_, data1), Series::Bool(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::Bool(new_name, new_data))
            }
            (Series::String(_, data1), Series::String(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::String(new_name, new_data))
            }
            (Series::DateTime(_, data1), Series::DateTime(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::DateTime(new_name, new_data))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Mismatched series types during append (should be caught by data_type check)."
                    .to_string(),
            )),
        }
    }

    /// Calculates the sum of all non-null values in the series.
    ///
    /// Returns an error if the operation is not supported for the series' data type.
    pub fn sum(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let sum_val = data.iter().fold(None, |acc, &x| match (acc, x) {
                    (Some(current_sum), Some(val)) => Some(current_sum + val),
                    (None, Some(val)) => Some(val),
                    (acc, None) => acc,
                });
                Ok(sum_val.map(Value::I32))
            }
            Series::F64(_, data) => {
                let sum_val = data.iter().fold(None, |acc, &x| match (acc, x) {
                    (Some(current_sum), Some(val)) => Some(current_sum + val),
                    (None, Some(val)) => Some(val),
                    (acc, None) => acc,
                });
                Ok(sum_val.map(Value::F64))
            }
            Series::DateTime(_, data) => {
                let sum_val = data.iter().fold(None, |acc, &x| match (acc, x) {
                    (Some(current_sum), Some(val)) => Some(current_sum + val),
                    (None, Some(val)) => Some(val),
                    (acc, None) => acc,
                });
                Ok(sum_val.map(Value::DateTime))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Sum operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Counts the number of non-null values in the series.
    pub fn count(&self) -> usize {
        match self {
            Series::I32(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::F64(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::Bool(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::String(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::DateTime(_, data) => data.iter().filter(|x| x.is_some()).count(),
        }
    }

    /// Finds the minimum non-null value in the series.
    ///
    /// Returns an error if the operation is not supported for the series' data type.
    pub fn min(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let min_val = data.iter().filter_map(|&x| x).min();
                Ok(min_val.map(Value::I32))
            }
            Series::F64(_, data) => {
                let min_val = data
                    .iter()
                    .filter_map(|&x| x)
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                Ok(min_val.map(Value::F64))
            }
            Series::DateTime(_, data) => {
                let min_val = data.iter().filter_map(|&x| x).min();
                Ok(min_val.map(Value::DateTime))
            }
            Series::String(_, data) => {
                let min_val = data.iter().filter_map(|x| x.as_ref()).min_by(|a, b| a.cmp(b));
                Ok(min_val.map(|s| Value::String(s.clone())))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Min operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Finds the maximum non-null value in the series.
    ///
    /// Returns an error if the operation is not supported for the series' data type.
    pub fn max(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let max_val = data.iter().filter_map(|&x| x).max();
                Ok(max_val.map(Value::I32))
            }
            Series::F64(_, data) => {
                let max_val = data
                    .iter()
                    .filter_map(|&x| x)
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                Ok(max_val.map(Value::F64))
            }
            Series::DateTime(_, data) => {
                let max_val = data.iter().filter_map(|&x| x).max();
                Ok(max_val.map(Value::DateTime))
            }
            Series::String(_, data) => {
                let max_val = data.iter().filter_map(|x| x.as_ref()).max_by(|a, b| a.cmp(b));
                Ok(max_val.map(|s| Value::String(s.clone())))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Max operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Calculates the mean of all non-null values in the series.
    ///
    /// Returns an error if the operation is not supported for the series' data type.
    pub fn mean(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let sum_val: i64 = data.iter().filter_map(|&x| x.map(|v| v as i64)).sum();
                let count_val = data.iter().filter(|&x| x.is_some()).count() as i64;
                if count_val == 0 {
                    Ok(None)
                } else {
                    Ok(Some(Value::F64(sum_val as f64 / count_val as f64)))
                }
            }
            Series::F64(_, data) => {
                let sum_val: f64 = data.iter().filter_map(|&x| x).sum();
                let count_val = data.iter().filter(|&x| x.is_some()).count() as f64;
                if count_val == 0.0 {
                    Ok(None)
                } else {
                    Ok(Some(Value::F64(sum_val / count_val)))
                }
            }
            Series::DateTime(_, data) => {
                let sum_val: i64 = data.iter().filter_map(|&x| x).sum();
                let count_val = data.iter().filter(|&x| x.is_some()).count() as i64;
                if count_val == 0 {
                    Ok(None)
                } else {
                    Ok(Some(Value::F64(sum_val as f64 / count_val as f64)))
                }
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Mean operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Calculates the median of all non-null values in the series.
    ///
    /// Returns an error if the operation is not supported for the series' data type.
    pub fn median(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let mut non_null_data: Vec<i32> = data.iter().filter_map(|&x| x).collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data.sort_unstable();
                let mid = non_null_data.len() / 2;
                if non_null_data.len() % 2 == 0 {
                    // Even number of elements
                    let median_val = (non_null_data[mid - 1] + non_null_data[mid]) as f64 / 2.0;
                    Ok(Some(Value::F64(median_val)))
                } else {
                    // Odd number of elements
                    Ok(Some(Value::I32(non_null_data[mid])))
                }
            }
            Series::F64(_, data) => {
                let mut non_null_data: Vec<f64> = data.iter().filter_map(|&x| x).collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let mid = non_null_data.len() / 2;
                if non_null_data.len() % 2 == 0 {
                    // Even number of elements
                    let median_val = (non_null_data[mid - 1] + non_null_data[mid]) / 2.0;
                    Ok(Some(Value::F64(median_val)))
                } else {
                    // Odd number of elements
                    Ok(Some(Value::F64(non_null_data[mid])))
                }
            }
            Series::DateTime(_, data) => {
                let mut non_null_data: Vec<i64> = data.iter().filter_map(|&x| x).collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data.sort_unstable();
                let mid = non_null_data.len() / 2;
                if non_null_data.len() % 2 == 0 {
                    // Even number of elements
                    let median_val = (non_null_data[mid - 1] + non_null_data[mid]) as f64 / 2.0;
                    Ok(Some(Value::F64(median_val)))
                } else {
                    // Odd number of elements
                    Ok(Some(Value::F64(non_null_data[mid] as f64)))
                }
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Median operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Calculates the standard deviation of all non-null values in the series.
    ///
    /// Returns an error if the operation is not supported for the series' data type.
    pub fn std_dev(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let non_null_data: Vec<f64> =
                    data.iter().filter_map(|&x| x.map(|v| v as f64)).collect();
                let n = non_null_data.len();
                if n < 2 {
                    return Ok(None);
                }
                let mean = non_null_data.iter().sum::<f64>() / n as f64;
                let variance = non_null_data
                    .iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>()
                    / (n - 1) as f64;
                Ok(Some(Value::F64(variance.sqrt())))
            }
            Series::F64(_, data) => {
                let non_null_data: Vec<f64> = data.iter().filter_map(|&x| x).collect();
                let n = non_null_data.len();
                if n < 2 {
                    return Ok(None);
                }
                let mean = non_null_data.iter().sum::<f64>() / n as f64;
                let variance = non_null_data
                    .iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>()
                    / (n - 1) as f64;
                Ok(Some(Value::F64(variance.sqrt())))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Standard deviation operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Calculates the Pearson correlation coefficient between this series and another series.
    ///
    /// Returns an error if the series have different lengths or if the operation is not supported
    /// for the series' data types.
    pub fn correlation(&self, other: &Series) -> Result<Option<Value>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have the same length for correlation calculation.".to_string(),
            ));
        }

        match (self, other) {
            (Series::I32(_, data1), Series::I32(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| {
                        x.and_then(|x_val| y.map(|y_val| (x_val as f64, y_val as f64)))
                    })
                    .unzip();
                Self::calculate_correlation(&x_vals, &y_vals)
            }
            (Series::F64(_, data1), Series::F64(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val, y_val))))
                    .unzip();
                Self::calculate_correlation(&x_vals, &y_vals)
            }
            (Series::I32(_, data1), Series::F64(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val as f64, y_val))))
                    .unzip();
                Self::calculate_correlation(&x_vals, &y_vals)
            }
            (Series::F64(_, data1), Series::I32(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val, y_val as f64))))
                    .unzip();
                Self::calculate_correlation(&x_vals, &y_vals)
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Correlation not supported for series of types {:?} and {:?}",
                self.data_type(),
                other.data_type()
            ))),
        }
    }

    /// Helper function to calculate Pearson correlation coefficient.
    fn calculate_correlation(x_vals: &[f64], y_vals: &[f64]) -> Result<Option<Value>, VeloxxError> {
        let n = x_vals.len();
        if n < 2 {
            return Ok(None);
        }

        let sum_x: f64 = x_vals.iter().sum();
        let sum_y: f64 = y_vals.iter().sum();
        let mean_x = sum_x / n as f64;
        let mean_y = sum_y / n as f64;

        let numerator: f64 = x_vals
            .iter()
            .zip(y_vals.iter())
            .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
            .sum();

        let sum_sq_dev_x: f64 = x_vals.iter().map(|&x| (x - mean_x).powi(2)).sum();
        let sum_sq_dev_y: f64 = y_vals.iter().map(|&y| (y - mean_y).powi(2)).sum();

        let denominator = (sum_sq_dev_x * sum_sq_dev_y).sqrt();

        if denominator == 0.0 {
            Ok(Some(Value::F64(0.0))) // Or handle as an error, depending on desired behavior for zero variance
        } else {
            Ok(Some(Value::F64(numerator / denominator)))
        }
    }

    /// Calculates the covariance between this series and another series.
    ///
    /// Returns an error if the series have different lengths or if the operation is not supported
    /// for the series' data types.
    pub fn covariance(&self, other: &Series) -> Result<Option<Value>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation("Series must have the same length for covariance calculation.".to_string()));
        }

        match (self, other) {
            (Series::I32(_, data1), Series::I32(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| {
                        x.and_then(|x_val| y.map(|y_val| (x_val as f64, y_val as f64)))
                    })
                    .unzip();
                Self::calculate_covariance(&x_vals, &y_vals)
            }
            (Series::F64(_, data1), Series::F64(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val, y_val))))
                    .unzip();
                Self::calculate_covariance(&x_vals, &y_vals)
            }
            (Series::I32(_, data1), Series::F64(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val as f64, y_val))))
                    .unzip();
                Self::calculate_covariance(&x_vals, &y_vals)
            }
            (Series::F64(_, data1), Series::I32(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val, y_val as f64))))
                    .unzip();
                Self::calculate_covariance(&x_vals, &y_vals)
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Covariance not supported for series of types {:?} and {:?}",
                self.data_type(),
                other.data_type()
            ))),
        }
    }

    /// Helper function to calculate covariance.
    fn calculate_covariance(x_vals: &[f64], y_vals: &[f64]) -> Result<Option<Value>, VeloxxError> {
        let n = x_vals.len();
        if n < 2 {
            return Ok(None);
        }

        let sum_x: f64 = x_vals.iter().sum();
        let sum_y: f64 = y_vals.iter().sum();
        let mean_x = sum_x / n as f64;
        let mean_y = sum_y / n as f64;

        let numerator: f64 = x_vals
            .iter()
            .zip(y_vals.iter())
            .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
            .sum();

        Ok(Some(Value::F64(numerator / (n - 1) as f64)))
    }

    /// Returns a new series containing only the unique non-null values from this series.
    pub fn unique(&self) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                let mut unique_set = HashSet::new();
                let unique_data: Vec<Option<i32>> = data
                    .iter()
                    .filter_map(|&x| {
                        if unique_set.insert(x) {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .collect();
                Ok(Series::I32(name, unique_data))
            }
            Series::F64(_, data) => {
                let mut unique_bits = HashSet::new();
                let mut unique_data: Vec<Option<f64>> = Vec::new();
                let mut has_null = false;

                for &val_opt in data.iter() {
                    match val_opt {
                        Some(f_val) => {
                            if unique_bits.insert(f_val.to_bits()) {
                                unique_data.push(Some(f_val));
                            }
                        }
                        None => {
                            if !has_null {
                                unique_data.push(None);
                                has_null = true;
                            }
                        }
                    }
                }
                Ok(Series::F64(name, unique_data))
            }
            Series::Bool(_, data) => {
                let mut unique_set = HashSet::new();
                let unique_data: Vec<Option<bool>> = data
                    .iter()
                    .filter_map(|&x| {
                        if unique_set.insert(x) {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .collect();
                Ok(Series::Bool(name, unique_data))
            }
            Series::String(_, data) => {
                let mut unique_set = HashSet::new();
                let unique_data: Vec<Option<String>> = data
                    .iter()
                    .filter_map(|x| {
                        if unique_set.insert(x.clone()) {
                            Some(x.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                Ok(Series::String(name, unique_data))
            }
            Series::DateTime(_, data) => {
                let mut unique_set = HashSet::new();
                let unique_data: Vec<Option<i64>> = data
                    .iter()
                    .filter_map(|&x| {
                        if unique_set.insert(x) {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .collect();
                Ok(Series::DateTime(name, unique_data))
            }
        }
    }

    /// Converts the series data to a `Vec<f64>`, ignoring null values.
    ///
    /// Returns an error if the series' data type cannot be converted to `f64`.
    pub fn to_vec_f64(&self) -> Result<Vec<f64>, VeloxxError> {
        match self {
            Series::I32(_, data) => Ok(data.iter().filter_map(|&x| x.map(|v| v as f64)).collect()),
            Series::F64(_, data) => Ok(data.iter().filter_map(|&x| x).collect()),
            Series::DateTime(_, data) => {
                Ok(data.iter().filter_map(|&x| x.map(|v| v as f64)).collect())
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Cannot convert series of type {:?} to Vec<f64>.",
                self.data_type()
            ))),
        }
    }

    /// Interpolates null values in the series using linear interpolation.
    ///
    /// This operation is only supported for numeric (I32, F64) series.
    /// Nulls at the beginning or end of the series, or consecutive nulls
    /// where no surrounding non-null values exist, will remain null.
    pub fn interpolate_nulls(&self) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                let mut interpolated_data = Vec::with_capacity(data.len());
                let mut last_known_idx: Option<usize> = None;
                let mut next_known_indices: Vec<Option<usize>> = vec![None; data.len()];
                let mut last_non_null_idx: Option<usize> = None;

                // Pre-calculate next known indices
                for i in (0..data.len()).rev() {
                    if data[i].is_some() {
                        last_non_null_idx = Some(i);
                    }
                    next_known_indices[i] = last_non_null_idx;
                }

                // Forward pass for interpolation
                for i in 0..data.len() {
                    if data[i].is_some() {
                        interpolated_data.push(data[i]);
                        last_known_idx = Some(i);
                    } else if let Some(prev_idx) = last_known_idx {
                        if let Some(next_idx) = next_known_indices[i] {
                            let prev_val = data[prev_idx].unwrap() as f64;
                            let next_val = data[next_idx].unwrap() as f64;
                            let interpolated_val = prev_val
                                + (next_val - prev_val)
                                    * ((i - prev_idx) as f64 / (next_idx - prev_idx) as f64);
                            interpolated_data.push(Some(interpolated_val as i32));
                        } else {
                            interpolated_data.push(None);
                        }
                    } else {
                        interpolated_data.push(None);
                    }
                }
                Ok(Series::I32(name, interpolated_data))
            }
            Series::F64(_, data) => {
                let mut interpolated_data = Vec::with_capacity(data.len());
                let mut last_known_idx: Option<usize> = None;
                let mut next_known_indices: Vec<Option<usize>> = vec![None; data.len()];
                let mut last_non_null_idx: Option<usize> = None;

                // Pre-calculate next known indices
                for i in (0..data.len()).rev() {
                    if data[i].is_some() {
                        last_non_null_idx = Some(i);
                    }
                    next_known_indices[i] = last_non_null_idx;
                }

                // Forward pass for interpolation
                for i in 0..data.len() {
                    if data[i].is_some() {
                        interpolated_data.push(data[i]);
                        last_known_idx = Some(i);
                    } else if let Some(prev_idx) = last_known_idx {
                        if let Some(next_idx) = next_known_indices[i] {
                            let prev_val = data[prev_idx].unwrap();
                            let next_val = data[next_idx].unwrap();
                            let interpolated_val = prev_val
                                + (next_val - prev_val)
                                    * ((i - prev_idx) as f64 / (next_idx - prev_idx) as f64);
                            interpolated_data.push(Some(interpolated_val));
                        } else {
                            interpolated_data.push(None);
                        }
                    } else {
                        interpolated_data.push(None);
                    }
                }
                Ok(Series::F64(name, interpolated_data))
            }
            Series::DateTime(_, data) => {
                let mut interpolated_data = data.clone();
                let mut last_known_idx: Option<usize> = None;

                // Forward pass
                for i in 0..interpolated_data.len() {
                    if interpolated_data[i].is_some() {
                        last_known_idx = Some(i);
                    } else if let Some(prev_idx) = last_known_idx {
                        // Find next non-null value
                        let next_known_idx =
                            (i..interpolated_data.len()).find(|&j| interpolated_data[j].is_some());

                        if let Some(next_idx) = next_known_idx {
                            let prev_val = interpolated_data[prev_idx].unwrap() as f64;
                            let next_val = interpolated_data[next_idx].unwrap() as f64;
                            let interpolated_val = prev_val
                                + (next_val - prev_val)
                                    * ((i - prev_idx) as f64 / (next_idx - prev_idx) as f64);
                            interpolated_data[i] = Some(interpolated_val as i64);
                        }
                    }
                }
                Ok(Series::DateTime(name, interpolated_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Interpolate nulls operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of the series, returning a new series of the same type.
    ///
    /// The function `f` takes an `Option<T>` (where `T` is the series' underlying type) and returns an `Option<T>`.
    /// This method is type-specific and avoids the overhead of `Value` enum conversions.
    pub fn apply_i32<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<i32>) -> Option<i32>,
    {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                let new_data = data.iter().map(|&x| f(x)).collect();
                Ok(Series::I32(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_i32.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of the series, returning a new series of the same type.
    pub fn apply_f64<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<f64>) -> Option<f64>,
    {
        let name = self.name().to_string();
        match self {
            Series::F64(_, data) => {
                let new_data = data.iter().map(|&x| f(x)).collect();
                Ok(Series::F64(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_f64.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of the series, returning a new series of the same type.
    pub fn apply_bool<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<bool>) -> Option<bool>,
    {
        let name = self.name().to_string();
        match self {
            Series::Bool(_, data) => {
                let new_data = data.iter().map(|&x| f(x)).collect();
                Ok(Series::Bool(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_bool.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of the series, returning a new series of the same type.
    pub fn apply_string<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<&String>) -> Option<String>,
    {
        let name = self.name().to_string();
        match self {
            Series::String(_, data) => {
                let new_data = data.iter().map(|x| f(x.as_ref())).collect();
                Ok(Series::String(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_string.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of the series, returning a new series of the same type.
    pub fn apply_datetime<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<i64>) -> Option<i64>,
    {
        let name = self.name().to_string();
        match self {
            Series::DateTime(_, data) => {
                let new_data = data.iter().map(|&x| f(x)).collect();
                Ok(Series::DateTime(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_datetime.",
                self.data_type()
            ))),
        }
    }
}
