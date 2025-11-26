//! Performance extensions for Series
//!
//! This module provides performance-optimized methods for Series operations

use crate::performance::{MemoryAnalyzer, ParallelAggregations};
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
            use crate::performance::optimized_simd::OptimizedSimdOps;

            if self.len() != other.len() {
                return Err(VeloxxError::InvalidOperation(
                    "Series must have same length for SIMD operations".to_string(),
                ));
            }

            match (self, other) {
                (Series::F64(name, a, a_bitmap), Series::F64(_, b, b_bitmap)) => {
                    let len = a.len();
                    let mut result_values = vec![0.0; len];
                    let mut result_bitmap = vec![false; len];

                    // Masking (can be optimized further but this is O(N))
                    for i in 0..len {
                        result_bitmap[i] = a_bitmap[i] & b_bitmap[i];
                    }

                    // Zero-copy SIMD addition on full arrays
                    // We use optimized_simd_add from OptimizedSimdOps trait
                    a.optimized_simd_add(b, &mut result_values);

                    // Use Series::F64 constructor directly to avoid Vec<Option<T>> conversion overhead
                    Ok(Series::F64(
                        format!("{}_simd_add", name),
                        result_values,
                        result_bitmap,
                    ))
                }
                (Series::I32(name, a, a_bitmap), Series::I32(_, b, b_bitmap)) => {
                    let len = a.len();
                    let mut result_values = vec![0; len];
                    let mut result_bitmap = vec![false; len];

                    for i in 0..len {
                        result_bitmap[i] = a_bitmap[i] & b_bitmap[i];
                    }

                    a.optimized_simd_add(b, &mut result_values);

                    Ok(Series::I32(
                        format!("{}_simd_add", name),
                        result_values,
                        result_bitmap,
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
        ParallelAggregations::par_mean(self)
    }

    fn par_min(&self) -> Result<Value, VeloxxError> {
        ParallelAggregations::par_min(self)
    }

    fn par_max(&self) -> Result<Value, VeloxxError> {
        ParallelAggregations::par_max(self)
    }

    fn memory_usage(&self) -> usize {
        use crate::performance::memory::MemoryAnalyzer;
        MemoryAnalyzer::estimate_series_memory(self)
    }

    fn compression_suggestions(&self) -> Vec<&'static str> {
        MemoryAnalyzer::suggest_compression(self)
    }
}
