//! SIMD-optimized operations for numeric computations
//!
//! This module provides vectorized implementations of common mathematical
//! operations for improved performance on numeric data.

/// Trait for SIMD-optimized operations
pub trait SimdOps<T> {
    /// Vectorized addition
    fn simd_add(&self, other: &[T]) -> Vec<T>;

    /// Vectorized sum reduction
    fn simd_sum(&self) -> T;

    /// Vectorized mean calculation
    fn simd_mean(&self) -> Option<T>;
}

/// SIMD implementation for f64 slices (fallback to regular operations)
impl SimdOps<f64> for [f64] {
    fn simd_add(&self, other: &[f64]) -> Vec<f64> {
        if self.len() != other.len() {
            panic!("Arrays must have same length for SIMD operations");
        }

        self.iter().zip(other.iter()).map(|(a, b)| a + b).collect()
    }

    fn simd_sum(&self) -> f64 {
        self.iter().sum()
    }

    fn simd_mean(&self) -> Option<f64> {
        if self.is_empty() {
            None
        } else {
            Some(self.simd_sum() / self.len() as f64)
        }
    }
}

/// Check if CPU supports AVX instructions
pub fn has_avx_support() -> bool {
    false // Simplified for now
}

/// Check if CPU supports AVX2 instructions
pub fn has_avx2_support() -> bool {
    false // Simplified for now
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_add_f64() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let b = [1.0, 1.0, 1.0, 1.0, 1.0];
        let result = a.simd_add(&b);
        assert_eq!(result, vec![2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_simd_sum_f64() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let result = a.simd_sum();
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_simd_mean_f64() {
        let a = [2.0, 4.0, 6.0, 8.0];
        let result = a.simd_mean().unwrap();
        assert_eq!(result, 5.0);
    }
}
