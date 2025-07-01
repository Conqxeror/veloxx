use crate::{dataframe::DataFrame, series::Series, types::Value};

/// Represents a `DataFrame` that has been grouped by one or more columns.
///
/// This struct holds a reference to the original `DataFrame`, the columns used for grouping,
/// and a map where keys are unique combinations of group column values and values are
/// vectors of row indices belonging to that group.
pub struct GroupedDataFrame<'a> {
    dataframe: &'a DataFrame,
    group_columns: Vec<String>,
    groups: std::collections::BTreeMap<Vec<Value>, Vec<usize>>,
}

impl<'a> GroupedDataFrame<'a> {
    /// Creates a new `GroupedDataFrame` by grouping the provided `DataFrame` by the specified columns.
    ///
    /// # Arguments
    /// * `dataframe` - A reference to the `DataFrame` to be grouped.
    /// * `group_columns` - A `Vec<String>` containing the names of the columns to group by.
    ///
    /// # Returns
    /// A `Result` containing the new `GroupedDataFrame` or a `String` error message if a group column is not found.
    pub fn new(dataframe: &'a DataFrame, group_columns: Vec<String>) -> Result<Self, String> {
        let mut groups: std::collections::BTreeMap<Vec<Value>, Vec<usize>> =
            std::collections::BTreeMap::new();

        for i in 0..dataframe.row_count() {
            let mut key = Vec::with_capacity(group_columns.len());
            for col_name in group_columns.iter() {
                let series = dataframe
                    .get_column(col_name)
                    .ok_or(format!("Group column '{col_name}' not found."))?;
                key.push(series.get_value(i).unwrap_or(Value::Null));
            }
            groups.entry(key).or_default().push(i);
        }

        Ok(GroupedDataFrame {
            dataframe,
            group_columns,
            groups,
        })
    }

    /// Performs aggregation operations on the grouped data.
    ///
    /// # Arguments
    /// * `aggregations` - A `Vec` of tuples, where each tuple contains the column name
    ///   to aggregate and the aggregation function to apply (e.g., "sum", "mean", "count").
    ///
    /// # Returns
    /// A `Result` containing a new `DataFrame` with the aggregated results, or a `String` error message.
    pub fn agg(&self, aggregations: Vec<(&str, &str)>) -> Result<DataFrame, String> {
        let mut new_columns: std::collections::BTreeMap<String, Series> =
            std::collections::BTreeMap::new();
        let mut group_keys: Vec<Vec<Value>> = self.groups.keys().cloned().collect();
        group_keys.sort_unstable(); // Ensure consistent order of groups

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
            };
            new_columns.insert(col_name.clone(), new_series);
        }

        for (col_name, agg_func) in aggregations {
            let original_series = self
                .dataframe
                .get_column(col_name)
                .ok_or(format!("Column '{col_name}' not found for aggregation."))?;
            let mut aggregated_data: Vec<Option<Value>> = Vec::with_capacity(group_keys.len());

            for key in group_keys.iter() {
                let row_indices = self.groups.get(key).unwrap();
                let series_for_group = original_series.filter(row_indices)?;

                let aggregated_value = match agg_func {
                    "sum" => series_for_group.sum()?,
                    "count" => Some(Value::I32(series_for_group.count() as i32)),
                    "min" => series_for_group.min()?,
                    "max" => series_for_group.max()?,
                    "mean" => series_for_group.mean()?,
                    "median" => series_for_group.median()?,
                    "std_dev" => series_for_group.std_dev()?,
                    _ => return Err(format!("Unsupported aggregation function: {agg_func}")),
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
            };
            new_columns.insert(new_series_name, new_series);
        }

        DataFrame::new(new_columns)
    }
}
