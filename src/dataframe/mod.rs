use crate::series::Series;
use std::collections::BTreeMap;
use crate::error::VeloxxError;

pub mod cleaning;
pub mod display;
pub mod group_by;
pub mod io;
pub mod join;
pub mod manipulation;
pub mod sources;

/// Represents a tabular data structure with named columns, similar to a data frame in other data manipulation libraries.
///
/// Each column in a `DataFrame` is a `Series`, and all series must have the same length.
#[derive(Debug, Clone)]
pub struct DataFrame {
    pub(crate) columns: BTreeMap<String, Series>,
    pub(crate) row_count: usize,
}

impl DataFrame {
    /// Creates a new `DataFrame` from a `BTreeMap` of column names to `Series`.
    ///
    /// Returns an error if the series have inconsistent lengths or if column names
    /// in the BTreeMap do not match the names within the Series.
    pub fn new(columns: BTreeMap<String, Series>) -> Result<Self, VeloxxError> {
        if columns.is_empty() {
            return Ok(DataFrame {
                columns,
                row_count: 0,
            });
        }

        let mut row_count = 0;
        for (i, (col_name, series)) in columns.iter().enumerate() {
            if col_name != series.name() {
                return Err(VeloxxError::InvalidOperation(format!(
                    "Column name mismatch: HashMap key '{}' does not match Series name '{}'.",
                    col_name,
                    series.name()
                )));
            }
            if i == 0 {
                row_count = series.len();
            } else if series.len() != row_count {
                return Err(VeloxxError::InvalidOperation("All series in a DataFrame must have the same length.".to_string()));
            }
        }

        Ok(DataFrame { columns, row_count })
    }

    /// Returns the number of rows in the `DataFrame`.
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    /// Returns the number of columns in the `DataFrame`.
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Returns a vector containing the names of all columns in the `DataFrame`.
    pub fn column_names(&self) -> Vec<&String> {
        self.columns.keys().collect()
    }

    /// Returns a reference to the `Series` with the given name, if it exists.
    pub fn get_column(&self, name: &str) -> Option<&Series> {
        self.columns.get(name)
    }
}
