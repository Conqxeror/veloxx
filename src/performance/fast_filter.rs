// Fast Filter Module - SIMD-accelerated filtering operations
// Targeting 37x performance improvement to match Polars performance

use rayon::prelude::*;
use std::error::Error;

// Advanced SIMD imports for maximum performance
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Ultra-fast filtering with multi-tier SIMD optimization
pub struct FastFilter;

impl FastFilter {
    /// SIMD-accelerated numeric filtering with AVX2 optimization
    /// Targets 10-20x speedup over regular filtering operations
    pub fn simd_filter_i32_gt(
        values: &[i32],
        threshold: i32,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe {
                    return Self::avx2_filter_i32_gt(values, threshold, chunk_size);
                }
            }
        }

        // Fallback to portable SIMD
        Self::portable_filter_i32_gt(values, threshold, chunk_size)
    }

    /// AVX2-optimized integer filtering
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn avx2_filter_i32_gt(
        values: &[i32],
        threshold: i32,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        let mut result = vec![false; values.len()];
        let threshold_vec = _mm256_set1_epi32(threshold);

        let chunks: Vec<_> = values.chunks(chunk_size).collect();
        let result_chunks: Vec<_> = result.chunks_mut(chunk_size).collect();

        chunks
            .into_par_iter()
            .zip(result_chunks.into_par_iter())
            .for_each(|(chunk, result_chunk)| {
                let mut i = 0;

                // Process 8 integers at a time with AVX2
                while i + 8 <= chunk.len() {
                    let data_vec = _mm256_loadu_si256(chunk.as_ptr().add(i) as *const __m256i);
                    let cmp_result = _mm256_cmpgt_epi32(data_vec, threshold_vec);

                    // Extract comparison results to boolean array
                    let mask = _mm256_movemask_ps(_mm256_castsi256_ps(cmp_result));

                    for j in 0..8 {
                        if i + j < chunk.len() {
                            result_chunk[i + j] = (mask & (1 << j)) != 0;
                        }
                    }

                    i += 8;
                }

                // Handle remaining elements
                while i < chunk.len() {
                    result_chunk[i] = chunk[i] > threshold;
                    i += 1;
                }
            });

        Ok(result)
    }

    /// Portable SIMD filtering fallback
    fn portable_filter_i32_gt(
        values: &[i32],
        threshold: i32,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        let result: Vec<bool> = values
            .par_chunks(chunk_size)
            .flat_map(|chunk| chunk.iter().map(|&val| val > threshold).collect::<Vec<_>>())
            .collect();

        Ok(result)
    }

    /// SIMD-accelerated floating point filtering
    pub fn simd_filter_f64_gt(
        values: &[f64],
        threshold: f64,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe {
                    return Self::avx2_filter_f64_gt(values, threshold, chunk_size);
                }
            }
        }

        Self::portable_filter_f64_gt(values, threshold, chunk_size)
    }

    /// AVX2-optimized float filtering
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn avx2_filter_f64_gt(
        values: &[f64],
        threshold: f64,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        let mut result = vec![false; values.len()];
        let threshold_vec = _mm256_set1_pd(threshold);

        let chunks: Vec<_> = values.chunks(chunk_size).collect();
        let result_chunks: Vec<_> = result.chunks_mut(chunk_size).collect();

        chunks
            .into_par_iter()
            .zip(result_chunks.into_par_iter())
            .for_each(|(chunk, result_chunk)| {
                let mut i = 0;

                // Process 4 doubles at a time with AVX2
                while i + 4 <= chunk.len() {
                    let data_vec = _mm256_loadu_pd(chunk.as_ptr().add(i));
                    let cmp_result = _mm256_cmp_pd(data_vec, threshold_vec, _CMP_GT_OQ);

                    let mask = _mm256_movemask_pd(cmp_result);

                    for j in 0..4 {
                        if i + j < chunk.len() {
                            result_chunk[i + j] = (mask & (1 << j)) != 0;
                        }
                    }

                    i += 4;
                }

                // Handle remaining elements
                while i < chunk.len() {
                    result_chunk[i] = chunk[i] > threshold;
                    i += 1;
                }
            });

        Ok(result)
    }

    /// Portable floating point filtering
    fn portable_filter_f64_gt(
        values: &[f64],
        threshold: f64,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        let result: Vec<bool> = values
            .par_chunks(chunk_size)
            .flat_map(|chunk| chunk.iter().map(|&val| val > threshold).collect::<Vec<_>>())
            .collect();

        Ok(result)
    }

    /// Ultra-fast string filtering with parallel processing
    pub fn simd_filter_string_contains(
        values: &[String],
        pattern: &str,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        let result: Vec<bool> = values
            .par_chunks(chunk_size)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .map(|val| val.contains(pattern))
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(result)
    }

    /// Multi-condition SIMD filtering with expression fusion
    pub fn simd_filter_multi_condition(
        int_values: &[i32],
        float_values: &[f64],
        int_threshold: i32,
        float_threshold: f64,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe {
                    return Self::avx2_filter_multi_condition(
                        int_values,
                        float_values,
                        int_threshold,
                        float_threshold,
                        chunk_size,
                    );
                }
            }
        }

        Self::portable_filter_multi_condition(
            int_values,
            float_values,
            int_threshold,
            float_threshold,
            chunk_size,
        )
    }

    /// AVX2 multi-condition filtering with expression fusion
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn avx2_filter_multi_condition(
        int_values: &[i32],
        float_values: &[f64],
        int_threshold: i32,
        float_threshold: f64,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        let len = int_values.len().min(float_values.len());
        let mut result = vec![false; len];

        let _int_threshold_vec = _mm256_set1_epi32(int_threshold);
        let float_threshold_vec = _mm256_set1_pd(float_threshold);

        let int_chunks: Vec<_> = int_values[..len].chunks(chunk_size).collect();
        let float_chunks: Vec<_> = float_values[..len].chunks(chunk_size).collect();
        let result_chunks: Vec<_> = result.chunks_mut(chunk_size).collect();

        int_chunks
            .into_par_iter()
            .zip(float_chunks.into_par_iter())
            .zip(result_chunks.into_par_iter())
            .for_each(|((int_chunk, float_chunk), result_chunk)| {
                let mut i = 0;
                let process_len = int_chunk.len().min(float_chunk.len());

                // Process in aligned chunks for maximum SIMD efficiency
                while i + 4 <= process_len {
                    // Load and compare integers (need to align to 4 for float comparison)
                    let int_data = [
                        int_chunk.get(i).copied().unwrap_or(0),
                        int_chunk.get(i + 1).copied().unwrap_or(0),
                        int_chunk.get(i + 2).copied().unwrap_or(0),
                        int_chunk.get(i + 3).copied().unwrap_or(0),
                    ];

                    // Load float data
                    let float_data_vec = _mm256_set_pd(
                        float_chunk.get(i + 3).copied().unwrap_or(0.0),
                        float_chunk.get(i + 2).copied().unwrap_or(0.0),
                        float_chunk.get(i + 1).copied().unwrap_or(0.0),
                        float_chunk.get(i).copied().unwrap_or(0.0),
                    );

                    // Compare floats with SIMD
                    let float_cmp = _mm256_cmp_pd(float_data_vec, float_threshold_vec, _CMP_GT_OQ);
                    let float_mask = _mm256_movemask_pd(float_cmp);

                    // Combine conditions for each element
                    for j in 0..4 {
                        if i + j < process_len {
                            let int_condition = int_data[j] > int_threshold;
                            let float_condition = (float_mask & (1 << j)) != 0;
                            result_chunk[i + j] = int_condition && float_condition;
                        }
                    }

                    i += 4;
                }

                // Handle remaining elements
                while i < process_len {
                    result_chunk[i] =
                        int_chunk[i] > int_threshold && float_chunk[i] > float_threshold;
                    i += 1;
                }
            });

        Ok(result)
    }

    /// Portable multi-condition filtering
    fn portable_filter_multi_condition(
        int_values: &[i32],
        float_values: &[f64],
        int_threshold: i32,
        float_threshold: f64,
        chunk_size: usize,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        let len = int_values.len().min(float_values.len());

        let result: Vec<bool> = (0..len)
            .into_par_iter()
            .chunks(chunk_size)
            .flat_map(|chunk| {
                chunk
                    .into_iter()
                    .map(|i| int_values[i] > int_threshold && float_values[i] > float_threshold)
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_filter_i32_basic() {
        let values = vec![1, 5, 3, 8, 2, 9, 4, 7, 6];
        let threshold = 5;
        let chunk_size = 1024;

        let result = FastFilter::simd_filter_i32_gt(&values, threshold, chunk_size).unwrap();
        let expected = vec![false, false, false, true, false, true, false, true, true];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simd_filter_f64_basic() {
        let values = vec![1.1, 5.5, 3.3, 8.8, 2.2];
        let threshold = 5.0;
        let chunk_size = 1024;

        let result = FastFilter::simd_filter_f64_gt(&values, threshold, chunk_size).unwrap();
        let expected = vec![false, true, false, true, false];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simd_filter_string_contains() {
        let values = vec![
            "hello".to_string(),
            "world".to_string(),
            "hello world".to_string(),
            "test".to_string(),
        ];
        let pattern = "hello";
        let chunk_size = 1024;

        let result = FastFilter::simd_filter_string_contains(&values, pattern, chunk_size).unwrap();
        let expected = vec![true, false, true, false];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simd_filter_multi_condition() {
        let int_values = vec![1, 6, 3, 8, 2];
        let float_values = vec![1.1, 6.6, 3.3, 8.8, 2.2];
        let int_threshold = 5;
        let float_threshold = 5.0;
        let chunk_size = 1024;

        let result = FastFilter::simd_filter_multi_condition(
            &int_values,
            &float_values,
            int_threshold,
            float_threshold,
            chunk_size,
        )
        .unwrap();

        // Only elements where both conditions are true: int > 5 AND float > 5.0
        let expected = vec![false, true, false, true, false];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_dataset_performance() {
        let size = 100_000;
        let values: Vec<i32> = (0..size as i32).collect();
        let threshold = size as i32 / 2;
        let chunk_size = 8192; // Optimized chunk size for cache efficiency

        let result = FastFilter::simd_filter_i32_gt(&values, threshold, chunk_size).unwrap();

        assert_eq!(result.len(), size);
        // First half should be false, second half should be true
        assert!(!result[size / 4]);
        assert!(result[3 * size / 4]);
    }
}
