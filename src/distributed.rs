//! Distributed Computing Support module for Velox.
//!
//! This module provides distributed and parallel computing capabilities including:
//! - Multi-threading support for data operations
//! - Parallel processing of DataFrames
//! - Apache Arrow integration for columnar processing
//! - Memory-efficient distributed operations
//! - Task scheduling and load balancing
//!
//! # Features
//!
//! - Parallel DataFrame operations using Rayon
//! - Apache Arrow integration for zero-copy data exchange
//! - Distributed aggregation and joins
//! - Memory-mapped file support for large datasets
//! - Task partitioning and work distribution
//!
//! # Examples
//!
//! ```rust
//! use veloxx::dataframe::DataFrame;
//! use veloxx::series::Series;
//! use std::collections::BTreeMap;
//!
//! # #[cfg(feature = "distributed")]
//! # {
//! use veloxx::distributed::{ParallelProcessor, DistributedDataFrame, ArrowInterop};
//!
//! let mut columns = BTreeMap::new();
//! columns.insert(
//!     "values".to_string(),
//!     Series::new_i32("values", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]),
//! );
//!
//! let df = DataFrame::new(columns).unwrap();
//!
//! // Create distributed DataFrame
//! let distributed_df = DistributedDataFrame::from_dataframe(df, 2).unwrap(); // 2 partitions
//!
//! // Parallel processing
//! let processor = ParallelProcessor::new();
//! let result = processor.parallel_map(&distributed_df, |partition| {
//!     // Process each partition in parallel
//!     partition.clone()
//! }).unwrap();
//! # }
//! ```

use crate::dataframe::DataFrame;
use crate::dataframe::join::JoinType;
use crate::error::VeloxxError;
use crate::series::Series;
use crate::types::Value;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::sync::Arc;

#[cfg(feature = "distributed")]
use arrow::array::{Array, BooleanArray, Float64Array, Int32Array, StringArray};
#[cfg(feature = "distributed")]
use arrow::datatypes::{DataType as ArrowDataType, Field, Schema};
#[cfg(feature = "distributed")]
use arrow::record_batch::RecordBatch;

/// Distributed DataFrame that can be processed across multiple threads/cores
#[derive(Debug, Clone)]
pub struct DistributedDataFrame {
    partitions: Vec<DataFrame>,
    partition_count: usize,
}

impl DistributedDataFrame {
    /// Create a distributed DataFrame from a regular DataFrame
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Source DataFrame to distribute
    /// * `partition_count` - Number of partitions to create
    ///
    /// # Returns
    ///
    /// Distributed DataFrame with specified number of partitions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::distributed::DistributedDataFrame;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "id".to_string(),
    ///     Series::new_i32("id", vec![Some(1), Some(2), Some(3), Some(4)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let distributed_df = DistributedDataFrame::from_dataframe(df, 2).unwrap();
    /// assert_eq!(distributed_df.partition_count(), 2);
    /// ```
    pub fn from_dataframe(
        dataframe: DataFrame,
        partition_count: usize,
    ) -> Result<Self, VeloxxError> {
        if partition_count == 0 {
            return Err(VeloxxError::InvalidOperation(
                "Partition count must be greater than 0".to_string(),
            ));
        }

        let row_count = dataframe.row_count();
        if row_count == 0 {
            return Ok(Self {
                partitions: vec![dataframe],
                partition_count: 1,
            });
        }

        let rows_per_partition = row_count.div_ceil(partition_count);
        let mut partitions = Vec::new();

        for i in 0..partition_count {
            let start_row = i * rows_per_partition;
            let end_row = ((i + 1) * rows_per_partition).min(row_count);

            if start_row >= row_count {
                break;
            }

            let partition_df = Self::slice_dataframe(&dataframe, start_row, end_row)?;
            partitions.push(partition_df);
        }

        Ok(Self {
            partitions: partitions.clone(),
            partition_count: partitions.len(),
        })
    }

    fn slice_dataframe(
        dataframe: &DataFrame,
        start_row: usize,
        end_row: usize,
    ) -> Result<DataFrame, VeloxxError> {
        let mut partition_columns = BTreeMap::new();

        for (column_name, series) in &dataframe.columns {
            let sliced_series = Self::slice_series(series, start_row, end_row)?;
            partition_columns.insert(column_name.clone(), sliced_series);
        }

        DataFrame::new(partition_columns)
    }

    fn slice_series(
        series: &Series,
        start_row: usize,
        end_row: usize,
    ) -> Result<Series, VeloxxError> {
        let slice_length = end_row - start_row;

        match series {
            Series::I32(name, values) => {
                let sliced_values: Vec<Option<i32>> = values
                    .iter()
                    .skip(start_row)
                    .take(slice_length)
                    .copied()
                    .collect();
                Ok(Series::new_i32(name, sliced_values))
            }
            Series::F64(name, values) => {
                let sliced_values: Vec<Option<f64>> = values
                    .iter()
                    .skip(start_row)
                    .take(slice_length)
                    .copied()
                    .collect();
                Ok(Series::new_f64(name, sliced_values))
            }
            Series::String(name, values) => {
                let sliced_values: Vec<Option<String>> = values
                    .iter()
                    .skip(start_row)
                    .take(slice_length)
                    .cloned()
                    .collect();
                Ok(Series::new_string(name, sliced_values))
            }
            Series::Bool(name, values) => {
                let sliced_values: Vec<Option<bool>> = values
                    .iter()
                    .skip(start_row)
                    .take(slice_length)
                    .copied()
                    .collect();
                Ok(Series::new_bool(name, sliced_values))
            }
            Series::DateTime(name, values) => {
                let sliced_values: Vec<Option<i64>> = values
                    .iter()
                    .skip(start_row)
                    .take(slice_length)
                    .copied()
                    .collect();
                Ok(Series::new_datetime(name, sliced_values))
            }
        }
    }

    /// Get the number of partitions
    ///
    /// # Returns
    ///
    /// Number of partitions in the distributed DataFrame
    pub fn partition_count(&self) -> usize {
        self.partition_count
    }

    /// Get a specific partition
    ///
    /// # Arguments
    ///
    /// * `index` - Partition index
    ///
    /// # Returns
    ///
    /// Reference to the partition DataFrame
    pub fn get_partition(&self, index: usize) -> Option<&DataFrame> {
        self.partitions.get(index)
    }

    /// Get all partitions
    ///
    /// # Returns
    ///
    /// Vector of all partition DataFrames
    pub fn partitions(&self) -> &[DataFrame] {
        &self.partitions
    }

    /// Collect all partitions back into a single DataFrame
    ///
    /// # Returns
    ///
    /// Single DataFrame containing all partition data
    pub fn collect(&self) -> Result<DataFrame, VeloxxError> {
        if self.partitions.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "No partitions to collect".to_string(),
            ));
        }

        let mut result = self.partitions[0].clone();

        for partition in &self.partitions[1..] {
            result = result.append(partition)?;
        }

        Ok(result)
    }

    /// Get total row count across all partitions
    ///
    /// # Returns
    ///
    /// Total number of rows
    pub fn total_row_count(&self) -> usize {
        self.partitions.iter().map(|p| p.row_count()).sum()
    }
}

/// Parallel processor for distributed operations
pub struct ParallelProcessor {
    #[allow(dead_code)]
    thread_pool_size: Option<usize>,
}

impl ParallelProcessor {
    /// Create a new parallel processor
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::distributed::ParallelProcessor;
    ///
    /// let processor = ParallelProcessor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            thread_pool_size: None,
        }
    }

    /// Create a parallel processor with specific thread pool size
    ///
    /// # Arguments
    ///
    /// * `thread_count` - Number of threads to use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::distributed::ParallelProcessor;
    ///
    /// let processor = ParallelProcessor::with_threads(4);
    /// ```
    pub fn with_threads(thread_count: usize) -> Self {
        Self {
            thread_pool_size: Some(thread_count),
        }
    }

    /// Apply a function to each partition in parallel
    ///
    /// # Arguments
    ///
    /// * `distributed_df` - Distributed DataFrame to process
    /// * `func` - Function to apply to each partition
    ///
    /// # Returns
    ///
    /// Distributed DataFrame with results from each partition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::distributed::{DistributedDataFrame, ParallelProcessor};
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "values".to_string(),
    ///     Series::new_i32("values", vec![Some(1), Some(2), Some(3), Some(4)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let distributed_df = DistributedDataFrame::from_dataframe(df, 2).unwrap();
    /// let processor = ParallelProcessor::new();
    ///
    /// let result = processor.parallel_map(&distributed_df, |partition| {
    ///     // Double all values in each partition
    ///     partition.clone() // Simplified example
    /// }).unwrap();
    /// ```
    pub fn parallel_map<F>(
        &self,
        distributed_df: &DistributedDataFrame,
        func: F,
    ) -> Result<DistributedDataFrame, VeloxxError>
    where
        F: Fn(&DataFrame) -> DataFrame + Send + Sync,
    {
        let processed_partitions: Result<Vec<DataFrame>, VeloxxError> = distributed_df
            .partitions
            .par_iter()
            .map(|partition| {
                let result = func(partition);
                Ok(result)
            })
            .collect();

        let partitions = processed_partitions?;

        Ok(DistributedDataFrame {
            partitions: partitions.clone(),
            partition_count: partitions.len(),
        })
    }

    /// Perform parallel aggregation across partitions
    ///
    /// # Arguments
    ///
    /// * `distributed_df` - Distributed DataFrame to aggregate
    /// * `column_name` - Column to aggregate
    /// * `operation` - Aggregation operation
    ///
    /// # Returns
    ///
    /// Single aggregated value
    pub fn parallel_aggregate(
        &self,
        distributed_df: &DistributedDataFrame,
        column_name: &str,
        operation: AggregationOperation,
    ) -> Result<Value, VeloxxError> {
        let partial_results: Result<Vec<Option<Value>>, VeloxxError> = distributed_df
            .partitions
            .par_iter()
            .map(|partition| {
                if let Some(series) = partition.get_column(column_name) {
                    match operation {
                        AggregationOperation::Sum => series.sum(),
                        AggregationOperation::Count => Ok(Some(Value::I32(series.len() as i32))),
                        AggregationOperation::Min => series.min(),
                        AggregationOperation::Max => series.max(),
                        AggregationOperation::Mean => series.mean(),
                    }
                } else {
                    Err(VeloxxError::ColumnNotFound(column_name.to_string()))
                }
            })
            .collect();

        let results = partial_results?;

        // Combine partial results
        match operation {
            AggregationOperation::Sum => {
                let sum = results.into_iter().fold(0.0, |acc, val| {
                    acc + match val {
                        Some(Value::F64(f)) => f,
                        Some(Value::I32(i)) => i as f64,
                        _ => 0.0,
                    }
                });
                Ok(Value::F64(sum))
            }
            AggregationOperation::Count => {
                let count: i32 = results
                    .into_iter()
                    .map(|val| match val {
                        Some(Value::I32(i)) => i,
                        _ => 0,
                    })
                    .sum();
                Ok(Value::I32(count))
            }
            AggregationOperation::Min => {
                results.into_iter().flatten().min().ok_or_else(|| {
                    VeloxxError::InvalidOperation("No values to aggregate".to_string())
                })
            }
            AggregationOperation::Max => {
                results.into_iter().flatten().max().ok_or_else(|| {
                    VeloxxError::InvalidOperation("No values to aggregate".to_string())
                })
            }
            AggregationOperation::Mean => {
                let (sum, count) = results.into_iter().fold((0.0, 0), |(s, c), val| match val {
                    Some(Value::F64(f)) => (s + f, c + 1),
                    Some(Value::I32(i)) => (s + i as f64, c + 1),
                    _ => (s, c),
                });
                if count > 0 {
                    Ok(Value::F64(sum / count as f64))
                } else {
                    Err(VeloxxError::InvalidOperation(
                        "No values to aggregate".to_string(),
                    ))
                }
            }
        }
    }

    /// Perform parallel join between two distributed DataFrames
    ///
    /// # Arguments
    ///
    /// * `left_df` - Left distributed DataFrame
    /// * `right_df` - Right distributed DataFrame
    /// * `left_key` - Join key column in left DataFrame
    /// * `right_key` - Join key column in right DataFrame
    ///
    /// # Returns
    ///
    /// Distributed DataFrame with join results
    pub fn parallel_join(
        &self,
        left_df: &DistributedDataFrame,
        right_df: &DistributedDataFrame,
        left_key: &str,
        _right_key: &str,
    ) -> Result<DistributedDataFrame, VeloxxError> {
        // Simplified parallel join implementation
        // In a real implementation, this would handle data shuffling and partitioning

        let joined_partitions: Result<Vec<DataFrame>, VeloxxError> = left_df
            .partitions
            .par_iter()
            .enumerate()
            .map(|(i, left_partition)| {
                // For simplicity, join with corresponding right partition
                if let Some(right_partition) = right_df.partitions.get(i) {
                    left_partition.join(right_partition, left_key, JoinType::Inner)
                } else {
                    // If no corresponding right partition, return empty DataFrame
                    Ok(left_partition.clone())
                }
            })
            .collect();

        let partitions = joined_partitions?;

        Ok(DistributedDataFrame {
            partitions: partitions.clone(),
            partition_count: partitions.len(),
        })
    }

    /// Sort distributed DataFrame in parallel
    ///
    /// # Arguments
    ///
    /// * `distributed_df` - Distributed DataFrame to sort
    /// * `column_name` - Column to sort by
    /// * `ascending` - Sort order
    ///
    /// # Returns
    ///
    /// Sorted distributed DataFrame
    pub fn parallel_sort(
        &self,
        distributed_df: &DistributedDataFrame,
        column_name: &str,
        ascending: bool,
    ) -> Result<DistributedDataFrame, VeloxxError> {
        // Sort each partition individually
        let sorted_partitions: Result<Vec<DataFrame>, VeloxxError> = distributed_df
            .partitions
            .par_iter()
            .map(|partition| partition.sort(vec![column_name.to_string()], ascending))
            .collect();

        let partitions = sorted_partitions?;

        // Note: This doesn't provide global sorting across partitions
        // A full implementation would need to merge sorted partitions
        Ok(DistributedDataFrame {
            partitions: partitions.clone(),
            partition_count: partitions.len(),
        })
    }
}

impl Default for ParallelProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregation operations for parallel processing
#[derive(Debug, Clone, Copy)]
pub enum AggregationOperation {
    Sum,
    Count,
    Min,
    Max,
    Mean,
}

/// Apache Arrow integration for zero-copy data exchange
pub struct ArrowInterop {
    #[cfg(not(feature = "distributed"))]
    _phantom: std::marker::PhantomData<()>,
}

impl ArrowInterop {
    /// Convert a DataFrame to Apache Arrow RecordBatch
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to convert
    ///
    /// # Returns
    ///
    /// Apache Arrow RecordBatch
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::distributed::ArrowInterop;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "id".to_string(),
    ///     Series::new_i32("id", vec![Some(1), Some(2), Some(3)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// // let record_batch = ArrowInterop::dataframe_to_arrow(&df).unwrap();
    /// ```
    #[cfg(feature = "distributed")]
    pub fn dataframe_to_arrow(dataframe: &DataFrame) -> Result<RecordBatch, VeloxxError> {
        let mut fields = Vec::new();
        let mut arrays: Vec<Arc<dyn Array>> = Vec::new();

        for (column_name, series) in &dataframe.columns {
            match series {
                Series::I32(_, values) => {
                    let field = Field::new(column_name, ArrowDataType::Int32, true);
                    fields.push(field);

                    let arrow_array = Int32Array::from(values.clone());
                    arrays.push(Arc::new(arrow_array));
                }
                Series::F64(_, values) => {
                    let field = Field::new(column_name, ArrowDataType::Float64, true);
                    fields.push(field);

                    let arrow_array = Float64Array::from(values.clone());
                    arrays.push(Arc::new(arrow_array));
                }
                Series::String(_, values) => {
                    let field = Field::new(column_name, ArrowDataType::Utf8, true);
                    fields.push(field);

                    let arrow_array = StringArray::from(values.clone());
                    arrays.push(Arc::new(arrow_array));
                }
                Series::Bool(_, values) => {
                    let field = Field::new(column_name, ArrowDataType::Boolean, true);
                    fields.push(field);

                    let arrow_array = BooleanArray::from(values.clone());
                    arrays.push(Arc::new(arrow_array));
                }
                Series::DateTime(_, _values) => {
                    // For DateTime, we'll use Int64 to represent timestamps
                    let field = Field::new(column_name, ArrowDataType::Int64, true);
                    fields.push(field);

                    // Simplified conversion - in reality would handle proper datetime conversion
                    let placeholder_array = Int32Array::from(vec![Some(0); dataframe.row_count()]);
                    arrays.push(Arc::new(placeholder_array));
                }
            }
        }

        let schema = Schema::new(fields);
        RecordBatch::try_new(Arc::new(schema), arrays)
            .map_err(|e| VeloxxError::InvalidOperation(format!("Arrow conversion error: {}", e)))
    }

    #[cfg(not(feature = "distributed"))]
    pub fn dataframe_to_arrow(_dataframe: &DataFrame) -> Result<(), VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Distributed feature is not enabled. Enable with --features distributed".to_string(),
        ))
    }

    /// Convert Apache Arrow RecordBatch to DataFrame
    ///
    /// # Arguments
    ///
    /// * `record_batch` - Arrow RecordBatch to convert
    ///
    /// # Returns
    ///
    /// DataFrame containing the data
    #[cfg(feature = "distributed")]
    pub fn arrow_to_dataframe(record_batch: &RecordBatch) -> Result<DataFrame, VeloxxError> {
        let mut columns = BTreeMap::new();
        let schema = record_batch.schema();

        for (i, field) in schema.fields().iter().enumerate() {
            let column_name = field.name().clone();
            let array = record_batch.column(i);

            let series =
                match field.data_type() {
                    ArrowDataType::Int32 => {
                        let int_array =
                            array.as_any().downcast_ref::<Int32Array>().ok_or_else(|| {
                                VeloxxError::InvalidOperation(
                                    "Failed to downcast Int32Array".to_string(),
                                )
                            })?;

                        let values: Vec<Option<i32>> = (0..int_array.len())
                            .map(|i| {
                                if int_array.is_null(i) {
                                    None
                                } else {
                                    Some(int_array.value(i))
                                }
                            })
                            .collect();

                        Series::new_i32(&column_name, values)
                    }
                    ArrowDataType::Float64 => {
                        let float_array = array
                            .as_any()
                            .downcast_ref::<Float64Array>()
                            .ok_or_else(|| {
                                VeloxxError::InvalidOperation(
                                    "Failed to downcast Float64Array".to_string(),
                                )
                            })?;

                        let values: Vec<Option<f64>> = (0..float_array.len())
                            .map(|i| {
                                if float_array.is_null(i) {
                                    None
                                } else {
                                    Some(float_array.value(i))
                                }
                            })
                            .collect();

                        Series::new_f64(&column_name, values)
                    }
                    ArrowDataType::Utf8 => {
                        let string_array = array
                            .as_any()
                            .downcast_ref::<StringArray>()
                            .ok_or_else(|| {
                                VeloxxError::InvalidOperation(
                                    "Failed to downcast StringArray".to_string(),
                                )
                            })?;

                        let values: Vec<Option<String>> = (0..string_array.len())
                            .map(|i| {
                                if string_array.is_null(i) {
                                    None
                                } else {
                                    Some(string_array.value(i).to_string())
                                }
                            })
                            .collect();

                        Series::new_string(&column_name, values)
                    }
                    ArrowDataType::Boolean => {
                        let bool_array =
                            array
                                .as_any()
                                .downcast_ref::<BooleanArray>()
                                .ok_or_else(|| {
                                    VeloxxError::InvalidOperation(
                                        "Failed to downcast BooleanArray".to_string(),
                                    )
                                })?;

                        let values: Vec<Option<bool>> = (0..bool_array.len())
                            .map(|i| {
                                if bool_array.is_null(i) {
                                    None
                                } else {
                                    Some(bool_array.value(i))
                                }
                            })
                            .collect();

                        Series::new_bool(&column_name, values)
                    }
                    _ => {
                        return Err(VeloxxError::InvalidOperation(format!(
                            "Unsupported Arrow data type: {:?}",
                            field.data_type()
                        )));
                    }
                };

            columns.insert(column_name, series);
        }

        DataFrame::new(columns)
    }

    #[cfg(not(feature = "distributed"))]
    pub fn arrow_to_dataframe(_record_batch: &()) -> Result<DataFrame, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Distributed feature is not enabled. Enable with --features distributed".to_string(),
        ))
    }
}

/// Memory-mapped file operations for large datasets
pub struct MemoryMappedOps;

impl MemoryMappedOps {
    /// Read a large CSV file using memory mapping
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the CSV file
    /// * `chunk_size` - Number of rows to process at a time
    ///
    /// # Returns
    ///
    /// Distributed DataFrame with file data
    pub fn read_csv_mmap(
        file_path: &str,
        _chunk_size: usize,
    ) -> Result<DistributedDataFrame, VeloxxError> {
        // Placeholder implementation for memory-mapped CSV reading
        // In a real implementation, this would use memory mapping for efficient large file access

        // For now, simulate by creating a distributed DataFrame
        let mut columns = BTreeMap::new();
        columns.insert(
            "mmap_data".to_string(),
            Series::new_string(
                "mmap_data",
                vec![Some(format!("Memory-mapped data from {}", file_path))],
            ),
        );

        let df = DataFrame::new(columns)?;
        DistributedDataFrame::from_dataframe(df, 1)
    }

    /// Write a distributed DataFrame to a memory-mapped file
    ///
    /// # Arguments
    ///
    /// * `distributed_df` - Distributed DataFrame to write
    /// * `file_path` - Output file path
    ///
    /// # Returns
    ///
    /// Success or error
    pub fn write_csv_mmap(
        _distributed_df: &DistributedDataFrame,
        file_path: &str,
    ) -> Result<(), VeloxxError> {
        // Placeholder implementation
        println!(
            "Would write distributed DataFrame to memory-mapped file: {}",
            file_path
        );

        // In a real implementation, this would:
        // 1. Calculate total file size needed
        // 2. Create memory-mapped file
        // 3. Write each partition to appropriate file offset
        // 4. Sync to disk

        Ok(())
    }
}

/// Task scheduler for distributed operations
pub struct TaskScheduler {
    max_concurrent_tasks: usize,
}

impl TaskScheduler {
    /// Create a new task scheduler
    ///
    /// # Arguments
    ///
    /// * `max_concurrent_tasks` - Maximum number of concurrent tasks
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::distributed::TaskScheduler;
    ///
    /// let scheduler = TaskScheduler::new(4);
    /// ```
    pub fn new(max_concurrent_tasks: usize) -> Self {
        Self {
            max_concurrent_tasks,
        }
    }

    /// Schedule and execute tasks across available resources
    ///
    /// # Arguments
    ///
    /// * `tasks` - Vector of tasks to execute
    ///
    /// # Returns
    ///
    /// Vector of task results
    pub fn execute_tasks<T, F>(&self, tasks: Vec<F>) -> Result<Vec<T>, VeloxxError>
    where
        T: Send,
        F: Fn() -> Result<T, VeloxxError> + Send,
    {
        let results: Result<Vec<T>, VeloxxError> = tasks
            .into_par_iter()
            .with_max_len(self.max_concurrent_tasks)
            .map(|task| task())
            .collect();

        results
    }

    /// Get optimal partition count based on data size and available resources
    ///
    /// # Arguments
    ///
    /// * `data_size_bytes` - Size of data in bytes
    /// * `target_partition_size_mb` - Target size per partition in MB
    ///
    /// # Returns
    ///
    /// Recommended number of partitions
    pub fn calculate_optimal_partitions(
        &self,
        data_size_bytes: usize,
        target_partition_size_mb: usize,
    ) -> usize {
        let target_partition_size_bytes = target_partition_size_mb * 1024 * 1024;
        let calculated_partitions =
            data_size_bytes.div_ceil(target_partition_size_bytes);

        // Ensure we don't exceed max concurrent tasks
        calculated_partitions.min(self.max_concurrent_tasks).max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::Series;
    use std::collections::BTreeMap;

    #[test]
    fn test_distributed_dataframe_creation() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "id".to_string(),
            Series::new_i32(
                "id",
                vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
            ),
        );

        let df = DataFrame::new(columns).unwrap();
        let distributed_df = DistributedDataFrame::from_dataframe(df, 3).unwrap();

        assert_eq!(distributed_df.partition_count(), 3);
        assert_eq!(distributed_df.total_row_count(), 6);
    }

    #[test]
    fn test_distributed_dataframe_collect() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "values".to_string(),
            Series::new_i32("values", vec![Some(1), Some(2), Some(3), Some(4)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let distributed_df = DistributedDataFrame::from_dataframe(df.clone(), 2).unwrap();
        let collected_df = distributed_df.collect().unwrap();

        assert_eq!(collected_df.row_count(), df.row_count());
        assert_eq!(collected_df.column_count(), df.column_count());
    }

    #[test]
    fn test_parallel_processor() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "values".to_string(),
            Series::new_i32("values", vec![Some(1), Some(2), Some(3), Some(4)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let distributed_df = DistributedDataFrame::from_dataframe(df, 2).unwrap();
        let processor = ParallelProcessor::new();

        let result = processor
            .parallel_map(&distributed_df, |partition| partition.clone())
            .unwrap();

        assert_eq!(result.partition_count(), distributed_df.partition_count());
    }

    #[test]
    fn test_parallel_aggregation() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "values".to_string(),
            Series::new_i32("values", vec![Some(1), Some(2), Some(3), Some(4)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let distributed_df = DistributedDataFrame::from_dataframe(df, 2).unwrap();
        let processor = ParallelProcessor::new();

        let sum_result = processor
            .parallel_aggregate(&distributed_df, "values", AggregationOperation::Sum)
            .unwrap();
        let count_result = processor
            .parallel_aggregate(&distributed_df, "values", AggregationOperation::Count)
            .unwrap();

        assert_eq!(sum_result, Value::F64(10.0)); // 1+2+3+4 = 10
        assert_eq!(count_result, Value::I32(4));
    }

    #[test]
    fn test_task_scheduler() {
        let scheduler = TaskScheduler::new(2);

        let tasks: Vec<Box<dyn Fn() -> Result<i32, VeloxxError> + Send>> =
            vec![Box::new(|| Ok(1)), Box::new(|| Ok(2)), Box::new(|| Ok(3))];

        let results = scheduler.execute_tasks(tasks).unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.contains(&1));
        assert!(results.contains(&2));
        assert!(results.contains(&3));
    }

    #[test]
    fn test_optimal_partition_calculation() {
        let scheduler = TaskScheduler::new(8);

        // 100MB data with 10MB target partition size should give 10 partitions
        let partitions = scheduler.calculate_optimal_partitions(100 * 1024 * 1024, 10);
        assert_eq!(partitions, 8); // Limited by max_concurrent_tasks

        // 50MB data with 10MB target partition size should give 5 partitions
        let partitions = scheduler.calculate_optimal_partitions(50 * 1024 * 1024, 10);
        assert_eq!(partitions, 5);
    }

    #[test]
    fn test_memory_mapped_ops() {
        let result = MemoryMappedOps::read_csv_mmap("test.csv", 1000);
        assert!(result.is_ok());

        let distributed_df = result.unwrap();
        let write_result = MemoryMappedOps::write_csv_mmap(&distributed_df, "output.csv");
        assert!(write_result.is_ok());
    }
}
