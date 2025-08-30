// src/distributed/global_aggregate.rs
use crate::VeloxxError;
use rayon::prelude::*;
#[cfg(feature = "simd")]
use crate::performance::simd::SimdOps;

/// Parallel and SIMD global aggregation for large arrays
pub struct GlobalAggregate;

impl GlobalAggregate {
    /// SIMD + Parallel global sum for f64
    pub fn sum_f64(data: &[f64]) -> f64 {
        #[cfg(feature = "simd")]
        {
            data.simd_sum()
        }
        #[cfg(not(feature = "simd"))]
        {
            data.par_iter().sum()
        }
    }

    /// SIMD + Parallel global mean for f64
    pub fn mean_f64(data: &[f64]) -> Result<f64, VeloxxError> {
        if data.is_empty() { return Err(VeloxxError::InvalidOperation("Empty array".to_string())); }
        #[cfg(feature = "simd")]
        {
            Ok(data.simd_mean().unwrap())
        }
        #[cfg(not(feature = "simd"))]
        {
            Ok(data.par_iter().sum::<f64>() / data.len() as f64)
        }
    }
}
