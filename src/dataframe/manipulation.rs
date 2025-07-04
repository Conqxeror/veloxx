use crate::{
    conditions::Condition, dataframe::DataFrame, expressions::Expr, series::Series, types::{DataType, Value},
};
use std::collections::BTreeMap;

impl DataFrame {
    /// Selects a subset of columns from the `DataFrame`.
    ///
    /// # Arguments
    /// * `names` - A `Vec<String>` containing the names of the columns to select.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with only the selected columns, or a `String` error message if a column is not found.
    pub fn select_columns(&self, names: Vec<String>) -> Result<Self, String> {
        let mut selected_columns = BTreeMap::new();
        for name in names {
            if let Some(series) = self.columns.get(&name) {
                selected_columns.insert(name, series.clone());
            } else {
                return Err(format!("Column '{name}' not found."));
            }
        }
        DataFrame::new(selected_columns)
    }

    /// Drops specified columns from the `DataFrame`.
    ///
    /// # Arguments
    /// * `names` - A `Vec<String>` containing the names of the columns to drop.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` without the dropped columns, or a `String` error message if a column is not found.
    pub fn drop_columns(&self, names: Vec<String>) -> Result<Self, String> {
        let mut new_columns: BTreeMap<String, Series> = self.columns.clone();
        for name in names {
            if new_columns.remove(&name).is_none() {
                return Err(format!("Column '{name}' not found."));
            }
        }
        DataFrame::new(new_columns)
    }

    /// Renames a column in the `DataFrame`.
    ///
    /// # Arguments
    /// * `old_name` - The current name of the column.
    /// * `new_name` - The new name for the column.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with the column renamed, or a `String` error message
    /// if the old column is not found or the new name already exists.
    pub fn rename_column(&self, old_name: &str, new_name: &str) -> Result<Self, String> {
        let mut new_columns: BTreeMap<String, Series> = self.columns.clone();
        if let Some(mut series) = new_columns.remove(old_name) {
            if new_columns.contains_key(new_name) {
                return Err(format!("Column with new name '{new_name}' already exists."));
            }
            series.set_name(new_name);
            new_columns.insert(new_name.to_string(), series);
            DataFrame::new(new_columns)
        } else {
            Err(format!("Column '{old_name}' not found."))
        }
    }

    /// Sorts the `DataFrame` by one or more columns.
    ///
    /// # Arguments
    /// * `by_columns` - A `Vec<String>` containing the names of the columns to sort by.
    /// * `ascending` - A boolean indicating whether to sort in ascending order (`true`) or descending order (`false`).
    ///
    /// # Returns
    /// A `Result` containing a new sorted `DataFrame` or a `String` error message if a column is not found.
    pub fn sort(&self, by_columns: Vec<String>, ascending: bool) -> Result<Self, String> {
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

        let column_indices: Result<Vec<usize>, String> = by_columns
            .iter()
            .map(|col_name| {
                self.column_names()
                    .iter()
                    .position(|&name| name == col_name)
                    .ok_or(format!("Column '{col_name}' not found for sorting."))
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
    /// # Arguments
    /// * `new_col_name` - The name of the new column.
    /// * `expr` - The `Expr` defining how to compute the values for the new column.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with the added column, or a `String` error message
    /// if the column already exists or the expression cannot be evaluated.
    pub fn with_column(&self, new_col_name: &str, expr: &Expr) -> Result<Self, String> {
        let mut new_columns: BTreeMap<String, Series> = self.columns.clone();
        if new_columns.contains_key(new_col_name) {
            return Err(format!("Column '{new_col_name}' already exists."));
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
                    .map(|v| if let Value::Bool(x) = v { Some(x) } else { None })
                    .collect(),
            ),
            Some(DataType::String) => Series::new_string(
                new_col_name,
                evaluated_values
                    .into_iter()
                    .map(|v| if let Value::String(x) = v { Some(x) } else { None })
                    .collect(),
            ),
            Some(DataType::DateTime) => Series::new_datetime(
                new_col_name,
                evaluated_values
                    .into_iter()
                    .map(|v| if let Value::DateTime(x) = v { Some(x) } else { None })
                    .collect(),
            ),
            None => Series::new_string(new_col_name, vec![None; self.row_count]), // All nulls, default to String
        };

        new_columns.insert(new_col_name.to_string(), new_series);
        DataFrame::new(new_columns)
    }

    /// Filters the `DataFrame` based on a given condition.
    ///
    /// # Arguments
    /// * `condition` - The `Condition` to apply for filtering rows.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with only the rows that satisfy the condition, or a `String` error message.
    pub fn filter(&self, condition: &Condition) -> Result<Self, String> {
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
    /// # Arguments
    /// * `row_indices` - A slice of `usize` containing the indices of the rows to keep.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with only the specified rows, or a `String` error message.
    pub fn filter_by_indices(&self, row_indices: &[usize]) -> Result<Self, String> {
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
    /// Both DataFrames must have the same number of columns, identical column names,
    /// and matching data types for each column.
    ///
    /// # Arguments
    /// * `other` - The `DataFrame` to append.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with rows from both DataFrames, or a `String` error message.
    pub fn append(&self, other: &DataFrame) -> Result<Self, String> {
        if self.column_count() != other.column_count() {
            return Err("Cannot append DataFrames with different number of columns.".to_string());
        }

        let self_column_names: Vec<&String> = self.column_names();
        let other_column_names: Vec<&String> = other.column_names();

        // Check if column names and order are identical
        for i in 0..self_column_names.len() {
            if self_column_names[i] != other_column_names[i] {
                return Err(
                    "Cannot append DataFrames with different column names or order.".to_string(),
                );
            }
            if self.get_column(self_column_names[i]).unwrap().data_type()
                != other.get_column(other_column_names[i]).unwrap().data_type()
            {
                return Err(format!(
                    "Cannot append DataFrames with mismatched data types for column '{}'.",
                    self_column_names[i]
                ));
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
    /// # Arguments
    /// * `group_columns` - A `Vec<String>` containing the names of the columns to group by.
    ///
    /// # Returns
    /// A `Result` containing a `GroupedDataFrame` or a `String` error message.
    pub fn group_by(
        &self,
        group_columns: Vec<String>,
    ) -> Result<crate::dataframe::group_by::GroupedDataFrame, String> {
        crate::dataframe::group_by::GroupedDataFrame::new(self, group_columns)
    }

    /// Generates descriptive statistics for the `DataFrame`.
    ///
    /// For numeric columns, it calculates count, mean, standard deviation, min, max, and median.
    /// For non-numeric columns, only count is provided.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with descriptive statistics, or a `String` error message.
    pub fn describe(&self) -> Result<DataFrame, String> {
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
    /// Both columns must be numeric.
    ///
    /// # Arguments
    /// * `col1_name` - The name of the first column.
    /// * `col2_name` - The name of the second column.
    ///
    /// # Returns
    /// A `Result` containing the correlation coefficient as `f64`, or a `String` error message.
    pub fn correlation(&self, col1_name: &str, col2_name: &str) -> Result<f64, String> {
        let series1 = self
            .get_column(col1_name)
            .ok_or(format!("Column '{col1_name}' not found."))?;
        let series2 = self
            .get_column(col2_name)
            .ok_or(format!("Column '{col2_name}' not found."))?;

        let data1: Vec<f64> = series1.to_vec_f64()?;
        let data2: Vec<f64> = series2.to_vec_f64()?;

        if data1.len() != data2.len() {
            return Err(
                "Columns must have the same number of non-null values for correlation.".to_string(),
            );
        }

        let n = data1.len();
        if n == 0 {
            return Err("Cannot compute correlation for empty columns.".to_string());
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
    /// Both columns must be numeric.
    ///
    /// # Arguments
    /// * `col1_name` - The name of the first column.
    /// * `col2_name` - The name of the second column.
    ///
    /// # Returns
    /// A `Result` containing the covariance as `f64`, or a `String` error message.
    pub fn covariance(&self, col1_name: &str, col2_name: &str) -> Result<f64, String> {
        let series1 = self
            .get_column(col1_name)
            .ok_or(format!("Column '{col1_name}' not found."))?;
        let series2 = self
            .get_column(col2_name)
            .ok_or(format!("Column '{col2_name}' not found."))?;

        let data1: Vec<f64> = series1.to_vec_f64()?;
        let data2: Vec<f64> = series2.to_vec_f64()?;

        if data1.len() != data2.len() {
            return Err(
                "Columns must have the same number of non-null values for covariance.".to_string(),
            );
        }

        let n = data1.len();
        if n < 2 {
            return Err(
                "Cannot compute covariance for columns with less than 2 non-null values."
                    .to_string(),
            );
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
    /// Each inner `Vec` represents a row, and each `Option<Value>` represents a cell value.
    ///
    /// # Returns
    /// A `Vec<Vec<Option<Value>>>` representation of the `DataFrame`.
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