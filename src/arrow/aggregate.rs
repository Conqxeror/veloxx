//! Aggregation operations using Arrow's compute functions

#[cfg(feature = "arrow")]
use arrow_array::{Float64Array, Int32Array};

#[cfg(feature = "arrow")]
use arrow_arith::aggregate;

#[cfg(feature = "arrow")]
use crate::arrow::series::ArrowSeries;
#[cfg(feature = "arrow")]
use crate::types::Value;
#[cfg(feature = "arrow")]
use crate::VeloxxError;

/// Aggregation operations for Arrow Series
#[cfg(feature = "arrow")]
pub trait ArrowAggregate {
    /// Calculate the mean of the series
    fn mean(&self) -> Result<Option<Value>, VeloxxError>;

    /// Calculate the minimum value of the series
    fn min(&self) -> Result<Option<Value>, VeloxxError>;

    /// Calculate the maximum value of the series
    fn max(&self) -> Result<Option<Value>, VeloxxError>;

    /// Calculate the standard deviation of the series
    fn std(&self) -> Result<Option<Value>, VeloxxError>;
}

#[cfg(feature = "arrow")]
impl ArrowAggregate for ArrowSeries {
    fn mean(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            ArrowSeries::F64(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                let sum = aggregate::sum(arr).unwrap_or(0.0);
                let count = arr.len() as f64;
                let mean = if count > 0.0 { sum / count } else { 0.0 };
                Ok(Some(Value::F64(mean)))
            }
            ArrowSeries::I32(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                let sum = aggregate::sum(arr).unwrap_or(0) as f64;
                let count = arr.len() as f64;
                let mean = if count > 0.0 { sum / count } else { 0.0 };
                Ok(Some(Value::F64(mean)))
            }
            _ => Err(VeloxxError::Unsupported(
                "Mean operation not supported for this series type".to_string(),
            )),
        }
    }

    fn min(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            ArrowSeries::F64(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                let min = aggregate::min(arr);
                Ok(min.map(Value::F64))
            }
            ArrowSeries::I32(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                let min = aggregate::min(arr);
                Ok(min.map(Value::I32))
            }
            _ => Err(VeloxxError::Unsupported(
                "Min operation not supported for this series type".to_string(),
            )),
        }
    }

    fn max(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            ArrowSeries::F64(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                let max = aggregate::max(arr);
                Ok(max.map(Value::F64))
            }
            ArrowSeries::I32(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                let max = aggregate::max(arr);
                Ok(max.map(Value::I32))
            }
            _ => Err(VeloxxError::Unsupported(
                "Max operation not supported for this series type".to_string(),
            )),
        }
    }

    fn std(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            ArrowSeries::F64(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                // Calculate standard deviation using the formula: sqrt(E[X^2] - E[X]^2)
                let mean = self.mean()?.unwrap().as_f64().unwrap();

                // Calculate sum of squares
                let squared_diffs: Vec<f64> =
                    arr.values().iter().map(|&x| (x - mean).powi(2)).collect();
                let squared_arr = Float64Array::from(squared_diffs);
                let sum_squared_diffs = aggregate::sum(&squared_arr).unwrap_or(0.0);
                let variance = sum_squared_diffs / arr.len() as f64;
                let std_dev = variance.sqrt();

                Ok(Some(Value::F64(std_dev)))
            }
            ArrowSeries::I32(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                // Calculate standard deviation using the formula: sqrt(E[X^2] - E[X]^2)
                let mean = self.mean()?.unwrap().as_f64().unwrap();

                // Calculate sum of squares
                let squared_diffs: Vec<f64> = arr
                    .values()
                    .iter()
                    .map(|&x| (x as f64 - mean).powi(2))
                    .collect();
                let squared_arr = Float64Array::from(squared_diffs);
                let sum_squared_diffs = aggregate::sum(&squared_arr).unwrap_or(0.0);
                let variance = sum_squared_diffs / arr.len() as f64;
                let std_dev = variance.sqrt();

                Ok(Some(Value::F64(std_dev)))
            }
            _ => Err(VeloxxError::Unsupported(
                "Standard deviation operation not supported for this series type".to_string(),
            )),
        }
    }
}
