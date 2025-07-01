use crate::{dataframe::DataFrame, series::Series, types::Value};
use std::collections::BTreeMap;

impl DataFrame {
    /// Removes rows from the `DataFrame` that contain any null values.
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with null rows dropped, or a `String` error message.
    pub fn drop_nulls(&self) -> Result<Self, String> {
        let mut row_indices_to_keep: Vec<usize> = Vec::new();

        for i in 0..self.row_count {
            let mut has_null = false;
            for (_, series) in self.columns.iter() {
                if series.get_value(i).is_none() {
                    has_null = true;
                    break;
                }
            }
            if !has_null {
                row_indices_to_keep.push(i);
            }
        }

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
    pub fn fill_nulls(&self, value: Value) -> Result<Self, String> {
        let mut new_columns: BTreeMap<String, Series> = BTreeMap::new();

        for (col_name, series) in self.columns.iter() {
            let new_series = match (series.data_type(), &value) {
                (crate::types::DataType::I32, Value::I32(_)) |
                (crate::types::DataType::F64, Value::F64(_)) |
                (crate::types::DataType::Bool, Value::Bool(_)) |
                (crate::types::DataType::String, Value::String(_)) => {
                    series.fill_nulls(&value)?
                },
                _ => {
                    // If types don't match, just clone the original series
                    series.clone()
                }
            };
            new_columns.insert(col_name.clone(), new_series);
        }

        DataFrame::new(new_columns)
    }
}
