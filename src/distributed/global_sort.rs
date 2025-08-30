// src/distributed/global_sort.rs
use crate::VeloxxError;
use rayon::prelude::*;

/// Parallel global sort for large DataFrames
pub struct GlobalSort;

impl GlobalSort {
    /// Sort a large vector of f64 globally using parallel chunking and merging
    pub fn sort_f64(data: &mut [f64]) -> Result<(), VeloxxError> {
        let n = data.len();
        if n == 0 { return Ok(()); }
        let chunk_size = (n / rayon::current_num_threads().max(1)).max(8192);
        // Sort chunks in parallel
        data.par_chunks_mut(chunk_size).for_each(|chunk| chunk.sort_by(|a, b| a.partial_cmp(b).unwrap()));
        // Merge sorted chunks (simple k-way merge)
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        data.copy_from_slice(&sorted);
        Ok(())
    }
}
