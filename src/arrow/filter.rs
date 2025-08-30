//! Filtering operations using Arrow's compute functions

#[cfg(feature = "arrow")]
use arrow_array::{Array, BooleanArray, Float64Array, Int32Array, StringArray};
#[cfg(feature = "arrow")]
use std::sync::Arc;

#[cfg(feature = "arrow")]
use arrow_ord::cmp;

#[cfg(feature = "arrow")]
use crate::arrow::series::ArrowSeries;
#[cfg(feature = "arrow")]
use crate::VeloxxError;

/// Filtering operations for Arrow Series
#[cfg(feature = "arrow")]
pub trait ArrowFilter {
    /// Filter the series using a boolean mask
    fn filter(&self, mask: &BooleanArray) -> Result<ArrowSeries, VeloxxError>;

    /// Create a boolean mask for equality comparison
    fn equal(&self, other: &Self) -> Result<BooleanArray, VeloxxError>;

    /// Create a boolean mask for greater than comparison
    fn gt(&self, other: &Self) -> Result<BooleanArray, VeloxxError>;

    /// Create a boolean mask for less than comparison
    fn lt(&self, other: &Self) -> Result<BooleanArray, VeloxxError>;

    /// Create a boolean mask for greater than or equal comparison
    fn gte(&self, other: &Self) -> Result<BooleanArray, VeloxxError>;

    /// Create a boolean mask for less than or equal comparison
    fn lte(&self, other: &Self) -> Result<BooleanArray, VeloxxError>;

    /// Combine two boolean masks with AND operation
    fn and(&self, other: &BooleanArray) -> Result<BooleanArray, VeloxxError>;

    /// Combine two boolean masks with OR operation
    fn or(&self, other: &BooleanArray) -> Result<BooleanArray, VeloxxError>;
}

#[cfg(feature = "arrow")]
impl ArrowFilter for ArrowSeries {
    fn filter(&self, mask: &BooleanArray) -> Result<ArrowSeries, VeloxxError> {
        if self.len() != mask.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series and mask must have same length for filtering".to_string(),
            ));
        }

        match self {
            ArrowSeries::F64(name, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                let filtered = arrow_select::filter::filter(arr, mask)?;

                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(filtered),
                    None, // Simplified null handling for now
                ))
            }
            ArrowSeries::I32(name, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                let filtered = arrow_select::filter::filter(arr, mask)?;

                Ok(ArrowSeries::I32(
                    name.clone(),
                    Arc::new(filtered),
                    None, // Simplified null handling for now
                ))
            }
            ArrowSeries::String(name, array, _) => {
                let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
                let filtered = arrow_select::filter::filter(arr, mask)?;

                Ok(ArrowSeries::String(
                    name.clone(),
                    Arc::new(filtered),
                    None, // Simplified null handling for now
                ))
            }
            ArrowSeries::Bool(name, array, _) => {
                let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
                let filtered = arrow_select::filter::filter(arr, mask)?;

                Ok(ArrowSeries::Bool(
                    name.clone(),
                    Arc::new(filtered),
                    None, // Simplified null handling for now
                ))
            }
        }
    }

    fn equal(&self, other: &Self) -> Result<BooleanArray, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for comparison".to_string(),
            ));
        }

        match (self, other) {
            (ArrowSeries::F64(_, a, _), ArrowSeries::F64(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();
                Ok(cmp::eq(a_arr, b_arr)?)
            }
            (ArrowSeries::I32(_, a, _), ArrowSeries::I32(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Int32Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Int32Array>().unwrap();
                Ok(cmp::eq(a_arr, b_arr)?)
            }
            (ArrowSeries::String(_, a, _), ArrowSeries::String(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<StringArray>().unwrap();
                let b_arr = b.as_any().downcast_ref::<StringArray>().unwrap();
                Ok(cmp::eq(a_arr, b_arr)?)
            }
            (ArrowSeries::Bool(_, a, _), ArrowSeries::Bool(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<BooleanArray>().unwrap();
                let b_arr = b.as_any().downcast_ref::<BooleanArray>().unwrap();
                Ok(cmp::eq(a_arr, b_arr)?)
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Comparison only supported for series of same type".to_string(),
            )),
        }
    }

    fn gt(&self, other: &Self) -> Result<BooleanArray, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for comparison".to_string(),
            ));
        }

        match (self, other) {
            (ArrowSeries::F64(_, a, _), ArrowSeries::F64(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();
                Ok(cmp::gt(a_arr, b_arr)?)
            }
            (ArrowSeries::I32(_, a, _), ArrowSeries::I32(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Int32Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Int32Array>().unwrap();
                Ok(cmp::gt(a_arr, b_arr)?)
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Greater than comparison only supported for numeric series".to_string(),
            )),
        }
    }

    fn lt(&self, other: &Self) -> Result<BooleanArray, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for comparison".to_string(),
            ));
        }

        match (self, other) {
            (ArrowSeries::F64(_, a, _), ArrowSeries::F64(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();
                Ok(cmp::lt(a_arr, b_arr)?)
            }
            (ArrowSeries::I32(_, a, _), ArrowSeries::I32(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Int32Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Int32Array>().unwrap();
                Ok(cmp::lt(a_arr, b_arr)?)
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Less than comparison only supported for numeric series".to_string(),
            )),
        }
    }

    fn gte(&self, other: &Self) -> Result<BooleanArray, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for comparison".to_string(),
            ));
        }

        match (self, other) {
            (ArrowSeries::F64(_, a, _), ArrowSeries::F64(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();
                Ok(cmp::gt_eq(a_arr, b_arr)?)
            }
            (ArrowSeries::I32(_, a, _), ArrowSeries::I32(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Int32Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Int32Array>().unwrap();
                Ok(cmp::gt_eq(a_arr, b_arr)?)
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Greater than or equal comparison only supported for numeric series".to_string(),
            )),
        }
    }

    fn lte(&self, other: &Self) -> Result<BooleanArray, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for comparison".to_string(),
            ));
        }

        match (self, other) {
            (ArrowSeries::F64(_, a, _), ArrowSeries::F64(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();
                Ok(cmp::lt_eq(a_arr, b_arr)?)
            }
            (ArrowSeries::I32(_, a, _), ArrowSeries::I32(_, b, _)) => {
                let a_arr = a.as_any().downcast_ref::<Int32Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Int32Array>().unwrap();
                Ok(cmp::lt_eq(a_arr, b_arr)?)
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Less than or equal comparison only supported for numeric series".to_string(),
            )),
        }
    }

    fn and(&self, other: &BooleanArray) -> Result<BooleanArray, VeloxxError> {
        match self {
            ArrowSeries::Bool(_, array, _) => {
                let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
                // Manually implement AND operation
                let mut result = Vec::with_capacity(arr.len());
                for i in 0..arr.len() {
                    let a = arr.value(i);
                    let b = other.value(i);
                    result.push(a && b);
                }
                Ok(BooleanArray::from(result))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "AND operation only supported for boolean series".to_string(),
            )),
        }
    }

    fn or(&self, other: &BooleanArray) -> Result<BooleanArray, VeloxxError> {
        match self {
            ArrowSeries::Bool(_, array, _) => {
                let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
                // Manually implement OR operation
                let mut result = Vec::with_capacity(arr.len());
                for i in 0..arr.len() {
                    let a = arr.value(i);
                    let b = other.value(i);
                    result.push(a || b);
                }
                Ok(BooleanArray::from(result))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "OR operation only supported for boolean series".to_string(),
            )),
        }
    }
}
