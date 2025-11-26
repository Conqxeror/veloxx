use crate::series::Series;
use crate::types::Value;
use crate::VeloxxError;
use rayon::prelude::*;

impl Series {
    /// Calculate the sum of all values in the series
    pub fn sum(&self) -> Result<Value, VeloxxError> {
        match self {
            Series::I32(_, values, bitmap) => {
                let sum: i32 = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .sum();
                Ok(Value::I32(sum))
            }
            Series::F64(_, values, bitmap) => {
                let sum: f64 = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .sum();
                Ok(Value::F64(sum))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Sum operation not supported for this data type".to_string(),
            )),
        }
    }

    /// Calculate the minimum value in the series
    pub fn min(&self) -> Result<Value, VeloxxError> {
        match self {
            Series::I32(_, values, bitmap) => {
                let min = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .min();
                match min {
                    Some(val) => Ok(Value::I32(val)),
                    None => Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    )),
                }
            }
            Series::F64(_, values, bitmap) => {
                let min = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .reduce(|| f64::INFINITY, f64::min);
                if min == f64::INFINITY {
                    Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    ))
                } else {
                    Ok(Value::F64(min))
                }
            }
            Series::String(_, values, bitmap) => {
                let min = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, &b)| if b { Some(v) } else { None })
                    .min();
                match min {
                    Some(val) => Ok(Value::String(val.clone())),
                    None => Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    )),
                }
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Min operation not supported for this data type".to_string(),
            )),
        }
    }

    /// Calculate the maximum value in the series
    pub fn max(&self) -> Result<Value, VeloxxError> {
        match self {
            Series::I32(_, values, bitmap) => {
                let max = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .max();
                match max {
                    Some(val) => Ok(Value::I32(val)),
                    None => Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    )),
                }
            }
            Series::F64(_, values, bitmap) => {
                let max = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .reduce(|| f64::NEG_INFINITY, f64::max);
                if max == f64::NEG_INFINITY {
                    Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    ))
                } else {
                    Ok(Value::F64(max))
                }
            }
            Series::String(_, values, bitmap) => {
                let max = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, &b)| if b { Some(v) } else { None })
                    .max();
                match max {
                    Some(val) => Ok(Value::String(val.clone())),
                    None => Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    )),
                }
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Max operation not supported for this data type".to_string(),
            )),
        }
    }

    /// Calculate the mean of all values in the series
    pub fn mean(&self) -> Result<Value, VeloxxError> {
        match self {
            Series::I32(_, values, bitmap) => {
                let valid_values: Vec<i32> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if valid_values.is_empty() {
                    return Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    ));
                }
                let sum: i32 = valid_values.iter().sum();
                Ok(Value::F64(sum as f64 / valid_values.len() as f64))
            }
            Series::F64(_, values, bitmap) => {
                let valid_values: Vec<f64> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if valid_values.is_empty() {
                    return Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    ));
                }
                let sum: f64 = valid_values.iter().sum();
                Ok(Value::F64(sum / valid_values.len() as f64))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Mean operation not supported for this data type".to_string(),
            )),
        }
    }

    /// Calculate the standard deviation of all values in the series
    pub fn std_dev(&self) -> Result<Value, VeloxxError> {
        match self {
            Series::I32(_, values, bitmap) => {
                let valid_values: Vec<i32> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if valid_values.is_empty() {
                    return Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    ));
                }
                let mean: f64 =
                    valid_values.iter().map(|&x| x as f64).sum::<f64>() / valid_values.len() as f64;
                let variance = if valid_values.len() > 1 {
                    valid_values
                        .iter()
                        .map(|&x| {
                            let diff = x as f64 - mean;
                            diff * diff
                        })
                        .sum::<f64>()
                        / (valid_values.len() - 1) as f64
                } else {
                    return Err(VeloxxError::InvalidOperation(
                        "Standard deviation requires at least 2 values".to_string(),
                    ));
                };
                Ok(Value::F64(variance.sqrt()))
            }
            Series::F64(_, values, bitmap) => {
                let valid_values: Vec<f64> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if valid_values.is_empty() {
                    return Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    ));
                }
                let mean: f64 = valid_values.iter().sum::<f64>() / valid_values.len() as f64;
                let variance = if valid_values.len() > 1 {
                    valid_values
                        .iter()
                        .map(|&x| {
                            let diff = x - mean;
                            diff * diff
                        })
                        .sum::<f64>()
                        / (valid_values.len() - 1) as f64
                } else {
                    return Err(VeloxxError::InvalidOperation(
                        "Standard deviation requires at least 2 values".to_string(),
                    ));
                };
                Ok(Value::F64(variance.sqrt()))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Standard deviation operation not supported for this data type".to_string(),
            )),
        }
    }

    /// Calculate the median of all values in the series
    pub fn median(&self) -> Result<Value, VeloxxError> {
        match self {
            Series::I32(_, values, bitmap) => {
                let mut valid_values: Vec<i32> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if valid_values.is_empty() {
                    return Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    ));
                }
                valid_values.sort_unstable();
                let len = valid_values.len();
                let median = if len.is_multiple_of(2) {
                    (valid_values[len / 2 - 1] + valid_values[len / 2]) as f64 / 2.0
                } else {
                    valid_values[len / 2] as f64
                };
                Ok(Value::F64(median))
            }
            Series::F64(_, values, bitmap) => {
                let mut valid_values: Vec<f64> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if valid_values.is_empty() {
                    return Err(VeloxxError::InvalidOperation(
                        "No valid values in series".to_string(),
                    ));
                }
                valid_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let len = valid_values.len();
                let median = if len.is_multiple_of(2) {
                    (valid_values[len / 2 - 1] + valid_values[len / 2]) / 2.0
                } else {
                    valid_values[len / 2]
                };
                Ok(Value::F64(median))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Median operation not supported for this data type".to_string(),
            )),
        }
    }

    /// Get unique values in the series
    pub fn unique(&self) -> Result<Series, VeloxxError> {
        match self {
            Series::I32(name, values, bitmap) => {
                use std::collections::HashSet;
                let mut unique_values = Vec::new();
                let mut unique_bitmap = Vec::new();
                let mut seen = HashSet::new();
                let mut has_null = false;

                for (&val, &valid) in values.iter().zip(bitmap.iter()) {
                    if valid && seen.insert(val) {
                        unique_values.push(val);
                        unique_bitmap.push(true);
                    } else if !valid && !has_null {
                        // Include one null value if it exists
                        has_null = true;
                        unique_values.push(0); // placeholder for null
                        unique_bitmap.push(false);
                    }
                }

                Ok(Series::I32(name.clone(), unique_values, unique_bitmap))
            }
            Series::F64(name, values, bitmap) => {
                use std::collections::HashSet;
                let mut unique_values = Vec::new();
                let mut unique_bitmap = Vec::new();
                let mut seen = HashSet::new();
                let mut has_null = false;

                for (&val, &valid) in values.iter().zip(bitmap.iter()) {
                    if valid {
                        let key = val.to_bits(); // Use bit representation for hashing
                        if seen.insert(key) {
                            unique_values.push(val);
                            unique_bitmap.push(true);
                        }
                    } else if !valid && !has_null {
                        // Include one null value if it exists
                        has_null = true;
                        unique_values.push(0.0); // placeholder for null
                        unique_bitmap.push(false);
                    }
                }

                Ok(Series::F64(name.clone(), unique_values, unique_bitmap))
            }
            Series::String(name, values, bitmap) => {
                use std::collections::HashSet;
                let mut unique_values = Vec::new();
                let mut unique_bitmap = Vec::new();
                let mut seen = HashSet::new();
                let mut has_null = false;

                for (val, &valid) in values.iter().zip(bitmap.iter()) {
                    if valid && seen.insert(val.clone()) {
                        unique_values.push(val.clone());
                        unique_bitmap.push(true);
                    } else if !valid && !has_null {
                        // Include one null value if it exists
                        has_null = true;
                        unique_values.push(String::new()); // placeholder for null
                        unique_bitmap.push(false);
                    }
                }

                Ok(Series::String(name.clone(), unique_values, unique_bitmap))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Unique operation not supported for this data type".to_string(),
            )),
        }
    }
}
