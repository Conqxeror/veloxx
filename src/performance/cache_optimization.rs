// src/performance/cache_optimization.rs
use crate::VeloxxError;

/// Cache-friendly data structures and algorithms
pub struct CacheOptimization;

impl CacheOptimization {
    /// Block-wise processing for better cache locality
    pub fn blocked_filter_f64(
        values: &[f64],
        condition: &[bool],
        block_size: usize,
    ) -> Result<Vec<f64>, VeloxxError> {
        if values.len() != condition.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have the same length".to_string(),
            ));
        }

        let mut result = Vec::new();

        // Process in cache-friendly blocks
        for chunk_start in (0..values.len()).step_by(block_size) {
            let chunk_end = (chunk_start + block_size).min(values.len());

            // Pre-allocate space for the block
            let mut block_result = Vec::with_capacity(block_size);

            for i in chunk_start..chunk_end {
                if condition[i] {
                    block_result.push(values[i]);
                }
            }

            result.extend(block_result);
        }

        Ok(result)
    }

    /// Cache-optimized groupby operation with pre-sorted data
    pub fn cache_optimized_groupby_sum_f64(
        group_ids: &[u32],
        values: &[f64],
    ) -> Result<Vec<(u32, f64)>, VeloxxError> {
        if group_ids.len() != values.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have the same length".to_string(),
            ));
        }

        let mut groups: Vec<(u32, f64)> = Vec::new();

        if group_ids.is_empty() {
            return Ok(groups);
        }

        let mut current_group = group_ids[0];
        let mut current_sum = values[0];

        for i in 1..group_ids.len() {
            if group_ids[i] == current_group {
                current_sum += values[i];
            } else {
                groups.push((current_group, current_sum));
                current_group = group_ids[i];
                current_sum = values[i];
            }
        }

        // Don't forget the last group
        groups.push((current_group, current_sum));

        Ok(groups)
    }

    /// Prefetch-aware sequential scan
    pub fn prefetch_scan_f64(values: &[f64], predicate: fn(f64) -> bool) -> Vec<usize> {
        let mut indices = Vec::new();

        // Process with software prefetching hints
        const PREFETCH_DISTANCE: usize = 64; // Cache line prefetch distance

        for i in 0..values.len() {
            // Software prefetch hint (for future optimization)
            if i + PREFETCH_DISTANCE < values.len() {
                // In a real implementation, we'd use intrinsics like _mm_prefetch
                // For now, this is just a placeholder for the pattern
            }

            if predicate(values[i]) {
                indices.push(i);
            }
        }

        indices
    }
}

/// Column-oriented layout optimization for structs of arrays
#[derive(Debug)]
pub struct ColumnLayout {
    pub ids: Vec<u32>,
    pub values: Vec<f64>,
    pub flags: Vec<bool>,
}

impl ColumnLayout {
    pub fn new(capacity: usize) -> Self {
        Self {
            ids: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            flags: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, id: u32, value: f64, flag: bool) {
        self.ids.push(id);
        self.values.push(value);
        self.flags.push(flag);
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Filter by flag with cache-friendly access
    pub fn filter_by_flag(&self) -> Vec<(u32, f64)> {
        let mut result = Vec::new();

        for i in 0..self.len() {
            if self.flags[i] {
                result.push((self.ids[i], self.values[i]));
            }
        }

        result
    }

    /// Sum values where flag is true
    pub fn sum_where_flag(&self) -> f64 {
        let mut sum = 0.0;

        for i in 0..self.len() {
            if self.flags[i] {
                sum += self.values[i];
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocked_filter() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let condition = [true, false, true, false, true, false, true, false];

        let result = CacheOptimization::blocked_filter_f64(&values, &condition, 4).unwrap();
        assert_eq!(result, vec![1.0, 3.0, 5.0, 7.0]);
    }

    #[test]
    fn test_cache_optimized_groupby() {
        let group_ids = [1, 1, 2, 2, 2, 3, 3];
        let values = [10.0, 20.0, 5.0, 15.0, 25.0, 7.0, 13.0];

        let result =
            CacheOptimization::cache_optimized_groupby_sum_f64(&group_ids, &values).unwrap();
        assert_eq!(result, vec![(1, 30.0), (2, 45.0), (3, 20.0)]);
    }

    #[test]
    fn test_prefetch_scan() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        let indices = CacheOptimization::prefetch_scan_f64(&values, |x| x > 3.0);
        assert_eq!(indices, vec![3, 4]);
    }

    #[test]
    fn test_column_layout() {
        let mut layout = ColumnLayout::new(4);
        layout.push(1, 10.0, true);
        layout.push(2, 20.0, false);
        layout.push(3, 30.0, true);
        layout.push(4, 40.0, false);

        let filtered = layout.filter_by_flag();
        assert_eq!(filtered, vec![(1, 10.0), (3, 30.0)]);

        let sum = layout.sum_where_flag();
        assert_eq!(sum, 40.0);
    }
}
