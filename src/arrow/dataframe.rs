//! Arrow DataFrame implementation
//!
//! This module provides an Arrow-backed DataFrame implementation for improved performance.

#[cfg(feature = "arrow")]
use std::collections::HashMap;

#[cfg(feature = "arrow")]
use crate::arrow::series::ArrowSeries;

/// Arrow-backed DataFrame implementation
#[cfg(feature = "arrow")]
pub struct ArrowDataFrame {
    /// Collection of series in the DataFrame
    pub columns: HashMap<String, ArrowSeries>,
}

#[cfg(feature = "arrow")]
impl ArrowDataFrame {
    /// Create a new empty DataFrame
    pub fn new() -> Self {
        ArrowDataFrame {
            columns: HashMap::new(),
        }
    }

    /// Add a series to the DataFrame
    pub fn add_column(&mut self, series: ArrowSeries) {
        self.columns.insert(series.name().to_string(), series);
    }

    /// Get the number of rows in the DataFrame
    pub fn row_count(&self) -> usize {
        if self.columns.is_empty() {
            0
        } else {
            self.columns.values().next().unwrap().len()
        }
    }

    /// Get the number of columns in the DataFrame
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }
}

#[cfg(feature = "arrow")]
impl Default for ArrowDataFrame {
    fn default() -> Self {
        Self::new()
    }
}
