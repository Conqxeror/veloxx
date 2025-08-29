//! Enhanced operations for Arrow arrays using std::simd

#[cfg(feature = "arrow")]
use arrow_array::{BooleanArray, Float64Array, Int32Array};

#[cfg(feature = "arrow")]
use crate::performance::simd_std::StdSimdOps;
#[cfg(feature = "arrow")]
use crate::VeloxxError;

/// Enhanced operations that work directly on Arrow arrays
#[cfg(feature = "arrow")]
pub trait ArrowSimdEnhancedOps {
    /// Subtract two Arrow arrays
    fn simd_sub_arrow(&self, other: &Self) -> Result<Self, VeloxxError>
    where
        Self: Sized;

    /// Multiply two Arrow arrays
    fn simd_mul_arrow(&self, other: &Self) -> Result<Self, VeloxxError>
    where
        Self: Sized;

    /// Divide two Arrow arrays
    fn simd_div_arrow(&self, other: &Self) -> Result<Self, VeloxxError>
    where
        Self: Sized;

    /// Compare two Arrow arrays (element-wise equality)
    fn simd_eq_arrow(&self, other: &Self) -> Result<BooleanArray, VeloxxError>;

    /// Calculate the minimum value in an Arrow array
    fn simd_min_arrow(&self) -> Result<f64, VeloxxError>;

    /// Calculate the maximum value in an Arrow array
    fn simd_max_arrow(&self) -> Result<f64, VeloxxError>;
}

#[cfg(feature = "arrow")]
impl ArrowSimdEnhancedOps for Float64Array {
    fn simd_sub_arrow(&self, other: &Self) -> Result<Self, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for operations".to_string(),
            ));
        }

        #[cfg(feature = "simd")]
        {
            let values1 = self.values();
            let values2 = other.values();

            let result_values = values1.std_simd_sub(values2)?;

            Ok(Float64Array::from(result_values))
        }

        #[cfg(not(feature = "simd"))]
        {
            // Simple element-wise subtraction
            let values1 = self.values();
            let values2 = other.values();
            let result: Vec<f64> = values1
                .iter()
                .zip(values2.iter())
                .map(|(a, b)| a - b)
                .collect();

            Ok(Float64Array::from(result))
        }
    }

    fn simd_mul_arrow(&self, other: &Self) -> Result<Self, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for operations".to_string(),
            ));
        }

        #[cfg(feature = "simd")]
        {
            let values1 = self.values();
            let values2 = other.values();

            let result_values = values1.std_simd_mul(values2)?;

            Ok(Float64Array::from(result_values))
        }

        #[cfg(not(feature = "simd"))]
        {
            // Simple element-wise multiplication
            let values1 = self.values();
            let values2 = other.values();
            let result: Vec<f64> = values1
                .iter()
                .zip(values2.iter())
                .map(|(a, b)| a * b)
                .collect();

            Ok(Float64Array::from(result))
        }
    }

    fn simd_div_arrow(&self, other: &Self) -> Result<Self, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for operations".to_string(),
            ));
        }

        #[cfg(feature = "simd")]
        {
            let values1 = self.values();
            let values2 = other.values();

            let result_values = values1.std_simd_div(values2)?;

            Ok(Float64Array::from(result_values))
        }

        #[cfg(not(feature = "simd"))]
        {
            // Simple element-wise division
            let values1 = self.values();
            let values2 = other.values();
            let result: Vec<f64> = values1
                .iter()
                .zip(values2.iter())
                .map(|(a, b)| a / b)
                .collect();

            Ok(Float64Array::from(result))
        }
    }

    fn simd_eq_arrow(&self, other: &Self) -> Result<BooleanArray, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for operations".to_string(),
            ));
        }

        // Simple element-wise equality comparison
        let values1 = self.values();
        let values2 = other.values();
        let result: Vec<bool> = values1
            .iter()
            .zip(values2.iter())
            .map(|(a, b)| a == b)
            .collect();

        Ok(BooleanArray::from(result))
    }

    fn simd_min_arrow(&self) -> Result<f64, VeloxxError> {
        let values = self.values();
        if values.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "Cannot calculate min of empty array".to_string(),
            ));
        }

        Ok(*values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap())
    }

    fn simd_max_arrow(&self) -> Result<f64, VeloxxError> {
        let values = self.values();
        if values.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "Cannot calculate max of empty array".to_string(),
            ));
        }

        Ok(*values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap())
    }
}

#[cfg(feature = "arrow")]
impl ArrowSimdEnhancedOps for Int32Array {
    fn simd_sub_arrow(&self, other: &Self) -> Result<Self, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for operations".to_string(),
            ));
        }

        #[cfg(feature = "simd")]
        {
            let values1 = self.values();
            let values2 = other.values();

            let result_values = values1.std_simd_sub(values2)?;

            Ok(Int32Array::from(result_values))
        }

        #[cfg(not(feature = "simd"))]
        {
            // Simple element-wise subtraction
            let values1 = self.values();
            let values2 = other.values();
            let result: Vec<i32> = values1
                .iter()
                .zip(values2.iter())
                .map(|(a, b)| a - b)
                .collect();

            Ok(Int32Array::from(result))
        }
    }

    fn simd_mul_arrow(&self, other: &Self) -> Result<Self, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for operations".to_string(),
            ));
        }

        #[cfg(feature = "simd")]
        {
            let values1 = self.values();
            let values2 = other.values();

            let result_values = values1.std_simd_mul(values2)?;

            Ok(Int32Array::from(result_values))
        }

        #[cfg(not(feature = "simd"))]
        {
            // Simple element-wise multiplication
            let values1 = self.values();
            let values2 = other.values();
            let result: Vec<i32> = values1
                .iter()
                .zip(values2.iter())
                .map(|(a, b)| a * b)
                .collect();

            Ok(Int32Array::from(result))
        }
    }

    fn simd_div_arrow(&self, other: &Self) -> Result<Self, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for operations".to_string(),
            ));
        }

        #[cfg(feature = "simd")]
        {
            let values1 = self.values();
            let values2 = other.values();

            let result_values = values1.std_simd_div(values2)?;

            Ok(Int32Array::from(result_values))
        }

        #[cfg(not(feature = "simd"))]
        {
            // Simple element-wise division
            let values1 = self.values();
            let values2 = other.values();
            let result: Vec<i32> = values1
                .iter()
                .zip(values2.iter())
                .map(|(a, b)| a / b)
                .collect();

            Ok(Int32Array::from(result))
        }
    }

    fn simd_eq_arrow(&self, other: &Self) -> Result<BooleanArray, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for operations".to_string(),
            ));
        }

        // Simple element-wise equality comparison
        let values1 = self.values();
        let values2 = other.values();
        let result: Vec<bool> = values1
            .iter()
            .zip(values2.iter())
            .map(|(a, b)| a == b)
            .collect();

        Ok(BooleanArray::from(result))
    }

    fn simd_min_arrow(&self) -> Result<f64, VeloxxError> {
        let values = self.values();
        if values.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "Cannot calculate min of empty array".to_string(),
            ));
        }

        Ok(*values.iter().min().unwrap() as f64)
    }

    fn simd_max_arrow(&self) -> Result<f64, VeloxxError> {
        let values = self.values();
        if values.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "Cannot calculate max of empty array".to_string(),
            ));
        }

        Ok(*values.iter().max().unwrap() as f64)
    }
}

#[cfg(all(test, feature = "arrow"))]
mod tests {
    use super::*;
    use arrow_array::Float64Array;

    #[test]
    fn test_simd_sub_arrow_f64() {
        let a = Float64Array::from(vec![5.0, 4.0, 3.0, 2.0, 1.0]);
        let b = Float64Array::from(vec![1.0, 1.0, 1.0, 1.0, 1.0]);
        let result = a.simd_sub_arrow(&b).unwrap();
        assert_eq!(result.values(), &[4.0, 3.0, 2.0, 1.0, 0.0]);
    }

    #[test]
    fn test_simd_mul_arrow_f64() {
        let a = Float64Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let b = Float64Array::from(vec![2.0, 2.0, 2.0, 2.0, 2.0]);
        let result = a.simd_mul_arrow(&b).unwrap();
        assert_eq!(result.values(), &[2.0, 4.0, 6.0, 8.0, 10.0]);
    }

    #[test]
    fn test_simd_div_arrow_f64() {
        let a = Float64Array::from(vec![10.0, 8.0, 6.0, 4.0, 2.0]);
        let b = Float64Array::from(vec![2.0, 2.0, 2.0, 2.0, 2.0]);
        let result = a.simd_div_arrow(&b).unwrap();
        assert_eq!(result.values(), &[5.0, 4.0, 3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_simd_eq_arrow_f64() {
        let a = Float64Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let b = Float64Array::from(vec![1.0, 2.0, 2.0, 4.0, 6.0]);
        let result = a.simd_eq_arrow(&b).unwrap();
        // Convert BooleanArray to Vec<bool> for comparison
        let values: Vec<bool> = result
            .iter()
            .collect::<Vec<Option<bool>>>()
            .into_iter()
            .map(|x| x.unwrap())
            .collect();
        assert_eq!(values, vec![true, true, false, true, false]);
    }

    #[test]
    fn test_simd_min_arrow_f64() {
        let a = Float64Array::from(vec![5.0, 2.0, 8.0, 1.0, 9.0]);
        let result = a.simd_min_arrow().unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_simd_max_arrow_f64() {
        let a = Float64Array::from(vec![5.0, 2.0, 8.0, 1.0, 9.0]);
        let result = a.simd_max_arrow().unwrap();
        assert_eq!(result, 9.0);
    }
}
