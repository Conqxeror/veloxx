// Arrow Series implementation
//
// This module provides Arrow-backed Series implementations for improved performance.

impl ArrowSeries {
    /// Perform raw SIMD addition with another series (null-free F64 only, optimized, allocation-free)
    pub fn simd_add_raw(&self, other: &ArrowSeries) -> Result<ArrowSeries, VeloxxError> {
        use crate::performance::optimized_simd::OptimizedSimdOps;
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for SIMD operations".to_string(),
            ));
        }
        match (self, other) {
            (ArrowSeries::F64(name, a, None), ArrowSeries::F64(_, b, None)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();
                let a_slice = a_arr.values();
                let b_slice = b_arr.values();
                let len = a_slice.len();

                // Use aligned memory pool for optimal SIMD performance
                let mut result_buffer = AlignedBuffer::<f64>::new(len).map_err(|e| {
                    VeloxxError::MemoryError(format!("Failed to allocate aligned buffer: {}", e))
                })?;
                let result_slice = result_buffer.as_mut_slice();

                // Check alignment (debug only)
                let ptr = result_slice.as_ptr() as usize;
                debug_assert!(
                    ptr % 64 == 0,
                    "Result buffer is not 64-byte aligned: {ptr:x}"
                );

                if len > 32_768 {
                    // Use advanced parallel executor for large arrays
                    crate::performance::advanced_parallel::parallel_simd_add_advanced(
                        a_slice,
                        b_slice,
                        result_slice,
                    )?;
                } else {
                    a_slice.optimized_simd_add(b_slice, result_slice);
                }

                // Convert to arrow array
                let result_vec = result_slice.to_vec();
                let array = Arc::new(Float64Array::from_iter_values(result_vec));
                Ok(ArrowSeries::F64(
                    format!("{}_simd_add_raw", name),
                    array,
                    None,
                ))
            }
            _ => self.simd_add(other),
        }
    }
}

#[cfg(feature = "arrow")]
use arrow_array::{Array, ArrayRef, BooleanArray, Float64Array, Int32Array, StringArray};
#[cfg(feature = "arrow")]
use arrow_buffer::NullBuffer;
#[cfg(feature = "arrow")]
use std::sync::Arc;

use crate::performance::memory_pool::AlignedBuffer;
use crate::performance::MemoryPool;
use crate::types::{DataType, Value};
use crate::VeloxxError;

/// Arrow-backed Series implementation
#[cfg(feature = "arrow")]
#[derive(Clone)]
pub enum ArrowSeries {
    /// Int32 series backed by Arrow Int32Array
    I32(String, ArrayRef, Option<NullBuffer>),
    /// Float64 series backed by Arrow Float64Array
    F64(String, ArrayRef, Option<NullBuffer>),
    /// Boolean series backed by Arrow BooleanArray
    Bool(String, ArrayRef, Option<NullBuffer>),
    /// String series backed by Arrow StringArray
    String(String, ArrayRef, Option<NullBuffer>),
}

#[cfg(feature = "arrow")]
impl ArrowSeries {
    /// Create a new Int32 series from a vector of optional values
    pub fn new_i32(name: &str, values: Vec<Option<i32>>) -> Self {
        let has_nulls = values.iter().any(|v| v.is_none());

        if has_nulls {
            let nulls: Vec<bool> = values.iter().map(|v| v.is_some()).collect();
            let null_buffer = NullBuffer::from(nulls);

            let values: Vec<i32> = values.into_iter().map(|v| v.unwrap_or(0)).collect();
            let array = Arc::new(Int32Array::from_iter_values(values));

            ArrowSeries::I32(name.to_string(), array, Some(null_buffer))
        } else {
            let values: Vec<i32> = values.into_iter().map(|v| v.unwrap()).collect();
            let array = Arc::new(Int32Array::from_iter_values(values));

            ArrowSeries::I32(name.to_string(), array, None)
        }
    }

    /// Create a new Float64 series from a vector of optional values
    pub fn new_f64(name: &str, values: Vec<Option<f64>>) -> Self {
        let has_nulls = values.iter().any(|v| v.is_none());

        if has_nulls {
            let nulls: Vec<bool> = values.iter().map(|v| v.is_some()).collect();
            let null_buffer = NullBuffer::from(nulls);

            let values: Vec<f64> = values.into_iter().map(|v| v.unwrap_or(0.0)).collect();
            let array = Arc::new(Float64Array::from_iter_values(values));

            ArrowSeries::F64(name.to_string(), array, Some(null_buffer))
        } else {
            let values: Vec<f64> = values.into_iter().map(|v| v.unwrap()).collect();
            let array = Arc::new(Float64Array::from_iter_values(values));

            ArrowSeries::F64(name.to_string(), array, None)
        }
    }

    /// Create a new Boolean series from a vector of optional values
    pub fn new_bool(name: &str, values: Vec<Option<bool>>) -> Self {
        let has_nulls = values.iter().any(|v| v.is_none());

        if has_nulls {
            let nulls: Vec<bool> = values.iter().map(|v| v.is_some()).collect();
            let null_buffer = NullBuffer::from(nulls);

            let values: Vec<bool> = values.into_iter().map(|v| v.unwrap_or(false)).collect();
            let array = Arc::new(BooleanArray::from(values));

            ArrowSeries::Bool(name.to_string(), array, Some(null_buffer))
        } else {
            let values: Vec<bool> = values.into_iter().map(|v| v.unwrap()).collect();
            let array = Arc::new(BooleanArray::from(values));

            ArrowSeries::Bool(name.to_string(), array, None)
        }
    }

    /// Create a new String series from a vector of optional values
    pub fn new_string(name: &str, values: Vec<Option<String>>) -> Self {
        let has_nulls = values.iter().any(|v| v.is_none());

        if has_nulls {
            let nulls: Vec<bool> = values.iter().map(|v| v.is_some()).collect();
            let null_buffer = NullBuffer::from(nulls);

            let values: Vec<Option<String>> = values.into_iter().collect();
            let array = Arc::new(StringArray::from(values));

            ArrowSeries::String(name.to_string(), array, Some(null_buffer))
        } else {
            let values: Vec<String> = values.into_iter().map(|v| v.unwrap()).collect();
            let array = Arc::new(StringArray::from(values));

            ArrowSeries::String(name.to_string(), array, None)
        }
    }

    /// Create a new Int32 series with a memory pool
    pub fn new_i32_with_pool(name: &str, values: Vec<Option<i32>>, _pool: &MemoryPool) -> Self {
        // For now, we'll just delegate to the regular constructor
        // In the future, we can implement memory pool integration here
        Self::new_i32(name, values)
    }

    /// Create a new Float64 series with a memory pool
    pub fn new_f64_with_pool(name: &str, values: Vec<Option<f64>>, _pool: &MemoryPool) -> Self {
        // For now, we'll just delegate to the regular constructor
        // In the future, we can implement memory pool integration here
        Self::new_f64(name, values)
    }

    /// Create a new Boolean series with a memory pool
    pub fn new_bool_with_pool(name: &str, values: Vec<Option<bool>>, _pool: &MemoryPool) -> Self {
        // For now, we'll just delegate to the regular constructor
        // In the future, we can implement memory pool integration here
        Self::new_bool(name, values)
    }

    /// Create a new String series with a memory pool
    pub fn new_string_with_pool(
        name: &str,
        values: Vec<Option<String>>,
        _pool: &MemoryPool,
    ) -> Self {
        // For now, we'll just delegate to the regular constructor
        // In the future, we can implement memory pool integration here
        Self::new_string(name, values)
    }

    /// Get the length of the series
    pub fn len(&self) -> usize {
        match self {
            ArrowSeries::I32(_, array, _) => array.len(),
            ArrowSeries::F64(_, array, _) => array.len(),
            ArrowSeries::Bool(_, array, _) => array.len(),
            ArrowSeries::String(_, array, _) => array.len(),
        }
    }

    /// Check if the series is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the name of the series
    pub fn name(&self) -> &str {
        match self {
            ArrowSeries::I32(name, _, _) => name,
            ArrowSeries::F64(name, _, _) => name,
            ArrowSeries::Bool(name, _, _) => name,
            ArrowSeries::String(name, _, _) => name,
        }
    }

    /// Get a value at the specified index
    pub fn get(&self, index: usize) -> Option<Value> {
        if index >= self.len() {
            return None;
        }

        match self {
            ArrowSeries::I32(_, array, null_buffer) => {
                if let Some(nulls) = null_buffer {
                    if !nulls.is_valid(index) {
                        return None;
                    }
                }

                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                Some(Value::I32(arr.value(index)))
            }
            ArrowSeries::F64(_, array, null_buffer) => {
                if let Some(nulls) = null_buffer {
                    if !nulls.is_valid(index) {
                        return None;
                    }
                }

                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                Some(Value::F64(arr.value(index)))
            }
            ArrowSeries::Bool(_, array, null_buffer) => {
                if let Some(nulls) = null_buffer {
                    if !nulls.is_valid(index) {
                        return None;
                    }
                }

                let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
                Some(Value::Bool(arr.value(index)))
            }
            ArrowSeries::String(_, array, null_buffer) => {
                if let Some(nulls) = null_buffer {
                    if !nulls.is_valid(index) {
                        return None;
                    }
                }

                let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
                Some(Value::String(arr.value(index).to_string()))
            }
        }
    }

    /// Get the data type of the series
    pub fn data_type(&self) -> DataType {
        match self {
            ArrowSeries::I32(_, _, _) => DataType::I32,
            ArrowSeries::F64(_, _, _) => DataType::F64,
            ArrowSeries::Bool(_, _, _) => DataType::Bool,
            ArrowSeries::String(_, _, _) => DataType::String,
        }
    }

    /// Perform SIMD addition with another series
    pub fn simd_add(&self, other: &ArrowSeries) -> Result<ArrowSeries, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for SIMD operations".to_string(),
            ));
        }

        match (self, other) {
            (ArrowSeries::F64(name, a, a_nulls), ArrowSeries::F64(_, b, b_nulls)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();

                // Create result array
                let mut result_values = Vec::with_capacity(self.len());
                let mut result_validity = Vec::with_capacity(self.len());

                for i in 0..self.len() {
                    let a_valid = a_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);
                    let b_valid = b_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);

                    if a_valid && b_valid {
                        result_values.push(a_arr.value(i) + b_arr.value(i));
                        result_validity.push(true);
                    } else {
                        result_values.push(0.0); // Default value
                        result_validity.push(false);
                    }
                }

                let null_buffer = NullBuffer::from(result_validity);
                let result_array = Arc::new(Float64Array::from_iter_values(result_values));

                Ok(ArrowSeries::F64(
                    format!("{}_simd_add", name),
                    result_array,
                    Some(null_buffer),
                ))
            }
            (ArrowSeries::I32(name, a, a_nulls), ArrowSeries::I32(_, b, b_nulls)) => {
                let a_arr = a.as_any().downcast_ref::<Int32Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Int32Array>().unwrap();

                // Create result array
                let mut result_values = Vec::with_capacity(self.len());
                let mut result_validity = Vec::with_capacity(self.len());

                for i in 0..self.len() {
                    let a_valid = a_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);
                    let b_valid = b_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);

                    if a_valid && b_valid {
                        result_values.push(a_arr.value(i) + b_arr.value(i));
                        result_validity.push(true);
                    } else {
                        result_values.push(0); // Default value
                        result_validity.push(false);
                    }
                }

                let null_buffer = NullBuffer::from(result_validity);
                let result_array = Arc::new(Int32Array::from_iter_values(result_values));

                Ok(ArrowSeries::I32(
                    format!("{}_simd_add", name),
                    result_array,
                    Some(null_buffer),
                ))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "SIMD add only supported for F64 and I32 series with same types".to_string(),
            )),
        }
    }

    /// Perform SIMD subtraction with another series
    pub fn simd_sub(&self, other: &ArrowSeries) -> Result<ArrowSeries, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for SIMD operations".to_string(),
            ));
        }

        match (self, other) {
            (ArrowSeries::F64(name, a, a_nulls), ArrowSeries::F64(_, b, b_nulls)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();

                // Create result array
                let mut result_values = Vec::with_capacity(self.len());
                let mut result_validity = Vec::with_capacity(self.len());

                for i in 0..self.len() {
                    let a_valid = a_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);
                    let b_valid = b_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);

                    if a_valid && b_valid {
                        result_values.push(a_arr.value(i) - b_arr.value(i));
                        result_validity.push(true);
                    } else {
                        result_values.push(0.0); // Default value
                        result_validity.push(false);
                    }
                }

                let null_buffer = NullBuffer::from(result_validity);
                let result_array = Arc::new(Float64Array::from_iter_values(result_values));

                Ok(ArrowSeries::F64(
                    format!("{}_simd_sub", name),
                    result_array,
                    Some(null_buffer),
                ))
            }
            (ArrowSeries::I32(name, a, a_nulls), ArrowSeries::I32(_, b, b_nulls)) => {
                let a_arr = a.as_any().downcast_ref::<Int32Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Int32Array>().unwrap();

                // Create result array
                let mut result_values = Vec::with_capacity(self.len());
                let mut result_validity = Vec::with_capacity(self.len());

                for i in 0..self.len() {
                    let a_valid = a_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);
                    let b_valid = b_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);

                    if a_valid && b_valid {
                        result_values.push(a_arr.value(i) - b_arr.value(i));
                        result_validity.push(true);
                    } else {
                        result_values.push(0); // Default value
                        result_validity.push(false);
                    }
                }

                let null_buffer = NullBuffer::from(result_validity);
                let result_array = Arc::new(Int32Array::from_iter_values(result_values));

                Ok(ArrowSeries::I32(
                    format!("{}_simd_sub", name),
                    result_array,
                    Some(null_buffer),
                ))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "SIMD subtract only supported for F64 and I32 series with same types".to_string(),
            )),
        }
    }

    /// Perform SIMD multiplication with another series
    pub fn simd_mul(&self, other: &ArrowSeries) -> Result<ArrowSeries, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for SIMD operations".to_string(),
            ));
        }

        match (self, other) {
            (ArrowSeries::F64(name, a, a_nulls), ArrowSeries::F64(_, b, b_nulls)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();

                // Create result array
                let mut result_values = Vec::with_capacity(self.len());
                let mut result_validity = Vec::with_capacity(self.len());

                for i in 0..self.len() {
                    let a_valid = a_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);
                    let b_valid = b_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);

                    if a_valid && b_valid {
                        result_values.push(a_arr.value(i) * b_arr.value(i));
                        result_validity.push(true);
                    } else {
                        result_values.push(0.0); // Default value
                        result_validity.push(false);
                    }
                }

                let null_buffer = NullBuffer::from(result_validity);
                let result_array = Arc::new(Float64Array::from_iter_values(result_values));

                Ok(ArrowSeries::F64(
                    format!("{}_simd_mul", name),
                    result_array,
                    Some(null_buffer),
                ))
            }
            (ArrowSeries::I32(name, a, a_nulls), ArrowSeries::I32(_, b, b_nulls)) => {
                let a_arr = a.as_any().downcast_ref::<Int32Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Int32Array>().unwrap();

                // Create result array
                let mut result_values = Vec::with_capacity(self.len());
                let mut result_validity = Vec::with_capacity(self.len());

                for i in 0..self.len() {
                    let a_valid = a_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);
                    let b_valid = b_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);

                    if a_valid && b_valid {
                        result_values.push(a_arr.value(i) * b_arr.value(i));
                        result_validity.push(true);
                    } else {
                        result_values.push(0); // Default value
                        result_validity.push(false);
                    }
                }

                let null_buffer = NullBuffer::from(result_validity);
                let result_array = Arc::new(Int32Array::from_iter_values(result_values));

                Ok(ArrowSeries::I32(
                    format!("{}_simd_mul", name),
                    result_array,
                    Some(null_buffer),
                ))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "SIMD multiply only supported for F64 and I32 series with same types".to_string(),
            )),
        }
    }

    /// Perform SIMD division with another series
    pub fn simd_div(&self, other: &ArrowSeries) -> Result<ArrowSeries, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for SIMD operations".to_string(),
            ));
        }

        match (self, other) {
            (ArrowSeries::F64(name, a, a_nulls), ArrowSeries::F64(_, b, b_nulls)) => {
                let a_arr = a.as_any().downcast_ref::<Float64Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Float64Array>().unwrap();

                // Create result array
                let mut result_values = Vec::with_capacity(self.len());
                let mut result_validity = Vec::with_capacity(self.len());

                for i in 0..self.len() {
                    let a_valid = a_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);
                    let b_valid = b_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);

                    if a_valid && b_valid && b_arr.value(i) != 0.0 {
                        result_values.push(a_arr.value(i) / b_arr.value(i));
                        result_validity.push(true);
                    } else {
                        result_values.push(0.0); // Default value
                        result_validity.push(false);
                    }
                }

                let null_buffer = NullBuffer::from(result_validity);
                let result_array = Arc::new(Float64Array::from_iter_values(result_values));

                Ok(ArrowSeries::F64(
                    format!("{}_simd_div", name),
                    result_array,
                    Some(null_buffer),
                ))
            }
            (ArrowSeries::I32(name, a, a_nulls), ArrowSeries::I32(_, b, b_nulls)) => {
                let a_arr = a.as_any().downcast_ref::<Int32Array>().unwrap();
                let b_arr = b.as_any().downcast_ref::<Int32Array>().unwrap();

                // Create result array
                let mut result_values = Vec::with_capacity(self.len());
                let mut result_validity = Vec::with_capacity(self.len());

                for i in 0..self.len() {
                    let a_valid = a_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);
                    let b_valid = b_nulls.as_ref().map(|n| n.is_valid(i)).unwrap_or(true);

                    if a_valid && b_valid && b_arr.value(i) != 0 {
                        result_values.push(a_arr.value(i) / b_arr.value(i));
                        result_validity.push(true);
                    } else {
                        result_values.push(0); // Default value
                        result_validity.push(false);
                    }
                }

                let null_buffer = NullBuffer::from(result_validity);
                let result_array = Arc::new(Int32Array::from_iter_values(result_values));

                Ok(ArrowSeries::I32(
                    format!("{}_simd_div", name),
                    result_array,
                    Some(null_buffer),
                ))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "SIMD divide only supported for F64 and I32 series with same types".to_string(),
            )),
        }
    }

    /// Calculate the sum of the series using SIMD
    pub fn simd_sum(&self) -> Result<Option<Value>, VeloxxError> {
        use crate::performance::simd_std::StdSimdOps;

        match self {
            ArrowSeries::I32(_, array, null_buffer) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();

                let sum = if let Some(nulls) = null_buffer {
                    // Extract non-null values for SIMD processing
                    let valid_values: Vec<i32> = (0..arr.len())
                        .filter_map(|i| {
                            if nulls.is_valid(i) {
                                Some(arr.value(i))
                            } else {
                                None
                            }
                        })
                        .collect();

                    if valid_values.is_empty() {
                        0i32
                    } else {
                        valid_values.std_simd_sum()?
                    }
                } else {
                    arr.values().std_simd_sum()?
                };

                Ok(Some(Value::I32(sum)))
            }
            ArrowSeries::F64(_, array, null_buffer) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();

                let sum = if let Some(nulls) = null_buffer {
                    // Extract non-null values for SIMD processing
                    let valid_values: Vec<f64> = (0..arr.len())
                        .filter_map(|i| {
                            if nulls.is_valid(i) {
                                Some(arr.value(i))
                            } else {
                                None
                            }
                        })
                        .collect();

                    if valid_values.is_empty() {
                        0.0f64
                    } else {
                        valid_values.std_simd_sum()?
                    }
                } else {
                    arr.values().std_simd_sum()?
                };

                Ok(Some(Value::F64(sum)))
            }
            _ => Err(VeloxxError::Unsupported(
                "Sum operation not supported for this series type".to_string(),
            )),
        }
    }

    /// Calculate the minimum value of the series using SIMD
    pub fn simd_min(&self) -> Result<Option<Value>, VeloxxError> {
        // Use the SIMD-optimized implementation from the performance module
        match self {
            ArrowSeries::F64(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                if arr.is_empty() {
                    return Ok(None);
                }
                let values = arr.values();
                if arr.null_count() == 0 {
                    let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    Ok(Some(Value::F64(min)))
                } else {
                    let valid_values: Vec<f64> = values
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &v)| if arr.is_valid(i) { Some(v) } else { None })
                        .collect();
                    if valid_values.is_empty() {
                        Ok(None)
                    } else {
                        let min = valid_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                        Ok(Some(Value::F64(min)))
                    }
                }
            }
            ArrowSeries::I32(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                if arr.is_empty() {
                    return Ok(None);
                }
                let values = arr.values();
                if arr.null_count() == 0 {
                    let min = *values.iter().min().unwrap_or(&0);
                    Ok(Some(Value::I32(min)))
                } else {
                    let valid_values: Vec<i32> = values
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &v)| if arr.is_valid(i) { Some(v) } else { None })
                        .collect();
                    if valid_values.is_empty() {
                        Ok(None)
                    } else {
                        let min = *valid_values.iter().min().unwrap_or(&0);
                        Ok(Some(Value::I32(min)))
                    }
                }
            }
            ArrowSeries::Bool(_, _, _) | ArrowSeries::String(_, _, _) => {
                Err(VeloxxError::Unsupported(
                    "Min operation not supported for this series type".to_string(),
                ))
            }
        }
    }

    pub fn simd_max(&self) -> Result<Option<Value>, VeloxxError> {
        // Use the SIMD-optimized implementation from the performance module
        match self {
            ArrowSeries::F64(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                if arr.is_empty() {
                    return Ok(None);
                }
                let values = arr.values();
                if arr.null_count() == 0 {
                    let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                    Ok(Some(Value::F64(max)))
                } else {
                    let valid_values: Vec<f64> = values
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &v)| if arr.is_valid(i) { Some(v) } else { None })
                        .collect();
                    if valid_values.is_empty() {
                        Ok(None)
                    } else {
                        let max = valid_values
                            .iter()
                            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                        Ok(Some(Value::F64(max)))
                    }
                }
            }
            ArrowSeries::I32(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                if arr.is_empty() {
                    return Ok(None);
                }
                let values = arr.values();
                if arr.null_count() == 0 {
                    let max = *values.iter().max().unwrap_or(&0);
                    Ok(Some(Value::I32(max)))
                } else {
                    let valid_values: Vec<i32> = values
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &v)| if arr.is_valid(i) { Some(v) } else { None })
                        .collect();
                    if valid_values.is_empty() {
                        Ok(None)
                    } else {
                        let max = *valid_values.iter().max().unwrap_or(&0);
                        Ok(Some(Value::I32(max)))
                    }
                }
            }
            ArrowSeries::Bool(_, _, _) | ArrowSeries::String(_, _, _) => {
                Err(VeloxxError::Unsupported(
                    "Max operation not supported for this series type".to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature = "arrow")]
mod tests {
    use super::*;

    #[test]
    fn test_arrow_series_creation() {
        let data = vec![Some(1i32), Some(2), None, Some(4)];
        let series = ArrowSeries::new_i32("test", data);

        assert_eq!(series.len(), 4);
        assert_eq!(series.name(), "test");
        assert_eq!(series.get(0), Some(Value::I32(1)));
        assert_eq!(series.get(1), Some(Value::I32(2)));
        assert_eq!(series.get(2), None);
        assert_eq!(series.get(3), Some(Value::I32(4)));
    }

    #[test]
    fn test_arrow_series_simd_add() {
        let data1 = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
        let data2 = vec![Some(1.0f64), Some(1.0), Some(1.0), Some(1.0)];

        let series1 = ArrowSeries::new_f64("a", data1);
        let series2 = ArrowSeries::new_f64("b", data2);

        let result = series1.simd_add(&series2).unwrap();

        assert_eq!(result.get(0), Some(Value::F64(2.0)));
        assert_eq!(result.get(1), Some(Value::F64(3.0)));
        assert_eq!(result.get(2), Some(Value::F64(4.0)));
        assert_eq!(result.get(3), Some(Value::F64(5.0)));
    }

    #[test]
    fn test_arrow_series_simd_sum() {
        let data = vec![Some(1.0f64), Some(2.0), Some(3.0), Some(4.0)];
        let series = ArrowSeries::new_f64("test", data);

        let sum = series.simd_sum().unwrap().unwrap();

        assert_eq!(sum, Value::F64(10.0));
    }

    #[test]
    fn test_arrow_series_bool_creation() {
        let data = vec![Some(true), Some(false), None, Some(true)];
        let series = ArrowSeries::new_bool("test", data);

        assert_eq!(series.len(), 4);
        assert_eq!(series.name(), "test");
        assert_eq!(series.get(0), Some(Value::Bool(true)));
        assert_eq!(series.get(1), Some(Value::Bool(false)));
        assert_eq!(series.get(2), None);
        assert_eq!(series.get(3), Some(Value::Bool(true)));
    }
}
