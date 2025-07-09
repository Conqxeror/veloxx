use crate::error::VeloxxError;
use crate::{
    dataframe::DataFrame,
    series::Series,
    types::{FlatValue, Value},
};
use bincode::{config, decode_from_slice, encode_to_vec};
use std::collections::HashMap;

/// Represents a `DataFrame` that has been grouped by one or more columns.
///
/// This struct is typically created by calling the `group_by` method on a `DataFrame`.
/// It holds a reference to the original `DataFrame`, the columns used for grouping,
/// and an internal map that stores the row indices belonging to each unique group.
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
/// // Now `grouped_df` can be used to perform aggregations.
/// ```
pub struct GroupedDataFrame<'a> {
    dataframe: &'a DataFrame,
    group_columns: Vec<String>,
    groups: HashMap<Vec<u8>, Vec<usize>>,
}

impl<'a> GroupedDataFrame<'a> {
    /// Creates a new `GroupedDataFrame` by grouping the provided `DataFrame` by the specified columns.
    ///
    /// This method iterates through the `DataFrame` and collects row indices for each unique
    /// combination of values in the `group_columns`. The keys for the groups are serialized
    /// `FlatValue` vectors to allow for hashing and comparison.
    ///
    /// # Arguments
    ///
    /// * `dataframe` - A reference to the `DataFrame` to be grouped.
    /// * `group_columns` - A `Vec<String>` containing the names of the columns to group by.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(GroupedDataFrame)` if the grouping is successful,
    /// or `Err(VeloxxError::ColumnNotFound)` if any of the `group_columns` do not exist,
    /// or `Err(VeloxxError::InvalidOperation)` if there's an issue with key serialization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::dataframe::group_by::GroupedDataFrame;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("category".to_string(), Series::new_string("category", vec![Some("A".to_string()), Some("B".to_string()), Some("A".to_string())]));
    /// columns.insert("value".to_string(), Series::new_i32("value", vec![Some(10), Some(20), Some(30)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let grouped_df = GroupedDataFrame::new(&df, vec!["category".to_string()]).unwrap();
    /// // The `grouped_df` now holds the grouped structure.
    /// ```
    pub fn new(dataframe: &'a DataFrame, group_columns: Vec<String>) -> Result<Self, VeloxxError> {
        let mut groups: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();

        for i in 0..dataframe.row_count() {
            let mut key_values: Vec<FlatValue> = Vec::with_capacity(group_columns.len());
            for col_name in group_columns.iter() {
                let series = dataframe
                    .get_column(col_name)
                    .ok_or(VeloxxError::ColumnNotFound(col_name.to_string()))?;
                key_values.push(series.get_value(i).unwrap_or(Value::Null).into());
            }
            let serialized_key = encode_to_vec(key_values, config::standard()).map_err(|e| {
                VeloxxError::InvalidOperation(format!("Failed to serialize group key: {e}"))
            })?;
            groups.entry(serialized_key).or_default().push(i);
        }

        Ok(GroupedDataFrame {
            dataframe,
            group_columns,
            groups,
        })
    }

    /// Performs aggregation operations on the grouped data.
    ///
    /// This method takes a list of aggregation instructions, where each instruction specifies
    /// a column to aggregate and the aggregation function to apply (e.g., "sum", "mean", "count",
    /// "min", "max", "median", "std_dev"). It returns a new `DataFrame` where each row represents
    /// a unique group, and the aggregated values form new columns.
    ///
    /// # Arguments
    ///
    /// * `aggregations` - A `Vec` of tuples, where each tuple contains:
    ///   - `&str`: The name of the column on which to perform the aggregation.
    ///   - `&str`: The aggregation function to apply (e.g., "sum", "mean", "count", "min", "max", "median", "std_dev").
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` containing a new `DataFrame` with the aggregated results,
    /// or `Err(VeloxxError::ColumnNotFound)` if an aggregation column does not exist,
    /// or `Err(VeloxxError::Unsupported)` if an unsupported aggregation function is specified,
    /// or `Err(VeloxxError::InvalidOperation)` if there's an issue with key deserialization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::dataframe::group_by::GroupedDataFrame;
    /// use std::collections::BTreeMap;
    /// use veloxx::types::Value;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("city".to_string(), Series::new_string("city", vec![Some("New York".to_string()), Some("London".to_string()), Some("New York".to_string())]));
    /// columns.insert("sales".to_string(), Series::new_f64("sales", vec![Some(100.0), Some(150.0), Some(200.0)]));
    /// columns.insert("quantity".to_string(), Series::new_i32("quantity", vec![Some(10), Some(15), Some(20)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let grouped_df = df.group_by(vec!["city".to_string()]).unwrap();
    ///
    /// let aggregated_df = grouped_df.agg(vec![
    ///     ("sales", "sum"),
    ///     ("quantity", "mean"),
    ///     ("sales", "count"),
    /// ]).unwrap();
    ///
    /// println!("Aggregated DataFrame:\n{}", aggregated_df);
    /// // Expected output (order of rows might vary):
    /// // city           sales_sum      quantity_mean  sales_count    
    /// // --------------- --------------- --------------- ---------------
    /// // London         150.00          15.00          1              
    /// // New York       300.00          15.00          2              
    /// ```
    pub fn agg(&self, aggregations: Vec<(&str, &str)>) -> Result<DataFrame, VeloxxError> {
        let mut new_columns: std::collections::BTreeMap<String, Series> =
            std::collections::BTreeMap::new();
        let group_keys_serialized: Vec<Vec<u8>> = self.groups.keys().cloned().collect();

        // Deserialize keys for sorting and further processing
        let group_keys: Vec<Vec<Value>> = group_keys_serialized
            .into_iter()
            .map(|key_bytes| {
                decode_from_slice(&key_bytes, config::standard())
                    .map(|(val, _): (Vec<FlatValue>, _)| {
                        val.into_iter().map(|fv| fv.into()).collect()
                    })
                    .map_err(|e| {
                        VeloxxError::InvalidOperation(format!(
                            "Failed to deserialize group key: {e}"
                        ))
                    })
            })
            .collect::<Result<Vec<Vec<Value>>, VeloxxError>>()?;

        // group_keys.sort_unstable(); // Ensure consistent order of groups

        // Add group columns to new_columns
        for col_name in self.group_columns.iter() {
            let original_series = self.dataframe.get_column(col_name).unwrap();
            let mut data_for_new_series: Vec<Option<Value>> = Vec::with_capacity(group_keys.len());
            for key in group_keys.iter() {
                let col_idx = self
                    .group_columns
                    .iter()
                    .position(|x| x == col_name)
                    .unwrap();
                data_for_new_series.push(Some(key[col_idx].clone()));
            }
            let new_series = match original_series.data_type() {
                crate::types::DataType::I32 => Series::new_i32(
                    col_name,
                    data_for_new_series
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
                    col_name,
                    data_for_new_series
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
                    col_name,
                    data_for_new_series
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
                    col_name,
                    data_for_new_series
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
                    col_name,
                    data_for_new_series
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
            new_columns.insert(col_name.clone(), new_series);
        }

        for (col_name, agg_func) in aggregations {
            let original_series = self
                .dataframe
                .get_column(col_name)
                .ok_or(VeloxxError::ColumnNotFound(col_name.to_string()))?;
            let mut aggregated_data: Vec<Option<Value>> = Vec::with_capacity(group_keys.len());

            for key in group_keys.iter() {
                let serialized_key = encode_to_vec(key, config::standard()).map_err(|e| {
                    VeloxxError::InvalidOperation(format!(
                        "Failed to serialize group key for lookup: {e}"
                    ))
                })?;
                let row_indices = self.groups.get(&serialized_key).unwrap();
                let series_for_group = original_series.filter(row_indices)?;

                let aggregated_value = match agg_func {
                    "sum" => series_for_group.sum()?,
                    "count" => Some(Value::I32(series_for_group.count() as i32)),
                    "min" => series_for_group.min()?,
                    "max" => series_for_group.max()?,
                    "mean" => series_for_group.mean()?,
                    "median" => series_for_group.median()?,
                    "std_dev" => series_for_group.std_dev()?,
                    _ => {
                        return Err(VeloxxError::Unsupported(format!(
                            "Unsupported aggregation function: {agg_func}"
                        )))
                    }
                };
                aggregated_data.push(aggregated_value);
            }

            let new_series_name = format!("{col_name}_{agg_func}");
            let new_series = match original_series.data_type() {
                crate::types::DataType::I32 => Series::new_i32(
                    &new_series_name,
                    aggregated_data
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
                    &new_series_name,
                    aggregated_data
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
                    &new_series_name,
                    aggregated_data
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
                    &new_series_name,
                    aggregated_data
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
                    &new_series_name,
                    aggregated_data
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
            new_columns.insert(new_series_name, new_series);
        }

        DataFrame::new(new_columns)
    }
}
