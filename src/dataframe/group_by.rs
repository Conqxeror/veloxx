// SIMD aggregation wrappers for use in fallback aggregation
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
fn simd_sum_i32(values: &[i32]) -> i32 {
    use crate::performance::simd_std::StdSimdOps;
    values.std_simd_sum().unwrap_or(0)
}

#[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))]
fn simd_sum_i32(values: &[i32]) -> i32 {
    values.iter().sum()
}

#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
fn simd_mean_i32(values: &[i32]) -> f64 {
    use crate::performance::simd_std::StdSimdOps;
    values.std_simd_mean().unwrap().unwrap_or(0.0)
}

#[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))]
fn simd_mean_i32(values: &[i32]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().map(|&x| x as f64).sum::<f64>() / values.len() as f64
    }
}

#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
fn simd_min_i32(values: &[i32]) -> i32 {
    *values.iter().min().unwrap_or(&0)
}

#[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))]
fn simd_min_i32(values: &[i32]) -> i32 {
    *values.iter().min().unwrap_or(&0)
}

#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
fn simd_max_i32(values: &[i32]) -> i32 {
    *values.iter().max().unwrap_or(&0)
}

#[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))]
fn simd_max_i32(values: &[i32]) -> i32 {
    *values.iter().max().unwrap_or(&0)
}

#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
fn simd_sum_f64(values: &[f64]) -> f64 {
    use crate::performance::simd_std::optimized::std_simd_sum_optimized;
    std_simd_sum_optimized(values).unwrap_or(0.0)
}

#[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))]
fn simd_sum_f64(values: &[f64]) -> f64 {
    values.iter().sum()
}

#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
fn simd_mean_f64(values: &[f64]) -> f64 {
    use crate::performance::simd_std::StdSimdOps;
    values.std_simd_mean().unwrap().unwrap_or(0.0)
}

#[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))]
fn simd_mean_f64(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn simd_min_f64(values: &[f64]) -> f64 {
    *values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0)
}

fn simd_max_f64(values: &[f64]) -> f64 {
    *values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0)
}
use crate::{
    dataframe::DataFrame, series::Series, types::Value, VeloxxError,
};
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
use crate::performance::simd_eq_str;
#[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))]
use crate::performance::simd_string::simd_eq_str;
// use bincode::{config, decode_from_slice, encode_to_vec};
use std::collections::HashMap;

// Helper struct to reduce argument count for dense groupby
#[allow(clippy::too_many_arguments)]
struct DenseGroupByParams<'a> {
    group_values: &'a [i32],
    group_bitmap: &'a [bool],
    values: &'a [f64],
    value_bitmap: &'a [bool],
    group_col_name: &'a str,
    value_col_name: &'a str,
    min_key: i32,
    range: usize,
}

/// Represents a `DataFrame` that has been grouped by one or more columns.
///
/// This struct is typically created by calling the `group_by` method on a `DataFrame`.
/// It holds a reference to the original `DataFrame`, the columns used for grouping,
/// and an internal map that stores the row indices belonging to each unique group.
///
/// # Examples
///
/// ```rust
/// use veloxx::dataframe::DataFrame;
/// use veloxx::series::Series;
/// use std::collections::HashMap;
///
/// let mut columns = HashMap::new();
/// columns.insert("city".to_string(), Series::new_string("city", vec![Some("New York".to_string()), Some("London".to_string()), Some("New York".to_string())]));
/// columns.insert("sales".to_string(), Series::new_f64("sales", vec![Some(100.0), Some(150.0), Some(200.0)]));
/// let df = DataFrame::new(columns).unwrap();
///
/// let grouped_df = df.group_by(vec!["city".to_string()]).unwrap();
/// // Now `grouped_df` can be used to perform aggregations.
/// ```
pub struct GroupedDataFrame<'a> {
    dataframe: &'a DataFrame,
    group_columns: Vec<String>,
    // Use contiguous Vecs for group storage for cache locality
    group_keys: Vec<Vec<String>>,   // direct keys
    group_indices: Vec<Vec<usize>>, // row indices for each group
}

impl<'a> GroupedDataFrame<'a> {
    /// Aggregate sum for all non-group columns and return a new DataFrame
    pub fn agg_sum(&self) -> Result<DataFrame, VeloxxError> {
        // Collect all non-group columns
        let all_columns: Vec<String> = self.dataframe.column_names().into_iter().cloned().collect();
        let agg_columns: Vec<String> = all_columns
            .into_iter()
            .filter(|col| !self.group_columns.contains(col))
            .collect();

        // Build aggregation spec: sum for each non-group column
        let aggregations: Vec<(String, &str)> =
            agg_columns.iter().map(|col| (col.clone(), "sum")).collect();

        // Use the existing aggregation logic
        self.agg(aggregations.iter().map(|(c, f)| (c.as_str(), *f)).collect())
    }
    /// Creates a new `GroupedDataFrame` by grouping the provided `DataFrame` by the specified columns.
    ///
    /// This method iterates through the `DataFrame` and collects row indices for each unique
    /// combination of values in the `group_columns`. The keys for the groups are serialized
    /// `FlatValue` vectors to allow for hashing and comparison.
    ///
    /// # Arguments
    ///
    /// * `dataframe` - A reference to the `DataFrame` to be grouped.
    /// * `group_columns` - A `Vec<String>` containing the names of the columns to group by.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(GroupedDataFrame)` if the grouping is successful,
    /// or `Err(VeloxxError::ColumnNotFound)` if any of the `group_columns` do not exist,
    /// or `Err(VeloxxError::InvalidOperation)` if there's an issue with key serialization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::dataframe::group_by::GroupedDataFrame;
    /// use std::collections::HashMap;
    ///
    /// let mut columns = HashMap::new();
    /// columns.insert("category".to_string(), Series::new_string("category", vec![Some("A".to_string()), Some("B".to_string()), Some("A".to_string())]));
    /// columns.insert("value".to_string(), Series::new_i32("value", vec![Some(10), Some(20), Some(30)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let grouped_df = GroupedDataFrame::new(&df, vec!["category".to_string()]).unwrap();
    /// // The `grouped_df` now holds the grouped structure.
    /// ```
    pub fn new(dataframe: &'a DataFrame, group_columns: Vec<String>) -> Result<Self, VeloxxError> {
        use rayon::prelude::*;
        let row_count = dataframe.row_count();
        // Use direct key representation for string/categorical columns
        let key_row_pairs: Vec<(Vec<String>, usize)> = (0..row_count)
            .into_par_iter()
            .map(|i| {
                let mut key: Vec<String> = Vec::with_capacity(group_columns.len());
                for col_name in &group_columns {
                    let series = dataframe.get_column(col_name).expect("Column not found");
                    match series {
                        crate::series::Series::String(_, values, validity) => {
                            if i < values.len() && validity[i] {
                                key.push(values[i].clone());
                            } else {
                                key.push("<NULL>".to_string());
                            }
                        }
                        _ => {
                            key.push(format!("{:?}", series.get_value(i).unwrap_or(Value::Null)));
                        }
                    }
                }
                (key, i)
            })
            .collect();

        // Merge into groups HashMap serially
        let mut groups: HashMap<Vec<String>, Vec<usize>> = HashMap::with_capacity(row_count);
        for (key, i) in key_row_pairs {
            // Use SIMD string comparison for string/categorical keys
            let mut matching_key = None;
            for existing_key in groups.keys() {
                if key.len() == existing_key.len()
                    && key
                        .iter()
                        .zip(existing_key.iter())
                        .all(|(a, b)| simd_eq_str(a, b))
                {
                    matching_key = Some(existing_key.clone());
                    break;
                }
            }
            if let Some(matched) = matching_key {
                groups.get_mut(&matched).unwrap().push(i);
            } else {
                groups.insert(key, vec![i]);
            }
        }

        let mut group_keys = Vec::with_capacity(groups.len());
        let mut group_indices = Vec::with_capacity(groups.len());
        for (k, v) in groups.into_iter() {
            group_keys.push(k); // Direct key representation
            group_indices.push(v);
        }
        Ok(GroupedDataFrame {
            dataframe,
            group_columns,
            group_keys,
            group_indices,
        })
    }

    /// Performs aggregation operations on the grouped data.
    ///
    /// This method takes a list of aggregation instructions, where each instruction specifies
    /// a column to aggregate and the aggregation function to apply (e.g., "sum", "mean", "count",
    /// "min", "max", "median", "std_dev"). It returns a new `DataFrame` where each row represents
    /// a unique group, and the aggregated values form new columns.
    ///
    /// # Arguments
    ///
    /// * `aggregations` - A `Vec` of tuples, where each tuple contains:
    ///   - `&str`: The name of the column on which to perform the aggregation.
    ///   - `&str`: The aggregation function to apply (e.g., "sum", "mean", "count", "min", "max", "median", "std_dev").
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with the aggregated results,
    /// or `Err(VeloxxError::ColumnNotFound)` if an aggregation column does not exist,
    /// or `Err(VeloxxError::Unsupported)` if an unsupported aggregation function is specified,
    /// or `Err(VeloxxError::InvalidOperation)` if there's an issue with key deserialization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::dataframe::group_by::GroupedDataFrame;
    /// use std::collections::HashMap;
    /// use veloxx::types::Value;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("city".to_string(), Series::new_string("city", vec![Some("New York".to_string()), Some("London".to_string()), Some("New York".to_string())]));
    /// columns.insert("sales".to_string(), Series::new_f64("sales", vec![Some(100.0), Some(150.0), Some(200.0)]));
    /// columns.insert("quantity".to_string(), Series::new_i32("quantity", vec![Some(10), Some(15), Some(20)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let grouped_df = df.group_by(vec!["city".to_string()]).unwrap();
    ///
    /// let aggregated_df = grouped_df.agg(vec![
    ///     ("sales", "sum"),
    ///     ("quantity", "mean"),
    ///     ("sales", "count"),
    /// ]).unwrap();
    ///
    /// println!("Aggregated DataFrame:\n{}", aggregated_df);
    /// // Expected output (order of rows might vary):
    /// // city           sales_sum      quantity_mean  sales_count    
    /// // --------------- --------------- --------------- ---------------
    /// // London         150.00          15.00          1              
    /// // New York       300.00          15.00          2              
    /// ```
    pub fn agg(&self, aggregations: Vec<(&str, &str)>) -> Result<DataFrame, VeloxxError> {
        // Try the super-fast path that avoids GroupedDataFrame creation entirely
        // This should only be reached if we're already in a GroupedDataFrame, which means
        // the expensive setup already happened. In that case, use our existing fast path.
        if let Some(fast_result) = self.try_fast_groupby_sum(&aggregations)? {
            return Ok(fast_result);
        }

        // Fallback to the original complex implementation
        self.agg_fallback(aggregations)
    }

    /// Attempts to use high-performance vectorized groupby for simple sum operations
    fn try_fast_groupby_sum(
        &self,
        aggregations: &[(&str, &str)],
    ) -> Result<Option<DataFrame>, VeloxxError> {
        // This optimization is only for a very specific case:
        // - Single group column
        // - Single aggregation that is sum
        if self.group_columns.len() != 1 || aggregations.len() != 1 {
            return Ok(None);
        }

        let (value_col, agg_func) = aggregations[0];
        if agg_func != "sum" {
            return Ok(None);
        }

        let group_col = &self.group_columns[0];

        // Get the series
        let group_series = match self.dataframe.get_column(group_col) {
            Some(s) => s,
            None => return Ok(None),
        };

        let value_series = match self.dataframe.get_column(value_col) {
            Some(s) => s,
            None => return Ok(None),
        };

        // Use our optimized dense groupby implementation
        match self.fast_groupby_dense(group_series, value_series, group_col, value_col) {
            Ok(result_columns) => {
                let row_count = result_columns
                    .values()
                    .next()
                    .map(|series| series.len())
                    .unwrap_or(0);

                Ok(Some(DataFrame {
                    columns: result_columns,
                    row_count,
                }))
            }
            Err(_) => Ok(None), // Fall back to regular implementation
        }
    }

    /// Optimized dense groupby for small integer ranges with parallel aggregation
    fn fast_groupby_dense(
        &self,
        group_series: &Series,
        value_series: &Series,
        group_col_name: &str,
        value_col_name: &str,
    ) -> Result<std::collections::HashMap<String, Series>, VeloxxError> {
        match (group_series, value_series) {
            (Series::I32(_, group_values, group_bitmap), Series::F64(_, values, value_bitmap)) => {
                // Check for dense small-range optimization opportunity
                if let Some((min_key, max_key)) =
                    self.min_max_i32_with_bitmap(group_values, group_bitmap, value_bitmap)
                {
                    let range = (max_key as i64 - min_key as i64).unsigned_abs() + 1;
                    const MAX_DENSE_RANGE: u64 = 1_000_000; // Heuristic threshold
                    if range > 0 && range < MAX_DENSE_RANGE {
                        return self.dense_parallel_groupby(DenseGroupByParams {
                            group_values,
                            group_bitmap,
                            values,
                            value_bitmap,
                            group_col_name,
                            value_col_name,
                            min_key,
                            range: range.try_into().unwrap(),
                        });
                    }
                }
                // Fallback to hashmap-based groupby if not dense
                self.hashmap_groupby(
                    group_values,
                    group_bitmap,
                    values,
                    value_bitmap,
                    group_col_name,
                    value_col_name,
                )
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Fast groupby dense only supports I32 group keys and F64 value keys".to_string(),
            )),
        }
    }

    /// Helper to find min/max in an i32 slice, respecting bitmaps
    fn min_max_i32_with_bitmap(
        &self,
        group_values: &[i32],
        group_bitmap: &[bool],
        value_bitmap: &[bool],
    ) -> Option<(i32, i32)> {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut found_any = false;

        for i in 0..group_values.len() {
            if group_bitmap[i] && value_bitmap[i] {
                let val = group_values[i];
                min = min.min(val);
                max = max.max(val);
                found_any = true;
            }
        }

        if found_any {
            Some((min, max))
        } else {
            None
        }
    }

    fn dense_parallel_groupby(
        &self,
        params: DenseGroupByParams,
    ) -> Result<std::collections::HashMap<String, Series>, VeloxxError> {
        if params.group_values.len() != params.values.len() {
            return Err(VeloxxError::InvalidOperation(
                "Group and value arrays must have same length".to_string(),
            ));
        }

        // For smaller datasets, use sequential processing to avoid parallel overhead
        let data_len = params.group_values.len();
        if data_len < 50_000 || params.range < 50 {
            // Optimized sequential version for small datasets - use Vec instead of allocating tuples
            let mut sums = vec![0.0f64; params.range];
            let mut counts = vec![0usize; params.range];

            for i in 0..data_len {
                if params.group_bitmap[i] && params.value_bitmap[i] {
                    let group_index = (params.group_values[i] - params.min_key) as usize;
                    if group_index < params.range {
                        sums[group_index] += params.values[i];
                        counts[group_index] += 1;
                    }
                }
            }

            let mut group_keys = Vec::new();
            let mut sum_values = Vec::new();

            for group_index in 0..params.range {
                if counts[group_index] > 0 {
                    group_keys.push(params.min_key + group_index as i32);
                    sum_values.push(sums[group_index]);
                }
            }

            let mut result = std::collections::HashMap::new();
            result.insert(
                params.group_col_name.to_string(),
                Series::I32(
                    params.group_col_name.to_string(),
                    group_keys.clone(),
                    vec![true; group_keys.len()],
                ),
            );
            result.insert(
                params.value_col_name.to_string(),
                Series::F64(
                    params.value_col_name.to_string(),
                    sum_values.clone(),
                    vec![true; sum_values.len()],
                ),
            );

            return Ok(result);
        }

        // Use parallel processing for larger datasets
        use rayon::prelude::*;

        // Determine parallel processing parameters
        const CHUNK_SIZE: usize = 8192;
        let n_threads = rayon::current_num_threads().max(1);
        let chunk_size = data_len.div_ceil(n_threads).max(CHUNK_SIZE);

        // Process data in parallel chunks
        let partial_results: Vec<Vec<(f64, usize)>> = (0..data_len)
            .step_by(chunk_size)
            .par_bridge()
            .map(|chunk_start| {
                let chunk_end = (chunk_start + chunk_size).min(data_len);
                let mut local_accumulators = vec![(0.0f64, 0usize); params.range];

                for i in chunk_start..chunk_end {
                    if params.group_bitmap[i] && params.value_bitmap[i] {
                        let group_index = (params.group_values[i] - params.min_key) as usize;
                        if group_index < params.range {
                            local_accumulators[group_index].0 += params.values[i];
                            local_accumulators[group_index].1 += 1;
                        }
                    }
                }

                local_accumulators
            })
            .collect();

        // Merge parallel results
        let mut final_sums = vec![0.0f64; params.range];
        let mut final_counts = vec![0usize; params.range];

        for local_accumulators in partial_results {
            for i in 0..params.range {
                final_sums[i] += local_accumulators[i].0;
                final_counts[i] += local_accumulators[i].1;
            }
        }

        // Collect final results
        let mut group_keys = Vec::new();
        let mut sum_values = Vec::new();

        for group_index in 0..params.range {
            if final_counts[group_index] > 0 {
                group_keys.push(params.min_key + group_index as i32);
                sum_values.push(final_sums[group_index]);
            }
        }

        let mut result = std::collections::HashMap::new();
        result.insert(
            params.group_col_name.to_string(),
            Series::I32(
                params.group_col_name.to_string(),
                group_keys.clone(),
                vec![true; group_keys.len()],
            ),
        );
        result.insert(
            params.value_col_name.to_string(),
            Series::F64(
                params.value_col_name.to_string(),
                sum_values.clone(),
                vec![true; sum_values.len()],
            ),
        );

        Ok(result)
    }

    fn hashmap_groupby(
        &self,
        group_values: &[i32],
        group_bitmap: &[bool],
        values: &[f64],
        value_bitmap: &[bool],
        group_col_name: &str,
        value_col_name: &str,
    ) -> Result<HashMap<String, Series>, VeloxxError> {
        #[cfg(not(target_arch = "wasm32"))]
        use fxhash::FxHashMap;
        #[cfg(target_arch = "wasm32")]
        use std::collections::HashMap as FxHashMap;

        use rayon::prelude::*;
        // Use FxHashMap for better performance on integer keys
        let chunk_size = 8192;
        let data_len = group_values.len();
        let partial_maps: Vec<FxHashMap<i32, (f64, usize)>> = (0..data_len)
            .step_by(chunk_size)
            .par_bridge()
            .map(|chunk_start| {
                let chunk_end = (chunk_start + chunk_size).min(data_len);
                let mut local_map: FxHashMap<i32, (f64, usize)> = FxHashMap::default();
                for i in chunk_start..chunk_end {
                    if group_bitmap[i] && value_bitmap[i] {
                        let entry = local_map.entry(group_values[i]).or_insert((0.0f64, 0usize));
                        entry.0 += values[i];
                        entry.1 += 1;
                    }
                }
                local_map
            })
            .collect();

        // Merge partial maps
        let mut groups: FxHashMap<i32, (f64, usize)> = FxHashMap::default();
        for partial in partial_maps {
            for (key, (sum, count)) in partial {
                let entry = groups.entry(key).or_insert((0.0f64, 0usize));
                entry.0 += sum;
                entry.1 += count;
            }
        }

        let mut group_keys: Vec<i32> = groups.keys().copied().collect();
        group_keys.sort_unstable();
        let sum_values: Vec<f64> = group_keys.iter().map(|&k| groups[&k].0).collect();

        let mut result = std::collections::HashMap::new();
        result.insert(
            group_col_name.to_string(),
            Series::I32(
                group_col_name.to_string(),
                group_keys.clone(),
                vec![true; group_keys.len()],
            ),
        );
        result.insert(
            value_col_name.to_string(),
            Series::F64(
                value_col_name.to_string(),
                sum_values.clone(),
                vec![true; sum_values.len()],
            ),
        );

        Ok(result)
    }

    /// Original complex groupby implementation as fallback
    fn agg_fallback(&self, aggregations: Vec<(&str, &str)>) -> Result<DataFrame, VeloxxError> {
        use crate::performance::memory_compression::UltraFastMemoryPool;
        use crate::performance::series_ext::SeriesPerformanceExt;
        let _memory_pool = UltraFastMemoryPool::new(4096);
        let mut _memory_usage_before = 0;
        for col_name in self.group_columns.iter() {
            if let Some(series) = self.dataframe.get_column(col_name) {
                _memory_usage_before += series.memory_usage();
            }
        }
        use rayon::prelude::*;
        let mut new_columns: HashMap<String, Series> = HashMap::new();
        // Use direct Vec<String> group keys
        let group_keys: &Vec<Vec<String>> = &self.group_keys;

        // Add group columns to new_columns
        for col_name in self.group_columns.iter() {
            let original_series = self.dataframe.get_column(col_name).unwrap();
            let col_idx = self
                .group_columns
                .iter()
                .position(|x| x == col_name)
                .unwrap();
            let mut data_for_new_series: Vec<Option<Value>> =
                Vec::with_capacity(self.group_keys.len());
            for key in self.group_keys.iter() {
                // All keys are Vec<String>, so wrap as Value::String
                let v = if key[col_idx] == "<NULL>" {
                    None
                } else {
                    Some(Value::String(key[col_idx].clone()))
                };
                data_for_new_series.push(v);
            }
            let new_series = match original_series.data_type() {
                crate::types::DataType::I32 => Series::new_i32(
                    col_name,
                    data_for_new_series
                        .into_iter()
                        .map(|x| {
                            x.and_then(|v| {
                                if let Value::I32(val) = v {
                                    Some(val)
                                } else {
                                    None
                                }
                            })
                        })
                        .collect(),
                ),
                crate::types::DataType::F64 => Series::new_f64(
                    col_name,
                    data_for_new_series
                        .into_iter()
                        .map(|x| {
                            x.and_then(|v| {
                                if let Value::F64(val) = v {
                                    Some(val)
                                } else {
                                    None
                                }
                            })
                        })
                        .collect(),
                ),
                crate::types::DataType::Bool => Series::new_bool(
                    col_name,
                    data_for_new_series
                        .into_iter()
                        .map(|x| {
                            x.and_then(|v| {
                                if let Value::Bool(val) = v {
                                    Some(val)
                                } else {
                                    None
                                }
                            })
                        })
                        .collect(),
                ),
                crate::types::DataType::String => Series::new_string(
                    col_name,
                    data_for_new_series
                        .into_iter()
                        .map(|x| {
                            x.and_then(|v| {
                                if let Value::String(val) = v {
                                    Some(val)
                                } else {
                                    None
                                }
                            })
                        })
                        .collect(),
                ),
                crate::types::DataType::DateTime => Series::new_datetime(
                    col_name,
                    data_for_new_series
                        .into_iter()
                        .map(|x| {
                            x.and_then(|v| {
                                if let Value::DateTime(val) = v {
                                    Some(val)
                                } else {
                                    None
                                }
                            })
                        })
                        .collect(),
                ),
            };
            new_columns.insert(col_name.clone(), new_series);
        }

        for (col_name, agg_func) in aggregations {
            let original_series = self
                .dataframe
                .get_column(col_name)
                .ok_or(VeloxxError::ColumnNotFound(col_name.to_string()))?;

            // Parallel aggregation for each group
            let aggregated_data: Vec<Option<Value>> = group_keys
                .par_iter()
                .map(|key| {
                    // Find the index of this key in self.group_keys using direct comparison
                    let key_idx = self.group_keys.iter().position(|k| k == key)?;
                    let row_indices = &self.group_indices[key_idx];
                    match original_series.data_type() {
                        crate::types::DataType::I32 => {
                            let values: Vec<i32> = row_indices
                                .iter()
                                .filter_map(|&i| original_series.get_i32(i))
                                .collect();
                            match agg_func {
                                "sum" => Some(Value::I32(simd_sum_i32(&values))),
                                "mean" => Some(Value::F64(simd_mean_i32(&values))),
                                "min" => Some(Value::I32(simd_min_i32(&values))),
                                "max" => Some(Value::I32(simd_max_i32(&values))),
                                "count" => Some(Value::I32(values.len() as i32)),
                                _ => None,
                            }
                        }
                        crate::types::DataType::F64 => {
                            let values: Vec<f64> = row_indices
                                .iter()
                                .filter_map(|&i| original_series.get_f64(i))
                                .collect();
                            match agg_func {
                                "sum" => Some(Value::F64(simd_sum_f64(&values))),
                                "mean" => Some(Value::F64(simd_mean_f64(&values))),
                                "min" => Some(Value::F64(simd_min_f64(&values))),
                                "max" => Some(Value::F64(simd_max_f64(&values))),
                                "count" => Some(Value::I32(values.len() as i32)),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                })
                .collect();

            let new_series_name = format!("{col_name}_{agg_func}");
            let new_series = if agg_func == "mean" {
                Series::new_f64(
                    &new_series_name,
                    aggregated_data
                        .into_iter()
                        .map(|x| {
                            x.and_then(|v| {
                                if let Value::F64(val) = v {
                                    Some(val)
                                } else {
                                    None
                                }
                            })
                        })
                        .collect(),
                )
            } else {
                match original_series.data_type() {
                    crate::types::DataType::I32 => Series::new_i32(
                        &new_series_name,
                        aggregated_data
                            .into_iter()
                            .map(|x| {
                                x.and_then(|v| {
                                    if let Value::I32(val) = v {
                                        Some(val)
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect(),
                    ),
                    crate::types::DataType::F64 => Series::new_f64(
                        &new_series_name,
                        aggregated_data
                            .into_iter()
                            .map(|x| {
                                x.and_then(|v| {
                                    if let Value::F64(val) = v {
                                        Some(val)
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect(),
                    ),
                    crate::types::DataType::Bool => Series::new_bool(
                        &new_series_name,
                        aggregated_data
                            .into_iter()
                            .map(|x| {
                                x.and_then(|v| {
                                    if let Value::Bool(val) = v {
                                        Some(val)
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect(),
                    ),
                    crate::types::DataType::String => Series::new_string(
                        &new_series_name,
                        aggregated_data
                            .into_iter()
                            .map(|x| {
                                x.and_then(|v| {
                                    if let Value::String(val) = v {
                                        Some(val)
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect(),
                    ),
                    crate::types::DataType::DateTime => Series::new_datetime(
                        &new_series_name,
                        aggregated_data
                            .into_iter()
                            .map(|x| {
                                x.and_then(|v| {
                                    if let Value::DateTime(val) = v {
                                        Some(val)
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect(),
                    ),
                }
            };
            new_columns.insert(new_series_name, new_series);
        }

        DataFrame::new(new_columns)
    }
}
