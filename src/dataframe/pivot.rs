use crate::dataframe::DataFrame;
use crate::series::Series;
use crate::types::{DataType, Value};
use crate::VeloxxError;
use indexmap::IndexMap;
use std::collections::HashMap;

/// Pivot operation for DataFrames.
///
/// This module provides the `pivot` functionality to reshape a DataFrame from long to wide format.
/// It involves three key components:
/// - `index`: Columns to group by (the new rows).
/// - `columns`: The column whose unique values will become new column headers.
/// - `values`: The column(s) to aggregate for each cell.
pub trait Pivot {
    /// Reshape the DataFrame from long to wide format.
    ///
    /// # Arguments
    ///
    /// * `values` - The column name containing the values to be aggregated.
    /// * `index` - The column names to group by (these will be the row identifiers).
    /// * `columns` - The column name whose unique values will become the new column headers.
    /// * `agg_fn` - The aggregation function to apply (e.g., "sum", "mean", "min", "max", "count").
    ///
    /// # Returns
    ///
    /// A new `DataFrame` with the reshaped data.
    fn pivot(
        &self,
        values: &str,
        index: Vec<String>,
        columns: &str,
        agg_fn: &str,
    ) -> Result<DataFrame, VeloxxError>;
}

impl Pivot for DataFrame {
    fn pivot(
        &self,
        values: &str,
        index: Vec<String>,
        columns: &str,
        agg_fn: &str,
    ) -> Result<DataFrame, VeloxxError> {
        // 1. Group by the index columns
        // We need to group by `index` AND the `columns` column first to get unique cells.
        // Actually, standard pivot logic:
        // Group by [index + columns] -> Aggregate [values] -> Reshape.

        let mut group_cols = index.clone();
        group_cols.push(columns.to_string());

        let grouped = self.group_by(group_cols.clone())?;

        // 2. Aggregate
        // The resulting DF will have: [index cols..., columns_col, values_aggregated]
        let agg_res = grouped.agg(vec![(values, agg_fn)])?;

        // The name of the aggregated value column
        let val_col_name = format!("{}_{}", values, agg_fn);

        // 3. Identify unique values in the `columns` column to form new headers
        let pivot_col_series = agg_res
            .get_column(columns)
            .ok_or(VeloxxError::ColumnNotFound(columns.to_string()))?;

        // Get unique values from the pivot column to determine new column headers
        // We need to collect them to a Vec<String> to ensure stable order.
        let mut unique_headers: Vec<String> = Vec::new();
        let len = pivot_col_series.len();

        for i in 0..len {
            let val_str = match pivot_col_series.get_value(i) {
                Some(Value::String(s)) => s,
                Some(v) => v.to_string(),
                None => "null".to_string(),
            };
            if !unique_headers.contains(&val_str) {
                unique_headers.push(val_str);
            }
        }
        unique_headers.sort(); // Sort headers for deterministic output

        // 4. Construct the new DataFrame
        // We need to map each (index_tuple) -> { header -> value }

        // Build a map of (IndexKey) -> { Header -> Value }
        // IndexKey is a tuple of values from `index` columns.

        // Let's use a HashMap<Vec<String>, HashMap<String, Value>>
        // where outer key is the row identifier, inner key is the column header.

        let mut row_map: HashMap<Vec<String>, HashMap<String, Value>> = HashMap::new();
        let mut row_order: Vec<Vec<String>> = Vec::new(); // To preserve row order based on first appearance

        let agg_len = agg_res.row_count();
        let agg_val_series = agg_res
            .get_column(&val_col_name)
            .ok_or(VeloxxError::ColumnNotFound(val_col_name.clone()))?;

        for i in 0..agg_len {
            // Extract index key
            let mut key = Vec::with_capacity(index.len());
            for idx_col in &index {
                let s = agg_res.get_column(idx_col).unwrap();
                let v = s.get_value(i).unwrap_or(Value::Null).to_string();
                key.push(v);
            }

            // Extract header
            let header_val = match pivot_col_series.get_value(i) {
                Some(Value::String(s)) => s,
                Some(v) => v.to_string(),
                None => "null".to_string(),
            };

            // Extract value
            let cell_val = agg_val_series.get_value(i).unwrap_or(Value::Null);

            if !row_map.contains_key(&key) {
                row_map.insert(key.clone(), HashMap::new());
                row_order.push(key.clone());
            }

            if let Some(inner_map) = row_map.get_mut(&key) {
                inner_map.insert(header_val, cell_val);
            }
        }

        // 5. Build final Series
        let mut final_columns = IndexMap::new();

        // RESTARTING LOOP with Value keys for type safety
        let mut row_map_typed: HashMap<Vec<Value>, HashMap<String, Value>> = HashMap::new();
        let mut row_order_typed: Vec<Vec<Value>> = Vec::new();

        for i in 0..agg_len {
            let mut key = Vec::with_capacity(index.len());
            for idx_col in &index {
                let s = agg_res.get_column(idx_col).unwrap();
                let v = s.get_value(i).unwrap_or(Value::Null);
                key.push(v);
            }

            let header_val = match pivot_col_series.get_value(i) {
                Some(Value::String(s)) => s,
                Some(v) => v.to_string(),
                None => "null".to_string(),
            };

            let cell_val = agg_val_series.get_value(i).unwrap_or(Value::Null);

            if !row_map_typed.contains_key(&key) {
                row_map_typed.insert(key.clone(), HashMap::new());
                row_order_typed.push(key.clone());
            }

            if let Some(inner_map) = row_map_typed.get_mut(&key) {
                inner_map.insert(header_val, cell_val);
            }
        }

        // Now build columns
        // Index columns
        for (i, col_name) in index.iter().enumerate() {
            let mut data = Vec::with_capacity(row_order_typed.len());
            for row_key in &row_order_typed {
                data.push(Some(row_key[i].clone()));
            }

            // Create series from Vec<Option<Value>>
            // We need a helper `Series::from_any_values` or match on first value type.
            // Or check original series type again.
            let orig_series = agg_res.get_column(col_name).unwrap();
            let new_s = Series::from_values(col_name, data, orig_series.data_type())?;
            final_columns.insert(col_name.clone(), new_s);
        }

        // Pivot columns (headers)
        // What is the type of the values? It's the type of `values` column.
        let val_dtype = agg_val_series.data_type();

        for header in &unique_headers {
            let mut data = Vec::with_capacity(row_order_typed.len());
            for row_key in &row_order_typed {
                let inner_map = row_map_typed.get(row_key).unwrap();
                let val = inner_map.get(header).cloned();
                data.push(val);
            }
            let new_s = Series::from_values(header, data, val_dtype.clone())?;
            final_columns.insert(header.clone(), new_s);
        }

        Ok(DataFrame::new(final_columns))
    }
}

// Helper extension for Series to create from generic Values
impl Series {
    fn from_values(
        name: &str,
        values: Vec<Option<Value>>,
        dtype: DataType,
    ) -> Result<Series, VeloxxError> {
        match dtype {
            DataType::I32 => {
                let extracted: Vec<Option<i32>> = values
                    .into_iter()
                    .map(|v| v.and_then(|val| val.as_i32()))
                    .collect();
                Ok(Series::new_i32(name, extracted))
            }
            DataType::F64 => {
                let extracted: Vec<Option<f64>> = values
                    .into_iter()
                    .map(|v| v.and_then(|val| val.as_f64()))
                    .collect();
                Ok(Series::new_f64(name, extracted))
            }
            DataType::Bool => {
                let extracted: Vec<Option<bool>> = values
                    .into_iter()
                    .map(|v| v.and_then(|val| val.as_bool()))
                    .collect();
                Ok(Series::new_bool(name, extracted))
            }
            DataType::String => {
                let extracted: Vec<Option<String>> = values
                    .into_iter()
                    .map(|v| v.and_then(|val| val.as_string().cloned()))
                    .collect();
                Ok(Series::new_string(name, extracted))
            }
            DataType::DateTime => {
                let extracted: Vec<Option<i64>> = values
                    .into_iter()
                    .map(|v| v.and_then(|val| val.as_datetime()))
                    .collect();
                Ok(Series::new_datetime(name, extracted))
            }
        }
    }
}
