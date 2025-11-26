//! Parallel processing optimizations using Rayon
//!
//! This module provides parallel implementations of data operations
//! for improved performance on multi-core systems.

#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
use crate::performance::optimized_simd::OptimizedSimdOps;
use crate::series::Series;
use crate::types::Value;
use crate::VeloxxError;
use rayon::prelude::*;

/// Parallel aggregation functions
pub struct ParallelAggregations;

impl ParallelAggregations {
    /// Parallel sum calculation for numeric series
    pub fn par_sum(series: &Series) -> Result<Value, VeloxxError> {
        const PARALLEL_THRESHOLD: usize = 500_000;

        match series {
            Series::I32(_, values, bitmap) => {
                // Check for nulls - fast path if no nulls (all true)
                // Checking all bits might be slow? But faster than filtering.
                // Actually, we can just check if we can rely on 0s?
                // Let's assume we must check.
                let has_nulls = bitmap.iter().any(|&b| !b);

                if !has_nulls {
                    #[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
                    {
                        if values.len() < PARALLEL_THRESHOLD {
                            return Ok(Value::I32(values.optimized_simd_sum()));
                        } else {
                            let sum: i32 = values
                                .par_chunks(16 * 1024)
                                .map(|chunk| chunk.optimized_simd_sum())
                                .sum();
                            return Ok(Value::I32(sum));
                        }
                    }
                    #[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))] 
                    {
                        let sum: i32 = values.par_iter().sum();
                        return Ok(Value::I32(sum));
                    }
                }

                // Fallback for nulls
                let sum: i32 = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, b)| if *b { Some(*v) } else { None })
                    .sum();
                Ok(Value::I32(sum))
            }
            Series::F64(_, values, bitmap) => {
                let has_nulls = bitmap.iter().any(|&b| !b);

                if !has_nulls {
                    #[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
                    {
                        if values.len() < PARALLEL_THRESHOLD {
                            return Ok(Value::F64(values.optimized_simd_sum()));
                        } else {
                            let sum: f64 = values
                                .par_chunks(16 * 1024)
                                .map(|chunk| chunk.optimized_simd_sum())
                                .sum();
                            return Ok(Value::F64(sum));
                        }
                    }
                    #[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))] 
                    {
                        let sum: f64 = values.par_iter().sum();
                        return Ok(Value::F64(sum));
                    }
                }

                let sum: f64 = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, b)| if *b { Some(*v) } else { None })
                    .sum();
                Ok(Value::F64(sum))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Sum operation not supported for this data type".to_string(),
            )),
        }
    }

    /// Parallel mean calculation for numeric series
    pub fn par_mean(series: &Series) -> Result<Value, VeloxxError> {
        let count = series.count();
        if count == 0 {
            return Ok(Value::Null);
        }

        match series {
            Series::I32(_, values, bitmap) => {
                let sum: i32 = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, b)| if *b { Some(*v) } else { None })
                    .sum();
                Ok(Value::F64(sum as f64 / count as f64))
            }
            Series::F64(_, values, bitmap) => {
                let sum: f64 = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, b)| if *b { Some(*v) } else { None })
                    .sum();
                Ok(Value::F64(sum / count as f64))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Mean operation not supported for this data type".to_string(),
            )),
        }
    }

    /// Parallel min calculation
    pub fn par_min(series: &Series) -> Result<Value, VeloxxError> {
        match series {
            Series::I32(_, values, bitmap) => {
                if let Some(min) = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, b)| if *b { Some(v) } else { None })
                    .min()
                {
                    Ok(Value::I32(*min))
                } else {
                    Ok(Value::Null)
                }
            }
            Series::F64(_, values, bitmap) => {
                if let Some(min) = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, b)| if *b { Some(v) } else { None })
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                {
                    Ok(Value::F64(*min))
                } else {
                    Ok(Value::Null)
                }
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Min operation not supported for this data type".to_string(),
            )),
        }
    }

    /// Parallel max calculation
    pub fn par_max(series: &Series) -> Result<Value, VeloxxError> {
        match series {
            Series::I32(_, values, bitmap) => {
                if let Some(max) = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, b)| if *b { Some(v) } else { None })
                    .max()
                {
                    Ok(Value::I32(*max))
                } else {
                    Ok(Value::Null)
                }
            }
            Series::F64(_, values, bitmap) => {
                if let Some(max) = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(v, b)| if *b { Some(v) } else { None })
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                {
                    Ok(Value::F64(*max))
                } else {
                    Ok(Value::Null)
                }
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Max operation not supported for this data type".to_string(),
            )),
        }
    }
}

/// Parallel sorting operations
pub struct ParallelSort;

impl ParallelSort {
    /// Parallel sort indices for a series
    pub fn par_sort_indices(series: &Series, ascending: bool) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..series.len()).collect();

        indices.par_sort_by(|&a, &b| {
            let val_a = series.get_value(a);
            let val_b = series.get_value(b);

            let cmp = match (val_a, val_b) {
                (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            };

            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });

        indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum() {
        let series = Series::new_i32("test", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let result = ParallelAggregations::par_sum(&series).unwrap();
        assert_eq!(result, Value::I32(15));
    }

    #[test]
    fn test_parallel_mean() {
        let series = Series::new_f64("test", vec![Some(2.0), Some(4.0), Some(6.0), Some(8.0)]);
        let result = ParallelAggregations::par_mean(&series).unwrap();
        assert_eq!(result, Value::F64(5.0));
    }

    #[test]
    fn test_parallel_min_max() {
        let series = Series::new_i32("test", vec![Some(5), Some(1), Some(9), Some(3), Some(7)]);

        let min_result = ParallelAggregations::par_min(&series).unwrap();
        assert_eq!(min_result, Value::I32(1));

        let max_result = ParallelAggregations::par_max(&series).unwrap();
        assert_eq!(max_result, Value::I32(9));
    }

    #[test]
    fn test_parallel_sort_indices() {
        let series = Series::new_i32("test", vec![Some(5), Some(1), Some(9), Some(3), Some(7)]);

        let indices = ParallelSort::par_sort_indices(&series, true);
        assert_eq!(indices, vec![1, 3, 0, 4, 2]); // Sorted: 1, 3, 5, 7, 9

        let indices_desc = ParallelSort::par_sort_indices(&series, false);
        assert_eq!(indices_desc, vec![2, 4, 0, 3, 1]); // Sorted: 9, 7, 5, 3, 1
    }
}
