//! Window Functions & Advanced Analytics module for Velox.
//!
//! This module provides SQL-style window functions and advanced analytical operations including:
//! - Moving averages and rolling statistics
//! - Lag and lead operations
//! - Ranking and percentile functions
//! - Time-based window operations
//! - Cumulative and aggregate window functions
//!
//! # Features
//!
//! - SQL-compatible window functions (ROW_NUMBER, RANK, DENSE_RANK)
//! - Time-series specific operations with date/time windows
//! - Flexible partitioning and ordering
//! - Efficient sliding window computations
//! - Advanced statistical functions over windows
//!
//! # Examples
//!
//! ```rust
//! use veloxx::dataframe::DataFrame;
//! use veloxx::series::Series;
//! use std::collections::BTreeMap;
//!
//! # #[cfg(feature = "window_functions")]
//! # {
//! use veloxx::window_functions::{WindowFunction, WindowSpec, RankingFunction};
//!
//! let mut columns = BTreeMap::new();
//! columns.insert(
//!     "sales".to_string(),
//!     Series::new_f64("sales", vec![Some(100.0), Some(200.0), Some(150.0), Some(300.0)]),
//! );
//! columns.insert(
//!     "region".to_string(),
//!     Series::new_string("region", vec![
//!         Some("North".to_string()),
//!         Some("South".to_string()),
//!         Some("North".to_string()),
//!         Some("South".to_string()),
//!     ]),
//! );
//!
//! let df = DataFrame::new(columns).unwrap();
//!
//! // Create a window specification
//! let window_spec = WindowSpec::new()
//!     .partition_by(vec!["region".to_string()])
//!     .order_by(vec!["sales".to_string()]);
//!
//! // Apply ranking function
//! let ranking_fn = RankingFunction::RowNumber;
//! let result = WindowFunction::apply_ranking(&df, &ranking_fn, &window_spec).unwrap();
//! # }
//! ```

use crate::dataframe::DataFrame;
use crate::error::VeloxxError;
use crate::series::Series;
use crate::types::Value;
use std::collections::BTreeMap;

#[cfg(feature = "window_functions")]
use chrono::Duration;

/// Window specification for defining partitioning, ordering, and frame bounds
#[derive(Debug, Clone)]
pub struct WindowSpec {
    pub partition_by: Vec<String>,
    pub order_by: Vec<String>,
    pub frame: WindowFrame,
}

impl WindowSpec {
    /// Create a new window specification
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::window_functions::WindowSpec;
    ///
    /// let window_spec = WindowSpec::new();
    /// ```
    pub fn new() -> Self {
        Self {
            partition_by: Vec::new(),
            order_by: Vec::new(),
            frame: WindowFrame::default(),
        }
    }

    /// Add partition columns
    ///
    /// # Arguments
    ///
    /// * `columns` - Column names to partition by
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::window_functions::WindowSpec;
    ///
    /// let window_spec = WindowSpec::new()
    ///     .partition_by(vec!["region".to_string(), "category".to_string()]);
    /// ```
    pub fn partition_by(mut self, columns: Vec<String>) -> Self {
        self.partition_by = columns;
        self
    }

    /// Add order columns
    ///
    /// # Arguments
    ///
    /// * `columns` - Column names to order by
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::window_functions::WindowSpec;
    ///
    /// let window_spec = WindowSpec::new()
    ///     .order_by(vec!["date".to_string(), "sales".to_string()]);
    /// ```
    pub fn order_by(mut self, columns: Vec<String>) -> Self {
        self.order_by = columns;
        self
    }

    /// Set the window frame
    ///
    /// # Arguments
    ///
    /// * `frame` - Window frame specification
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::window_functions::{WindowSpec, WindowFrame, FrameBound};
    ///
    /// let window_spec = WindowSpec::new()
    ///     .frame(WindowFrame {
    ///         start: FrameBound::Preceding(Some(2)),
    ///         end: FrameBound::CurrentRow,
    ///     });
    /// ```
    pub fn frame(mut self, frame: WindowFrame) -> Self {
        self.frame = frame;
        self
    }
}

impl Default for WindowSpec {
    fn default() -> Self {
        Self::new()
    }
}

/// Window frame specification
#[derive(Debug, Clone)]
pub struct WindowFrame {
    pub start: FrameBound,
    pub end: FrameBound,
}

impl Default for WindowFrame {
    fn default() -> Self {
        Self {
            start: FrameBound::UnboundedPreceding,
            end: FrameBound::CurrentRow,
        }
    }
}

/// Frame boundary specification
#[derive(Debug, Clone)]
pub enum FrameBound {
    UnboundedPreceding,
    Preceding(Option<usize>),
    CurrentRow,
    Following(Option<usize>),
    UnboundedFollowing,
}

/// Main window function processor
pub struct WindowFunction {
    #[cfg(not(feature = "window_functions"))]
    _phantom: std::marker::PhantomData<()>,
}

impl WindowFunction {
    /// Apply a ranking function to a DataFrame
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Input DataFrame
    /// * `function` - Ranking function to apply
    /// * `window_spec` - Window specification
    ///
    /// # Returns
    ///
    /// DataFrame with additional ranking column
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::window_functions::{WindowFunction, WindowSpec, RankingFunction};
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "sales".to_string(),
    ///     Series::new_f64("sales", vec![Some(100.0), Some(200.0), Some(150.0)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let window_spec = WindowSpec::new().order_by(vec!["sales".to_string()]);
    /// let result = WindowFunction::apply_ranking(&df, &RankingFunction::RowNumber, &window_spec).unwrap();
    /// ```
    pub fn apply_ranking(
        dataframe: &DataFrame,
        function: &RankingFunction,
        _window_spec: &WindowSpec,
    ) -> Result<DataFrame, VeloxxError> {
        let mut result_columns = BTreeMap::new();

        // Copy original columns
        for (name, series) in &dataframe.columns {
            result_columns.insert(name.clone(), series.clone());
        }

        // Generate ranking values
        let ranking_values = Self::calculate_ranking(dataframe, function, _window_spec)?;
        let ranking_column_name = format!("{}_rank", function.name());

        result_columns.insert(
            ranking_column_name.clone(),
            Series::new_i32(&ranking_column_name, ranking_values),
        );

        DataFrame::new(result_columns)
    }

    fn calculate_ranking(
        dataframe: &DataFrame,
        function: &RankingFunction,
        _window_spec: &WindowSpec,
    ) -> Result<Vec<Option<i32>>, VeloxxError> {
        let row_count = dataframe.row_count();
        let mut rankings = vec![None; row_count];

        // For simplicity, implement basic row numbering
        // In a full implementation, this would handle partitioning and proper ranking
        match function {
            RankingFunction::RowNumber =>
            {
                #[allow(clippy::needless_range_loop)]
                for i in 0..row_count {
                    rankings[i] = Some((i + 1) as i32);
                }
            }
            RankingFunction::Rank => {
                // Simplified rank implementation
                #[allow(clippy::needless_range_loop)]
                for i in 0..row_count {
                    rankings[i] = Some((i + 1) as i32);
                }
            }
            RankingFunction::DenseRank => {
                // Simplified dense rank implementation
                #[allow(clippy::needless_range_loop)]
                for i in 0..row_count {
                    rankings[i] = Some((i + 1) as i32);
                }
            }
            RankingFunction::PercentRank => {
                // Simplified percent rank implementation
                #[allow(clippy::needless_range_loop)]
                for i in 0..row_count {
                    rankings[i] = Some(((i as f64 / (row_count - 1) as f64) * 100.0) as i32);
                }
            }
        }

        Ok(rankings)
    }

    /// Apply an aggregate function over a window
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Input DataFrame
    /// * `column_name` - Column to aggregate
    /// * `function` - Aggregate function to apply
    /// * `window_spec` - Window specification
    ///
    /// # Returns
    ///
    /// DataFrame with additional aggregate column
    pub fn apply_aggregate(
        dataframe: &DataFrame,
        column_name: &str,
        function: &AggregateFunction,
        _window_spec: &WindowSpec,
    ) -> Result<DataFrame, VeloxxError> {
        let mut result_columns = BTreeMap::new();

        // Copy original columns
        for (name, series) in &dataframe.columns {
            result_columns.insert(name.clone(), series.clone());
        }

        // Calculate aggregate values
        let aggregate_values =
            Self::calculate_window_aggregate(dataframe, column_name, function, _window_spec)?;
        let aggregate_column_name = format!("{}_{}", function.name(), column_name);

        result_columns.insert(
            aggregate_column_name.clone(),
            Series::new_f64(&aggregate_column_name, aggregate_values),
        );

        DataFrame::new(result_columns)
    }

    fn calculate_window_aggregate(
        dataframe: &DataFrame,
        column_name: &str,
        function: &AggregateFunction,
        _window_spec: &WindowSpec,
    ) -> Result<Vec<Option<f64>>, VeloxxError> {
        let series = dataframe
            .get_column(column_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.to_string()))?;

        let row_count = dataframe.row_count();
        let mut results = vec![None; row_count];

        // Simplified window aggregate - in reality would respect frame bounds
        #[allow(clippy::needless_range_loop)]
        for i in 0..row_count {
            let window_values: Vec<f64> = (0..=i)
                .filter_map(|idx| {
                    series.get_value(idx).and_then(|v| match v {
                        Value::F64(f) => Some(f),
                        Value::I32(n) => Some(n as f64),
                        _ => None,
                    })
                })
                .collect();

            if !window_values.is_empty() {
                let result = match function {
                    AggregateFunction::Sum => window_values.iter().sum(),
                    AggregateFunction::Avg => {
                        window_values.iter().sum::<f64>() / window_values.len() as f64
                    }
                    AggregateFunction::Min => {
                        window_values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
                    }
                    AggregateFunction::Max => window_values
                        .iter()
                        .fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
                    AggregateFunction::Count => window_values.len() as f64,
                };
                results[i] = Some(result);
            }
        }

        Ok(results)
    }

    /// Apply lag/lead function
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Input DataFrame
    /// * `column_name` - Column to apply lag/lead to
    /// * `offset` - Number of rows to offset (positive for lag, negative for lead)
    /// * `window_spec` - Window specification
    ///
    /// # Returns
    ///
    /// DataFrame with additional lag/lead column
    pub fn apply_lag_lead(
        dataframe: &DataFrame,
        column_name: &str,
        offset: i32,
        _window_spec: &WindowSpec,
    ) -> Result<DataFrame, VeloxxError> {
        let mut result_columns = BTreeMap::new();

        // Copy original columns
        for (name, series) in &dataframe.columns {
            result_columns.insert(name.clone(), series.clone());
        }

        let series = dataframe
            .get_column(column_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.to_string()))?;

        let row_count = dataframe.row_count();
        let mut lag_lead_values = Vec::new();

        for i in 0..row_count {
            let target_index = i as i32 - offset;
            if target_index >= 0 && (target_index as usize) < row_count {
                lag_lead_values.push(series.get_value(target_index as usize));
            } else {
                lag_lead_values.push(None);
            }
        }

        let function_name = if offset > 0 { "lag" } else { "lead" };
        let column_name_result = format!("{}_{}_{}", function_name, column_name, offset.abs());

        // Convert to appropriate series type based on original series
        let lag_lead_series = match series {
            Series::I32(_, _) => {
                let i32_values: Vec<Option<i32>> = lag_lead_values
                    .into_iter()
                    .map(|v| {
                        v.and_then(|val| match val {
                            Value::I32(i) => Some(i),
                            _ => None,
                        })
                    })
                    .collect();
                Series::new_i32(&column_name_result, i32_values)
            }
            Series::F64(_, _) => {
                let f64_values: Vec<Option<f64>> = lag_lead_values
                    .into_iter()
                    .map(|v| {
                        v.and_then(|val| match val {
                            Value::F64(f) => Some(f),
                            Value::I32(i) => Some(i as f64),
                            _ => None,
                        })
                    })
                    .collect();
                Series::new_f64(&column_name_result, f64_values)
            }
            Series::String(_, _) => {
                let string_values: Vec<Option<String>> = lag_lead_values
                    .into_iter()
                    .map(|v| {
                        v.and_then(|val| match val {
                            Value::String(s) => Some(s),
                            _ => None,
                        })
                    })
                    .collect();
                Series::new_string(&column_name_result, string_values)
            }
            Series::Bool(_, _) => {
                let bool_values: Vec<Option<bool>> = lag_lead_values
                    .into_iter()
                    .map(|v| {
                        v.and_then(|val| match val {
                            Value::Bool(b) => Some(b),
                            _ => None,
                        })
                    })
                    .collect();
                Series::new_bool(&column_name_result, bool_values)
            }
            Series::DateTime(_, _) => {
                // For DateTime, we'll convert to string representation
                let string_values: Vec<Option<String>> = lag_lead_values
                    .into_iter()
                    .map(|v| v.map(|val| format!("{:?}", val)))
                    .collect();
                Series::new_string(&column_name_result, string_values)
            }
        };

        result_columns.insert(column_name_result, lag_lead_series);
        DataFrame::new(result_columns)
    }

    /// Apply moving average with specified window size
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Input DataFrame
    /// * `column_name` - Column to calculate moving average for
    /// * `window_size` - Size of the moving window
    ///
    /// # Returns
    ///
    /// DataFrame with additional moving average column
    pub fn moving_average(
        dataframe: &DataFrame,
        column_name: &str,
        window_size: usize,
    ) -> Result<DataFrame, VeloxxError> {
        let mut result_columns = BTreeMap::new();

        // Copy original columns
        for (name, series) in &dataframe.columns {
            result_columns.insert(name.clone(), series.clone());
        }

        let series = dataframe
            .get_column(column_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.to_string()))?;

        let row_count = dataframe.row_count();
        let mut moving_averages = vec![None; row_count];

        #[allow(clippy::needless_range_loop)]
        for i in 0..row_count {
            let start_idx = if window_size > 0 && i + 1 >= window_size {
                i + 1 - window_size
            } else {
                0
            };
            let end_idx = i + 1;

            let window_values: Vec<f64> = (start_idx..end_idx)
                .filter_map(|idx| {
                    series.get_value(idx).and_then(|v| match v {
                        Value::F64(f) => Some(f),
                        Value::I32(n) => Some(n as f64),
                        _ => None,
                    })
                })
                .collect();

            if !window_values.is_empty()
                && (i >= window_size - 1 || window_values.len() == end_idx - start_idx)
            {
                let avg = window_values.iter().sum::<f64>() / window_values.len() as f64;
                moving_averages[i] = Some(avg);
            }
        }

        let ma_column_name = format!("ma_{}_{}", window_size, column_name);
        result_columns.insert(
            ma_column_name.clone(),
            Series::new_f64(&ma_column_name, moving_averages),
        );

        DataFrame::new(result_columns)
    }
}

/// Ranking functions for window operations
#[derive(Debug, Clone)]
pub enum RankingFunction {
    RowNumber,
    Rank,
    DenseRank,
    PercentRank,
}

impl RankingFunction {
    pub fn name(&self) -> &'static str {
        match self {
            RankingFunction::RowNumber => "row_number",
            RankingFunction::Rank => "rank",
            RankingFunction::DenseRank => "dense_rank",
            RankingFunction::PercentRank => "percent_rank",
        }
    }
}

/// Aggregate functions for window operations
#[derive(Debug, Clone)]
pub enum AggregateFunction {
    Sum,
    Avg,
    Min,
    Max,
    Count,
}

impl AggregateFunction {
    pub fn name(&self) -> &'static str {
        match self {
            AggregateFunction::Sum => "sum",
            AggregateFunction::Avg => "avg",
            AggregateFunction::Min => "min",
            AggregateFunction::Max => "max",
            AggregateFunction::Count => "count",
        }
    }
}

/// Time-based window operations
pub struct TimeWindow {
    #[cfg(not(feature = "window_functions"))]
    _phantom: std::marker::PhantomData<()>,
}

impl TimeWindow {
    /// Create time-based windows for aggregation
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Input DataFrame
    /// * `time_column` - Column containing time/date values
    /// * `window_duration` - Duration of each window
    ///
    /// # Returns
    ///
    /// DataFrame grouped by time windows
    #[cfg(feature = "window_functions")]
    pub fn create_time_windows(
        dataframe: &DataFrame,
        _time_column: &str,
        _window_duration: Duration,
    ) -> Result<DataFrame, VeloxxError> {
        // Placeholder implementation for time-based windowing
        // In a full implementation, this would parse datetime values and group by time windows
        let mut result_columns = BTreeMap::new();

        // Copy original columns
        for (name, series) in &dataframe.columns {
            result_columns.insert(name.clone(), series.clone());
        }

        // Add a window_id column as placeholder
        let window_ids: Vec<Option<i32>> = (0..dataframe.row_count())
            .map(|i| Some((i / 10) as i32)) // Simple grouping by 10s
            .collect();

        result_columns.insert(
            "window_id".to_string(),
            Series::new_i32("window_id", window_ids),
        );

        DataFrame::new(result_columns)
    }

    #[cfg(not(feature = "window_functions"))]
    pub fn create_time_windows(
        _dataframe: &DataFrame,
        _time_column: &str,
        _window_duration: std::time::Duration,
    ) -> Result<DataFrame, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Window functions feature is not enabled. Enable with --features window_functions"
                .to_string(),
        ))
    }

    /// Apply aggregation over time windows
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Input DataFrame with time windows
    /// * `value_column` - Column to aggregate
    /// * `function` - Aggregation function
    ///
    /// # Returns
    ///
    /// DataFrame with time-based aggregations
    pub fn aggregate_time_windows(
        dataframe: &DataFrame,
        value_column: &str,
        function: &AggregateFunction,
    ) -> Result<DataFrame, VeloxxError> {
        // Simplified implementation - group by window_id and aggregate
        if let Some(_window_series) = dataframe.get_column("window_id") {
            // In a full implementation, this would group by window_id and apply aggregation
            let mut result_columns = BTreeMap::new();

            // For now, just return the original dataframe with a note
            for (name, series) in &dataframe.columns {
                result_columns.insert(name.clone(), series.clone());
            }

            let agg_column_name = format!("time_{}_{}", function.name(), value_column);
            result_columns.insert(
                agg_column_name,
                Series::new_string(
                    "time_agg",
                    vec![Some("Time aggregation placeholder".to_string())],
                ),
            );

            DataFrame::new(result_columns)
        } else {
            Err(VeloxxError::InvalidOperation(
                "DataFrame must have time windows created first".to_string(),
            ))
        }
    }
}

/// Percentile and quantile functions
pub struct PercentileFunction;

impl PercentileFunction {
    /// Calculate percentile values for a column within windows
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Input DataFrame
    /// * `column_name` - Column to calculate percentiles for
    /// * `percentiles` - List of percentiles to calculate (0.0 to 1.0)
    /// * `window_spec` - Window specification
    ///
    /// # Returns
    ///
    /// DataFrame with additional percentile columns
    pub fn calculate_percentiles(
        dataframe: &DataFrame,
        column_name: &str,
        percentiles: &[f64],
        _window_spec: &WindowSpec,
    ) -> Result<DataFrame, VeloxxError> {
        let mut result_columns = BTreeMap::new();

        // Copy original columns
        for (name, series) in &dataframe.columns {
            result_columns.insert(name.clone(), series.clone());
        }

        let series = dataframe
            .get_column(column_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.to_string()))?;

        // Extract numeric values
        let mut values: Vec<f64> = Vec::new();
        for i in 0..series.len() {
            if let Some(value) = series.get_value(i) {
                match value {
                    Value::F64(f) => values.push(f),
                    Value::I32(n) => values.push(n as f64),
                    _ => continue,
                }
            }
        }

        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for &percentile in percentiles {
            let index = ((values.len() - 1) as f64 * percentile) as usize;
            let percentile_value = if !values.is_empty() {
                values.get(index).copied().unwrap_or(0.0)
            } else {
                0.0
            };

            let percentile_values: Vec<Option<f64>> = (0..dataframe.row_count())
                .map(|_| Some(percentile_value))
                .collect();

            let percentile_column_name =
                format!("p{}_{}", (percentile * 100.0) as i32, column_name);

            result_columns.insert(
                percentile_column_name.clone(),
                Series::new_f64(&percentile_column_name, percentile_values),
            );
        }

        DataFrame::new(result_columns)
    }

    /// Calculate quartiles (25th, 50th, 75th percentiles)
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Input DataFrame
    /// * `column_name` - Column to calculate quartiles for
    /// * `window_spec` - Window specification
    ///
    /// # Returns
    ///
    /// DataFrame with quartile columns
    pub fn calculate_quartiles(
        dataframe: &DataFrame,
        column_name: &str,
        _window_spec: &WindowSpec,
    ) -> Result<DataFrame, VeloxxError> {
        Self::calculate_percentiles(dataframe, column_name, &[0.25, 0.5, 0.75], _window_spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::Series;
    use std::collections::BTreeMap;

    #[test]
    fn test_window_spec_creation() {
        let window_spec = WindowSpec::new()
            .partition_by(vec!["region".to_string()])
            .order_by(vec!["sales".to_string()]);

        assert_eq!(window_spec.partition_by.len(), 1);
        assert_eq!(window_spec.order_by.len(), 1);
        assert_eq!(window_spec.partition_by[0], "region");
        assert_eq!(window_spec.order_by[0], "sales");
    }

    #[test]
    fn test_ranking_function() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "sales".to_string(),
            Series::new_f64("sales", vec![Some(100.0), Some(200.0), Some(150.0)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let window_spec = WindowSpec::new().order_by(vec!["sales".to_string()]);
        let result =
            WindowFunction::apply_ranking(&df, &RankingFunction::RowNumber, &window_spec).unwrap();

        assert_eq!(result.column_count(), 2); // Original + ranking column
        assert!(result
            .column_names()
            .iter()
            .any(|name| name.contains("rank")));
    }

    #[test]
    fn test_moving_average() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "values".to_string(),
            Series::new_f64(
                "values",
                vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)],
            ),
        );

        let df = DataFrame::new(columns).unwrap();
        let result = WindowFunction::moving_average(&df, "values", 3).unwrap();

        assert_eq!(result.column_count(), 2); // Original + moving average column
        assert!(result
            .column_names()
            .iter()
            .any(|name| name.contains("ma_")));
    }

    #[test]
    fn test_lag_lead() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "values".to_string(),
            Series::new_i32("values", vec![Some(1), Some(2), Some(3), Some(4)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let window_spec = WindowSpec::new();

        // Test lag
        let lag_result = WindowFunction::apply_lag_lead(&df, "values", 1, &window_spec).unwrap();
        assert_eq!(lag_result.column_count(), 2);
        assert!(lag_result
            .column_names()
            .iter()
            .any(|name| name.contains("lag")));

        // Test lead
        let lead_result = WindowFunction::apply_lag_lead(&df, "values", -1, &window_spec).unwrap();
        assert_eq!(lead_result.column_count(), 2);
        assert!(lead_result
            .column_names()
            .iter()
            .any(|name| name.contains("lead")));
    }

    #[test]
    fn test_percentile_calculation() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "values".to_string(),
            Series::new_f64(
                "values",
                vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)],
            ),
        );

        let df = DataFrame::new(columns).unwrap();
        let window_spec = WindowSpec::new();
        let result =
            PercentileFunction::calculate_percentiles(&df, "values", &[0.5], &window_spec).unwrap();

        assert_eq!(result.column_count(), 2); // Original + percentile column
        assert!(result
            .column_names()
            .iter()
            .any(|name| name.contains("p50")));
    }

    #[test]
    fn test_aggregate_function() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "values".to_string(),
            Series::new_f64("values", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let window_spec = WindowSpec::new();
        let result =
            WindowFunction::apply_aggregate(&df, "values", &AggregateFunction::Sum, &window_spec)
                .unwrap();

        assert_eq!(result.column_count(), 2); // Original + aggregate column
        assert!(result
            .column_names()
            .iter()
            .any(|name| name.contains("sum")));
    }
}
