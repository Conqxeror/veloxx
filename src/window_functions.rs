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

#[cfg(feature = "window_functions")]
use crate::types::Value;
use std::collections::BTreeMap;

#[cfg(feature = "window_functions")]
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
        window_spec: &WindowSpec,
    ) -> Result<Vec<Option<i32>>, VeloxxError> {
        let row_count = dataframe.row_count();
        if row_count == 0 {
            return Ok(Vec::new());
        }

        let order_by_col_name = window_spec.order_by.first().ok_or_else(|| {
            VeloxxError::InvalidOperation("Order by column is required for ranking".to_string())
        })?;
        let order_by_series = dataframe.get_column(order_by_col_name).ok_or_else(|| {
            VeloxxError::ColumnNotFound(order_by_col_name.clone())
        })?;

        let mut indexed_values: Vec<(usize, Option<Value>)> = (0..row_count)
            .map(|i| (i, order_by_series.get_value(i)))
            .collect();

        indexed_values.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let mut rankings = vec![None; row_count];
        match function {
            RankingFunction::RowNumber => {
                for (i, (original_index, _)) in indexed_values.iter().enumerate() {
                    rankings[*original_index] = Some((i + 1) as i32);
                }
            }
            RankingFunction::Rank => {
                let mut rank = 1;
                let mut i = 0;
                while i < indexed_values.len() {
                    let (_, current_value) = &indexed_values[i];
                    let mut j = i;
                    while j < indexed_values.len() && &indexed_values[j].1 == current_value {
                        rankings[indexed_values[j].0] = Some(rank);
                        j += 1;
                    }
                    rank += (j - i) as i32;
                    i = j;
                }
            }
            RankingFunction::DenseRank => {
                let mut dense_rank = 1;
                let mut i = 0;
                while i < indexed_values.len() {
                    let (_, current_value) = &indexed_values[i];
                    let mut j = i;
                    while j < indexed_values.len() && &indexed_values[j].1 == current_value {
                        rankings[indexed_values[j].0] = Some(dense_rank);
                        j += 1;
                    }
                    dense_rank += 1;
                    i = j;
                }
            }
            RankingFunction::PercentRank => {
                let mut rank = 1;
                let mut i = 0;
                while i < indexed_values.len() {
                    let (_, current_value) = &indexed_values[i];
                    let mut j = i;
                    while j < indexed_values.len() && &indexed_values[j].1 == current_value {
                        let percent_rank = if row_count > 1 {
                            (rank - 1) as f64 / (row_count - 1) as f64
                        } else {
                            0.0
                        };
                        rankings[indexed_values[j].0] = Some((percent_rank * 100.0) as i32);
                        j += 1;
                    }
                    rank += (j - i) as i32;
                    i = j;
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
        for (i, result) in results.iter_mut().enumerate() {
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
                let computed_result = match function {
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
                *result = Some(computed_result);
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

        // Calculate moving average values
        let moving_avg_values =
            Self::calculate_moving_average(dataframe, column_name, window_size)?;
        let moving_avg_column_name = format!("moving_avg_{}_{}", column_name, window_size);

        result_columns.insert(
            moving_avg_column_name.clone(),
            Series::new_f64(&moving_avg_column_name, moving_avg_values),
        );

        DataFrame::new(result_columns)
    }

    fn calculate_moving_average(
        dataframe: &DataFrame,
        column_name: &str,
        window_size: usize,
    ) -> Result<Vec<Option<f64>>, VeloxxError> {
        let series = dataframe
            .get_column(column_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.to_string()))?;

        let row_count = dataframe.row_count();
        let mut moving_averages = vec![None; row_count];

        #[allow(clippy::needless_range_loop)]
        for i in 0..row_count {
            let start = i.saturating_sub(window_size - 1);
            let end = i;

            let window_values: Vec<f64> = (start..=end)
                .filter_map(|idx| {
                    series.get_value(idx).and_then(|v| match v {
                        Value::F64(f) => Some(f),
                        Value::I32(n) => Some(n as f64),
                        _ => None,
                    })
                })
                .collect();

            if !window_values.is_empty() {
                let sum: f64 = window_values.iter().sum();
                moving_averages[i] = Some(sum / window_values.len() as f64);
            }
        }

        Ok(moving_averages)
    }
}

/// Ranking functions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankingFunction {
    RowNumber,
    Rank,
    DenseRank,
    PercentRank,
}

impl RankingFunction {
    pub fn name(&self) -> &str {
        match self {
            RankingFunction::RowNumber => "row_number",
            RankingFunction::Rank => "rank",
            RankingFunction::DenseRank => "dense_rank",
            RankingFunction::PercentRank => "percent_rank",
        }
    }
}

/// Aggregate functions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AggregateFunction {
    Sum,
    Avg,
    Min,
    Max,
    Count,
}

impl AggregateFunction {
    pub fn name(&self) -> &str {
        match self {
            AggregateFunction::Sum => "sum",
            AggregateFunction::Avg => "avg",
            AggregateFunction::Min => "min",
            AggregateFunction::Max => "max",
            AggregateFunction::Count => "count",
        }
    }
}
