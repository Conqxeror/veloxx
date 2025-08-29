//! Arrow array utilities
//!
//! This module provides utilities for working with Arrow arrays.

#[cfg(feature = "arrow")]
use arrow_array::{ArrayRef, Float64Array, Int32Array};
#[cfg(feature = "arrow")]
use std::sync::Arc;

/// Convert a vector of optional f64 values to an Arrow Float64Array
#[cfg(feature = "arrow")]
pub fn vec_to_arrow_f64(values: Vec<Option<f64>>) -> ArrayRef {
    let has_nulls = values.iter().any(|v| v.is_none());

    if has_nulls {
        let values: Vec<f64> = values.into_iter().map(|v| v.unwrap_or(0.0)).collect();
        let array = Float64Array::from_iter_values(values);
        Arc::new(array)
    } else {
        let values: Vec<f64> = values.into_iter().map(|v| v.unwrap()).collect();
        let array = Float64Array::from_iter_values(values);
        Arc::new(array)
    }
}

/// Convert a vector of optional i32 values to an Arrow Int32Array
#[cfg(feature = "arrow")]
pub fn vec_to_arrow_i32(values: Vec<Option<i32>>) -> ArrayRef {
    let has_nulls = values.iter().any(|v| v.is_none());

    if has_nulls {
        let values: Vec<i32> = values.into_iter().map(|v| v.unwrap_or(0)).collect();
        let array = Int32Array::from_iter_values(values);
        Arc::new(array)
    } else {
        let values: Vec<i32> = values.into_iter().map(|v| v.unwrap()).collect();
        let array = Int32Array::from_iter_values(values);
        Arc::new(array)
    }
}
