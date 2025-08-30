//! SIMD-optimized operations for numeric computations
//!
//! This module provides vectorized implementations of common mathematical
//! operations for improved performance on numeric data.
#![cfg(feature = "simd")]

#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
use wide::{f64x4, i32x4};

/// Trait for SIMD-optimized operations
pub trait SimdOps<T> {
    /// Vectorized addition
    fn simd_add(&self, other: &[T]) -> Vec<T>;

    /// Vectorized subtraction
    fn simd_sub(&self, other: &[T]) -> Vec<T>;

    /// Vectorized multiplication
    fn simd_mul(&self, other: &[T]) -> Vec<T>;

    /// Vectorized division
    fn simd_div(&self, other: &[T]) -> Vec<T>;

    /// Vectorized sum reduction
    fn simd_sum(&self) -> T;

    /// Vectorized mean calculation
    fn simd_mean(&self) -> Option<T>;
}

/// SIMD implementation for f64 slices
impl SimdOps<f64> for [f64] {
    fn simd_add(&self, other: &[f64]) -> Vec<f64> {
        if self.len() != other.len() {
            panic!("Arrays must have same length for SIMD operations");
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
            let res = a + b;
            let res_array = res.to_array();
            result.extend_from_slice(&res_array);
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result.push(self[i] + other[i]);
        }

        result
    }

    fn simd_sub(&self, other: &[f64]) -> Vec<f64> {
        if self.len() != other.len() {
            panic!("Arrays must have same length for SIMD operations");
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

        result
    }

    fn simd_mul(&self, other: &[f64]) -> Vec<f64> {
        if self.len() != other.len() {
            panic!("Arrays must have same length for SIMD operations");
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

        result
    }

    fn simd_div(&self, other: &[f64]) -> Vec<f64> {
        if self.len() != other.len() {
            panic!("Arrays must have same length for SIMD operations");
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

        result
    }

    fn simd_sum(&self) -> f64 {
        let len = self.len();
        let simd_len = len / 4;
        let remainder = len % 4;
        let mut simd_sum = f64x4::ZERO;

        // Process 4 elements at a time using SIMD
        for i in 0..simd_len {
            let a = f64x4::from([
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
            .sum::<f64>();

        sum
    }

    fn simd_mean(&self) -> Option<f64> {
        if self.is_empty() {
            None
        } else {
            Some(self.simd_sum() / self.len() as f64)
        }
    }
}

/// SIMD implementation for i32 slices
impl SimdOps<i32> for [i32] {
    fn simd_add(&self, other: &[i32]) -> Vec<i32> {
        if self.len() != other.len() {
            panic!("Arrays must have same length for SIMD operations");
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

        result
    }

    fn simd_sub(&self, other: &[i32]) -> Vec<i32> {
        if self.len() != other.len() {
            panic!("Arrays must have same length for SIMD operations");
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

        result
    }

    fn simd_mul(&self, other: &[i32]) -> Vec<i32> {
        if self.len() != other.len() {
            panic!("Arrays must have same length for SIMD operations");
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

        result
    }

    fn simd_div(&self, other: &[i32]) -> Vec<i32> {
        if self.len() != other.len() {
            panic!("Arrays must have same length for SIMD operations");
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

        result
    }

    fn simd_sum(&self) -> i32 {
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
            .sum::<i32>();

        sum
    }

    fn simd_mean(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.simd_sum() / self.len() as i32)
        }
    }
}

/// Optimized SIMD operations that work directly with slices and avoid allocations
/// when possible
pub mod optimized {
    #[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
    use wide::f64x4;

    /// Perform SIMD addition in-place when possible
    pub fn simd_add_inplace(a: &mut [f64], b: &[f64]) {
        if a.len() != b.len() {
            panic!("Arrays must have same length for SIMD operations");
        }

        let len = a.len();
        let simd_len = len / 4;
        let _remainder = len % 4;

        // Process 4 elements at a time using SIMD
        for i in 0..simd_len {
            let a_vec = f64x4::from([a[i * 4], a[i * 4 + 1], a[i * 4 + 2], a[i * 4 + 3]]);
            let b_vec = f64x4::from([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]);
            let res = a_vec + b_vec;
            let res_array = res.to_array();
            a[i * 4] = res_array[0];
            a[i * 4 + 1] = res_array[1];
            a[i * 4 + 2] = res_array[2];
            a[i * 4 + 3] = res_array[3];
        }

        // Process remaining elements
        for i in (len - _remainder)..len {
            a[i] += b[i];
        }
    }

    /// Perform SIMD sum reduction more efficiently
    pub fn simd_sum_optimized(values: &[f64]) -> f64 {
        let len = values.len();
        let simd_len = len / 4;

        // Handle small arrays directly
        if simd_len == 0 {
            return values.iter().sum();
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

        sum
    }
}

/// Check if CPU supports AVX instructions
#[cfg(target_arch = "x86_64")]
pub fn has_avx_support() -> bool {
    #[cfg(target_feature = "avx")]
    {
        true
    }
    #[cfg(not(target_feature = "avx"))]
    {
        false
    }
}

/// Check if CPU supports AVX2 instructions
#[cfg(target_arch = "x86_64")]
pub fn has_avx2_support() -> bool {
    #[cfg(target_feature = "avx2")]
    {
        true
    }
    #[cfg(not(target_feature = "avx2"))]
    {
        false
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn has_avx_support() -> bool {
    false
}

#[cfg(not(target_arch = "x86_64"))]
pub fn has_avx2_support() -> bool {
    false
}

#[cfg(all(test, feature = "simd"))]
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

    #[test]
    fn test_simd_add_i32() {
        let a = [1, 2, 3, 4, 5];
        let b = [1, 1, 1, 1, 1];
        let result = a.simd_add(&b);
        assert_eq!(result, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_simd_sum_i32() {
        let a = [1, 2, 3, 4, 5];
        let result = a.simd_sum();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_simd_mean_i32() {
        let a = [2, 4, 6, 8];
        let result = a.simd_mean().unwrap();
        assert_eq!(result, 5);
    }
}
