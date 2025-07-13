//! Performance extensions for Series
//! 
//! This module provides performance-optimized methods for Series operations

use crate::series::Series;
use crate::types::Value;
use crate::error::VeloxxError;

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
        use crate::performance::simd::SimdOps;
        
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for SIMD operations".to_string(),
            ));
        }
        
        match (self, other) {
            (Series::F64(_, a), Series::F64(_, b)) => {
                let a_values: Vec<f64> = a.iter().filter_map(|v| *v).collect();
                let b_values: Vec<f64> = b.iter().filter_map(|v| *v).collect();
                
                if a_values.len() == a.len() && b_values.len() == b.len() {
                    let result = a_values.simd_add(&b_values);
                    let result_options: Vec<Option<f64>> = result.into_iter().map(Some).collect();
                    Ok(Series::new_f64(&format!("{}_simd_add", self.name()), result_options))
                } else {
                    Err(VeloxxError::InvalidOperation(
                        "SIMD operations require non-null values".to_string(),
                    ))
                }
            }
            _ => Err(VeloxxError::InvalidOperation(
                "SIMD add only supported for F64 series".to_string(),
            )),
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