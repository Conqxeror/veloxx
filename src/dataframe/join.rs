use crate::error::VeloxxError;
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
    /// This method combines two DataFrames based on a common column (`on_column`) and a specified
    /// `JoinType`. It creates a new DataFrame containing columns from both original DataFrames.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `DataFrame` to join with.
    /// * `on_column` - The name of the column to join on. This column must exist in both DataFrames
    ///   and have comparable data types.
    /// * `join_type` - The type of join to perform (`Inner`, `Left`, or `Right`).
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing the joined `DataFrame`,
    /// or `Err(VeloxxError::ColumnNotFound)` if the `on_column` is not found in either DataFrame,
    /// or `Err(VeloxxError::InvalidOperation)` if there are issues during the join process (e.g., incompatible types).
    ///
    /// # Examples
    ///
    /// ## Setup for Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    /// use veloxx::types::Value;
    ///
    /// // Left DataFrame
    /// let mut left_cols = BTreeMap::new();
    /// left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// let left_df = DataFrame::new(left_cols).unwrap();
    ///
    /// // Right DataFrame
    /// let mut right_cols = BTreeMap::new();
    /// right_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(2), Some(3), Some(4)]));
    /// right_cols.insert("city".to_string(), Series::new_string("city", vec![Some("London".to_string()), Some("Paris".to_string()), Some("Rome".to_string())]));
    /// let right_df = DataFrame::new(right_cols).unwrap();
    /// ```
    ///
    /// ## Inner Join
    ///
    /// Combines rows where `id` matches in both DataFrames.
    ///
    /// ```rust
    /// # use veloxx::dataframe::DataFrame;
    /// # use veloxx::series::Series;
    /// # use std::collections::BTreeMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = BTreeMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = BTreeMap::new();
    /// # right_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(2), Some(3), Some(4)]));
    /// # right_cols.insert("city".to_string(), Series::new_string("city", vec![Some("London".to_string()), Some("Paris".to_string()), Some("Rome".to_string())]));
    /// # let right_df = DataFrame::new(right_cols).unwrap();
    ///
    /// let inner_joined_df = left_df.join(&right_df, "id", JoinType::Inner).unwrap();
    /// // Expected rows: (id=2, name=Bob, city=London), (id=3, name=Charlie, city=Paris)
    /// assert_eq!(inner_joined_df.row_count(), 2);
    /// assert!(inner_joined_df.get_column("name").unwrap().get_value(0) == Some(Value::String("Bob".to_string())) || inner_joined_df.get_column("name").unwrap().get_value(0) == Some(Value::String("Charlie".to_string())));
    /// ```
    ///
    /// ## Left Join
    ///
    /// Returns all rows from `left_df`, and matching rows from `right_df`. Unmatched `right_df` columns will be null.
    ///
    /// ```rust
    /// # use veloxx::dataframe::DataFrame;
    /// # use veloxx::series::Series;
    /// # use std::collections::BTreeMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = BTreeMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = BTreeMap::new();
    /// # right_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(2), Some(3), Some(4)]));
    /// # right_cols.insert("city".to_string(), Series::new_string("city", vec![Some("London".to_string()), Some("Paris".to_string()), Some("Rome".to_string())]));
    /// # let right_df = DataFrame::new(right_cols).unwrap();
    ///
    /// let left_joined_df = left_df.join(&right_df, "id", JoinType::Left).unwrap();
    /// // Expected rows: (id=1, name=Alice, city=null), (id=2, name=Bob, city=London), (id=3, name=Charlie, city=Paris)
    /// assert_eq!(left_joined_df.row_count(), 3);
    /// assert_eq!(left_joined_df.get_column("city").unwrap().get_value(0), None);
    /// ```
    ///
    /// ## Right Join
    ///
    /// Returns all rows from `right_df`, and matching rows from `left_df`. Unmatched `left_df` columns will be null.
    ///
    /// ```rust
    /// # use veloxx::dataframe::DataFrame;
    /// # use veloxx::series::Series;
    /// # use std::collections::BTreeMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = BTreeMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = BTreeMap::new();
    /// # right_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(2), Some(3), Some(4)]));
    /// # right_cols.insert("city".to_string(), Series::new_string("city", vec![Some("London".to_string()), Some("Paris".to_string()), Some("Rome".to_string())]));
    /// # let right_df = DataFrame::new(right_cols).unwrap();
    ///
    /// let right_joined_df = left_df.join(&right_df, "id", JoinType::Right).unwrap();
    /// // Expected rows: (id=2, name=Bob, city=London), (id=3, name=Charlie, city=Paris), (id=4, name=null, city=Rome)
    /// assert_eq!(right_joined_df.row_count(), 3);
    /// assert_eq!(right_joined_df.get_column("name").unwrap().get_value(2), None);
    /// ```
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
