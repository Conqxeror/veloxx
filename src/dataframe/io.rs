use crate::error::VeloxxError;
use crate::{dataframe::DataFrame, series::Series};
use csv_core::{ReadFieldResult, Reader};
use microjson::JSONValue;
use std::collections::BTreeMap;
use std::io::Read;

impl DataFrame {
    pub fn from_csv(path: &str) -> Result<Self, VeloxxError> {
        let mut file = std::fs::File::open(path).map_err(|e| VeloxxError::FileIO(e.to_string()))?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .map_err(|e| VeloxxError::FileIO(e.to_string()))?;

        let mut trimmed_bytes = contents.as_slice();
        if let Some(i) = trimmed_bytes
            .iter()
            .rposition(|&x| x != b'\n' && x != b'\r')
        {
            trimmed_bytes = &trimmed_bytes[..=i];
        }

        if trimmed_bytes.is_empty() {
            return DataFrame::new(BTreeMap::new());
        }

        let mut rdr = Reader::new();
        let mut field_buf = [0; 1024]; // Buffer for a single field

        let mut column_names: Vec<String> = Vec::new();
        let mut all_rows_as_strings: Vec<Vec<String>> = Vec::new();
        let mut current_row_fields: Vec<String> = Vec::new();

        let mut bytes = trimmed_bytes;
        let mut is_header = true;

        loop {
            let (result, bytes_consumed, bytes_written) = rdr.read_field(bytes, &mut field_buf);

            let field_str = String::from_utf8(field_buf[..bytes_written].to_vec())
                .map_err(|e| VeloxxError::Parsing(e.to_string()))?;
            current_row_fields.push(field_str);

            bytes = &bytes[bytes_consumed..];

            match result {
                ReadFieldResult::InputEmpty => {
                    if !current_row_fields.is_empty() {
                        if is_header {
                            column_names = current_row_fields.clone();
                        } else {
                            all_rows_as_strings.push(current_row_fields.clone());
                        }
                    }
                    break;
                }
                ReadFieldResult::OutputFull => {
                    return Err(VeloxxError::Parsing(
                        "CSV field too large for buffer.".to_string(),
                    ));
                }
                ReadFieldResult::Field { record_end } => {
                    if record_end {
                        if is_header {
                            column_names = current_row_fields.clone();
                            is_header = false;
                        } else {
                            all_rows_as_strings.push(current_row_fields.clone());
                        }
                        current_row_fields.clear();
                    }
                }
                ReadFieldResult::End => {
                    if !current_row_fields.is_empty() {
                        if is_header {
                            column_names = current_row_fields.clone();
                        } else {
                            all_rows_as_strings.push(current_row_fields.clone());
                        }
                    }
                    break;
                }
            }
        }

        if column_names.is_empty() {
            return DataFrame::new(BTreeMap::new());
        }

        let header = column_names;
        let data_rows = all_rows_as_strings.clone();
        for row in &all_rows_as_strings {
            if row.len() != header.len() {
                return Err(VeloxxError::Parsing(
                    "CSV rows have inconsistent number of columns.".to_string(),
                ));
            }
        }

        if data_rows.is_empty() {
            // If only header exists, create an empty DataFrame with correct columns
            let mut columns: BTreeMap<String, Series> = BTreeMap::new();
            for col_name in header {
                columns.insert(col_name.clone(), Series::new_string(&col_name, Vec::new()));
            }
            return DataFrame::new(columns);
        }

        // Infer types and create Series
        let num_cols = header.len();
        let num_rows = data_rows.len();
        let mut columns: BTreeMap<String, Series> = BTreeMap::new();

        for (col_idx, header_item) in header.iter().enumerate().take(num_cols) {
            let col_name = &header_item;
            let mut col_data_i32: Vec<Option<i32>> = Vec::with_capacity(num_rows);
            let mut col_data_f64: Vec<Option<f64>> = Vec::with_capacity(num_rows);
            let mut col_data_bool: Vec<Option<bool>> = Vec::with_capacity(num_rows);
            let mut col_data_string: Vec<Option<String>> = Vec::with_capacity(num_rows);
            let mut col_data_datetime: Vec<Option<i64>> = Vec::with_capacity(num_rows);

            let mut is_i32 = true;
            let mut is_f64 = true;
            let mut is_bool = true;
            let mut is_datetime = true;
            let is_string = true; // Always possible to be a string

            for data_row in all_rows_as_strings.iter().take(num_rows) {
                let cell_val = if col_idx < data_row.len() {
                    &data_row[col_idx]
                } else {
                    ""
                };

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

                // Try parsing as datetime (i64 for Unix timestamp)
                if is_datetime {
                    match cell_val.parse::<i64>() {
                        Ok(val) => col_data_datetime.push(Some(val)),
                        Err(_) => {
                            if cell_val.is_empty() {
                                col_data_datetime.push(None);
                            } else {
                                is_datetime = false;
                            }
                        }
                    }
                }

                // Always possible to be a string
                if is_string {
                    if cell_val.is_empty() {
                        col_data_string.push(None);
                    } else {
                        col_data_string.push(Some(cell_val.to_string()));
                    }
                }
            }

            let inferred_type = if is_i32 {
                crate::types::DataType::I32
            } else if is_f64 {
                crate::types::DataType::F64
            } else if is_bool {
                crate::types::DataType::Bool
            } else if is_datetime {
                crate::types::DataType::DateTime
            } else {
                crate::types::DataType::String
            };

            match inferred_type {
                crate::types::DataType::I32 => {
                    columns.insert(
                        col_name.to_string(),
                        Series::new_i32(col_name, col_data_i32),
                    );
                }
                crate::types::DataType::F64 => {
                    columns.insert(
                        col_name.to_string(),
                        Series::new_f64(col_name, col_data_f64),
                    );
                }
                crate::types::DataType::Bool => {
                    columns.insert(
                        col_name.to_string(),
                        Series::new_bool(col_name, col_data_bool),
                    );
                }
                crate::types::DataType::DateTime => {
                    columns.insert(
                        col_name.to_string(),
                        Series::new_datetime(col_name, col_data_datetime),
                    );
                }
                crate::types::DataType::String => {
                    columns.insert(
                        col_name.to_string(),
                        Series::new_string(col_name, col_data_string),
                    );
                }
            }
        }

        DataFrame::new(columns)
    }

    pub fn from_vec_of_vec(
        data: Vec<Vec<String>>,
        column_names: Vec<String>,
    ) -> Result<Self, VeloxxError> {
        if data.is_empty() {
            return DataFrame::new(BTreeMap::new());
        }

        if data[0].len() != column_names.len() {
            return Err(VeloxxError::InvalidOperation(
                "Number of columns in data does not match number of column names.".to_string(),
            ));
        }

        let num_rows = data.len();
        let num_cols = column_names.len();

        let mut columns: BTreeMap<String, Series> = BTreeMap::new();

        for (col_idx, column_name) in column_names.iter().enumerate().take(num_cols) {
            let col_name = &column_name;
            let mut all_i32 = true;
            let mut all_f64 = true;
            let mut all_bool = true;
            let mut all_datetime = true;
            let is_string = true; // Always possible to be a string

            for data_row in data.iter().take(num_rows) {
                let cell_val = &data_row[col_idx];

                if all_i32 && cell_val.parse::<i32>().is_err() && !cell_val.is_empty() {
                    all_i32 = false;
                }
                if all_f64 && cell_val.parse::<f64>().is_err() && !cell_val.is_empty() {
                    all_f64 = false;
                }
                if all_bool && cell_val.parse::<bool>().is_err() && !cell_val.is_empty() {
                    all_bool = false;
                }
                if all_datetime && cell_val.parse::<i64>().is_err() && !cell_val.is_empty() {
                    all_datetime = false;
                }
            }

            if all_i32 {
                let col_data: Vec<Option<i32>> = data
                    .iter()
                    .take(num_rows)
                    .map(|data_row| {
                        let cell_val = &data_row[col_idx];
                        if cell_val.is_empty() {
                            None
                        } else {
                            cell_val.parse::<i32>().ok()
                        }
                    })
                    .collect();
                columns.insert(col_name.to_string(), Series::new_i32(col_name, col_data));
            } else if all_f64 {
                let col_data: Vec<Option<f64>> = data
                    .iter()
                    .take(num_rows)
                    .map(|data_row| {
                        let cell_val = &data_row[col_idx];
                        if cell_val.is_empty() {
                            None
                        } else {
                            cell_val.parse::<f64>().ok()
                        }
                    })
                    .collect();
                columns.insert(col_name.to_string(), Series::new_f64(col_name, col_data));
            } else if all_bool {
                let col_data: Vec<Option<bool>> = data
                    .iter()
                    .take(num_rows)
                    .map(|data_row| {
                        let cell_val = &data_row[col_idx];
                        if cell_val.is_empty() {
                            None
                        } else {
                            cell_val.parse::<bool>().ok()
                        }
                    })
                    .collect();
                columns.insert(col_name.to_string(), Series::new_bool(col_name, col_data));
            } else if all_datetime {
                let col_data: Vec<Option<i64>> = data
                    .iter()
                    .take(num_rows)
                    .map(|data_row| {
                        let cell_val = &data_row[col_idx];
                        if cell_val.is_empty() {
                            None
                        } else {
                            cell_val.parse::<i64>().ok()
                        }
                    })
                    .collect();
                columns.insert(
                    col_name.to_string(),
                    Series::new_datetime(col_name, col_data),
                );
            } else if is_string {
                let col_data: Vec<Option<String>> = data
                    .iter()
                    .take(num_rows)
                    .map(|data_row| {
                        let cell_val = &data_row[col_idx];
                        if cell_val.is_empty() {
                            None
                        } else {
                            Some(cell_val.clone())
                        }
                    })
                    .collect();
                columns.insert(col_name.to_string(), Series::new_string(col_name, col_data));
            } else {
                return Err(VeloxxError::Parsing(format!(
                    "Could not infer type for column '{col_name}'."
                )));
            }
        }

        DataFrame::new(columns)
    }

    pub fn to_csv(&self, path: &str) -> Result<(), VeloxxError> {
        use std::io::Write;
        let mut file =
            std::fs::File::create(path).map_err(|e| VeloxxError::FileIO(e.to_string()))?;

        if self.column_count() == 0 {
            return Ok(());
        }

        let column_names: Vec<&str> = self.column_names().iter().map(|s| s.as_str()).collect();
        writeln!(file, "{}", column_names.join(","))
            .map_err(|e| VeloxxError::FileIO(e.to_string()))?;

        for i in 0..self.row_count() {
            let mut row_values: Vec<String> = Vec::new();
            for col_name in column_names.iter() {
                let series = self.get_column(col_name).unwrap();
                let value_str = match series.get_value(i) {
                    Some(crate::types::Value::I32(v)) => v.to_string(),
                    Some(crate::types::Value::F64(v)) => v.to_string(),
                    Some(crate::types::Value::Bool(v)) => v.to_string(),
                    Some(crate::types::Value::String(v)) => v.clone(),
                    Some(crate::types::Value::DateTime(v)) => v.to_string(),
                    Some(crate::types::Value::Null) => "".to_string(),
                    None => "".to_string(),
                };
                row_values.push(value_str);
            }
            writeln!(file, "{}", row_values.join(","))
                .map_err(|e| VeloxxError::FileIO(e.to_string()))?;
        }

        Ok(())
    }

    pub fn from_json(path: &str) -> Result<Self, VeloxxError> {
        let contents =
            std::fs::read_to_string(path).map_err(|e| VeloxxError::FileIO(e.to_string()))?;
        let json = JSONValue::load(&contents);
        let arr_iter = match json.iter_array() {
            Ok(arr) => arr,
            Err(_) => {
                return Err(VeloxxError::Parsing(
                    "JSON root must be an array".to_string(),
                ))
            }
        };
        let mut rows = Vec::new();
        for row_val in arr_iter {
            let obj_iter = match row_val.iter_object() {
                Ok(obj) => obj,
                Err(_) => {
                    return Err(VeloxxError::Parsing(
                        "Each row must be a JSON object".to_string(),
                    ))
                }
            };
            let mut row = std::collections::BTreeMap::new();
            for entry in obj_iter {
                let (k, v) = match entry {
                    Ok((k, v)) => (k, v),
                    Err(_) => {
                        return Err(VeloxxError::Parsing(
                            "Error reading key-value pair".to_string(),
                        ))
                    }
                };
                let value = if let Ok(f) = v.read_float() {
                    Some(crate::types::Value::F64(f as f64))
                } else if let Ok(i) = v.read_integer() {
                    Some(crate::types::Value::I32(i as i32))
                } else if let Ok(s) = v.read_string() {
                    Some(crate::types::Value::String(s.to_string()))
                } else if let Ok(b) = v.read_boolean() {
                    Some(crate::types::Value::Bool(b))
                } else if let Ok(dt) = v.read_integer() {
                    Some(crate::types::Value::DateTime(dt as i64))
                } else {
                    None
                };
                row.insert(k.to_string(), value);
            }
            rows.push(row);
        }
        if rows.is_empty() {
            return Err(VeloxxError::Parsing("JSON array is empty".to_string()));
        }
        let column_names: Vec<String> = rows[0].keys().cloned().collect();
        let mut columns: BTreeMap<String, Vec<Option<crate::types::Value>>> = BTreeMap::new();
        for name in &column_names {
            columns.insert(name.clone(), Vec::new());
        }
        for row in rows {
            for name in &column_names {
                columns
                    .get_mut(name)
                    .unwrap()
                    .push(row.get(name).cloned().unwrap_or(None));
            }
        }
        let mut series_map = BTreeMap::new();
        for (name, values) in columns {
            let series = if let Some(Some(crate::types::Value::F64(_))) =
                values.iter().find(|v| v.is_some())
            {
                Series::new_f64(
                    &name,
                    values
                        .into_iter()
                        .map(|v| match v {
                            Some(crate::types::Value::F64(f)) => Some(f),
                            _ => None,
                        })
                        .collect(),
                )
            } else if let Some(Some(crate::types::Value::I32(_))) =
                values.iter().find(|v| v.is_some())
            {
                Series::new_i32(
                    &name,
                    values
                        .into_iter()
                        .map(|v| match v {
                            Some(crate::types::Value::I32(i)) => Some(i),
                            _ => None,
                        })
                        .collect(),
                )
            } else if let Some(Some(crate::types::Value::Bool(_))) =
                values.iter().find(|v| v.is_some())
            {
                Series::new_bool(
                    &name,
                    values
                        .into_iter()
                        .map(|v| match v {
                            Some(crate::types::Value::Bool(b)) => Some(b),
                            _ => None,
                        })
                        .collect(),
                )
            } else if let Some(Some(crate::types::Value::DateTime(_))) =
                values.iter().find(|v| v.is_some())
            {
                Series::new_datetime(
                    &name,
                    values
                        .into_iter()
                        .map(|v| match v {
                            Some(crate::types::Value::DateTime(dt)) => Some(dt),
                            _ => None,
                        })
                        .collect(),
                )
            } else {
                Series::new_string(
                    &name,
                    values
                        .into_iter()
                        .map(|v| match v {
                            Some(crate::types::Value::String(s)) => Some(s),
                            _ => None,
                        })
                        .collect(),
                )
            };
            series_map.insert(name, series);
        }
        DataFrame::new(series_map)
    }
}
