use crate::error::VeloxxError;
use crate::{
    conditions::Condition,
    dataframe::DataFrame,
    expressions::Expr,
    series::Series,
    types::{DataType, Value},
};
use std::collections::BTreeMap;

impl DataFrame {
    /// Selects a subset of columns from the `DataFrame`.
    ///
    /// This method creates a new `DataFrame` containing only the columns specified
    /// in the `names` vector. The order of columns in the new DataFrame will match
    /// the order provided in `names`.
    ///
    /// # Arguments
    ///
    /// * `names` - A `Vec<String>` containing the names of the columns to select.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with only the selected columns,
    /// or `Err(VeloxxError::ColumnNotFound)` if any of the specified column names do not exist in the DataFrame.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("A".to_string(), Series::new_i32("A", vec![Some(1), Some(2)]));
    /// columns.insert("B".to_string(), Series::new_f64("B", vec![Some(1.1), Some(2.2)]));
    /// columns.insert("C".to_string(), Series::new_string("C", vec![Some("x".to_string()), Some("y".to_string())]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let selected_df = df.select_columns(vec!["A".to_string(), "C".to_string()]).unwrap();
    /// assert_eq!(selected_df.column_count(), 2);
    /// assert_eq!(selected_df.column_names(), vec![&"A".to_string(), &"C".to_string()]);
    /// ```
    pub fn select_columns(&self, names: Vec<String>) -> Result<Self, VeloxxError> {
        let mut selected_columns = BTreeMap::new();
        for name in names {
            if let Some(series) = self.columns.get(&name) {
                selected_columns.insert(name, series.clone());
            } else {
                return Err(VeloxxError::ColumnNotFound(name));
            }
        }
        DataFrame::new(selected_columns)
    }

    /// Drops specified columns from the `DataFrame`.
    ///
    /// This method creates a new `DataFrame` with the specified columns removed.
    /// The original DataFrame remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `names` - A `Vec<String>` containing the names of the columns to drop.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` without the dropped columns,
    /// or `Err(VeloxxError::ColumnNotFound)` if any of the specified column names do not exist in the DataFrame.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("A".to_string(), Series::new_i32("A", vec![Some(1), Some(2)]));
    /// columns.insert("B".to_string(), Series::new_f64("B", vec![Some(1.1), Some(2.2)]));
    /// columns.insert("C".to_string(), Series::new_string("C", vec![Some("x".to_string()), Some("y".to_string())]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let dropped_df = df.drop_columns(vec!["B".to_string()]).unwrap();
    /// assert_eq!(dropped_df.column_count(), 2);
    /// assert!(!dropped_df.column_names().contains(&&"B".to_string()));
    /// ```
    pub fn drop_columns(&self, names: Vec<String>) -> Result<Self, VeloxxError> {
        let mut new_columns: BTreeMap<String, Series> = self.columns.clone();
        for name in names {
            if new_columns.remove(&name).is_none() {
                return Err(VeloxxError::ColumnNotFound(name));
            }
        }
        DataFrame::new(new_columns)
    }

    /// Renames a column in the `DataFrame`.
    ///
    /// This method creates a new `DataFrame` with the specified column renamed.
    /// The original DataFrame remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `old_name` - The current name of the column to rename.
    /// * `new_name` - The new name for the column.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with the column renamed,
    /// or `Err(VeloxxError::ColumnNotFound)` if the `old_name` does not exist,
    /// or `Err(VeloxxError::InvalidOperation)` if the `new_name` already exists in the DataFrame.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("A".to_string(), Series::new_i32("A", vec![Some(1), Some(2)]));
    /// columns.insert("B".to_string(), Series::new_f64("B", vec![Some(1.1), Some(2.2)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let renamed_df = df.rename_column("A", "Alpha").unwrap();
    /// assert!(renamed_df.column_names().contains(&&"Alpha".to_string()));
    /// assert!(!renamed_df.column_names().contains(&&"A".to_string()));
    /// ```
    pub fn rename_column(&self, old_name: &str, new_name: &str) -> Result<Self, VeloxxError> {
        let mut new_columns: BTreeMap<String, Series> = self.columns.clone();
        if let Some(mut series) = new_columns.remove(old_name) {
            if new_columns.contains_key(new_name) {
                return Err(VeloxxError::InvalidOperation(format!(
                    "Column with new name '{new_name}' already exists."
                )));
            }
            series.set_name(new_name);
            new_columns.insert(new_name.to_string(), series);
            DataFrame::new(new_columns)
        } else {
            Err(VeloxxError::ColumnNotFound(old_name.to_string()))
        }
    }

    /// Sorts the `DataFrame` by one or more columns.
    ///
    /// This method creates a new `DataFrame` with rows sorted according to the values
    /// in the specified `by_columns`. Sorting is performed lexicographically for strings,
    /// numerically for numbers, and chronologically for DateTime values. Null values
    /// are always sorted first.
    ///
    /// # Arguments
    ///
    /// * `by_columns` - A `Vec<String>` containing the names of the columns to sort by.
    ///   The order of column names in this vector determines the primary, secondary, etc., sort keys.
    /// * `ascending` - A boolean indicating whether to sort in ascending order (`true`) or
    ///   descending order (`false`). This applies to all `by_columns`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new sorted `DataFrame`,
    /// or `Err(VeloxxError::ColumnNotFound)` if any of the `by_columns` do not exist.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    /// use veloxx::types::Value;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("name".to_string(), Series::new_string("name", vec![Some("Bob".to_string()), Some("Alice".to_string()), Some("Charlie".to_string())]));
    /// columns.insert("age".to_string(), Series::new_i32("age", vec![Some(25), Some(30), Some(20)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// // Sort by 'age' in ascending order
    /// let sorted_df_age_asc = df.sort(vec!["age".to_string()], true).unwrap();
    /// assert_eq!(sorted_df_age_asc.get_column("name").unwrap().get_value(0), Some(Value::String("Charlie".to_string())));
    ///
    /// // Sort by 'name' in descending order
    /// let sorted_df_name_desc = df.sort(vec!["name".to_string()], false).unwrap();
    /// assert_eq!(sorted_df_name_desc.get_column("name").unwrap().get_value(0), Some(Value::String("Charlie".to_string())));
    /// ```
    pub fn sort(&self, by_columns: Vec<String>, ascending: bool) -> Result<Self, VeloxxError> {
        if self.row_count == 0 {
            return Ok(self.clone());
        }

        let mut rows: Vec<Vec<Option<Value>>> = Vec::with_capacity(self.row_count);
        for i in 0..self.row_count {
            let mut row: Vec<Option<Value>> = Vec::with_capacity(self.column_count());
            for col_name in self.column_names().iter() {
                let series = self.columns.get(*col_name).unwrap();
                row.push(series.get_value(i));
            }
            rows.push(row);
        }

        let column_indices: Result<Vec<usize>, VeloxxError> = by_columns
            .iter()
            .map(|col_name| {
                self.column_names()
                    .iter()
                    .position(|&name| name == col_name)
                    .ok_or(VeloxxError::ColumnNotFound(format!(
                        "Column '{col_name}' not found for sorting."
                    )))
            })
            .collect();

        let column_indices = column_indices?;

        rows.sort_by(|a, b| {
            for &col_idx in column_indices.iter() {
                let val_a = &a[col_idx];
                let val_b = &b[col_idx];

                let cmp = match (val_a, val_b) {
                    (Some(Value::I32(v_a)), Some(Value::I32(v_b))) => v_a.cmp(v_b),
                    (Some(Value::F64(v_a)), Some(Value::F64(v_b))) => {
                        v_a.partial_cmp(v_b).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (Some(Value::Bool(v_a)), Some(Value::Bool(v_b))) => v_a.cmp(v_b),
                    (Some(Value::String(v_a)), Some(Value::String(v_b))) => v_a.cmp(v_b),
                    (Some(Value::DateTime(v_a)), Some(Value::DateTime(v_b))) => v_a.cmp(v_b),
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less, // Nulls come first
                    (Some(_), None) => std::cmp::Ordering::Greater, // Non-nulls come after nulls
                    _ => panic!("Mismatched types during comparison for sorting."),
                };

                if cmp != std::cmp::Ordering::Equal {
                    return if ascending { cmp } else { cmp.reverse() };
                }
            }
            std::cmp::Ordering::Equal
        });

        let mut new_columns_data: BTreeMap<String, Vec<Option<Value>>> = BTreeMap::new();
        for col_name in self.column_names().iter() {
            new_columns_data.insert((*col_name).clone(), Vec::with_capacity(self.row_count));
        }

        for row in rows {
            for (col_idx, col_name) in self.column_names().iter().enumerate() {
                new_columns_data
                    .get_mut(*col_name)
                    .unwrap()
                    .push(row[col_idx].clone());
            }
        }

        let mut new_series_map: BTreeMap<String, Series> = BTreeMap::new();
        for (col_name, data_vec) in new_columns_data {
            let original_series = self.columns.get(&col_name).unwrap();
            let new_series = match original_series.data_type() {
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
            new_series_map.insert(col_name, new_series);
        }

        DataFrame::new(new_series_map)
    }

    /// Adds a new column to the `DataFrame` based on an expression.
    ///
    /// This method evaluates the provided `Expr` for each row in the DataFrame
    /// and creates a new `Series` with the results. This new Series is then added
    /// to a new `DataFrame`.
    ///
    /// # Arguments
    ///
    /// * `new_col_name` - The name of the new column to be added.
    /// * `expr` - The `Expr` defining how to compute the values for the new column.
    ///   The expression will be evaluated for each row of the DataFrame.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with the added column,
    /// or `Err(VeloxxError::InvalidOperation)` if a column with `new_col_name` already exists,
    /// or `Err(VeloxxError)` if the expression cannot be evaluated for any row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::expressions::Expr;
    /// use veloxx::types::Value;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("a".to_string(), Series::new_i32("a", vec![Some(2), Some(3)]));
    /// columns.insert("b".to_string(), Series::new_i32("b", vec![Some(4), Some(5)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// // Calculate 'product' as 'a' * 'b' (supported operation)
    /// let product_expr = Expr::Multiply(
    ///     Box::new(Expr::Column("a".to_string())),
    ///     Box::new(Expr::Column("b".to_string())),
    /// );
    /// let df_with_product = df.with_column("product", &product_expr).unwrap(); // Should succeed
    /// assert_eq!(df_with_product.column_count(), 3);
    /// assert_eq!(df_with_product.get_column("product").unwrap().get_value(0), Some(Value::I32(8)));
    /// assert_eq!(df_with_product.get_column("product").unwrap().get_value(1), Some(Value::I32(15)));
    ///
    /// // Attempt to calculate 'bad' as 'a' * 2.0 (unsupported operation)
    /// let result = df.with_column("bad", &Expr::Multiply(
    ///     Box::new(Expr::Column("a".to_string())),
    ///     Box::new(Expr::Literal(Value::F64(2.0))),
    /// ));
    /// assert!(result.is_err()); // Multiplication may not be supported for all types
    /// ```
    pub fn with_column(&self, new_col_name: &str, expr: &Expr) -> Result<Self, VeloxxError> {
        let mut new_columns: BTreeMap<String, Series> = self.columns.clone();
        if new_columns.contains_key(new_col_name) {
            return Err(VeloxxError::InvalidOperation(format!(
                "Column '{new_col_name}' already exists."
            )));
        }

        let mut evaluated_values: Vec<Value> = Vec::with_capacity(self.row_count);
        let mut inferred_type: Option<crate::types::DataType> = None;

        for i in 0..self.row_count {
            let evaluated_value = expr.evaluate(self, i)?;
            if inferred_type.is_none() && evaluated_value != Value::Null {
                inferred_type = Some(evaluated_value.data_type());
            }
            evaluated_values.push(evaluated_value);
        }

        let new_series = match inferred_type {
            Some(DataType::I32) => Series::new_i32(
                new_col_name,
                evaluated_values
                    .into_iter()
                    .map(|v| if let Value::I32(x) = v { Some(x) } else { None })
                    .collect(),
            ),
            Some(DataType::F64) => Series::new_f64(
                new_col_name,
                evaluated_values
                    .into_iter()
                    .map(|v| if let Value::F64(x) = v { Some(x) } else { None })
                    .collect(),
            ),
            Some(DataType::Bool) => Series::new_bool(
                new_col_name,
                evaluated_values
                    .into_iter()
                    .map(|v| {
                        if let Value::Bool(x) = v {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
            Some(DataType::String) => Series::new_string(
                new_col_name,
                evaluated_values
                    .into_iter()
                    .map(|v| {
                        if let Value::String(x) = v {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
            Some(DataType::DateTime) => Series::new_datetime(
                new_col_name,
                evaluated_values
                    .into_iter()
                    .map(|v| {
                        if let Value::DateTime(x) = v {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
            None => Series::new_string(new_col_name, vec![None; self.row_count]), // All nulls, default to String
        };

        new_columns.insert(new_col_name.to_string(), new_series);
        DataFrame::new(new_columns)
    }

    /// Filters the `DataFrame` based on a given condition.
    ///
    /// This method evaluates the provided `Condition` for each row. Only rows for which
    /// the condition evaluates to `true` are included in the new `DataFrame`.
    ///
    /// # Arguments
    ///
    /// * `condition` - The `Condition` to apply for filtering rows.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with only the rows
    /// that satisfy the condition, or `Err(VeloxxError)` if the condition cannot be evaluated
    /// (e.g., due to a missing column or type mismatch).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::conditions::Condition;
    /// use veloxx::types::Value;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("age".to_string(), Series::new_i32("age", vec![Some(10), Some(20), Some(30)]));
    /// columns.insert("city".to_string(), Series::new_string("city", vec![Some("NY".to_string()), Some("LA".to_string()), Some("NY".to_string())]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// // Filter where age > 15
    /// let condition = Condition::Gt("age".to_string(), Value::I32(15));
    /// let filtered_df = df.filter(&condition).unwrap();
    /// assert_eq!(filtered_df.row_count(), 2);
    /// assert_eq!(filtered_df.get_column("age").unwrap().get_value(0), Some(Value::I32(20)));
    /// ```
    pub fn filter(&self, condition: &Condition) -> Result<Self, VeloxxError> {
        let mut row_indices_to_keep: Vec<usize> = Vec::new();

        for i in 0..self.row_count {
            if condition.evaluate(self, i)? {
                row_indices_to_keep.push(i);
            }
        }
        self.filter_by_indices(&row_indices_to_keep)
    }

    /// Filters the `DataFrame` based on a list of row indices.
    ///
    /// This is a lower-level filtering method that directly takes a slice of row indices.
    /// It creates a new `DataFrame` containing only the rows at the specified indices.
    /// The order of rows in the new DataFrame will match the order of `row_indices`.
    ///
    /// # Arguments
    ///
    /// * `row_indices` - A slice of `usize` containing the indices of the rows to keep.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with only the specified rows,
    /// or `Err(VeloxxError)` if an error occurs during series filtering (e.g., an index is out of bounds).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    /// use veloxx::types::Value;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("data".to_string(), Series::new_i32("data", vec![Some(10), Some(20), Some(30), Some(40)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let indices = vec![0, 2];
    /// let filtered_df = df.filter_by_indices(&indices).unwrap();
    /// assert_eq!(filtered_df.row_count(), 2);
    /// assert_eq!(filtered_df.get_column("data").unwrap().get_value(0), Some(Value::I32(10)));
    /// assert_eq!(filtered_df.get_column("data").unwrap().get_value(1), Some(Value::I32(30)));
    /// ```
    pub fn filter_by_indices(&self, row_indices: &[usize]) -> Result<Self, VeloxxError> {
        if row_indices.is_empty() {
            return Ok(DataFrame {
                columns: BTreeMap::new(),
                row_count: 0,
            });
        }

        let mut new_columns: BTreeMap<String, Series> = BTreeMap::new();
        for (col_name, series) in self.columns.iter() {
            let new_series = series.filter(row_indices)?;
            new_columns.insert(col_name.clone(), new_series);
        }

        DataFrame::new(new_columns)
    }

    /// Appends another `DataFrame` to the end of this `DataFrame`.
    ///
    /// This method concatenates the rows of `other` DataFrame to the end of the current DataFrame.
    /// For a successful append, both DataFrames must have:
    /// - The same number of columns.
    /// - Identical column names (case-sensitive).
    /// - Matching data types for each corresponding column.
    ///   The order of columns in both DataFrames is also important.
    ///
    /// # Arguments
    ///
    /// * `other` - The `DataFrame` to append.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with rows from both DataFrames,
    /// or `Err(VeloxxError::InvalidOperation)` if column counts, names, or order mismatch,
    /// or `Err(VeloxxError::DataTypeMismatch)` if corresponding columns have different data types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    /// use veloxx::types::Value;
    ///
    /// let mut df1_cols = BTreeMap::new();
    /// df1_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2)]));
    /// df1_cols.insert("value".to_string(), Series::new_f64("value", vec![Some(10.0), Some(20.0)]));
    /// let df1 = DataFrame::new(df1_cols).unwrap();
    ///
    /// let mut df2_cols = BTreeMap::new();
    /// df2_cols.insert("id".to_string(), Series::new_i32("id", vec![Some(3), Some(4)]));
    /// df2_cols.insert("value".to_string(), Series::new_f64("value", vec![Some(30.0), Some(40.0)]));
    /// let df2 = DataFrame::new(df2_cols).unwrap();
    ///
    /// let appended_df = df1.append(&df2).unwrap();
    /// assert_eq!(appended_df.row_count(), 4);
    /// assert_eq!(appended_df.get_column("id").unwrap().get_value(2), Some(Value::I32(3)));
    /// assert_eq!(appended_df.get_column("value").unwrap().get_value(3), Some(Value::F64(40.0)));
    /// ```
    pub fn append(&self, other: &DataFrame) -> Result<Self, VeloxxError> {
        if self.column_count() != other.column_count() {
            return Err(VeloxxError::InvalidOperation(
                "Cannot append DataFrames with different number of columns.".to_string(),
            ));
        }

        let self_column_names: Vec<&String> = self.column_names();
        let other_column_names: Vec<&String> = other.column_names();

        // Check if column names and order are identical
        for i in 0..self_column_names.len() {
            if self_column_names[i] != other_column_names[i] {
                return Err(VeloxxError::InvalidOperation(
                    "Cannot append DataFrames with different column names or order.".to_string(),
                ));
            }
            if self.get_column(self_column_names[i]).unwrap().data_type()
                != other.get_column(other_column_names[i]).unwrap().data_type()
            {
                return Err(VeloxxError::DataTypeMismatch(format!(
                    "Cannot append DataFrames with mismatched data types for column '{}'.",
                    self_column_names[i]
                )));
            }
        }

        let mut new_columns: BTreeMap<String, Series> = BTreeMap::new();
        for col_name in self_column_names.into_iter() {
            let self_series = self.get_column(col_name).unwrap();
            let other_series = other.get_column(col_name).unwrap();
            let appended_series = self_series.append(other_series)?;
            new_columns.insert(col_name.clone(), appended_series);
        }

        DataFrame::new(new_columns)
    }

    /// Groups the `DataFrame` by one or more columns.
    ///
    /// This method creates a `GroupedDataFrame` object, which can then be used to perform
    /// aggregation operations on the grouped data. The grouping is based on unique combinations
    /// of values in the specified `group_columns`.
    ///
    /// # Arguments
    ///
    /// * `group_columns` - A `Vec<String>` containing the names of the columns to group by.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(GroupedDataFrame)` if the grouping is successful,
    /// or `Err(VeloxxError::ColumnNotFound)` if any of the `group_columns` do not exist.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("city".to_string(), Series::new_string("city", vec![Some("New York".to_string()), Some("London".to_string()), Some("New York".to_string())]));
    /// columns.insert("sales".to_string(), Series::new_f64("sales", vec![Some(100.0), Some(150.0), Some(200.0)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let grouped_df = df.group_by(vec!["city".to_string()]).unwrap();
    /// // `grouped_df` can now be used with the `.agg()` method.
    /// ```
    pub fn group_by(
        &self,
        group_columns: Vec<String>,
    ) -> Result<crate::dataframe::group_by::GroupedDataFrame<'_>, VeloxxError> {
        crate::dataframe::group_by::GroupedDataFrame::new(self, group_columns)
    }

    /// Generates descriptive statistics for the `DataFrame`.
    ///
    /// This method calculates various statistical measures for each column in the DataFrame.
    /// For numeric columns (`I32`, `F64`, `DateTime`), it computes count, mean, standard deviation,
    /// minimum, maximum, and median. For non-numeric columns (`Bool`, `String`), only the count
    /// of non-null values is provided.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` where each row represents
    /// a statistical measure (e.g., "count", "mean"), and each column represents an original
    /// column from the input DataFrame. Returns `Err(VeloxxError)` if any statistical calculation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("age".to_string(), Series::new_i32("age", vec![Some(20), Some(30), Some(25), None, Some(35)]));
    /// columns.insert("city".to_string(), Series::new_string("city", vec![Some("NY".to_string()), Some("LA".to_string()), Some("NY".to_string()), Some("SF".to_string()), None]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let description_df = df.describe().unwrap();
    /// println!("Descriptive Statistics:\n{}", description_df);
    /// // Expected output (column order might vary):
    /// // column         count          mean           std            min            max            median         
    /// // --------------- --------------- --------------- --------------- --------------- --------------- ---------------
    /// // age            4              27.50          6.45           Value::I32(20) Value::I32(35) Value::F64(27.50)
    /// // city           4              null           null           null           null           null           
    /// ```
    pub fn describe(&self) -> Result<DataFrame, VeloxxError> {
        let mut descriptions: BTreeMap<String, Series> = BTreeMap::new();
        let mut counts: Vec<Option<i32>> = Vec::new();
        let mut means: Vec<Option<f64>> = Vec::new();
        let mut std_devs: Vec<Option<f64>> = Vec::new();
        let mut mins: Vec<Option<Value>> = Vec::new();
        let mut maxs: Vec<Option<Value>> = Vec::new();
        let mut medians: Vec<Option<Value>> = Vec::new();

        let mut column_names_vec: Vec<String> = Vec::new();

        for (col_name, series) in self.columns.iter() {
            column_names_vec.push(col_name.clone());
            counts.push(Some(series.count() as i32));

            match series.data_type() {
                crate::types::DataType::I32
                | crate::types::DataType::F64
                | crate::types::DataType::DateTime => {
                    means.push(series.mean()?.and_then(|v| {
                        if let Value::F64(val) = v {
                            Some(val)
                        } else {
                            None
                        }
                    }));
                    std_devs.push(series.std_dev()?.and_then(|v| {
                        if let Value::F64(val) = v {
                            Some(val)
                        } else {
                            None
                        }
                    }));
                    mins.push(series.min()?);
                    maxs.push(series.max()?);
                    medians.push(series.median()?);
                }
                _ => {
                    means.push(None);
                    std_devs.push(None);
                    mins.push(None);
                    maxs.push(None);
                    medians.push(None);
                }
            }
        }

        descriptions.insert(
            "column".to_string(),
            Series::new_string("column", column_names_vec.into_iter().map(Some).collect()),
        );
        descriptions.insert("count".to_string(), Series::new_i32("count", counts));
        descriptions.insert("mean".to_string(), Series::new_f64("mean", means));
        descriptions.insert("std".to_string(), Series::new_f64("std", std_devs));
        descriptions.insert(
            "min".to_string(),
            Series::new_string(
                "min",
                mins.into_iter()
                    .map(|x| x.map(|v| format!("{v:?}")))
                    .collect(),
            ),
        );
        descriptions.insert(
            "max".to_string(),
            Series::new_string(
                "max",
                maxs.into_iter()
                    .map(|x| x.map(|v| format!("{v:?}")))
                    .collect(),
            ),
        );
        descriptions.insert(
            "median".to_string(),
            Series::new_string(
                "median",
                medians
                    .into_iter()
                    .map(|x| x.map(|v| format!("{v:?}")))
                    .collect(),
            ),
        );

        DataFrame::new(descriptions)
    }

    /// Calculates the Pearson correlation coefficient between two columns in the `DataFrame`.
    ///
    /// This method computes the Pearson correlation coefficient, which measures the linear
    /// relationship between two sets of data. Both columns must be numeric (`I32` or `F64`).
    /// Null values are handled by pairwise deletion (rows with nulls in either column are excluded).
    ///
    /// # Arguments
    ///
    /// * `col1_name` - The name of the first numeric column.
    /// * `col2_name` - The name of the second numeric column.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(f64)` containing the correlation coefficient,
    /// or `Err(VeloxxError::ColumnNotFound)` if either column does not exist,
    /// or `Err(VeloxxError::InvalidOperation)` if columns have different numbers of non-null values
    /// or fewer than 2 non-null values, or `Err(VeloxxError::Unsupported)` if columns are not numeric.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("X".to_string(), Series::new_i32("X", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]));
    /// columns.insert("Y".to_string(), Series::new_f64("Y", vec![Some(2.0), Some(4.0), Some(5.0), Some(4.0), Some(5.0)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let correlation = df.correlation("X", "Y").unwrap();
    /// println!("actual correlation: {}", correlation);
    /// // Expected correlation for these values is approx 0.7746
    /// assert!((correlation - 0.7746).abs() < 0.0001);
    ///
    /// let mut cols_with_nulls = BTreeMap::new();
    /// cols_with_nulls.insert("A".to_string(), Series::new_i32("A", vec![Some(1), None, Some(3)]));
    /// cols_with_nulls.insert("B".to_string(), Series::new_i32("B", vec![Some(10), Some(20), None]));
    /// let df_nulls = DataFrame::new(cols_with_nulls).unwrap();
    /// // Print the result for documentation; behavior may depend on implementation
    /// let result = df_nulls.correlation("A", "B");
    /// println!("correlation with nulls: {:?}", result);
    /// ```
    pub fn correlation(&self, col1_name: &str, col2_name: &str) -> Result<f64, VeloxxError> {
        let series1 = self
            .get_column(col1_name)
            .ok_or(VeloxxError::ColumnNotFound(col1_name.to_string()))?;
        let series2 = self
            .get_column(col2_name)
            .ok_or(VeloxxError::ColumnNotFound(col2_name.to_string()))?;

        let data1: Vec<f64> = series1.to_vec_f64()?;
        let data2: Vec<f64> = series2.to_vec_f64()?;

        if data1.len() != data2.len() {
            return Err(VeloxxError::InvalidOperation(
                "Columns must have the same number of non-null values for correlation.".to_string(),
            ));
        }

        let n = data1.len();
        if n == 0 {
            return Err(VeloxxError::InvalidOperation(
                "Cannot compute correlation for empty columns.".to_string(),
            ));
        }

        let mean1 = data1.iter().sum::<f64>() / n as f64;
        let mean2 = data2.iter().sum::<f64>() / n as f64;

        let mut numerator = 0.0;
        let mut sum_sq_diff1 = 0.0;
        let mut sum_sq_diff2 = 0.0;

        for i in 0..n {
            let diff1 = data1[i] - mean1;
            let diff2 = data2[i] - mean2;
            numerator += diff1 * diff2;
            sum_sq_diff1 += diff1.powi(2);
            sum_sq_diff2 += diff2.powi(2);
        }

        let denominator = (sum_sq_diff1 * sum_sq_diff2).sqrt();

        if denominator == 0.0 {
            Ok(0.0) // Handle cases where one or both series have zero variance
        } else {
            Ok(numerator / denominator)
        }
    }

    /// Calculates the covariance between two columns in the `DataFrame`.
    ///
    /// This method computes the covariance, which measures how two variables change together.
    /// Both columns must be numeric (`I32` or `F64`). Null values are handled by pairwise deletion.
    ///
    /// # Arguments
    ///
    /// * `col1_name` - The name of the first numeric column.
    /// * `col2_name` - The name of the second numeric column.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(f64)` containing the covariance,
    /// or `Err(VeloxxError::ColumnNotFound)` if either column does not exist,
    /// or `Err(VeloxxError::InvalidOperation)` if columns have different numbers of non-null values
    /// or fewer than 2 non-null values, or `Err(VeloxxError::Unsupported)` if columns are not numeric.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("X".to_string(), Series::new_i32("X", vec![Some(1), Some(2), Some(3)]));
    /// columns.insert("Y".to_string(), Series::new_f64("Y", vec![Some(2.0), Some(3.0), Some(4.0)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let covariance = df.covariance("X", "Y").unwrap();
    /// // Expected covariance for these values is 1.0
    /// assert!((covariance - 1.0).abs() < 0.0001);
    /// ```
    pub fn covariance(&self, col1_name: &str, col2_name: &str) -> Result<f64, VeloxxError> {
        let series1 = self
            .get_column(col1_name)
            .ok_or(VeloxxError::ColumnNotFound(col1_name.to_string()))?;
        let series2 = self
            .get_column(col2_name)
            .ok_or(VeloxxError::ColumnNotFound(col2_name.to_string()))?;

        let data1: Vec<f64> = series1.to_vec_f64()?;
        let data2: Vec<f64> = series2.to_vec_f64()?;

        if data1.len() != data2.len() {
            return Err(VeloxxError::InvalidOperation(
                "Columns must have the same number of non-null values for covariance.".to_string(),
            ));
        }

        let n = data1.len();
        if n < 2 {
            return Err(VeloxxError::InvalidOperation(
                "Cannot compute covariance for columns with less than 2 non-null values."
                    .to_string(),
            ));
        }

        let mean1 = data1.iter().sum::<f64>() / n as f64;
        let mean2 = data2.iter().sum::<f64>() / n as f64;

        let mut sum_products = 0.0;
        for i in 0..n {
            sum_products += (data1[i] - mean1) * (data2[i] - mean2);
        }

        Ok(sum_products / (n - 1) as f64)
    }

    /// Converts the `DataFrame` into a `Vec<Vec<Option<Value>>>`.
    ///
    /// This method transforms the tabular data of the `DataFrame` into a nested vector
    /// structure, where the outer `Vec` represents rows and the inner `Vec` represents
    /// the values within each row. Each cell value is wrapped in an `Option<Value>`,
    /// allowing for the representation of nulls.
    ///
    /// # Returns
    ///
    /// A `Vec<Vec<Option<Value>>>` representation of the `DataFrame`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    /// use veloxx::types::Value;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("A".to_string(), Series::new_i32("A", vec![Some(1), Some(2)]));
    /// columns.insert("B".to_string(), Series::new_string("B", vec![Some("x".to_string()), None]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let vec_of_vec = df.to_vec_of_vec();
    ///
    /// // Note: Column order in the inner Vec<Option<Value>> depends on BTreeMap iteration order.
    /// // For consistent testing, you might need to sort columns or access by index if order is known.
    /// assert_eq!(vec_of_vec.len(), 2);
    /// // Example for accessing a specific value (assuming "A" is first, "B" is second)
    /// // assert_eq!(vec_of_vec[0][0], Some(Value::I32(1)));
    /// // assert_eq!(vec_of_vec[1][1], None);
    /// ```
    pub fn to_vec_of_vec(&self) -> Vec<Vec<Option<Value>>> {
        let mut result: Vec<Vec<Option<Value>>> = Vec::with_capacity(self.row_count);
        let column_names = self.column_names();

        for i in 0..self.row_count {
            let mut row: Vec<Option<Value>> = Vec::with_capacity(self.column_count());
            for col_name in column_names.iter() {
                let series = self.columns.get(*col_name).unwrap();
                row.push(series.get_value(i));
            }
            result.push(row);
        }
        result
    }
}
