use crate::VeloxxError;
use crate::{dataframe::DataFrame, series::Series, types::Value};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::collections::HashMap;

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
    /// use std::collections::HashMap;
    /// use veloxx::types::Value;
    ///
    /// // Left DataFrame
    /// let mut left_cols = HashMap::new();
    /// left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// let left_df = DataFrame::new(left_cols).unwrap();
    ///
    /// // Right DataFrame
    /// let mut right_cols = HashMap::new();
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
    /// # use std::collections::HashMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = HashMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = HashMap::new();
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
    /// # use std::collections::HashMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = HashMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = HashMap::new();
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
    /// # use std::collections::HashMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = HashMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = HashMap::new();
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
        let mut new_columns: HashMap<String, Series> = HashMap::new();

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
        let all_column_names: Vec<String> = {
            let mut temp_names = Vec::new();
            for col_name in self_col_names.iter() {
                temp_names.push(col_name.clone());
            }
            for col_name in other_col_names.iter() {
                if !temp_names.contains(col_name) {
                    temp_names.push(col_name.clone());
                }
            }
            temp_names
        };

        let mut column_types: HashMap<String, crate::types::DataType> = HashMap::new();

        for col_name in self_col_names.iter() {
            column_types.insert(
                col_name.clone(),
                self.get_column(col_name).unwrap().data_type(),
            );
        }
        for col_name in other_col_names.iter() {
            if !column_types.contains_key(col_name) {
                column_types.insert(
                    col_name.clone(),
                    other.get_column(col_name).unwrap().data_type(),
                );
            }
        }

        // Initialize new Series data vectors
        let mut series_data: std::collections::HashMap<String, Vec<Option<Value>>> =
            std::collections::HashMap::new();
        for col_name in all_column_names.iter() {
            series_data.insert(col_name.clone(), Vec::new());
        }

        match join_type {
            JoinType::Inner => {
                let other_on_series = other.get_column(on_column).unwrap();
                let other_join_map: std::collections::HashMap<Value, Vec<usize>> = (0..other
                    .row_count())
                    .into_par_iter()
                    .filter_map(|i| other_on_series.get_value(i).map(|val| (val, i)))
                    .fold(
                        std::collections::HashMap::new,
                        |mut map: std::collections::HashMap<Value, Vec<usize>>, (val, i)| {
                            map.entry(val).or_default().push(i);
                            map
                        },
                    )
                    .reduce(std::collections::HashMap::new, |mut acc, map| {
                        for (key, value) in map {
                            acc.entry(key).or_default().extend(value);
                        }
                        acc
                    });

                let self_on_series = self.get_column(on_column).unwrap();
                let results: Vec<Vec<(String, Option<Value>)>> = (0..self.row_count())
                    .into_par_iter()
                    .filter_map(|i| {
                        if let Some(self_join_val) = self_on_series.get_value(i) {
                            if let Some(other_indices) = other_join_map.get(&self_join_val) {
                                let self_col_names_cloned = self_col_names.clone();
                                let all_column_names_cloned = all_column_names.clone();
                                Some(
                                    other_indices
                                        .par_iter()
                                        .flat_map(move |&other_idx| {
                                            let mut row_values = Vec::new();
                                            for col_name in all_column_names_cloned.iter() {
                                                let value = if self_col_names_cloned
                                                    .contains(col_name)
                                                {
                                                    self.get_column(col_name).unwrap().get_value(i)
                                                } else {
                                                    other
                                                        .get_column(col_name)
                                                        .unwrap()
                                                        .get_value(other_idx)
                                                };
                                                row_values.push((col_name.clone(), value));
                                            }
                                            vec![row_values]
                                        })
                                        .collect::<Vec<_>>(),
                                )
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .collect();

                for row_values in results {
                    for (col_name, value) in row_values {
                        series_data.get_mut(&col_name).unwrap().push(value);
                    }
                }
            }
            JoinType::Left => {
                let other_on_series = other.get_column(on_column).unwrap();
                let other_join_map: std::collections::HashMap<Value, Vec<usize>> = (0..other
                    .row_count())
                    .into_par_iter()
                    .filter_map(|i| other_on_series.get_value(i).map(|val| (val, i)))
                    .fold(
                        std::collections::HashMap::new,
                        |mut map: std::collections::HashMap<Value, Vec<usize>>, (val, i)| {
                            map.entry(val).or_default().push(i);
                            map
                        },
                    )
                    .reduce(std::collections::HashMap::new, |mut acc, map| {
                        for (key, value) in map {
                            acc.entry(key).or_default().extend(value);
                        }
                        acc
                    });

                let self_on_series = self.get_column(on_column).unwrap();
                let collected_rows: Vec<Vec<(String, Option<Value>)>> = (0..self.row_count())
                    .into_par_iter()
                    .flat_map(|i| {
                        if let Some(self_join_val) = self_on_series.get_value(i) {
                            if let Some(other_indices) = other_join_map.get(&self_join_val) {
                                let self_col_names_cloned = self_col_names.clone();
                                let all_column_names_cloned = all_column_names.clone();
                                let _other_col_names_cloned = other_col_names.clone();
                                other_indices
                                    .par_iter()
                                    .map(move |&other_idx| {
                                        let mut row_values = Vec::new();
                                        for col_name in all_column_names_cloned.iter() {
                                            let value = if self_col_names_cloned.contains(col_name)
                                            {
                                                self.get_column(col_name).unwrap().get_value(i)
                                            } else {
                                                other
                                                    .get_column(col_name)
                                                    .unwrap()
                                                    .get_value(other_idx)
                                            };
                                            row_values.push((col_name.clone(), value));
                                        }
                                        row_values
                                    })
                                    .collect::<Vec<_>>()
                            } else {
                                let all_column_names_cloned = all_column_names.clone();
                                let self_col_names_cloned = self_col_names.clone();
                                let mut row_values = Vec::new();
                                for col_name in all_column_names_cloned.iter() {
                                    let value = if self_col_names_cloned.contains(col_name) {
                                        self.get_column(col_name).unwrap().get_value(i)
                                    } else {
                                        None
                                    };
                                    row_values.push((col_name.clone(), value));
                                }
                                vec![row_values]
                            }
                        } else {
                            let all_column_names_cloned = all_column_names.clone();
                            let self_col_names_cloned = self_col_names.clone();
                            let mut row_values = Vec::new();
                            for col_name in all_column_names_cloned.iter() {
                                let value = if self_col_names_cloned.contains(col_name) {
                                    self.get_column(col_name).unwrap().get_value(i)
                                } else {
                                    None
                                };
                                row_values.push((col_name.clone(), value));
                            }
                            vec![row_values]
                        }
                    })
                    .collect();

                for row_values in collected_rows {
                    for (col_name, value) in row_values {
                        series_data.get_mut(&col_name).unwrap().push(value);
                    }
                }
            }
            JoinType::Right => {
                let self_on_series = self.get_column(on_column).unwrap();
                let self_join_map: std::collections::HashMap<Value, Vec<usize>> = (0..self
                    .row_count())
                    .into_par_iter()
                    .filter_map(|i| self_on_series.get_value(i).map(|val| (val, i)))
                    .fold(
                        std::collections::HashMap::new,
                        |mut map: std::collections::HashMap<Value, Vec<usize>>, (val, i)| {
                            map.entry(val).or_default().push(i);
                            map
                        },
                    )
                    .reduce(std::collections::HashMap::new, |mut acc, map| {
                        for (key, value) in map {
                            acc.entry(key).or_default().extend(value);
                        }
                        acc
                    });

                let other_on_series = other.get_column(on_column).unwrap();
                let collected_rows: Vec<Vec<(String, Option<Value>)>> = (0..other.row_count())
                    .into_par_iter()
                    .flat_map(|i| {
                        if let Some(other_join_val) = other_on_series.get_value(i) {
                            if let Some(self_indices) = self_join_map.get(&other_join_val) {
                                let other_col_names_cloned = other_col_names.clone();
                                let all_column_names_cloned = all_column_names.clone();
                                let _self_col_names_cloned = self_col_names.clone();
                                self_indices
                                    .par_iter()
                                    .map(move |&self_idx| {
                                        let mut row_values = Vec::new();
                                        for col_name in all_column_names_cloned.iter() {
                                            let value = if other_col_names_cloned.contains(col_name)
                                            {
                                                other.get_column(col_name).unwrap().get_value(i)
                                            } else {
                                                self.get_column(col_name)
                                                    .unwrap()
                                                    .get_value(self_idx)
                                            };
                                            row_values.push((col_name.clone(), value));
                                        }
                                        row_values
                                    })
                                    .collect::<Vec<_>>()
                            } else {
                                let all_column_names_cloned = all_column_names.clone();
                                let other_col_names_cloned = other_col_names.clone();
                                let mut row_values = Vec::new();
                                for col_name in all_column_names_cloned.iter() {
                                    let value = if other_col_names_cloned.contains(col_name) {
                                        other.get_column(col_name).unwrap().get_value(i)
                                    } else {
                                        None
                                    };
                                    row_values.push((col_name.clone(), value));
                                }
                                vec![row_values]
                            }
                        } else {
                            let all_column_names_cloned = all_column_names.clone();
                            let other_col_names_cloned = other_col_names.clone();
                            let mut row_values = Vec::new();
                            for col_name in all_column_names_cloned.iter() {
                                let value = if other_col_names_cloned.contains(col_name) {
                                    other.get_column(col_name).unwrap().get_value(i)
                                } else {
                                    None
                                };
                                row_values.push((col_name.clone(), value));
                            }
                            vec![row_values]
                        }
                    })
                    .collect();

                for row_values in collected_rows {
                    for (col_name, value) in row_values {
                        series_data.get_mut(&col_name).unwrap().push(value);
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
