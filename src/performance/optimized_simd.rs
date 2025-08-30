#[cfg(target_arch = "x86_64")]
#[inline]
fn avx2_simd_add_f64(a: &[f64], b: &[f64], result: &mut [f64]) -> bool {
    use std::arch::x86_64::*;
    if !is_x86_feature_detected!("avx2") {
        return false;
    }
    let len = a.len();
    let chunks = len / 4;
    let remainder = len % 4;
    unsafe {
        for i in 0..chunks {
            let pa = a.as_ptr().add(i * 4) as *const __m256d;
            let pb = b.as_ptr().add(i * 4) as *const __m256d;
            let pr = result.as_mut_ptr().add(i * 4) as *mut __m256d;
            let va = _mm256_loadu_pd(pa as *const f64);
            let vb = _mm256_loadu_pd(pb as *const f64);
            let vsum = _mm256_add_pd(va, vb);
            _mm256_storeu_pd(pr as *mut f64, vsum);
        }
        for i in (len - remainder)..len {
            result[i] = a[i] + b[i];
        }
    }
    true
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn avx2_simd_sub_f64(a: &[f64], b: &[f64], result: &mut [f64]) -> bool {
    use std::arch::x86_64::*;
    if !is_x86_feature_detected!("avx2") {
        return false;
    }
    let len = a.len();
    let chunks = len / 4;
    let remainder = len % 4;
    unsafe {
        for i in 0..chunks {
            let pa = a.as_ptr().add(i * 4) as *const __m256d;
            let pb = b.as_ptr().add(i * 4) as *const __m256d;
            let pr = result.as_mut_ptr().add(i * 4) as *mut __m256d;
            let va = _mm256_loadu_pd(pa as *const f64);
            let vb = _mm256_loadu_pd(pb as *const f64);
            let vdiff = _mm256_sub_pd(va, vb);
            _mm256_storeu_pd(pr as *mut f64, vdiff);
        }
        for i in (len - remainder)..len {
            result[i] = a[i] - b[i];
        }
    }
    true
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn avx2_simd_mul_f64(a: &[f64], b: &[f64], result: &mut [f64]) -> bool {
    use std::arch::x86_64::*;
    if !is_x86_feature_detected!("avx2") {
        return false;
    }
    let len = a.len();
    let chunks = len / 4;
    let remainder = len % 4;
    unsafe {
        for i in 0..chunks {
            let pa = a.as_ptr().add(i * 4) as *const __m256d;
            let pb = b.as_ptr().add(i * 4) as *const __m256d;
            let pr = result.as_mut_ptr().add(i * 4) as *mut __m256d;
            let va = _mm256_loadu_pd(pa as *const f64);
            let vb = _mm256_loadu_pd(pb as *const f64);
            let vprod = _mm256_mul_pd(va, vb);
            _mm256_storeu_pd(pr as *mut f64, vprod);
        }
        for i in (len - remainder)..len {
            result[i] = a[i] * b[i];
        }
    }
    true
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn avx2_simd_div_f64(a: &[f64], b: &[f64], result: &mut [f64]) -> bool {
    use std::arch::x86_64::*;
    if !is_x86_feature_detected!("avx2") {
        return false;
    }
    let len = a.len();
    let chunks = len / 4;
    let remainder = len % 4;
    unsafe {
        for i in 0..chunks {
            let pa = a.as_ptr().add(i * 4) as *const __m256d;
            let pb = b.as_ptr().add(i * 4) as *const __m256d;
            let pr = result.as_mut_ptr().add(i * 4) as *mut __m256d;
            let va = _mm256_loadu_pd(pa as *const f64);
            let vb = _mm256_loadu_pd(pb as *const f64);
            let vquot = _mm256_div_pd(va, vb);
            _mm256_storeu_pd(pr as *mut f64, vquot);
        }
        for i in (len - remainder)..len {
            result[i] = a[i] / b[i];
        }
    }
    true
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn avx2_simd_sum_f64(a: &[f64]) -> Option<f64> {
    use std::arch::x86_64::*;
    if !is_x86_feature_detected!("avx2") {
        return None;
    }
    let len = a.len();
    let chunks = len / 4;
    let remainder = len % 4;
    unsafe {
        let mut vacc = _mm256_setzero_pd();
        for i in 0..chunks {
            let pa = a.as_ptr().add(i * 4) as *const __m256d;
            let va = _mm256_loadu_pd(pa as *const f64);
            vacc = _mm256_add_pd(vacc, va);
        }
        // Horizontal sum of the 4 accumulated values
        let hi = _mm256_extractf128_pd(vacc, 1);
        let lo = _mm256_castpd256_pd128(vacc);
        let sum_hi_lo = _mm_add_pd(hi, lo);
        let sum_final = _mm_hadd_pd(sum_hi_lo, sum_hi_lo);
        let mut result = _mm_cvtsd_f64(sum_final);
        // Add remaining elements
        result += a.iter().skip(len - remainder).take(remainder).sum::<f64>();
        Some(result)
    }
}

use rayon::prelude::*;
// Optimized SIMD operations using the `wide` crate
//
// This module provides highly optimized vectorized implementations of common
// mathematical operations using the `wide` crate for cross-platform SIMD support.

#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
use wide::{f64x4, i32x4};

/// Trait for optimized SIMD operations
pub trait OptimizedSimdOps<T> {
    /// Vectorized addition with reduced allocations
    fn optimized_simd_add(&self, other: &[T], result: &mut [T]);

    /// Vectorized subtraction with reduced allocations
    fn optimized_simd_sub(&self, other: &[T], result: &mut [T]);

    /// Vectorized multiplication with reduced allocations
    fn optimized_simd_mul(&self, other: &[T], result: &mut [T]);

    /// Vectorized division with reduced allocations
    fn optimized_simd_div(&self, other: &[T], result: &mut [T]);

    /// Vectorized sum reduction
    fn optimized_simd_sum(&self) -> T;
}

/// Optimized SIMD implementation for f64 slices
impl OptimizedSimdOps<f64> for [f64] {
    // ...existing code...

    fn optimized_simd_add(&self, other: &[f64], result: &mut [f64]) {
        assert_eq!(self.len(), other.len(), "Arrays must have same length");
        assert_eq!(
            self.len(),
            result.len(),
            "Result array must have same length"
        );
        #[cfg(target_arch = "x86_64")]
        {
            if avx2_simd_add_f64(self, other, result) {
                return;
            }
        }
        // Fallback to portable SIMD
        let len = self.len();
        let simd_len = len / 4;
        let remainder = len % 4;
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
            result[i * 4] = res_array[0];
            result[i * 4 + 1] = res_array[1];
            result[i * 4 + 2] = res_array[2];
            result[i * 4 + 3] = res_array[3];
        }
        for i in (len - remainder)..len {
            result[i] = self[i] + other[i];
        }
    }

    fn optimized_simd_sub(&self, other: &[f64], result: &mut [f64]) {
        assert_eq!(self.len(), other.len(), "Arrays must have same length");
        assert_eq!(
            self.len(),
            result.len(),
            "Result array must have same length"
        );
        #[cfg(target_arch = "x86_64")]
        {
            if avx2_simd_sub_f64(self, other, result) {
                return;
            }
        }
        // Fallback to portable SIMD
        let len = self.len();
        let simd_len = len / 4;
        let remainder = len % 4;
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
            result[i * 4] = res_array[0];
            result[i * 4 + 1] = res_array[1];
            result[i * 4 + 2] = res_array[2];
            result[i * 4 + 3] = res_array[3];
        }
        for i in (len - remainder)..len {
            result[i] = self[i] - other[i];
        }
    }

    fn optimized_simd_mul(&self, other: &[f64], result: &mut [f64]) {
        assert_eq!(self.len(), other.len(), "Arrays must have same length");
        assert_eq!(
            self.len(),
            result.len(),
            "Result array must have same length"
        );
        #[cfg(target_arch = "x86_64")]
        {
            if avx2_simd_mul_f64(self, other, result) {
                return;
            }
        }
        // Fallback to portable SIMD
        let len = self.len();
        let simd_len = len / 4;
        let remainder = len % 4;
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
            result[i * 4] = res_array[0];
            result[i * 4 + 1] = res_array[1];
            result[i * 4 + 2] = res_array[2];
            result[i * 4 + 3] = res_array[3];
        }
        for i in (len - remainder)..len {
            result[i] = self[i] * other[i];
        }
    }

    fn optimized_simd_div(&self, other: &[f64], result: &mut [f64]) {
        assert_eq!(self.len(), other.len(), "Arrays must have same length");
        assert_eq!(
            self.len(),
            result.len(),
            "Result array must have same length"
        );
        #[cfg(target_arch = "x86_64")]
        {
            if avx2_simd_div_f64(self, other, result) {
                return;
            }
        }
        // Fallback to portable SIMD
        let len = self.len();
        let simd_len = len / 4;
        let remainder = len % 4;
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
            result[i * 4] = res_array[0];
            result[i * 4 + 1] = res_array[1];
            result[i * 4 + 2] = res_array[2];
            result[i * 4 + 3] = res_array[3];
        }
        for i in (len - remainder)..len {
            result[i] = self[i] / other[i];
        }
    }

    fn optimized_simd_sum(&self) -> f64 {
        #[cfg(target_arch = "x86_64")]
        {
            let result = avx2_simd_sum_f64(self);
            if let Some(sum) = result {
                return sum;
            }
        }
        // Fallback to portable SIMD
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
}

/// Optimized SIMD implementation for i32 slices
impl OptimizedSimdOps<i32> for [i32] {
    fn optimized_simd_add(&self, other: &[i32], result: &mut [i32]) {
        assert_eq!(self.len(), other.len(), "Arrays must have same length");
        assert_eq!(
            self.len(),
            result.len(),
            "Result array must have same length"
        );

        let len = self.len();
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
            result[i * 4] = res_array[0];
            result[i * 4 + 1] = res_array[1];
            result[i * 4 + 2] = res_array[2];
            result[i * 4 + 3] = res_array[3];
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result[i] = self[i] + other[i];
        }
    }

    fn optimized_simd_sub(&self, other: &[i32], result: &mut [i32]) {
        assert_eq!(self.len(), other.len(), "Arrays must have same length");
        assert_eq!(
            self.len(),
            result.len(),
            "Result array must have same length"
        );

        let len = self.len();
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
            result[i * 4] = res_array[0];
            result[i * 4 + 1] = res_array[1];
            result[i * 4 + 2] = res_array[2];
            result[i * 4 + 3] = res_array[3];
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result[i] = self[i] - other[i];
        }
    }

    fn optimized_simd_mul(&self, other: &[i32], result: &mut [i32]) {
        assert_eq!(self.len(), other.len(), "Arrays must have same length");
        assert_eq!(
            self.len(),
            result.len(),
            "Result array must have same length"
        );

        let len = self.len();
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
            result[i * 4] = res_array[0];
            result[i * 4 + 1] = res_array[1];
            result[i * 4 + 2] = res_array[2];
            result[i * 4 + 3] = res_array[3];
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result[i] = self[i] * other[i];
        }
    }

    fn optimized_simd_div(&self, other: &[i32], result: &mut [i32]) {
        assert_eq!(self.len(), other.len(), "Arrays must have same length");
        assert_eq!(
            self.len(),
            result.len(),
            "Result array must have same length"
        );

        let len = self.len();
        let simd_len = len / 4;
        let remainder = len % 4;

        // Process 4 elements at a time using SIMD
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

            result[i * 4] = res[0];
            result[i * 4 + 1] = res[1];
            result[i * 4 + 2] = res[2];
            result[i * 4 + 3] = res[3];
        }

        // Process remaining elements
        for i in (len - remainder)..len {
            result[i] = self[i] / other[i];
        }
    }

    fn optimized_simd_sum(&self) -> i32 {
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
}

/// Parallel SIMD addition using Rayon for large slices
pub fn parallel_simd_add_f64(a: &[f64], b: &[f64], result: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "Arrays must have same length");
    assert_eq!(a.len(), result.len(), "Result array must have same length");
    let chunk_size = 16_384; // Tune for cache size and CPU
    a.par_chunks(chunk_size)
        .zip(b.par_chunks(chunk_size))
        .zip(result.par_chunks_mut(chunk_size))
        .for_each(|((a, b), r)| {
            a.optimized_simd_add(b, r);
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_simd_add_f64() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let b = [1.0, 1.0, 1.0, 1.0, 1.0];
        let mut result = [0.0; 5];
        a.optimized_simd_add(&b, &mut result);
        assert_eq!(result, [2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_optimized_simd_sum_f64() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let result = a.optimized_simd_sum();
        assert_eq!(result, 15.0);
    }
}
