use crate::{dataframe::DataFrame, series::Series, types::Value};
use std::collections::BTreeMap;
use crate::error::VeloxxError;

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
    pub fn join(
        &self,
        other: &DataFrame,
        on_column: &str,
        join_type: JoinType,
    ) -> Result<Self, VeloxxError> {
        let mut new_columns: BTreeMap<String, Series> = BTreeMap::new();

        let self_col_names: Vec<String> =
            self.column_names().iter().map(|s| (*s).clone()).collect();
        let other_col_names: Vec<String> =
            other.column_names().iter().map(|s| (*s).clone()).collect();

        // Check if join column exists in both DataFrames
        if !self_col_names.contains(&on_column.to_string()) {
            return Err(VeloxxError::ColumnNotFound(format!(
                "Join column '{on_column}' not found in left DataFrame."
            )));
        }
        if !other_col_names.contains(&on_column.to_string()) {
            return Err(VeloxxError::ColumnNotFound(format!(
                "Join column '{on_column}' not found in right DataFrame."
            )));
        }

        // Determine all unique column names and their types
        let mut all_column_names: Vec<String> = Vec::new();
        let mut column_types: BTreeMap<String, crate::types::DataType> = BTreeMap::new();

        for col_name in self_col_names.iter() {
            all_column_names.push(col_name.clone());
            column_types.insert(
                col_name.clone(),
                self.get_column(col_name).unwrap().data_type(),
            );
        }
        for col_name in other_col_names.iter() {
            if !all_column_names.contains(col_name) {
                all_column_names.push(col_name.clone());
                column_types.insert(
                    col_name.clone(),
                    other.get_column(col_name).unwrap().data_type(),
                );
            }
        }

        // Initialize new Series data vectors
        let mut series_data: BTreeMap<String, Vec<Option<Value>>> = BTreeMap::new();
        for col_name in all_column_names.iter() {
            series_data.insert(col_name.clone(), Vec::new());
        }

        match join_type {
            JoinType::Inner => {
                let mut other_join_map: std::collections::HashMap<Value, Vec<usize>> =
                    std::collections::HashMap::new();
                let other_on_series = other.get_column(on_column).unwrap();
                for i in 0..other.row_count() {
                    if let Some(val) = other_on_series.get_value(i) {
                        other_join_map.entry(val).or_default().push(i);
                    }
                }

                let self_on_series = self.get_column(on_column).unwrap();
                for i in 0..self.row_count() {
                    if let Some(self_join_val) = self_on_series.get_value(i) {
                        if let Some(other_indices) = other_join_map.get(&self_join_val) {
                            for &other_idx in other_indices {
                                // Populate data for all columns
                                for col_name in all_column_names.iter() {
                                    let value = if self_col_names.contains(col_name) {
                                        self.get_column(col_name).unwrap().get_value(i)
                                    } else {
                                        other.get_column(col_name).unwrap().get_value(other_idx)
                                    };
                                    series_data.get_mut(col_name).unwrap().push(value);
                                }
                            }
                        }
                    }
                }
            }
            JoinType::Left => {
                // Left join logic (similar optimization can be applied)
                let mut other_join_map: std::collections::HashMap<Value, Vec<usize>> =
                    std::collections::HashMap::new();
                let other_on_series = other.get_column(on_column).unwrap();
                for i in 0..other.row_count() {
                    if let Some(val) = other_on_series.get_value(i) {
                        other_join_map.entry(val).or_default().push(i);
                    }
                }

                let self_on_series = self.get_column(on_column).unwrap();
                for i in 0..self.row_count() {
                    if let Some(self_join_val) = self_on_series.get_value(i) {
                        if let Some(other_indices) = other_join_map.get(&self_join_val) {
                            for &other_idx in other_indices {
                                // Matched row
                                for col_name in all_column_names.iter() {
                                    let value = if self_col_names.contains(col_name) {
                                        self.get_column(col_name).unwrap().get_value(i)
                                    } else {
                                        other.get_column(col_name).unwrap().get_value(other_idx)
                                    };
                                    series_data.get_mut(col_name).unwrap().push(value);
                                }
                            }
                        } else {
                            // Unmatched self_row
                            for col_name in all_column_names.iter() {
                                let value = if self_col_names.contains(col_name) {
                                    self.get_column(col_name).unwrap().get_value(i)
                                } else {
                                    None
                                };
                                series_data.get_mut(col_name).unwrap().push(value);
                            }
                        }
                    } else {
                        // self_row has null in on_column, treat as unmatched for now
                        for col_name in all_column_names.iter() {
                            let value = if self_col_names.contains(col_name) {
                                self.get_column(col_name).unwrap().get_value(i)
                            } else {
                                None
                            };
                            series_data.get_mut(col_name).unwrap().push(value);
                        }
                    }
                }
            }
            JoinType::Right => {
                // Right join logic (similar optimization can be applied)
                let mut self_join_map: std::collections::HashMap<Value, Vec<usize>> =
                    std::collections::HashMap::new();
                let self_on_series = self.get_column(on_column).unwrap();
                for i in 0..self.row_count() {
                    if let Some(val) = self_on_series.get_value(i) {
                        self_join_map.entry(val).or_default().push(i);
                    }
                }

                let other_on_series = other.get_column(on_column).unwrap();
                for i in 0..other.row_count() {
                    if let Some(other_join_val) = other_on_series.get_value(i) {
                        if let Some(self_indices) = self_join_map.get(&other_join_val) {
                            for &self_idx in self_indices {
                                // Matched row
                                for col_name in all_column_names.iter() {
                                    let value = if other_col_names.contains(col_name) {
                                        other.get_column(col_name).unwrap().get_value(i)
                                    } else {
                                        self.get_column(col_name).unwrap().get_value(self_idx)
                                    };
                                    series_data.get_mut(col_name).unwrap().push(value);
                                }
                            }
                        } else {
                            // Unmatched other_row
                            for col_name in all_column_names.iter() {
                                let value = if other_col_names.contains(col_name) {
                                    other.get_column(col_name).unwrap().get_value(i)
                                } else {
                                    None
                                };
                                series_data.get_mut(col_name).unwrap().push(value);
                            }
                        }
                    } else {
                        // other_row has null in on_column, treat as unmatched for now
                        for col_name in all_column_names.iter() {
                            let value = if other_col_names.contains(col_name) {
                                other.get_column(col_name).unwrap().get_value(i)
                            } else {
                                None
                            };
                            series_data.get_mut(col_name).unwrap().push(value);
                        }
                    }
                }
            }
        }

        // Create new Series objects
        for (col_name, data_vec) in series_data {
            let col_data_type = column_types.get(&col_name).unwrap();
            let new_series = match col_data_type {
                crate::types::DataType::I32 => Series::new_i32(
                    &col_name,
                    data_vec
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
                    &col_name,
                    data_vec
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
                    &col_name,
                    data_vec
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
                    &col_name,
                    data_vec
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
                    &col_name,
                    data_vec
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
            new_columns.insert(col_name, new_series);
        }

        DataFrame::new(new_columns)
    }
}
