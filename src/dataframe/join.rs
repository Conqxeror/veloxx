use crate::VeloxxError;
use crate::{dataframe::DataFrame, series::Series, types::Value};
use indexmap::IndexMap;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;

#[derive(PartialEq)]
/// Defines the type of join to be performed between two DataFrames.
pub enum JoinType {
    /// Returns only the rows that have matching values in both DataFrames.
    Inner,
    /// Returns all rows from the left DataFrame, and the matching rows from the right DataFrame.
    Left,
    /// Returns all rows from the right DataFrame, and the matching rows from the left DataFrame.
    Right,
    /// Returns all rows when there is a match in either left or right DataFrame.
    Outer,
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
    /// * `join_type` - The type of join to perform (`Inner`, `Left`, `Right`, or `Outer`).
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
    /// use indexmap::IndexMap;
    /// use veloxx::types::Value;
    ///
    /// // Left DataFrame
    /// let mut left_cols = IndexMap::new();
    /// left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// let left_df = DataFrame::new(left_cols).unwrap();
    ///
    /// // Right DataFrame
    /// let mut right_cols = IndexMap::new();
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
    /// # use indexmap::IndexMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = IndexMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = IndexMap::new();
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
    /// # use indexmap::IndexMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = IndexMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = IndexMap::new();
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
    /// # use indexmap::IndexMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = IndexMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = IndexMap::new();
    /// # right_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(2), Some(3), Some(4)]));
    /// # right_cols.insert("city".to_string(), Series::new_string("city", vec![Some("London".to_string()), Some("Paris".to_string()), Some("Rome".to_string())]));
    /// # let right_df = DataFrame::new(right_cols).unwrap();
    ///
    /// let right_joined_df = left_df.join(&right_df, "id", JoinType::Right).unwrap();
    /// // Expected rows: (id=2, name=Bob, city=London), (id=3, name=Charlie, city=Paris), (id=4, name=null, city=Rome)
    /// assert_eq!(right_joined_df.row_count(), 3);
    /// assert_eq!(right_joined_df.get_column("name").unwrap().get_value(2), None);
    /// ```
    ///
    /// ## Outer Join
    ///
    /// Returns all rows from both DataFrames.
    ///
    /// ```rust
    /// # use veloxx::dataframe::DataFrame;
    /// # use veloxx::series::Series;
    /// # use indexmap::IndexMap;
    /// # use veloxx::types::Value;
    /// # use veloxx::dataframe::join::JoinType;
    /// # let mut left_cols = IndexMap::new();
    /// # left_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    /// # left_cols.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]));
    /// # let left_df = DataFrame::new(left_cols).unwrap();
    /// # let mut right_cols = IndexMap::new();
    /// # right_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(2), Some(3), Some(4)]));
    /// # right_cols.insert("city".to_string(), Series::new_string("city", vec![Some("London".to_string()), Some("Paris".to_string()), Some("Rome".to_string())]));
    /// # let right_df = DataFrame::new(right_cols).unwrap();
    ///
    /// let outer_joined_df = left_df.join(&right_df, "id", JoinType::Outer).unwrap();
    /// // Expected rows: 1, 2, 3, 4
    /// assert_eq!(outer_joined_df.row_count(), 4);
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn join(
        &self,
        other: &DataFrame,
        on_column: &str,
        join_type: JoinType,
    ) -> Result<Self, VeloxxError> {
        let mut new_columns: IndexMap<String, Series> = IndexMap::new();

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

        let mut column_types: IndexMap<String, crate::types::DataType> = IndexMap::new();

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
        let mut series_data: indexmap::IndexMap<String, Vec<Option<Value>>> =
            indexmap::IndexMap::new();
        for col_name in all_column_names.iter() {
            series_data.insert(col_name.clone(), Vec::new());
        }

        match join_type {
            JoinType::Inner => {
                let other_on_series = other.get_column(on_column).unwrap();
                let other_join_map: indexmap::IndexMap<Value, Vec<usize>> = (0..other.row_count())
                    .into_par_iter()
                    .filter_map(|i| other_on_series.get_value(i).map(|val| (val, i)))
                    .fold(
                        indexmap::IndexMap::new,
                        |mut map: indexmap::IndexMap<Value, Vec<usize>>, (val, i)| {
                            map.entry(val).or_default().push(i);
                            map
                        },
                    )
                    .reduce(indexmap::IndexMap::new, |mut acc, map| {
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
                let other_join_map: indexmap::IndexMap<Value, Vec<usize>> = (0..other.row_count())
                    .into_par_iter()
                    .filter_map(|i| other_on_series.get_value(i).map(|val| (val, i)))
                    .fold(
                        indexmap::IndexMap::new,
                        |mut map: indexmap::IndexMap<Value, Vec<usize>>, (val, i)| {
                            map.entry(val).or_default().push(i);
                            map
                        },
                    )
                    .reduce(indexmap::IndexMap::new, |mut acc, map| {
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
                let self_join_map: indexmap::IndexMap<Value, Vec<usize>> = (0..self.row_count())
                    .into_par_iter()
                    .filter_map(|i| self_on_series.get_value(i).map(|val| (val, i)))
                    .fold(
                        indexmap::IndexMap::new,
                        |mut map: indexmap::IndexMap<Value, Vec<usize>>, (val, i)| {
                            map.entry(val).or_default().push(i);
                            map
                        },
                    )
                    .reduce(indexmap::IndexMap::new, |mut acc, map| {
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
            JoinType::Outer => {
                // Full Outer Join implementation
                // Strategy:
                // 1. Perform Left Outer Join logic
                // 2. Track which keys from Right DataFrame were matched
                // 3. Append rows from Right DataFrame that were NOT matched

                let other_on_series = other.get_column(on_column).unwrap();
                // Build map of Right DataFrame keys -> indices
                let other_join_map: indexmap::IndexMap<Value, Vec<usize>> = (0..other.row_count())
                    .into_par_iter()
                    .filter_map(|i| other_on_series.get_value(i).map(|val| (val, i)))
                    .fold(
                        indexmap::IndexMap::new,
                        |mut map: indexmap::IndexMap<Value, Vec<usize>>, (val, i)| {
                            map.entry(val).or_default().push(i);
                            map
                        },
                    )
                    .reduce(indexmap::IndexMap::new, |mut acc, map| {
                        for (key, value) in map {
                            acc.entry(key).or_default().extend(value);
                        }
                        acc
                    });

                // Keep track of matched right indices to handle the "Right Anti" part later
                // Using a thread-safe structure or collecting matched indices
                // Since we are doing parallel processing, collecting all matched indices might be expensive?
                // Alternative: Use a DashSet or concurrent bitset?
                // Or just collect locally and merge.

                // We'll process Left Join part and collect matched right indices
                let self_on_series = self.get_column(on_column).unwrap();

                // Process Left side (Left Outer Join)
                let collected_results: Vec<(Vec<Vec<(String, Option<Value>)>>, Vec<usize>)> = (0
                    ..self.row_count())
                    .into_par_iter()
                    .map(|i| {
                        let mut matched_indices = Vec::new();
                        let mut rows = Vec::new();

                        if let Some(self_join_val) = self_on_series.get_value(i) {
                            if let Some(other_indices) = other_join_map.get(&self_join_val) {
                                matched_indices.extend(other_indices.iter().cloned());
                                for &other_idx in other_indices {
                                    let mut row_values = Vec::new();
                                    for col_name in &all_column_names {
                                        let value = if self_col_names.contains(col_name) {
                                            self.get_column(col_name).unwrap().get_value(i)
                                        } else {
                                            other.get_column(col_name).unwrap().get_value(other_idx)
                                        };
                                        row_values.push((col_name.clone(), value));
                                    }
                                    rows.push(row_values);
                                }
                            } else {
                                // No match in right, emit Left row with nulls
                                let mut row_values = Vec::new();
                                for col_name in &all_column_names {
                                    let value = if self_col_names.contains(col_name) {
                                        self.get_column(col_name).unwrap().get_value(i)
                                    } else {
                                        None
                                    };
                                    row_values.push((col_name.clone(), value));
                                }
                                rows.push(row_values);
                            }
                        } else {
                            // Null key in left, emit Left row with nulls (assuming null != null for join usually, standard SQL behavior)
                            // If we want null=null matching, we'd handle it above. Here we assume strict equality.
                            let mut row_values = Vec::new();
                            for col_name in &all_column_names {
                                let value = if self_col_names.contains(col_name) {
                                    self.get_column(col_name).unwrap().get_value(i)
                                } else {
                                    None
                                };
                                row_values.push((col_name.clone(), value));
                            }
                            rows.push(row_values);
                        }
                        (rows, matched_indices)
                    })
                    .collect::<Vec<_>>();

                // Sequential post-processing
                let mut collected_rows = Vec::new();
                let mut matched_right_set = std::collections::HashSet::new();

                for (rows, indices) in collected_results {
                    collected_rows.extend(rows);
                    matched_right_set.extend(indices);
                }

                // Append Right rows that were not matched
                for i in 0..other.row_count() {
                    if !matched_right_set.contains(&i) {
                        let mut row_values = Vec::new();
                        for col_name in &all_column_names {
                            let value = if other_col_names.contains(col_name) {
                                other.get_column(col_name).unwrap().get_value(i)
                            } else {
                                None // Not in Right (so it's a Left column), set to Null
                            };
                            row_values.push((col_name.clone(), value));
                        }
                        // Add to collected_rows? No, we can directly push to series_data or append to collected_rows
                        // Pushing to series_data is non-trivial because collected_rows is waiting to be pushed.
                        // Better to push everything to series_data at once or append here.
                        // collected_rows is consumed below.
                        // We can't append to collected_rows easily because it's Vec of Vec.
                        // Actually we can.
                        // But wait, `collected_rows` is immutable from the unzip. We need to make it mutable or chain.
                        // Let's just process this loop into series_data directly?
                        // No, consistent processing is better.

                        // Let's add these to a separate vector and process both.
                    }
                }

                // Actually, let's gather right-only rows separately
                let right_only_rows: Vec<Vec<(String, Option<Value>)>> = (0..other.row_count())
                    .into_par_iter()
                    .filter(|i| !matched_right_set.contains(i))
                    .map(|i| {
                        let mut row_values = Vec::new();
                        for col_name in &all_column_names {
                            let value = if other_col_names.contains(col_name) {
                                other.get_column(col_name).unwrap().get_value(i)
                            } else {
                                None
                            };
                            row_values.push((col_name.clone(), value));
                        }
                        row_values
                    })
                    .collect();

                // Populate series_data
                for row_values in collected_rows {
                    for (col_name, value) in row_values {
                        series_data.get_mut(&col_name).unwrap().push(value);
                    }
                }
                for row_values in right_only_rows {
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

        Ok(DataFrame::new(new_columns))
    }
}
