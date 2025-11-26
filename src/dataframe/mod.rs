use indexmap::IndexMap;

use crate::series::Series;

pub mod cleaning;
pub mod display;
pub mod group_by;
pub mod io;
pub mod join;
pub mod manipulation;
pub mod pivot;
pub mod sources;
pub mod time_series;

pub use pivot::Pivot;

#[derive(Debug, Clone)]
pub struct DataFrame {
    pub columns: IndexMap<String, Series>,
}

impl DataFrame {
    pub fn new(columns: IndexMap<String, Series>) -> Self {
        DataFrame { columns }
    }

    pub fn add_column(&mut self, series: Series) {
        self.columns.insert(series.name().to_string(), series);
    }

    pub fn row_count(&self) -> usize {
        // Return the maximum column length among columns. This ensures row_count is
        // stable even when columns have mismatched lengths.
        self.columns.values().map(|s| s.len()).max().unwrap_or(0)
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn column_names(&self) -> Vec<String> {
        self.columns.keys().cloned().collect()
    }

    pub fn get_column(&self, name: &str) -> Option<&Series> {
        self.columns.get(name)
    }
}

impl Default for DataFrame {
    fn default() -> Self {
        Self::new(IndexMap::new())
    }
}
