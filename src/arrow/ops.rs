//! Optimized operations using Arrow's compute functions

#[cfg(feature = "arrow")]
use arrow_array::{Float64Array, Int32Array};
#[cfg(feature = "arrow")]
use std::sync::Arc;

#[cfg(feature = "arrow")]
use arrow_arith::aggregate;
#[cfg(feature = "arrow")]
use arrow_arith::numeric;
#[cfg(feature = "arrow")]
use arrow_ord::cmp;

#[cfg(feature = "arrow")]
use crate::arrow::series::ArrowSeries;
#[cfg(all(feature = "arrow", feature = "simd"))]
use crate::arrow::simd::ArrowSimdOps;
#[cfg(all(feature = "arrow", feature = "simd"))]
use crate::arrow::simd_enhanced::ArrowSimdEnhancedOps;
#[cfg(feature = "arrow")]
use crate::VeloxxError;

/// Optimized operations for Arrow Series
#[cfg(feature = "arrow")]
pub trait ArrowOps {
    /// Add two series together using Arrow's optimized compute functions
    fn arrow_add(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;

    /// Subtract one series from another using Arrow's optimized compute functions
    fn arrow_sub(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;

    /// Multiply two series together using Arrow's optimized compute functions
    fn arrow_mul(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;

    /// Divide one series by another using Arrow's optimized compute functions
    fn arrow_div(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;

    /// Calculate the sum of the series using Arrow's optimized compute functions
    fn arrow_sum(&self) -> Result<Option<f64>, VeloxxError>;

    /// Calculate the minimum value in the series using Arrow's optimized compute functions
    fn arrow_min(&self) -> Result<Option<f64>, VeloxxError>;

    /// Calculate the maximum value in the series using Arrow's optimized compute functions
    fn arrow_max(&self) -> Result<Option<f64>, VeloxxError>;

    /// Calculate the power of a series (element-wise)
    fn arrow_pow(&self, exponent: f64) -> Result<ArrowSeries, VeloxxError>;

    /// Calculate the square root of a series (element-wise)
    fn arrow_sqrt(&self) -> Result<ArrowSeries, VeloxxError>;

    /// Calculate the absolute value of a series (element-wise)
    fn arrow_abs(&self) -> Result<ArrowSeries, VeloxxError>;

    /// Compare two series for equality (element-wise)
    fn arrow_eq(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;

    /// Add two series together using SIMD optimization
    fn simd_add(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;

    /// Subtract one series from another using SIMD optimization
    fn simd_sub(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;

    /// Multiply two series together using SIMD optimization
    fn simd_mul(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;

    /// Divide one series by another using SIMD optimization
    fn simd_div(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;

    /// Calculate the sum of the series using SIMD optimization
    fn simd_sum(&self) -> Result<Option<f64>, VeloxxError>;

    /// Calculate the minimum value in the series using SIMD optimization
    fn simd_min(&self) -> Result<Option<f64>, VeloxxError>;

    /// Calculate the maximum value in the series using SIMD optimization
    fn simd_max(&self) -> Result<Option<f64>, VeloxxError>;

    /// Compare two series for equality (element-wise) using SIMD optimization
    fn simd_eq(&self, other: &Self) -> Result<ArrowSeries, VeloxxError>;
}

#[cfg(feature = "arrow")]
impl ArrowOps for ArrowSeries {
    fn arrow_add(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        match (self, other) {
            (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();
                let result = numeric::add(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();
                let result = numeric::add(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::I32(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            _ => Err(VeloxxError::Unsupported(
                "Addition operation not supported for these series types".to_string(),
            )),
        }
    }

    fn arrow_sub(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        match (self, other) {
            (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();
                let result = numeric::sub(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();
                let result = numeric::sub(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::I32(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            _ => Err(VeloxxError::Unsupported(
                "Subtraction operation not supported for these series types".to_string(),
            )),
        }
    }

    fn arrow_mul(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        match (self, other) {
            (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();
                let result = numeric::mul(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();
                let result = numeric::mul(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::I32(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            _ => Err(VeloxxError::Unsupported(
                "Multiplication operation not supported for these series types".to_string(),
            )),
        }
    }

    fn arrow_div(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        match (self, other) {
            (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();
                let result = numeric::div(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();
                let result = numeric::div(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::I32(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            _ => Err(VeloxxError::Unsupported(
                "Division operation not supported for these series types".to_string(),
            )),
        }
    }

    fn arrow_sum(&self) -> Result<Option<f64>, VeloxxError> {
        match self {
            ArrowSeries::F64(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                let sum = aggregate::sum(arr);
                Ok(sum)
            }
            ArrowSeries::I32(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                let sum = aggregate::sum(arr);
                Ok(sum.map(|v| v as f64))
            }
            _ => Err(VeloxxError::Unsupported(
                "Sum operation not supported for this series type".to_string(),
            )),
        }
    }

    fn arrow_min(&self) -> Result<Option<f64>, VeloxxError> {
        match self {
            ArrowSeries::F64(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                let min = aggregate::min(arr);
                Ok(min)
            }
            ArrowSeries::I32(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                let min = aggregate::min(arr);
                Ok(min.map(|v| v as f64))
            }
            _ => Err(VeloxxError::Unsupported(
                "Min operation not supported for this series type".to_string(),
            )),
        }
    }

    fn arrow_max(&self) -> Result<Option<f64>, VeloxxError> {
        match self {
            ArrowSeries::F64(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                let max = aggregate::max(arr);
                Ok(max)
            }
            ArrowSeries::I32(_, array, _) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                let max = aggregate::max(arr);
                Ok(max.map(|v| v as f64))
            }
            _ => Err(VeloxxError::Unsupported(
                "Max operation not supported for this series type".to_string(),
            )),
        }
    }

    fn arrow_pow(&self, exponent: f64) -> Result<ArrowSeries, VeloxxError> {
        match self {
            ArrowSeries::F64(name, array, null_buffer) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                // Apply power function element-wise
                let result: Float64Array = arr
                    .iter()
                    .map(|opt_v| opt_v.map(|v| v.powf(exponent)))
                    .collect();
                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(result),
                    null_buffer.clone(),
                ))
            }
            ArrowSeries::I32(name, array, null_buffer) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                // Apply power function element-wise and convert to f64
                let result: Float64Array = arr
                    .iter()
                    .map(|opt_v| opt_v.map(|v| (v as f64).powf(exponent)))
                    .collect();
                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(result),
                    null_buffer.clone(),
                ))
            }
            _ => Err(VeloxxError::Unsupported(
                "Power operation not supported for this series type".to_string(),
            )),
        }
    }

    fn arrow_sqrt(&self) -> Result<ArrowSeries, VeloxxError> {
        match self {
            ArrowSeries::F64(name, array, null_buffer) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                // Apply square root function element-wise
                let result: Float64Array =
                    arr.iter().map(|opt_v| opt_v.map(|v| v.sqrt())).collect();
                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(result),
                    null_buffer.clone(),
                ))
            }
            ArrowSeries::I32(name, array, null_buffer) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                // Apply square root function element-wise and convert to f64
                let result: Float64Array = arr
                    .iter()
                    .map(|opt_v| opt_v.map(|v| (v as f64).sqrt()))
                    .collect();
                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(result),
                    null_buffer.clone(),
                ))
            }
            _ => Err(VeloxxError::Unsupported(
                "Square root operation not supported for this series type".to_string(),
            )),
        }
    }

    fn arrow_abs(&self) -> Result<ArrowSeries, VeloxxError> {
        match self {
            ArrowSeries::F64(name, array, null_buffer) => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                // Apply absolute value function element-wise
                let result: Float64Array = arr.iter().map(|opt_v| opt_v.map(|v| v.abs())).collect();
                Ok(ArrowSeries::F64(
                    name.clone(),
                    Arc::new(result),
                    null_buffer.clone(),
                ))
            }
            ArrowSeries::I32(name, array, null_buffer) => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                // Apply absolute value function element-wise
                let result: Int32Array = arr.iter().map(|opt_v| opt_v.map(|v| v.abs())).collect();
                Ok(ArrowSeries::I32(
                    name.clone(),
                    Arc::new(result),
                    null_buffer.clone(),
                ))
            }
            _ => Err(VeloxxError::Unsupported(
                "Absolute value operation not supported for this series type".to_string(),
            )),
        }
    }

    fn arrow_eq(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        match (self, other) {
            (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();
                let result = cmp::eq(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::Bool(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();
                let result = cmp::eq(arr1, arr2).map_err(VeloxxError::from)?;
                Ok(ArrowSeries::Bool(
                    name.clone(),
                    Arc::new(result),
                    null_buffer1.clone(),
                ))
            }
            _ => Err(VeloxxError::Unsupported(
                "Equality comparison not supported for these series types".to_string(),
            )),
        }
    }

    fn simd_add(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        #[cfg(all(feature = "arrow", feature = "simd"))]
        {
            if self.len() != other.len() {
                return Err(VeloxxError::InvalidOperation(
                    "Series must have same length for SIMD operations".to_string(),
                ));
            }

            match (self, other) {
                (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();

                    // Perform SIMD addition directly on Arrow arrays
                    let result_array = arr1.simd_add_arrow(arr2)?;
                    Ok(ArrowSeries::F64(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();

                    // Perform SIMD addition directly on Arrow arrays
                    let result_array = arr1.simd_add_arrow(arr2)?;
                    Ok(ArrowSeries::I32(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                _ => Err(VeloxxError::Unsupported(
                    "SIMD addition operation not supported for these series types".to_string(),
                )),
            }
        }

        #[cfg(not(all(feature = "arrow", feature = "simd")))]
        {
            // Fall back to regular Arrow addition if SIMD is not enabled
            self.arrow_add(other)
        }
    }

    fn simd_sub(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        #[cfg(all(feature = "arrow", feature = "simd"))]
        {
            if self.len() != other.len() {
                return Err(VeloxxError::InvalidOperation(
                    "Series must have same length for SIMD operations".to_string(),
                ));
            }

            match (self, other) {
                (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();

                    // Perform SIMD subtraction directly on Arrow arrays
                    let result_array = arr1.simd_sub_arrow(arr2)?;
                    Ok(ArrowSeries::F64(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();

                    // Perform SIMD subtraction directly on Arrow arrays
                    let result_array = arr1.simd_sub_arrow(arr2)?;
                    Ok(ArrowSeries::I32(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                _ => Err(VeloxxError::Unsupported(
                    "SIMD subtraction operation not supported for these series types".to_string(),
                )),
            }
        }

        #[cfg(not(all(feature = "arrow", feature = "simd")))]
        {
            // Fall back to regular Arrow subtraction if SIMD is not enabled
            self.arrow_sub(other)
        }
    }

    fn simd_mul(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        #[cfg(all(feature = "arrow", feature = "simd"))]
        {
            if self.len() != other.len() {
                return Err(VeloxxError::InvalidOperation(
                    "Series must have same length for SIMD operations".to_string(),
                ));
            }

            match (self, other) {
                (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();

                    // Perform SIMD multiplication directly on Arrow arrays
                    let result_array = arr1.simd_mul_arrow(arr2)?;
                    Ok(ArrowSeries::F64(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();

                    // Perform SIMD multiplication directly on Arrow arrays
                    let result_array = arr1.simd_mul_arrow(arr2)?;
                    Ok(ArrowSeries::I32(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                _ => Err(VeloxxError::Unsupported(
                    "SIMD multiplication operation not supported for these series types"
                        .to_string(),
                )),
            }
        }

        #[cfg(not(all(feature = "arrow", feature = "simd")))]
        {
            // Fall back to regular Arrow multiplication if SIMD is not enabled
            self.arrow_mul(other)
        }
    }

    fn simd_div(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        #[cfg(all(feature = "arrow", feature = "simd"))]
        {
            if self.len() != other.len() {
                return Err(VeloxxError::InvalidOperation(
                    "Series must have same length for SIMD operations".to_string(),
                ));
            }

            match (self, other) {
                (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();

                    // Perform SIMD division directly on Arrow arrays
                    let result_array = arr1.simd_div_arrow(arr2)?;
                    Ok(ArrowSeries::F64(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();

                    // Perform SIMD division directly on Arrow arrays
                    let result_array = arr1.simd_div_arrow(arr2)?;
                    Ok(ArrowSeries::I32(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                _ => Err(VeloxxError::Unsupported(
                    "SIMD division operation not supported for these series types".to_string(),
                )),
            }
        }

        #[cfg(not(all(feature = "arrow", feature = "simd")))]
        {
            // Fall back to regular Arrow division if SIMD is not enabled
            self.arrow_div(other)
        }
    }

    fn simd_sum(&self) -> Result<Option<f64>, VeloxxError> {
        #[cfg(all(feature = "arrow", feature = "simd"))]
        {
            match self {
                ArrowSeries::F64(_, array, _) => {
                    let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                    let sum = arr.simd_sum_arrow()?;
                    Ok(Some(sum))
                }
                ArrowSeries::I32(_, array, _) => {
                    let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                    let sum = arr.simd_sum_arrow()?;
                    Ok(Some(sum))
                }
                _ => Err(VeloxxError::Unsupported(
                    "SIMD sum operation not supported for this series type".to_string(),
                )),
            }
        }

        #[cfg(not(all(feature = "arrow", feature = "simd")))]
        {
            // Fall back to regular Arrow sum if SIMD is not enabled
            self.arrow_sum()
        }
    }

    fn simd_min(&self) -> Result<Option<f64>, VeloxxError> {
        #[cfg(all(feature = "arrow", feature = "simd"))]
        {
            match self {
                ArrowSeries::F64(_, array, _) => {
                    let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                    let min = arr.simd_min_arrow()?;
                    Ok(Some(min))
                }
                ArrowSeries::I32(_, array, _) => {
                    let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                    let min = arr.simd_min_arrow()?;
                    Ok(Some(min))
                }
                _ => Err(VeloxxError::Unsupported(
                    "SIMD min operation not supported for this series type".to_string(),
                )),
            }
        }

        #[cfg(not(all(feature = "arrow", feature = "simd")))]
        {
            // Fall back to regular Arrow min if SIMD is not enabled
            self.arrow_min()
        }
    }

    fn simd_max(&self) -> Result<Option<f64>, VeloxxError> {
        #[cfg(all(feature = "arrow", feature = "simd"))]
        {
            match self {
                ArrowSeries::F64(_, array, _) => {
                    let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                    let max = arr.simd_max_arrow()?;
                    Ok(Some(max))
                }
                ArrowSeries::I32(_, array, _) => {
                    let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                    let max = arr.simd_max_arrow()?;
                    Ok(Some(max))
                }
                _ => Err(VeloxxError::Unsupported(
                    "SIMD max operation not supported for this series type".to_string(),
                )),
            }
        }

        #[cfg(not(all(feature = "arrow", feature = "simd")))]
        {
            // Fall back to regular Arrow max if SIMD is not enabled
            self.arrow_max()
        }
    }

    fn simd_eq(&self, other: &Self) -> Result<ArrowSeries, VeloxxError> {
        #[cfg(all(feature = "arrow", feature = "simd"))]
        {
            if self.len() != other.len() {
                return Err(VeloxxError::InvalidOperation(
                    "Series must have same length for SIMD operations".to_string(),
                ));
            }

            match (self, other) {
                (ArrowSeries::F64(name, array1, null_buffer1), ArrowSeries::F64(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Float64Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Float64Array>().unwrap();

                    // Perform SIMD equality comparison directly on Arrow arrays
                    let result_array = arr1.simd_eq_arrow(arr2)?;
                    Ok(ArrowSeries::Bool(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                (ArrowSeries::I32(name, array1, null_buffer1), ArrowSeries::I32(_, array2, _)) => {
                    let arr1 = array1.as_any().downcast_ref::<Int32Array>().unwrap();
                    let arr2 = array2.as_any().downcast_ref::<Int32Array>().unwrap();

                    // Perform SIMD equality comparison directly on Arrow arrays
                    let result_array = arr1.simd_eq_arrow(arr2)?;
                    Ok(ArrowSeries::Bool(
                        name.clone(),
                        Arc::new(result_array),
                        null_buffer1.clone(),
                    ))
                }
                _ => Err(VeloxxError::Unsupported(
                    "SIMD equality comparison not supported for these series types".to_string(),
                )),
            }
        }

        #[cfg(not(all(feature = "arrow", feature = "simd")))]
        {
            // Fall back to regular Arrow equality comparison if SIMD is not enabled
            self.arrow_eq(other)
        }
    }
}
