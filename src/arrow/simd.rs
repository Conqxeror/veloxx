//! Direct SIMD operations on Arrow arrays using std::simd

#[cfg(feature = "arrow")]
use arrow_array::{Float64Array, Int32Array};

#[cfg(feature = "arrow")]
use crate::performance::simd_std::StdSimdOps;
#[cfg(feature = "arrow")]
use crate::VeloxxError;

/// SIMD operations that work directly on Arrow arrays
#[cfg(feature = "arrow")]
pub trait ArrowSimdOps {
    /// Add two Arrow arrays using SIMD
    fn simd_add_arrow(&self, other: &Self) -> Result<Self, VeloxxError>
    where
        Self: Sized;

    /// Sum all elements in an Arrow array using SIMD
    fn simd_sum_arrow(&self) -> Result<f64, VeloxxError>;
}

#[cfg(feature = "arrow")]
impl ArrowSimdOps for Float64Array {
    fn simd_add_arrow(&self, other: &Self) -> Result<Self, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        #[cfg(feature = "simd")]
        {
            let values1 = self.values();
            let values2 = other.values();

            let result_values = values1.std_simd_add(values2)?;

            Ok(Float64Array::from(result_values))
        }

        #[cfg(not(feature = "simd"))]
        {
            // Fall back to regular addition if SIMD is not enabled
            use arrow_arith::numeric;
            numeric::add(self, other).map_err(VeloxxError::from)
        }
    }

    fn simd_sum_arrow(&self) -> Result<f64, VeloxxError> {
        #[cfg(feature = "simd")]
        {
            let values = self.values();
            values.std_simd_sum()
        }

        #[cfg(not(feature = "simd"))]
        {
            // Fall back to regular sum if SIMD is not enabled
            use arrow_arith::aggregate;
            Ok(aggregate::sum(self).unwrap_or(0.0))
        }
    }
}

#[cfg(feature = "arrow")]
impl ArrowSimdOps for Int32Array {
    fn simd_add_arrow(&self, other: &Self) -> Result<Self, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        #[cfg(feature = "simd")]
        {
            let values1 = self.values();
            let values2 = other.values();

            let result_values = values1.std_simd_add(values2)?;

            Ok(Int32Array::from(result_values))
        }

        #[cfg(not(feature = "simd"))]
        {
            // Fall back to regular addition if SIMD is not enabled
            use arrow_arith::numeric;
            numeric::add(self, other).map_err(VeloxxError::from)
        }
    }

    fn simd_sum_arrow(&self) -> Result<f64, VeloxxError> {
        #[cfg(feature = "simd")]
        {
            let values = self.values();
            Ok(values.std_simd_sum()? as f64)
        }

        #[cfg(not(feature = "simd"))]
        {
            // Fall back to regular sum if SIMD is not enabled
            use arrow_arith::aggregate;
            Ok(aggregate::sum(self).map(|v| v as f64).unwrap_or(0.0))
        }
    }
}

#[cfg(all(test, feature = "arrow", feature = "simd"))]
mod tests {
    use super::*;
    use arrow_array::Float64Array;

    #[test]
    fn test_simd_add_arrow_f64() {
        let a = Float64Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let b = Float64Array::from(vec![1.0, 1.0, 1.0, 1.0, 1.0]);
        let result = a.simd_add_arrow(&b).unwrap();
        assert_eq!(result.values(), &[2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_simd_sum_arrow_f64() {
        let a = Float64Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = a.simd_sum_arrow().unwrap();
        assert_eq!(result, 15.0);
    }
}
