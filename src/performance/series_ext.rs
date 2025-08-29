//! Performance extensions for Series
//!
//! This module provides performance-optimized methods for Series operations

use crate::series::Series;
use crate::types::Value;
use crate::VeloxxError;

/// Performance extension trait for Series
pub trait SeriesPerformanceExt {
    /// Use SIMD operations for fast numeric computations
    fn simd_add(&self, other: &Series) -> Result<Series, VeloxxError>;

    /// Use parallel processing for aggregations
    fn par_sum(&self) -> Result<Value, VeloxxError>;

    /// Use parallel processing for mean calculation
    fn par_mean(&self) -> Result<Value, VeloxxError>;

    /// Use parallel processing for min calculation
    fn par_min(&self) -> Result<Value, VeloxxError>;

    /// Use parallel processing for max calculation
    fn par_max(&self) -> Result<Value, VeloxxError>;

    /// Get memory usage estimate for this series
    fn memory_usage(&self) -> usize;

    /// Get compression suggestions for this series
    fn compression_suggestions(&self) -> Vec<&'static str>;
}

impl SeriesPerformanceExt for Series {
    fn simd_add(&self, other: &Series) -> Result<Series, VeloxxError> {
        #[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
        {
            use crate::performance::simd::SimdOps;

            if self.len() != other.len() {
                return Err(VeloxxError::InvalidOperation(
                    "Series must have same length for SIMD operations".to_string(),
                ));
            }

            match (self, other) {
                (Series::F64(name, a, a_bitmap), Series::F64(_, b, b_bitmap)) => {
                    // For SIMD operations, we need to handle the bitmap correctly
                    // We'll create vectors with only valid values for SIMD processing

                    // First, count valid values to pre-allocate correctly
                    let valid_count = a_bitmap
                        .iter()
                        .zip(b_bitmap.iter())
                        .filter(|&(&a, &b)| a && b)
                        .count();

                    // Pre-allocate vectors with exact capacity
                    let mut a_values: Vec<f64> = Vec::with_capacity(valid_count);
                    let mut b_values: Vec<f64> = Vec::with_capacity(valid_count);
                    let mut result_bitmap = vec![false; a_bitmap.len()];

                    // Fill vectors more efficiently
                    for i in 0..a.len() {
                        if a_bitmap[i] && b_bitmap[i] {
                            a_values.push(a[i]);
                            b_values.push(b[i]);
                            result_bitmap[i] = true;
                        }
                    }

                    let simd_result = a_values.simd_add(&b_values);

                    // Create the result series with correct values and bitmap
                    let mut result_values = Vec::with_capacity(simd_result.len());

                    let mut simd_idx = 0;
                    for &is_valid in &result_bitmap {
                        if is_valid {
                            result_values.push(Some(simd_result[simd_idx]));
                            simd_idx += 1;
                        } else {
                            result_values.push(None);
                        }
                    }

                    Ok(Series::new_f64(
                        &format!("{}_simd_add", name),
                        result_values,
                    ))
                }
                (Series::I32(name, a, a_bitmap), Series::I32(_, b, b_bitmap)) => {
                    // For SIMD operations, we need to handle the bitmap correctly
                    // We'll create vectors with only valid values for SIMD processing

                    // First, count valid values to pre-allocate correctly
                    let valid_count = a_bitmap
                        .iter()
                        .zip(b_bitmap.iter())
                        .filter(|&(&a, &b)| a && b)
                        .count();

                    // Pre-allocate vectors with exact capacity
                    let mut a_values: Vec<i32> = Vec::with_capacity(valid_count);
                    let mut b_values: Vec<i32> = Vec::with_capacity(valid_count);
                    let mut result_bitmap = vec![false; a_bitmap.len()];

                    // Fill vectors more efficiently
                    for i in 0..a.len() {
                        if a_bitmap[i] && b_bitmap[i] {
                            a_values.push(a[i]);
                            b_values.push(b[i]);
                            result_bitmap[i] = true;
                        }
                    }

                    let simd_result = a_values.simd_add(&b_values);

                    // Create the result series with correct values and bitmap
                    let mut result_values = Vec::with_capacity(simd_result.len());

                    let mut simd_idx = 0;
                    for &is_valid in &result_bitmap {
                        if is_valid {
                            result_values.push(Some(simd_result[simd_idx]));
                            simd_idx += 1;
                        } else {
                            result_values.push(None);
                        }
                    }

                    Ok(Series::new_i32(
                        &format!("{}_simd_add", name),
                        result_values,
                    ))
                }
                _ => Err(VeloxxError::InvalidOperation(
                    "SIMD add only supported for F64 and I32 series with same types".to_string(),
                )),
            }
        }
        #[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))]
        {
            // Fallback implementation for WASM or when SIMD is not available
            // Use a simple element-wise addition
            match (self, other) {
                (Series::I32(name, values, bitmap), Series::I32(_, other_values, other_bitmap)) => {
                    let mut new_values = Vec::with_capacity(values.len());
                    let mut new_bitmap = Vec::with_capacity(values.len());
                    for i in 0..values.len().min(other_values.len()) {
                        if bitmap[i] && other_bitmap[i] {
                            new_values.push(values[i] + other_values[i]);
                            new_bitmap.push(true);
                        } else {
                            new_values.push(0);
                            new_bitmap.push(false);
                        }
                    }
                    Ok(Series::I32(name.clone(), new_values, new_bitmap))
                }
                (Series::F64(name, values, bitmap), Series::F64(_, other_values, other_bitmap)) => {
                    let mut new_values = Vec::with_capacity(values.len());
                    let mut new_bitmap = Vec::with_capacity(values.len());
                    for i in 0..values.len().min(other_values.len()) {
                        if bitmap[i] && other_bitmap[i] {
                            new_values.push(values[i] + other_values[i]);
                            new_bitmap.push(true);
                        } else {
                            new_values.push(0.0);
                            new_bitmap.push(false);
                        }
                    }
                    Ok(Series::F64(name.clone(), new_values, new_bitmap))
                }
                _ => Err(VeloxxError::InvalidOperation(
                    "Addition not supported for these series types".to_string(),
                )),
            }
        }
    }

    fn par_sum(&self) -> Result<Value, VeloxxError> {
        use crate::performance::parallel::ParallelAggregations;
        ParallelAggregations::par_sum(self)
    }

    fn par_mean(&self) -> Result<Value, VeloxxError> {
        use crate::performance::parallel::ParallelAggregations;
        ParallelAggregations::par_mean(self)
    }

    fn par_min(&self) -> Result<Value, VeloxxError> {
        use crate::performance::parallel::ParallelAggregations;
        ParallelAggregations::par_min(self)
    }

    fn par_max(&self) -> Result<Value, VeloxxError> {
        use crate::performance::parallel::ParallelAggregations;
        ParallelAggregations::par_max(self)
    }

    fn memory_usage(&self) -> usize {
        use crate::performance::memory::MemoryAnalyzer;
        MemoryAnalyzer::estimate_series_memory(self)
    }

    fn compression_suggestions(&self) -> Vec<&'static str> {
        use crate::performance::memory::MemoryAnalyzer;
        MemoryAnalyzer::suggest_compression(self)
    }
}
