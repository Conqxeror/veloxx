use crate::dataframe::DataFrame;
use crate::error::VeloxxError;
use crate::series::Series;
use std::collections::BTreeMap;

/// A trait for types that can be converted into a `DataFrame`.
///
/// This trait provides a standardized way to create a `DataFrame` from various
/// data sources, promoting reusability and consistency in data ingestion.
pub trait DataFrameSource {
    /// Converts the implementor into a `DataFrame`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` if the conversion is successful,
    /// or `Err(VeloxxError)` if an error occurs during the conversion process.
    fn to_dataframe(&self) -> Result<DataFrame, VeloxxError>;
}

impl DataFrameSource for Vec<Vec<String>> {
    /// Converts a `Vec<Vec<String>>` into a `DataFrame`.
    ///
    /// This implementation assumes that the first inner `Vec<String>` represents
    /// the column headers (names), and subsequent inner `Vec<String>`s represent
    /// the data rows. Column types are inferred based on the values present in each column.
    /// Empty strings in the input data are treated as null values.
    ///
    /// # Arguments
    ///
    /// * `self` - A `Vec<Vec<String>>` where the first sub-vector is the header
    ///   and the rest are data rows.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` if the DataFrame is successfully created,
    /// or `Err(VeloxxError::Parsing)` if a column's type cannot be inferred,
    /// or `Err(VeloxxError::InvalidOperation)` if there are structural issues (e.g., inconsistent row lengths).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::dataframe::sources::DataFrameSource;
    /// use veloxx::types::Value;
    ///
    /// let data = vec![
    ///     vec!["id".to_string(), "name".to_string(), "age".to_string()],
    ///     vec!["1".to_string(), "Alice".to_string(), "30".to_string()],
    ///     vec!["2".to_string(), "Bob".to_string(), "".to_string()], // Empty string for null
    ///     vec!["3".to_string(), "Charlie".to_string(), "25".to_string()],
    /// ];
    ///
    /// let df = data.to_dataframe().unwrap();
    ///
    /// assert_eq!(df.row_count(), 3);
    /// assert_eq!(df.column_count(), 3);
    /// assert_eq!(df.get_column("id").unwrap().get_value(0), Some(Value::I32(1)));
    /// assert_eq!(df.get_column("age").unwrap().get_value(1), None);
    /// assert_eq!(df.get_column("name").unwrap().get_value(2), Some(Value::String("Charlie".to_string())));
    /// ```
    fn to_dataframe(&self) -> Result<DataFrame, VeloxxError> {
        if self.is_empty() {
            return DataFrame::new(BTreeMap::new());
        }

        let column_names: Vec<String> = self[0].clone(); // Assuming first row is header
        let data_rows = self[1..].to_vec();

        let num_rows = data_rows.len();
        let num_cols = column_names.len();

        let mut columns: BTreeMap<String, Series> = BTreeMap::new();

        for (col_idx, _column_name) in column_names.iter().enumerate().take(num_cols) {
            let col_name = &column_names[col_idx];
            let mut col_data_i32: Vec<Option<i32>> = Vec::with_capacity(num_rows);
            let mut col_data_f64: Vec<Option<f64>> = Vec::with_capacity(num_rows);
            let mut col_data_bool: Vec<Option<bool>> = Vec::with_capacity(num_rows);
            let mut col_data_string: Vec<Option<String>> = Vec::with_capacity(num_rows);

            let mut is_i32 = true;
            let mut is_f64 = true;
            let mut is_bool = true;
            let is_string = true; // Always possible to be a string

            for data_row in data_rows.iter().take(num_rows) {
                let cell_val = &data_row[col_idx];

                // Try parsing as i32
                if is_i32 {
                    match cell_val.parse::<i32>() {
                        Ok(val) => col_data_i32.push(Some(val)),
                        Err(_) => {
                            if cell_val.is_empty() {
                                col_data_i32.push(None);
                            } else {
                                is_i32 = false;
                            }
                        }
                    }
                }

                // Try parsing as f64
                if is_f64 {
                    match cell_val.parse::<f64>() {
                        Ok(val) => col_data_f64.push(Some(val)),
                        Err(_) => {
                            if cell_val.is_empty() {
                                col_data_f64.push(None);
                            } else {
                                is_f64 = false;
                            }
                        }
                    }
                }

                // Try parsing as bool
                if is_bool {
                    match cell_val.parse::<bool>() {
                        Ok(val) => col_data_bool.push(Some(val)),
                        Err(_) => {
                            if cell_val.is_empty() {
                                col_data_bool.push(None);
                            } else {
                                is_bool = false;
                            }
                        }
                    }
                }

                // Always possible to be a string
                if is_string {
                    if cell_val.is_empty() {
                        col_data_string.push(None);
                    } else {
                        col_data_string.push(Some(cell_val.clone()));
                    }
                }
            }

            // Determine the most specific type
            if is_i32 {
                columns.insert(col_name.clone(), Series::new_i32(col_name, col_data_i32));
            } else if is_f64 {
                columns.insert(col_name.clone(), Series::new_f64(col_name, col_data_f64));
            } else if is_bool {
                columns.insert(col_name.clone(), Series::new_bool(col_name, col_data_bool));
            } else if is_string {
                columns.insert(
                    col_name.clone(),
                    Series::new_string(col_name, col_data_string),
                );
            } else {
                return Err(VeloxxError::Parsing(format!(
                    "Could not infer type for column '{col_name}'."
                )));
            }
        }

        DataFrame::new(columns)
    }
}
