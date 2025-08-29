//! Parallel group_by implementation for better performance on large datasets

use crate::VeloxxError;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;

/// Parallel group_by result
pub struct ParallelGroupByResult<K, V> {
    /// The grouped data
    pub groups: HashMap<K, Vec<V>>,
}

/// Trait for types that can be grouped in parallel
pub trait ParallelGroupBy<K, V> {
    /// Group data by key in parallel
    fn parallel_group_by<F>(&self, key_fn: F) -> Result<ParallelGroupByResult<K, V>, VeloxxError>
    where
        F: Fn(&V) -> K + Sync + Send,
        K: Eq + Hash + Sync + Send,
        V: Sync + Send;
}

impl<K, V> ParallelGroupBy<K, V> for Vec<V>
where
    K: Eq + Hash + Sync + Send,
    V: Clone + Sync + Send,
{
    fn parallel_group_by<F>(&self, key_fn: F) -> Result<ParallelGroupByResult<K, V>, VeloxxError>
    where
        F: Fn(&V) -> K + Sync + Send,
    {
        // Create a mutex-protected hash map to collect results
        let groups: Mutex<HashMap<K, Vec<V>>> = Mutex::new(HashMap::new());

        // Process items in parallel
        self.par_iter().try_for_each(|item| {
            let key = key_fn(item);

            // Lock the mutex and add the item to the appropriate group
            let mut groups = groups.lock().map_err(|_| {
                VeloxxError::ExecutionError(
                    "Failed to acquire lock for parallel group_by".to_string(),
                )
            })?;

            groups
                .entry(key)
                .or_insert_with(Vec::new)
                .push(item.clone());
            Ok::<(), VeloxxError>(())
        })?;

        // Extract the final result
        let groups = groups.into_inner().map_err(|_| {
            VeloxxError::ExecutionError(
                "Failed to extract results from parallel group_by".to_string(),
            )
        })?;

        Ok(ParallelGroupByResult { groups })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::Series as ArrowSeries; // Use unified Series for tests to avoid module path issues

    #[test]
    fn test_parallel_group_by_series() {
        // Create test data
        let series1 = ArrowSeries::new_i32("series1", vec![Some(1), Some(2), Some(3)]);

        let series2 = ArrowSeries::new_i32("series2", vec![Some(4), Some(5), Some(6)]);

        let series3 = ArrowSeries::new_i32("series1", vec![Some(7), Some(8), Some(9)]);

        let data = vec![series1, series2, series3];

        // Group by series name
        let result = data.parallel_group_by(|s| s.name().to_string()).unwrap();

        // Verify results
        assert_eq!(result.groups.len(), 2);
        assert_eq!(result.groups.get("series1").unwrap().len(), 2);
        assert_eq!(result.groups.get("series2").unwrap().len(), 1);
    }

    #[test]
    fn test_parallel_group_by_integers() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Group by even/odd
        let result = data.parallel_group_by(|&x| x % 2).unwrap();

        // Verify results
        assert_eq!(result.groups.len(), 2);
        assert_eq!(result.groups.get(&0).unwrap().len(), 5); // Even numbers
        assert_eq!(result.groups.get(&1).unwrap().len(), 5); // Odd numbers
    }
}
