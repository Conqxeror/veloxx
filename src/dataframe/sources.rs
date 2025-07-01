use crate::dataframe::DataFrame;
use crate::series::Series;
use std::collections::BTreeMap;

/// A trait for types that can be converted into a `DataFrame`.
pub trait DataFrameSource {
    fn to_dataframe(&self) -> Result<DataFrame, String>;
}

impl DataFrameSource for Vec<Vec<String>> {
    /// Converts a `Vec<Vec<String>>` into a `DataFrame`.
    ///
    /// The first inner `Vec<String>` is assumed to be the header (column names).
    /// Subsequent inner `Vec<String>`s are treated as rows of data.
    /// Column types are inferred based on the values in each column.
    ///
    /// # Returns
    /// A `Result` containing the new `DataFrame` or a `String` error message.
    fn to_dataframe(&self) -> Result<DataFrame, String> {
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
                return Err(format!("Could not infer type for column '{col_name}'."));
            }
        }

        DataFrame::new(columns)
    }
}
