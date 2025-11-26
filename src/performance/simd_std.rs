//! SIMD-optimized operations using the `wide` crate for better performance
//!
//! This module provides vectorized implementations of common mathematical
//! operations using the `wide` crate for cross-platform SIMD support.
#![cfg(feature = "simd")]

use crate::VeloxxError;
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
use wide::{f64x4, i32x4};

/// Trait for SIMD-optimized operations using the `wide` crate
pub trait StdSimdOps<T> {
    /// Vectorized element-wise addition
    fn std_simd_add(&self, other: &[T]) -> Result<Vec<T>, VeloxxError>;

    /// Vectorized element-wise subtraction
    fn std_simd_sub(&self, other: &[T]) -> Result<Vec<T>, VeloxxError>;

    /// Vectorized element-wise multiplication
    fn std_simd_mul(&self, other: &[T]) -> Result<Vec<T>, VeloxxError>;

    /// Vectorized element-wise division
    fn std_simd_div(&self, other: &[T]) -> Result<Vec<T>, VeloxxError>;

    /// Vectorized sum reduction
    fn std_simd_sum(&self) -> Result<T, VeloxxError>;

    /// Vectorized mean calculation (always returns f64 for precision)
    fn std_simd_mean(&self) -> Result<Option<f64>, VeloxxError>;
}

/// SIMD implementation for f64 slices using the `wide` crate
impl StdSimdOps<f64> for [f64] {
    fn std_simd_add(&self, other: &[f64]) -> Result<Vec<f64>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        let len = self.len();
        let mut result = vec![0.0; len];
        let simd_len = len / 4;
        let remainder = len % 4;

        // SIMD loop
        for i in 0..simd_len {
            let a = f64x4::from([
                self[i * 4],
                self[i * 4 + 1],
                self[i * 4 + 2],
                self[i * 4 + 3],
            ]);
            let b = f64x4::from([
                other[i * 4],
                other[i * 4 + 1],
                other[i * 4 + 2],
                other[i * 4 + 3],
            ]);
            let res = a + b;
            let res_array = res.to_array();
            result[i * 4..i * 4 + 4].copy_from_slice(&res_array);
        }

        // Remainder loop
        for i in (len - remainder)..len {
            result[i] = self[i] + other[i];
        }

        Ok(result)
    }

    fn std_simd_sub(&self, other: &[f64]) -> Result<Vec<f64>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        let len = self.len();
        let mut result = Vec::with_capacity(len);
        let simd_len = len / 4;
        let remainder = len % 4;

        // Process 4 elements at a time using SIMD
        for i in 0..simd_len {
            let a = f64x4::from([
                self[i * 4],
                self[i * 4 + 1],
                self[i * 4 + 2],
                self[i * 4 + 3],
            ]);
            let b = f64x4::from([
                other[i * 4],
                other[i * 4 + 1],
                other[i * 4 + 2],
                other[i * 4 + 3],
            ]);
            let res = a - b;
            let res_array = res.to_array();
            result.extend_from_slice(&res_array);
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result.push(self[i] - other[i]);
        }

        Ok(result)
    }

    fn std_simd_mul(&self, other: &[f64]) -> Result<Vec<f64>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        let len = self.len();
        let mut result = Vec::with_capacity(len);
        let simd_len = len / 4;
        let remainder = len % 4;

        // Process 4 elements at a time using SIMD
        for i in 0..simd_len {
            let a = f64x4::from([
                self[i * 4],
                self[i * 4 + 1],
                self[i * 4 + 2],
                self[i * 4 + 3],
            ]);
            let b = f64x4::from([
                other[i * 4],
                other[i * 4 + 1],
                other[i * 4 + 2],
                other[i * 4 + 3],
            ]);
            let res = a * b;
            let res_array = res.to_array();
            result.extend_from_slice(&res_array);
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result.push(self[i] * other[i]);
        }

        Ok(result)
    }

    fn std_simd_div(&self, other: &[f64]) -> Result<Vec<f64>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        let len = self.len();
        let mut result = Vec::with_capacity(len);
        let simd_len = len / 4;
        let remainder = len % 4;

        // Process 4 elements at a time using SIMD
        for i in 0..simd_len {
            let a = f64x4::from([
                self[i * 4],
                self[i * 4 + 1],
                self[i * 4 + 2],
                self[i * 4 + 3],
            ]);
            let b = f64x4::from([
                other[i * 4],
                other[i * 4 + 1],
                other[i * 4 + 2],
                other[i * 4 + 3],
            ]);
            let res = a / b;
            let res_array = res.to_array();
            result.extend_from_slice(&res_array);
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result.push(self[i] / other[i]);
        }

        Ok(result)
    }

    fn std_simd_sum(&self) -> Result<f64, VeloxxError> {
        // Use the optimized SIMD sum implementation
        optimized::std_simd_sum_optimized(self)
    }

    fn std_simd_mean(&self) -> Result<Option<f64>, VeloxxError> {
        if self.is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.std_simd_sum()? / self.len() as f64))
        }
    }
}

/// SIMD implementation for i32 slices using the `wide` crate
impl StdSimdOps<i32> for [i32] {
    fn std_simd_add(&self, other: &[i32]) -> Result<Vec<i32>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        let len = self.len();
        let mut result = Vec::with_capacity(len);
        let simd_len = len / 4;
        let remainder = len % 4;

        // Process 4 elements at a time using SIMD
        for i in 0..simd_len {
            let a = i32x4::from([
                self[i * 4],
                self[i * 4 + 1],
                self[i * 4 + 2],
                self[i * 4 + 3],
            ]);
            let b = i32x4::from([
                other[i * 4],
                other[i * 4 + 1],
                other[i * 4 + 2],
                other[i * 4 + 3],
            ]);
            let res = a + b;
            let res_array = res.to_array();
            result.extend_from_slice(&res_array);
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result.push(self[i] + other[i]);
        }

        Ok(result)
    }

    fn std_simd_sub(&self, other: &[i32]) -> Result<Vec<i32>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        let len = self.len();
        let mut result = Vec::with_capacity(len);
        let simd_len = len / 4;
        let remainder = len % 4;

        // Process 4 elements at a time using SIMD
        for i in 0..simd_len {
            let a = i32x4::from([
                self[i * 4],
                self[i * 4 + 1],
                self[i * 4 + 2],
                self[i * 4 + 3],
            ]);
            let b = i32x4::from([
                other[i * 4],
                other[i * 4 + 1],
                other[i * 4 + 2],
                other[i * 4 + 3],
            ]);
            let res = a - b;
            let res_array = res.to_array();
            result.extend_from_slice(&res_array);
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result.push(self[i] - other[i]);
        }

        Ok(result)
    }

    fn std_simd_mul(&self, other: &[i32]) -> Result<Vec<i32>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        let len = self.len();
        let mut result = Vec::with_capacity(len);
        let simd_len = len / 4;
        let remainder = len % 4;

        // Process 4 elements at a time using SIMD
        for i in 0..simd_len {
            let a = i32x4::from([
                self[i * 4],
                self[i * 4 + 1],
                self[i * 4 + 2],
                self[i * 4 + 3],
            ]);
            let b = i32x4::from([
                other[i * 4],
                other[i * 4 + 1],
                other[i * 4 + 2],
                other[i * 4 + 3],
            ]);
            let res = a * b;
            let res_array = res.to_array();
            result.extend_from_slice(&res_array);
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result.push(self[i] * other[i]);
        }

        Ok(result)
    }

    fn std_simd_div(&self, other: &[i32]) -> Result<Vec<i32>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length for SIMD operations".to_string(),
            ));
        }

        let len = self.len();
        let mut result = Vec::with_capacity(len);
        let simd_len = len / 4;
        let remainder = len % 4;

        // Process 4 elements at a time using SIMD
        // For integer division, we'll convert to f64, perform division, then convert back
        for i in 0..simd_len {
            let a = [
                self[i * 4] as f64,
                self[i * 4 + 1] as f64,
                self[i * 4 + 2] as f64,
                self[i * 4 + 3] as f64,
            ];
            let b = [
                other[i * 4] as f64,
                other[i * 4 + 1] as f64,
                other[i * 4 + 2] as f64,
                other[i * 4 + 3] as f64,
            ];

            let res = [
                (a[0] / b[0]) as i32,
                (a[1] / b[1]) as i32,
                (a[2] / b[2]) as i32,
                (a[3] / b[3]) as i32,
            ];

            result.extend_from_slice(&res);
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result.push(self[i] / other[i]);
        }

        Ok(result)
    }

    fn std_simd_sum(&self) -> Result<i32, VeloxxError> {
        let len = self.len();
        let simd_len = len / 4;
        let remainder = len % 4;
        let mut simd_sum = i32x4::ZERO;

        // Process 4 elements at a time using SIMD
        for i in 0..simd_len {
            let a = i32x4::from([
                self[i * 4],
                self[i * 4 + 1],
                self[i * 4 + 2],
                self[i * 4 + 3],
            ]);
            simd_sum += a;
        }

        // Sum the SIMD result
        let mut sum = simd_sum.reduce_add();

        // Add remaining elements
        sum += self
            .iter()
            .skip(len - remainder)
            .take(remainder)
            .copied()
            .sum::<i32>();

        Ok(sum)
    }

    fn std_simd_mean(&self) -> Result<Option<f64>, VeloxxError> {
        if self.is_empty() {
            Ok(None)
        } else {
            let sum = self.std_simd_sum()? as f64;
            Ok(Some(sum / self.len() as f64))
        }
    }
}

/// Optimized SIMD operations using the `wide` crate
pub mod optimized {
    #[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
    use wide::f64x4;

    /// Perform SIMD sum reduction more efficiently
    use crate::VeloxxError;
    pub fn std_simd_sum_optimized(values: &[f64]) -> Result<f64, VeloxxError> {
        let len = values.len();
        let simd_len = len / 4;

        // Handle small arrays directly
        if simd_len == 0 {
            return Ok(values.iter().sum());
        }

        let mut chunks = values.chunks_exact(4);
        let mut simd_sum = f64x4::ZERO;

        // Process 4 elements at a time using SIMD
        for chunk in chunks.by_ref() {
            let vec = f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]);
            simd_sum += vec;
        }

        // Sum the SIMD result
        let mut sum = simd_sum.reduce_add();

        // Add remaining elements
        sum += chunks.remainder().iter().sum::<f64>();

        Ok(sum)
    }
}

#[cfg(all(test, feature = "simd"))]
mod tests {
    use super::*;

    #[test]
    fn test_std_simd_add_f64() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let b = [1.0, 1.0, 1.0, 1.0, 1.0];
        let result = a.std_simd_add(&b).unwrap();
        assert_eq!(result, vec![2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_std_simd_sum_f64() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let result = a.std_simd_sum().unwrap();
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_std_simd_mean_f64() {
        let a = [2.0, 4.0, 6.0, 8.0];
        let result = a.std_simd_mean().unwrap().unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_std_simd_add_i32() {
        let a = [1, 2, 3, 4, 5];
        let b = [1, 1, 1, 1, 1];
        let result = a.std_simd_add(&b).unwrap();
        assert_eq!(result, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_std_simd_sum_i32() {
        let a = [1, 2, 3, 4, 5];
        let result = a.std_simd_sum().unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_std_simd_mean_i32() {
        let a = [2, 4, 6, 8];
        let result = a.std_simd_mean().unwrap().unwrap();
        assert_eq!(result, 5.0);
    }
}
