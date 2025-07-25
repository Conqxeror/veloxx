use crate::error::VeloxxError;
use crate::{dataframe::DataFrame, series::Series, types::Value};
use std::collections::BTreeMap;

impl DataFrame {
    /// Removes rows from the `DataFrame` that contain any null values.
    ///
    /// This method iterates through each row of the DataFrame. If any cell in a row
    /// contains a `None` (null) value, that entire row is excluded from the resulting DataFrame.
    /// A new DataFrame is returned, leaving the original unchanged.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with all rows
    /// containing nulls removed, or `Err(VeloxxError)` if an error occurs during series filtering.
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
    /// columns.insert("A".to_string(), Series::new_i32("A", vec![Some(1), None, Some(3)]));
    /// columns.insert("B".to_string(), Series::new_f64("B", vec![Some(1.1), Some(2.2), Some(3.3)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// // Row 1 (index 0): [1, 1.1]
    /// // Row 2 (index 1): [None, 2.2] - will be dropped
    /// // Row 3 (index 2): [3, 3.3]
    ///
    /// let cleaned_df = df.drop_nulls(None).unwrap();
    /// assert_eq!(cleaned_df.row_count(), 2);
    /// assert_eq!(cleaned_df.get_column("A").unwrap().get_value(0), Some(Value::I32(1)));
    /// assert_eq!(cleaned_df.get_column("A").unwrap().get_value(1), Some(Value::I32(3)));
    /// ```
    pub fn drop_nulls(&self, subset: Option<&[String]>) -> Result<Self, VeloxxError> {
        let columns_to_check: Vec<&Series> = if let Some(subset) = subset {
            subset
                .iter()
                .filter_map(|name| self.columns.get(name))
                .collect()
        } else {
            self.columns.values().collect()
        };

        let row_indices_to_keep: Vec<usize> = (0..self.row_count)
            .filter(|&i| {
                columns_to_check
                    .iter()
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
    /// This method creates a new `DataFrame` where `None` (null) values in each column
    /// are replaced by the provided `value`. The filling only occurs if the `value`'s
    /// `DataType` matches the `DataType` of the column being processed. Columns with
    /// incompatible types will remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `value` - The `Value` to use for filling nulls. Its type determines which columns are affected.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with nulls filled,
    /// or `Err(VeloxxError)` if an error occurs during series filling.
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
    /// columns.insert("A".to_string(), Series::new_i32("A", vec![Some(1), None, Some(3)]));
    /// columns.insert("B".to_string(), Series::new_string("B", vec![Some("x".to_string()), None, Some("z".to_string())]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// // Fill integer nulls with 99
    /// let filled_df_i32 = df.fill_nulls(Value::I32(99)).unwrap();
    /// assert_eq!(filled_df_i32.get_column("A").unwrap().get_value(1), Some(Value::I32(99)));
    /// assert_eq!(filled_df_i32.get_column("B").unwrap().get_value(1), None); // String column unaffected
    ///
    /// // Fill string nulls with "missing"
    /// let filled_df_string = df.fill_nulls(Value::String("missing".to_string())).unwrap();
    /// assert_eq!(filled_df_string.get_column("A").unwrap().get_value(1), None); // I32 column unaffected
    /// assert_eq!(filled_df_string.get_column("B").unwrap().get_value(1), Some(Value::String("missing".to_string())));
    /// ```
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

    /// Interpolates null values in a specific column using linear interpolation.
    ///
    /// This method performs linear interpolation on null values in the specified column.
    /// It only works on numeric columns (I32 and F64). Null values at the beginning or end
    /// of the series remain as null.
    ///
    /// # Arguments
    ///
    /// * `column_name` - The name of the column to interpolate
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with interpolated values,
    /// or `Err(VeloxxError)` if the column doesn't exist or interpolation fails.
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
    /// columns.insert("A".to_string(), Series::new_f64("A", vec![Some(1.0), None, Some(3.0)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let interpolated_df = df.interpolate_nulls("A").unwrap();
    /// assert_eq!(interpolated_df.get_column("A").unwrap().get_value(1), Some(Value::F64(2.0)));
    /// ```
    pub fn interpolate_nulls(&self, column_name: &str) -> Result<Self, VeloxxError> {
        let series = self.get_column(column_name).ok_or(VeloxxError::ColumnNotFound(column_name.to_string()))?;
        let interpolated = series.interpolate_nulls()?;
        let mut new_columns = self.columns.clone();
        new_columns.insert(column_name.to_string(), interpolated);
        DataFrame::new(new_columns)
    }
}