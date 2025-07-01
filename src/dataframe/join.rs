use crate::{dataframe::DataFrame, series::Series, types::Value};
use std::collections::BTreeMap;

#[derive(PartialEq)]
/// Defines the type of join to be performed between two DataFrames.
pub enum JoinType {
    /// Returns only the rows that have matching values in both DataFrames.
    Inner,
    /// Returns all rows from the left DataFrame, and the matching rows from the right DataFrame.
    Left,
    /// Returns all rows from the right DataFrame, and the matching rows from the left DataFrame.
    Right,
}

impl DataFrame {
    /// Performs a join operation with another `DataFrame`.
    ///
    /// # Arguments
    /// * `other` - The other `DataFrame` to join with.
    /// * `on_column` - The name of the column to join on. This column must exist in both DataFrames.
    /// * `join_type` - The type of join to perform (`Inner`, `Left`, or `Right`).
    ///
    /// # Returns
    /// A `Result` containing the joined `DataFrame` or a `String` error message.
    pub fn join(&self, other: &DataFrame, on_column: &str, join_type: JoinType) -> Result<Self, String> {
        let mut new_columns: BTreeMap<String, Series> = BTreeMap::new();

        let self_col_names: Vec<String> = self.column_names().iter().map(|s| (*s).clone()).collect();
        let other_col_names: Vec<String> = other.column_names().iter().map(|s| (*s).clone()).collect();

        // Check if join column exists in both DataFrames
        if !self_col_names.contains(&on_column.to_string()) {
            return Err(format!("Join column '{on_column}' not found in left DataFrame."));
        }
        if !other_col_names.contains(&on_column.to_string()) {
            return Err(format!("Join column '{on_column}' not found in right DataFrame."));
        }

        // Collect data from both DataFrames into a common format for easier processing
        let mut self_rows: Vec<BTreeMap<String, Option<Value>>> = Vec::new();
        for i in 0..self.row_count {
            let mut row_map: BTreeMap<String, Option<Value>> = BTreeMap::new();
            for col_name in self.column_names() {
                row_map.insert(col_name.clone(), self.get_column(col_name).unwrap().get_value(i));
            }
            self_rows.push(row_map);
        }

        let mut other_rows: Vec<BTreeMap<String, Option<Value>>> = Vec::new();
        for i in 0..other.row_count {
            let mut row_map: BTreeMap<String, Option<Value>> = BTreeMap::new();
            for col_name in other.column_names() {
                row_map.insert(col_name.clone(), other.get_column(col_name).unwrap().get_value(i));
            }
            other_rows.push(row_map);
        }

        let mut joined_rows: Vec<BTreeMap<String, Option<Value>>> = Vec::new();

        match join_type {
            JoinType::Inner => {
                for self_row in self_rows.iter() {
                    if let Some(self_join_val) = self_row.get(on_column).and_then(|v| v.as_ref()) {
                        for other_row in other_rows.iter() {
                            if let Some(other_join_val) = other_row.get(on_column).and_then(|v| v.as_ref()) {
                                if self_join_val == other_join_val {
                                    let mut new_row = self_row.clone();
                                    for (key, value) in other_row.iter() {
                                        if key != on_column {
                                            new_row.insert(key.clone(), value.clone());
                                        }
                                    }
                                    joined_rows.push(new_row);
                                }
                            }
                        }
                    }
                }
            }
            JoinType::Left => {
                for self_row in self_rows.iter() {
                    let mut matched = false;
                    if let Some(self_join_val) = self_row.get(on_column).and_then(|v| v.as_ref()) {
                        for other_row in other_rows.iter() {
                            if let Some(other_join_val) = other_row.get(on_column).and_then(|v| v.as_ref()) {
                                if self_join_val == other_join_val {
                                    let mut new_row = self_row.clone();
                                    for (key, value) in other_row.iter() {
                                        if key != on_column {
                                            new_row.insert(key.clone(), value.clone());
                                        }
                                    }
                                    joined_rows.push(new_row);
                                    matched = true;
                                }
                            }
                        }
                    }
                    if !matched {
                        let mut new_row = self_row.clone();
                        for col_name in other_col_names.iter() {
                            if col_name != on_column {
                                new_row.insert(col_name.clone(), None);
                            }
                        }
                        joined_rows.push(new_row);
                    }
                }
            }
            JoinType::Right => {
                for other_row in other_rows.iter() {
                    let mut matched = false;
                    if let Some(other_join_val) = other_row.get(on_column).and_then(|v| v.as_ref()) {
                        for self_row in self_rows.iter() {
                            if let Some(self_join_val) = self_row.get(on_column).and_then(|v| v.as_ref()) {
                                if self_join_val == other_join_val {
                                    let mut new_row = self_row.clone();
                                    for (key, value) in other_row.iter() {
                                        if key != on_column {
                                            new_row.insert(key.clone(), value.clone());
                                        }
                                    }
                                    joined_rows.push(new_row);
                                    matched = true;
                                }
                            }
                        }
                    }
                    if !matched {
                        let mut new_row = other_row.clone();
                        for col_name in self_col_names.iter() {
                            if col_name != on_column {
                                new_row.insert(col_name.clone(), None);
                            }
                        }
                        joined_rows.push(new_row);
                    }
                }
            }
        }

        if joined_rows.is_empty() && (join_type == JoinType::Inner || (self.row_count == 0 && other.row_count == 0)) {
            return Ok(DataFrame { columns: BTreeMap::new(), row_count: 0 });
        }

        // Determine all unique column names and their types
        let mut all_column_names: Vec<String> = Vec::new();
        let mut column_types: BTreeMap<String, crate::types::DataType> = BTreeMap::new();

        for col_name in self_col_names.iter() {
            all_column_names.push(col_name.clone());
            column_types.insert(col_name.clone(), self.get_column(col_name).unwrap().data_type());
        }
        for col_name in other_col_names.iter() {
            if !all_column_names.contains(col_name) {
                all_column_names.push(col_name.clone());
                column_types.insert(col_name.clone(), other.get_column(col_name).unwrap().data_type());
            }
        }

        // Initialize new Series for each column
        let mut series_data: BTreeMap<String, Vec<Option<Value>>> = BTreeMap::new();
        for col_name in all_column_names.iter() {
            series_data.insert(col_name.clone(), Vec::with_capacity(joined_rows.len()));
        }

        // Populate new Series data
        for row_map in joined_rows.iter() {
            for col_name in all_column_names.iter() {
                series_data.get_mut(col_name).unwrap().push(row_map.get(col_name).unwrap_or(&None).clone());
            }
        }

        // Create new Series objects
        for (col_name, data_vec) in series_data {
            let col_data_type = column_types.get(&col_name).unwrap();
            let new_series = match col_data_type {
                crate::types::DataType::I32 => Series::new_i32(&col_name, data_vec.into_iter().map(|x| x.and_then(|v| if let Value::I32(val) = v { Some(val) } else { None })).collect()),
                crate::types::DataType::F64 => Series::new_f64(&col_name, data_vec.into_iter().map(|x| x.and_then(|v| if let Value::F64(val) = v { Some(val) } else { None })).collect()),
                crate::types::DataType::Bool => Series::new_bool(&col_name, data_vec.into_iter().map(|x| x.and_then(|v| if let Value::Bool(val) = v { Some(val) } else { None })).collect()),
                crate::types::DataType::String => Series::new_string(&col_name, data_vec.into_iter().map(|x| x.and_then(|v| if let Value::String(val) = v { Some(val) } else { None })).collect()),
            };
            new_columns.insert(col_name, new_series);
        }

        DataFrame::new(new_columns)
    }
}
