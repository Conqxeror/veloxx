use crate::{dataframe::DataFrame, series::Series, types::Value};
use std::collections::BTreeMap;
use crate::error::VeloxxError;

impl DataFrame {
    /// Removes rows from the `DataFrame` that contain any null values.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with null rows dropped, or a `String` error message.
    pub fn drop_nulls(&self) -> Result<Self, VeloxxError> {
        let row_indices_to_keep: Vec<usize> = (0..self.row_count)
            .filter(|&i| {
                self.columns
                    .values()
                    .all(|series| series.get_value(i).is_some())
            })
            .collect();

        let mut new_columns: BTreeMap<String, Series> = BTreeMap::new();
        for (col_name, series) in self.columns.iter() {
            let new_series = series.filter(&row_indices_to_keep)?;
            new_columns.insert(col_name.clone(), new_series);
        }

        DataFrame::new(new_columns)
    }

    /// Fills null values in the `DataFrame` with a specified `Value`.
    ///
    /// Nulls are filled only if the `Value` type matches the column's `DataType`.
    ///
    /// # Arguments
    /// * `value` - The `Value` to use for filling nulls.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with nulls filled, or a `String` error message.
    pub fn fill_nulls(&self, value: Value) -> Result<Self, VeloxxError> {
        let mut new_columns: BTreeMap<String, Series> = BTreeMap::new();

        for (col_name, series) in self.columns.iter() {
            let new_series = if series.data_type() == value.data_type() {
                series.fill_nulls(&value)?
            } else {
                series.clone()
            };
            new_columns.insert(col_name.clone(), new_series);
        }

        DataFrame::new(new_columns)
    }
}