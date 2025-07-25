use crate::error::VeloxxError;
use crate::types::{DataType, Value};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub enum Series {
    I32(String, Vec<Option<i32>>),
    F64(String, Vec<Option<f64>>),
    Bool(String, Vec<Option<bool>>),
    String(String, Vec<Option<String>>),
    DateTime(String, Vec<Option<i64>>),
}

impl Series {
    pub fn new_i32(name: &str, data: Vec<Option<i32>>) -> Self {
        Series::I32(name.to_string(), data)
    }

    pub fn new_f64(name: &str, data: Vec<Option<f64>>) -> Self {
        Series::F64(name.to_string(), data)
    }

    pub fn new_bool(name: &str, data: Vec<Option<bool>>) -> Self {
        Series::Bool(name.to_string(), data)
    }

    pub fn new_string(name: &str, data: Vec<Option<String>>) -> Self {
        Series::String(name.to_string(), data)
    }

    pub fn new_datetime(name: &str, data: Vec<Option<i64>>) -> Self {
        Series::DateTime(name.to_string(), data)
    }

    pub fn name(&self) -> &str {
        match self {
            Series::I32(name, _) => name,
            Series::F64(name, _) => name,
            Series::Bool(name, _) => name,
            Series::String(name, _) => name,
            Series::DateTime(name, _) => name,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Series::I32(_, v) => v.len(),
            Series::F64(_, v) => v.len(),
            Series::Bool(_, v) => v.len(),
            Series::String(_, v) => v.len(),
            Series::DateTime(_, v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn data_type(&self) -> DataType {
        match self {
            Series::I32(_, _) => DataType::I32,
            Series::F64(_, _) => DataType::F64,
            Series::Bool(_, _) => DataType::Bool,
            Series::String(_, _) => DataType::String,
            Series::DateTime(_, _) => DataType::DateTime,
        }
    }

    pub fn set_name(&mut self, new_name: &str) {
        match self {
            Series::I32(name, _) => *name = new_name.to_string(),
            Series::F64(name, _) => *name = new_name.to_string(),
            Series::Bool(name, _) => *name = new_name.to_string(),
            Series::String(name, _) => *name = new_name.to_string(),
            Series::DateTime(name, _) => *name = new_name.to_string(),
        }
    }

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

    pub fn count(&self) -> usize {
        match self {
            Series::I32(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::F64(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::Bool(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::String(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::DateTime(_, data) => data.iter().filter(|x| x.is_some()).count(),
        }
    }

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
                let min_val = data
                    .iter()
                    .filter_map(|x| x.as_ref())
                    .min_by(|a, b| a.cmp(b));
                Ok(min_val.map(|s| Value::String(s.clone())))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Min operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

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
                let max_val = data
                    .iter()
                    .filter_map(|x| x.as_ref())
                    .max_by(|a, b| a.cmp(b));
                Ok(max_val.map(|s| Value::String(s.clone())))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Max operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    pub fn mean(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let (sum, count) = data.iter().fold((0, 0), |(sum, count), &x| {
                    if let Some(val) = x {
                        (sum + val as i64, count + 1)
                    } else {
                        (sum, count)
                    }
                });
                if count > 0 {
                    Ok(Some(Value::F64(sum as f64 / count as f64)))
                } else {
                    Ok(None)
                }
            }
            Series::F64(_, data) => {
                let (sum, count) = data.iter().fold((0.0, 0), |(sum, count), &x| {
                    if let Some(val) = x {
                        (sum + val, count + 1)
                    } else {
                        (sum, count)
                    }
                });
                if count > 0 {
                    Ok(Some(Value::F64(sum / count as f64)))
                } else {
                    Ok(None)
                }
            }
            Series::DateTime(_, data) => {
                let (sum, count) = data.iter().fold((0, 0), |(sum, count), &x| {
                    if let Some(val) = x {
                        (sum + val, count + 1)
                    } else {
                        (sum, count)
                    }
                });
                if count > 0 {
                    Ok(Some(Value::F64(sum as f64 / count as f64)))
                } else {
                    Ok(None)
                }
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Mean operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

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
                    let median_val = (non_null_data[mid - 1] + non_null_data[mid]) as f64 / 2.0;
                    Ok(Some(Value::F64(median_val)))
                } else {
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
                    let median_val = (non_null_data[mid - 1] + non_null_data[mid]) / 2.0;
                    Ok(Some(Value::F64(median_val)))
                } else {
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
                    let median_val = (non_null_data[mid - 1] + non_null_data[mid]) as f64 / 2.0;
                    Ok(Some(Value::F64(median_val)))
                } else {
                    Ok(Some(Value::F64(non_null_data[mid] as f64)))
                }
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Median operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    pub fn to_list(&self) -> Vec<Value> {
        match self {
            Series::I32(_, data) => data
                .iter()
                .map(|&x| x.map(Value::I32).unwrap_or(Value::Null))
                .collect(),
            Series::F64(_, data) => data
                .iter()
                .map(|&x| x.map(Value::F64).unwrap_or(Value::Null))
                .collect(),
            Series::Bool(_, data) => data
                .iter()
                .map(|&x| x.map(Value::Bool).unwrap_or(Value::Null))
                .collect(),
            Series::String(_, data) => data
                .iter()
                .map(|x| {
                    x.as_ref()
                        .map(|s| Value::String(s.clone()))
                        .unwrap_or(Value::Null)
                })
                .collect(),
            Series::DateTime(_, data) => data
                .iter()
                .map(|&x| x.map(Value::DateTime).unwrap_or(Value::Null))
                .collect(),
        }
    }

    pub fn std_dev(&self) -> Result<Option<Value>, VeloxxError> {
        match self.mean()? {
            Some(Value::F64(mean_val)) => {
                let count = self.count();
                if count <= 1 {
                    return Ok(None);
                }

                let variance = match self {
                    Series::I32(_, data) => {
                        data.iter()
                            .filter_map(|&x| x)
                            .map(|x| (x as f64 - mean_val).powi(2))
                            .sum::<f64>()
                            / (count - 1) as f64
                    }
                    Series::F64(_, data) => {
                        data.iter()
                            .filter_map(|&x| x)
                            .map(|x| (x - mean_val).powi(2))
                            .sum::<f64>()
                            / (count - 1) as f64
                    }
                    Series::DateTime(_, data) => {
                        data.iter()
                            .filter_map(|&x| x)
                            .map(|x| (x as f64 - mean_val).powi(2))
                            .sum::<f64>()
                            / (count - 1) as f64
                    }
                    _ => {
                        return Err(VeloxxError::Unsupported(format!(
                            "Std Dev not supported for {:?} series",
                            self.data_type()
                        )))
                    }
                };

                Ok(Some(Value::F64(variance.sqrt())))
            }
            Some(_) => Err(VeloxxError::InvalidOperation(
                "Mean calculation did not return a F64 value.".to_string(),
            )),
            None => Ok(None),
        }
    }

    pub fn to_vec_f64(&self) -> Result<Vec<f64>, VeloxxError> {
        match self {
            Series::I32(_, data) => Ok(data
                .iter()
                .filter_map(|&x| x)
                .map(|val| val as f64)
                .collect()),
            Series::F64(_, data) => Ok(data.iter().filter_map(|&x| x).collect()),
            Series::DateTime(_, data) => Ok(data
                .iter()
                .filter_map(|&x| x)
                .map(|val| val as f64)
                .collect()),
            _ => Err(VeloxxError::Unsupported(format!(
                "Cannot convert series of type {:?} to Vec<f64>",
                self.data_type()
            ))),
        }
    }

    pub fn unique(&self) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                let mut unique_data = Vec::new();
                let mut seen = HashSet::new();
                for item in data.iter().filter_map(|&x| x) {
                    if seen.insert(item) {
                        unique_data.push(Some(item));
                    }
                }
                Ok(Series::I32(name, unique_data))
            }
            Series::F64(_, data) => {
                let mut unique_data = Vec::new();
                let mut seen = HashSet::new();
                for item in data.iter().filter_map(|&x| x) {
                    if seen.insert(item.to_bits()) {
                        unique_data.push(Some(item));
                    }
                }
                Ok(Series::F64(name, unique_data))
            }
            Series::Bool(_, data) => {
                let mut unique_data = Vec::new();
                let mut seen = HashSet::new();
                for item in data.iter().filter_map(|&x| x) {
                    if seen.insert(item) {
                        unique_data.push(Some(item));
                    }
                }
                Ok(Series::Bool(name, unique_data))
            }
            Series::String(_, data) => {
                let mut unique_data = Vec::new();
                let mut seen = HashSet::new();
                for item in data.iter().filter_map(|x| x.as_ref()) {
                    if seen.insert(item.clone()) {
                        unique_data.push(Some(item.clone()));
                    }
                }
                Ok(Series::String(name, unique_data))
            }
            Series::DateTime(_, data) => {
                let mut unique_data = Vec::new();
                let mut seen = HashSet::new();
                for item in data.iter().filter_map(|&x| x) {
                    if seen.insert(item) {
                        unique_data.push(Some(item));
                    }
                }
                Ok(Series::DateTime(name, unique_data))
            }
        }
    }

    pub fn correlation(&self, other: &Series) -> Result<Option<f64>, VeloxxError> {
        let self_f64 = self.to_vec_f64()?;
        let other_f64 = other.to_vec_f64()?;

        if self_f64.len() != other_f64.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have the same length for correlation".to_string(),
            ));
        }

        let n = self_f64.len() as f64;
        if n == 0.0 {
            return Ok(None);
        }

        let sum_x = self_f64.iter().sum::<f64>();
        let sum_y = other_f64.iter().sum::<f64>();

        let sum_x_sq = self_f64.iter().map(|&x| x * x).sum::<f64>();
        let sum_y_sq = other_f64.iter().map(|&y| y * y).sum::<f64>();

        let sum_xy = self_f64
            .iter()
            .zip(other_f64.iter())
            .map(|(&x, &y)| x * y)
            .sum::<f64>();

        let numerator = n * sum_xy - sum_x * sum_y;
        let denominator = ((n * sum_x_sq - sum_x.powi(2)) * (n * sum_y_sq - sum_y.powi(2))).sqrt();

        if denominator == 0.0 {
            Ok(None)
        } else {
            Ok(Some(numerator / denominator))
        }
    }

    pub fn covariance(&self, other: &Series) -> Result<Option<f64>, VeloxxError> {
        let self_f64 = self.to_vec_f64()?;
        let other_f64 = other.to_vec_f64()?;

        if self_f64.len() != other_f64.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have the same length for covariance".to_string(),
            ));
        }

        let n = self_f64.len();
        if n == 0 {
            return Ok(None);
        }

        let mean_x = self_f64.iter().sum::<f64>() / n as f64;
        let mean_y = other_f64.iter().sum::<f64>() / n as f64;

        let cov = self_f64
            .iter()
            .zip(other_f64.iter())
            .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
            .sum::<f64>()
            / (n - 1) as f64;

        Ok(Some(cov))
    }

    /// Checks if the series contains numeric data (I32 or F64).
    pub fn is_numeric(&self) -> bool {
        matches!(self, Series::I32(_, _) | Series::F64(_, _))
    }

    /// Interpolates null values using linear interpolation for numeric series.
    ///
    /// This method performs linear interpolation on null values. It only works
    /// on numeric series (I32 and F64). Null values at the beginning or end
    /// of the series remain as null.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Series)` containing a new series with interpolated values,
    /// or `Err(VeloxxError)` if interpolation is not supported for this series type.
    pub fn interpolate_nulls(&self) -> Result<Self, VeloxxError> {
        if !self.is_numeric() {
            return Err(VeloxxError::Unsupported(
                "Interpolation only supported for numeric series".to_string(),
            ));
        }

        let name = self.name().to_string();
        let data: Vec<Option<f64>> = match self {
            Series::I32(_, d) => d.iter().map(|&x| x.map(|v| v as f64)).collect(),
            Series::F64(_, d) => d.clone(),
            _ => unreachable!(),
        };

        let mut interpolated: Vec<Option<f64>> = Vec::with_capacity(data.len());
        let mut i = 0;

        while i < data.len() {
            if let Some(val) = data[i] {
                interpolated.push(Some(val));
                i += 1;
                continue;
            }

            let start = i;
            while i < data.len() && data[i].is_none() {
                i += 1;
            }
            let end = i;

            let fill_values = if start == 0 || end == data.len() {
                vec![None; end - start]
            } else {
                let prev = data[start - 1].unwrap();
                let next = data[end].unwrap();
                let count = (end - start + 1) as f64;
                (0..(end - start))
                    .map(|j| Some(prev + ((j as f64 + 1.0) / count) * (next - prev)))
                    .collect()
            };

            interpolated.extend(fill_values);
        }

        match self {
            Series::I32(_, _) => Ok(Series::new_i32(
                &name,
                interpolated
                    .iter()
                    .map(|&x| x.map(|v| v.round() as i32))
                    .collect(),
            )),
            Series::F64(_, _) => Ok(Series::new_f64(&name, interpolated)),
            _ => unreachable!(),
        }
    }
}
