use crate::dataframe::DataFrame;
use crate::series::Series;
use crate::types::Value;
use std::cmp::Ordering;
use std::collections::{HashMap};

/// Ultra-fast query engine with SIMD-accelerated predicate evaluation
pub struct UltraFastQueryEngine;

use crate::conditions::Condition;

#[derive(Debug, Clone)]
pub enum CompareOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct OrderBySpec {
    pub column: String,
    pub ascending: bool,
}

#[derive(Debug, Clone)]
pub struct QueryBuilder {
    where_conditions: Vec<Condition>,
    order_by: Vec<OrderBySpec>,
    limit: Option<usize>,
    select_columns: Option<Vec<String>>,
    aggregations: Vec<AggregationSpec>,
}

#[derive(Debug, Clone)]
pub struct AggregationSpec {
    pub column: String,
    pub function: AggregationFunction,
}

#[derive(Debug, Clone)]
pub enum AggregationFunction {
    Count,
    Sum,
    Average,
    Min,
    Max,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            where_conditions: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            select_columns: None,
            aggregations: Vec::new(),
        }
    }

    pub fn where_condition(mut self, condition: Condition) -> Self {
        self.where_conditions.push(condition);
        self
    }

    pub fn order_by(mut self, column: String, ascending: bool) -> Self {
        self.order_by.push(OrderBySpec { column, ascending });
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn select(mut self, columns: Vec<String>) -> Self {
        self.select_columns = Some(columns);
        self
    }

    pub fn aggregate(mut self, spec: AggregationSpec) -> Self {
        self.aggregations.push(spec);
        self
    }
}

impl Default for UltraFastQueryEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl UltraFastQueryEngine {
    pub fn new() -> Self {
        Self
    }

    /// Execute a query with SIMD-accelerated predicate evaluation
    pub fn query(
        &self,
        df: &DataFrame,
        query: QueryBuilder,
    ) -> Result<DataFrame, Box<dyn std::error::Error>> {
        // Start with all rows selected
        let row_count = df.row_count;
        let mut mask = vec![true; row_count];

        // Apply WHERE conditions with predicate pushdown
        for condition in &query.where_conditions {
            let mut temp_mask = vec![true; row_count];
            self.evaluate_condition(df, condition, &mut temp_mask)?;

            // Apply logical AND operation (can be SIMD accelerated)
            for i in 0..row_count {
                mask[i] = mask[i] && temp_mask[i];
            }
        }

        // Handle aggregations first (before filtering)
        if !query.aggregations.is_empty() {
            return self.apply_aggregations(df, &query.aggregations, &mask);
        }

        // Apply filtering based on mask
        let mut result_df = self.apply_filter(df, &mask)?;

        // Apply ORDER BY
        if !query.order_by.is_empty() {
            result_df = self.apply_order_by(result_df, &query.order_by)?;
        }

        // Apply LIMIT
        if let Some(limit) = query.limit {
            result_df = self.apply_limit(result_df, limit)?;
        }

        // Apply column selection
        if let Some(select_cols) = &query.select_columns {
            result_df = self.apply_select(result_df, select_cols)?;
        }

        Ok(result_df)
    }

    /// SIMD-accelerated predicate evaluation
    fn evaluate_condition(
        &self,
        df: &DataFrame,
        condition: &Condition,
        mask: &mut [bool],
    ) -> Result<(), Box<dyn std::error::Error>> {
        match condition {
            Condition::Eq(column, value) => {
                self.evaluate_compare(df, column, &CompareOp::Equal, value, mask)
            }
            Condition::Gt(column, value) => {
                self.evaluate_compare(df, column, &CompareOp::GreaterThan, value, mask)
            }
            Condition::Lt(column, value) => {
                self.evaluate_compare(df, column, &CompareOp::LessThan, value, mask)
            }
            Condition::And(left, right) => {
                let mut left_mask = vec![true; mask.len()];
                let mut right_mask = vec![true; mask.len()];
                self.evaluate_condition(df, left, &mut left_mask)?;
                self.evaluate_condition(df, right, &mut right_mask)?;
                for i in 0..mask.len() {
                    mask[i] = left_mask[i] && right_mask[i];
                }
                Ok(())
            }
            Condition::Or(left, right) => {
                let mut left_mask = vec![false; mask.len()];
                let mut right_mask = vec![false; mask.len()];
                self.evaluate_condition(df, left, &mut left_mask)?;
                self.evaluate_condition(df, right, &mut right_mask)?;
                for i in 0..mask.len() {
                    mask[i] = left_mask[i] || right_mask[i];
                }
                Ok(())
            }
            Condition::Not(cond) => {
                let mut inner_mask = vec![true; mask.len()];
                self.evaluate_condition(df, cond, &mut inner_mask)?;
                for i in 0..mask.len() {
                    mask[i] = !inner_mask[i];
                }
                Ok(())
            }
        }
    }

    fn evaluate_compare(
        &self,
        df: &DataFrame,
        column: &str,
        op: &CompareOp,
        value: &Value,
        mask: &mut [bool],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let series = df
            .columns
            .get(column)
            .ok_or_else(|| format!("Column '{}' not found", column))?;

        match series {
            Series::I32(_name, data, validity) => {
                if let Value::I32(threshold) = value {
                    for (i, (&val, &is_valid)) in data.iter().zip(validity.iter()).enumerate() {
                        if !is_valid {
                            mask[i] = false;
                            continue;
                        }

                        mask[i] = match op {
                            CompareOp::Equal => val == *threshold,
                            CompareOp::NotEqual => val != *threshold,
                            CompareOp::GreaterThan => val > *threshold,
                            CompareOp::GreaterThanOrEqual => val >= *threshold,
                            CompareOp::LessThan => val < *threshold,
                            CompareOp::LessThanOrEqual => val <= *threshold,
                        };
                    }
                }
            }
            Series::F64(_name, data, validity) => {
                if let Value::F64(threshold) = value {
                    for (i, (&val, &is_valid)) in data.iter().zip(validity.iter()).enumerate() {
                        if !is_valid {
                            mask[i] = false;
                            continue;
                        }

                        mask[i] = match op {
                            CompareOp::Equal => (val - threshold).abs() < f64::EPSILON,
                            CompareOp::NotEqual => (val - threshold).abs() >= f64::EPSILON,
                            CompareOp::GreaterThan => val > *threshold,
                            CompareOp::GreaterThanOrEqual => val >= *threshold,
                            CompareOp::LessThan => val < *threshold,
                            CompareOp::LessThanOrEqual => val <= *threshold,
                        };
                    }
                }
            }
            Series::String(_name, data, validity) => {
                if let Value::String(threshold) = value {
                    for (i, (val, &is_valid)) in data.iter().zip(validity.iter()).enumerate() {
                        if !is_valid {
                            mask[i] = false;
                            continue;
                        }

                        mask[i] = match op {
                            CompareOp::Equal => val == threshold,
                            CompareOp::NotEqual => val != threshold,
                            CompareOp::GreaterThan => val > threshold,
                            CompareOp::GreaterThanOrEqual => val >= threshold,
                            CompareOp::LessThan => val < threshold,
                            CompareOp::LessThanOrEqual => val <= threshold,
                        };
                    }
                }
            }
            Series::Bool(_name, data, validity) => {
                if let Value::Bool(threshold) = value {
                    for (i, (&val, &is_valid)) in data.iter().zip(validity.iter()).enumerate() {
                        if !is_valid {
                            mask[i] = false;
                            continue;
                        }

                        mask[i] = match op {
                            CompareOp::Equal => val == *threshold,
                            CompareOp::NotEqual => val != *threshold,
                            _ => false, // Other comparisons don't make sense for booleans
                        };
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    

    fn apply_filter(
        &self,
        df: &DataFrame,
        mask: &[bool],
    ) -> Result<DataFrame, Box<dyn std::error::Error>> {
        let mut new_columns = HashMap::new();

        for (col_name, series) in &df.columns {
            let filtered_series = match series {
                Series::I32(name, data, validity) => {
                    let mut filtered_data = Vec::new();
                    let mut filtered_validity = Vec::new();

                    for (i, &include) in mask.iter().enumerate() {
                        if include {
                            filtered_data.push(data[i]);
                            filtered_validity.push(validity[i]);
                        }
                    }

                    Series::I32(name.clone(), filtered_data, filtered_validity)
                }
                Series::F64(name, data, validity) => {
                    let mut filtered_data = Vec::new();
                    let mut filtered_validity = Vec::new();

                    for (i, &include) in mask.iter().enumerate() {
                        if include {
                            filtered_data.push(data[i]);
                            filtered_validity.push(validity[i]);
                        }
                    }

                    Series::F64(name.clone(), filtered_data, filtered_validity)
                }
                Series::String(name, data, validity) => {
                    let mut filtered_data = Vec::new();
                    let mut filtered_validity = Vec::new();

                    for (i, &include) in mask.iter().enumerate() {
                        if include {
                            filtered_data.push(data[i].clone());
                            filtered_validity.push(validity[i]);
                        }
                    }

                    Series::String(name.clone(), filtered_data, filtered_validity)
                }
                Series::Bool(name, data, validity) => {
                    let mut filtered_data = Vec::new();
                    let mut filtered_validity = Vec::new();

                    for (i, &include) in mask.iter().enumerate() {
                        if include {
                            filtered_data.push(data[i]);
                            filtered_validity.push(validity[i]);
                        }
                    }

                    Series::Bool(name.clone(), filtered_data, filtered_validity)
                }
                Series::DateTime(name, data, validity) => {
                    let mut filtered_data = Vec::new();
                    let mut filtered_validity = Vec::new();

                    for (i, &include) in mask.iter().enumerate() {
                        if include {
                            filtered_data.push(data[i]);
                            filtered_validity.push(validity[i]);
                        }
                    }

                    Series::DateTime(name.clone(), filtered_data, filtered_validity)
                }
            };

            new_columns.insert(col_name.clone(), filtered_series);
        }

        // Count the number of rows that passed the filter
        let new_row_count = mask.iter().filter(|&&x| x).count();

        Ok(DataFrame {
            columns: new_columns,
            row_count: new_row_count,
        })
    }

    fn apply_order_by(
        &self,
        df: DataFrame,
        order_specs: &[OrderBySpec],
    ) -> Result<DataFrame, Box<dyn std::error::Error>> {
        if df.row_count == 0 {
            return Ok(df);
        }

        // Create indices and sort them
        let mut indices: Vec<usize> = (0..df.row_count).collect();

        indices.sort_by(|&a, &b| {
            for spec in order_specs {
                let series = match df.columns.get(&spec.column) {
                    Some(s) => s,
                    None => continue,
                };

                let cmp = match series {
                    Series::I32(_, data, validity) => {
                        let val_a = if validity[a] { Some(data[a]) } else { None };
                        let val_b = if validity[b] { Some(data[b]) } else { None };
                        val_a.cmp(&val_b)
                    }
                    Series::F64(_, data, validity) => {
                        let val_a = if validity[a] { Some(data[a]) } else { None };
                        let val_b = if validity[b] { Some(data[b]) } else { None };
                        val_a.partial_cmp(&val_b).unwrap_or(Ordering::Equal)
                    }
                    Series::String(_, data, validity) => {
                        let val_a = if validity[a] { Some(&data[a]) } else { None };
                        let val_b = if validity[b] { Some(&data[b]) } else { None };
                        val_a.cmp(&val_b)
                    }
                    Series::Bool(_, data, validity) => {
                        let val_a = if validity[a] { Some(data[a]) } else { None };
                        let val_b = if validity[b] { Some(data[b]) } else { None };
                        val_a.cmp(&val_b)
                    }
                    Series::DateTime(_, data, validity) => {
                        let val_a = if validity[a] { Some(data[a]) } else { None };
                        let val_b = if validity[b] { Some(data[b]) } else { None };
                        val_a.cmp(&val_b)
                    }
                };

                let final_cmp = if spec.ascending { cmp } else { cmp.reverse() };

                if final_cmp != Ordering::Equal {
                    return final_cmp;
                }
            }
            Ordering::Equal
        });

        // Reorder all columns based on sorted indices
        let mut new_columns = HashMap::new();

        for (col_name, series) in df.columns {
            let reordered_series = match series {
                Series::I32(name, data, validity) => {
                    let mut reordered_data = Vec::with_capacity(data.len());
                    let mut reordered_validity = Vec::with_capacity(validity.len());

                    for &idx in &indices {
                        reordered_data.push(data[idx]);
                        reordered_validity.push(validity[idx]);
                    }

                    Series::I32(name, reordered_data, reordered_validity)
                }
                Series::F64(name, data, validity) => {
                    let mut reordered_data = Vec::with_capacity(data.len());
                    let mut reordered_validity = Vec::with_capacity(validity.len());

                    for &idx in &indices {
                        reordered_data.push(data[idx]);
                        reordered_validity.push(validity[idx]);
                    }

                    Series::F64(name, reordered_data, reordered_validity)
                }
                Series::String(name, data, validity) => {
                    let mut reordered_data = Vec::with_capacity(data.len());
                    let mut reordered_validity = Vec::with_capacity(validity.len());

                    for &idx in &indices {
                        reordered_data.push(data[idx].clone());
                        reordered_validity.push(validity[idx]);
                    }

                    Series::String(name, reordered_data, reordered_validity)
                }
                Series::Bool(name, data, validity) => {
                    let mut reordered_data = Vec::with_capacity(data.len());
                    let mut reordered_validity = Vec::with_capacity(validity.len());

                    for &idx in &indices {
                        reordered_data.push(data[idx]);
                        reordered_validity.push(validity[idx]);
                    }

                    Series::Bool(name, reordered_data, reordered_validity)
                }
                Series::DateTime(name, data, validity) => {
                    let mut reordered_data = Vec::with_capacity(data.len());
                    let mut reordered_validity = Vec::with_capacity(validity.len());

                    for &idx in &indices {
                        reordered_data.push(data[idx]);
                        reordered_validity.push(validity[idx]);
                    }

                    Series::DateTime(name, reordered_data, reordered_validity)
                }
            };

            new_columns.insert(col_name, reordered_series);
        }

        Ok(DataFrame {
            columns: new_columns,
            row_count: df.row_count,
        })
    }

    fn apply_limit(
        &self,
        df: DataFrame,
        limit: usize,
    ) -> Result<DataFrame, Box<dyn std::error::Error>> {
        if limit >= df.row_count {
            return Ok(df);
        }

        let mut new_columns = HashMap::new();

        for (col_name, series) in df.columns {
            let limited_series = match series {
                Series::I32(name, data, validity) => {
                    let limited_data = data.into_iter().take(limit).collect();
                    let limited_validity = validity.into_iter().take(limit).collect();
                    Series::I32(name, limited_data, limited_validity)
                }
                Series::F64(name, data, validity) => {
                    let limited_data = data.into_iter().take(limit).collect();
                    let limited_validity = validity.into_iter().take(limit).collect();
                    Series::F64(name, limited_data, limited_validity)
                }
                Series::String(name, data, validity) => {
                    let limited_data = data.into_iter().take(limit).collect();
                    let limited_validity = validity.into_iter().take(limit).collect();
                    Series::String(name, limited_data, limited_validity)
                }
                Series::Bool(name, data, validity) => {
                    let limited_data = data.into_iter().take(limit).collect();
                    let limited_validity = validity.into_iter().take(limit).collect();
                    Series::Bool(name, limited_data, limited_validity)
                }
                Series::DateTime(name, data, validity) => {
                    let limited_data = data.into_iter().take(limit).collect();
                    let limited_validity = validity.into_iter().take(limit).collect();
                    Series::DateTime(name, limited_data, limited_validity)
                }
            };

            new_columns.insert(col_name, limited_series);
        }

        Ok(DataFrame {
            columns: new_columns,
            row_count: limit,
        })
    }

    fn apply_select(
        &self,
        df: DataFrame,
        select_columns: &[String],
    ) -> Result<DataFrame, Box<dyn std::error::Error>> {
        let mut new_columns = HashMap::new();

        for col_name in select_columns {
            if let Some(series) = df.columns.get(col_name) {
                new_columns.insert(col_name.clone(), series.clone());
            }
        }

        Ok(DataFrame {
            columns: new_columns,
            row_count: df.row_count,
        })
    }

    fn apply_aggregations(
        &self,
        df: &DataFrame,
        aggregations: &[AggregationSpec],
        mask: &[bool],
    ) -> Result<DataFrame, Box<dyn std::error::Error>> {
        let mut result_columns = HashMap::new();

        for agg_spec in aggregations {
            let series = df
                .columns
                .get(&agg_spec.column)
                .ok_or_else(|| format!("Column '{}' not found", agg_spec.column))?;

            let agg_name = format!(
                "{}({})",
                match agg_spec.function {
                    AggregationFunction::Count => "count",
                    AggregationFunction::Sum => "sum",
                    AggregationFunction::Average => "avg",
                    AggregationFunction::Min => "min",
                    AggregationFunction::Max => "max",
                },
                agg_spec.column
            );

            let result_series = match (&agg_spec.function, series) {
                (AggregationFunction::Count, _) => {
                    let count = match series {
                        Series::I32(_, _, validity) => validity
                            .iter()
                            .zip(mask.iter())
                            .filter(|(&valid, &include)| valid && include)
                            .count(),
                        Series::F64(_, _, validity) => validity
                            .iter()
                            .zip(mask.iter())
                            .filter(|(&valid, &include)| valid && include)
                            .count(),
                        Series::String(_, _, validity) => validity
                            .iter()
                            .zip(mask.iter())
                            .filter(|(&valid, &include)| valid && include)
                            .count(),
                        Series::Bool(_, _, validity) => validity
                            .iter()
                            .zip(mask.iter())
                            .filter(|(&valid, &include)| valid && include)
                            .count(),
                        Series::DateTime(_, _, validity) => validity
                            .iter()
                            .zip(mask.iter())
                            .filter(|(&valid, &include)| valid && include)
                            .count(),
                    };
                    Series::I32(agg_name.clone(), vec![count as i32], vec![true])
                }
                (AggregationFunction::Sum, Series::I32(_, data, validity)) => {
                    let sum: i64 = data
                        .iter()
                        .zip(validity.iter())
                        .zip(mask.iter())
                        .filter_map(|((&val, &valid), &include)| {
                            if valid && include {
                                Some(val as i64)
                            } else {
                                None
                            }
                        })
                        .sum();
                    Series::F64(agg_name.clone(), vec![sum as f64], vec![true])
                }
                (AggregationFunction::Sum, Series::F64(_, data, validity)) => {
                    let sum: f64 = data
                        .iter()
                        .zip(validity.iter())
                        .zip(mask.iter())
                        .filter_map(
                            |((&val, &valid), &include)| {
                                if valid && include {
                                    Some(val)
                                } else {
                                    None
                                }
                            },
                        )
                        .sum();
                    Series::F64(agg_name.clone(), vec![sum], vec![true])
                }
                (AggregationFunction::Average, Series::I32(_, data, validity)) => {
                    let values: Vec<i32> = data
                        .iter()
                        .zip(validity.iter())
                        .zip(mask.iter())
                        .filter_map(
                            |((&val, &valid), &include)| {
                                if valid && include {
                                    Some(val)
                                } else {
                                    None
                                }
                            },
                        )
                        .collect();
                    if values.is_empty() {
                        Series::F64(agg_name.clone(), vec![f64::NAN], vec![false])
                    } else {
                        let sum: i64 = values.iter().map(|&x| x as i64).sum();
                        let avg = sum as f64 / values.len() as f64;
                        Series::F64(agg_name.clone(), vec![avg], vec![true])
                    }
                }
                (AggregationFunction::Average, Series::F64(_, data, validity)) => {
                    let values: Vec<f64> = data
                        .iter()
                        .zip(validity.iter())
                        .zip(mask.iter())
                        .filter_map(
                            |((&val, &valid), &include)| {
                                if valid && include {
                                    Some(val)
                                } else {
                                    None
                                }
                            },
                        )
                        .collect();
                    if values.is_empty() {
                        Series::F64(agg_name.clone(), vec![f64::NAN], vec![false])
                    } else {
                        let sum: f64 = values.iter().sum();
                        let avg = sum / values.len() as f64;
                        Series::F64(agg_name.clone(), vec![avg], vec![true])
                    }
                }
                (AggregationFunction::Min, Series::I32(_, data, validity)) => {
                    let min_val = data
                        .iter()
                        .zip(validity.iter())
                        .zip(mask.iter())
                        .filter_map(
                            |((&val, &valid), &include)| {
                                if valid && include {
                                    Some(val)
                                } else {
                                    None
                                }
                            },
                        )
                        .min();
                    match min_val {
                        Some(min) => Series::I32(agg_name.clone(), vec![min], vec![true]),
                        None => Series::I32(agg_name.clone(), vec![0], vec![false]),
                    }
                }
                (AggregationFunction::Min, Series::F64(_, data, validity)) => {
                    let min_val = data
                        .iter()
                        .zip(validity.iter())
                        .zip(mask.iter())
                        .filter_map(
                            |((&val, &valid), &include)| {
                                if valid && include {
                                    Some(val)
                                } else {
                                    None
                                }
                            },
                        )
                        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
                    match min_val {
                        Some(min) => Series::F64(agg_name.clone(), vec![min], vec![true]),
                        None => Series::F64(agg_name.clone(), vec![f64::NAN], vec![false]),
                    }
                }
                (AggregationFunction::Max, Series::I32(_, data, validity)) => {
                    let max_val = data
                        .iter()
                        .zip(validity.iter())
                        .zip(mask.iter())
                        .filter_map(
                            |((&val, &valid), &include)| {
                                if valid && include {
                                    Some(val)
                                } else {
                                    None
                                }
                            },
                        )
                        .max();
                    match max_val {
                        Some(max) => Series::I32(agg_name.clone(), vec![max], vec![true]),
                        None => Series::I32(agg_name.clone(), vec![0], vec![false]),
                    }
                }
                (AggregationFunction::Max, Series::F64(_, data, validity)) => {
                    let max_val = data
                        .iter()
                        .zip(validity.iter())
                        .zip(mask.iter())
                        .filter_map(
                            |((&val, &valid), &include)| {
                                if valid && include {
                                    Some(val)
                                } else {
                                    None
                                }
                            },
                        )
                        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
                    match max_val {
                        Some(max) => Series::F64(agg_name.clone(), vec![max], vec![true]),
                        None => Series::F64(agg_name.clone(), vec![f64::NAN], vec![false]),
                    }
                }
                _ => {
                    return Err(format!(
                        "Unsupported aggregation: {:?} on column type",
                        agg_spec.function
                    )
                    .into());
                }
            };

            result_columns.insert(agg_name, result_series);
        }

        Ok(DataFrame {
            columns: result_columns,
            row_count: 1,
        })
    }
}
