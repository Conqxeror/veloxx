use crate::error::VeloxxError;
use crate::series::Series;
use std::collections::BTreeMap;

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
/// `DataFrame` provides methods for data manipulation, cleaning, joining, grouping, and display.
///
/// # Examples
///
/// ## Creating a DataFrame from Series
///
/// ```rust
/// use veloxx::dataframe::DataFrame;
/// use veloxx::series::Series;
/// use std::collections::BTreeMap;
///
/// let mut columns = BTreeMap::new();
/// columns.insert(
///     "name".to_string(),
///     Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string())]),
/// );
/// columns.insert(
///     "age".to_string(),
///     Series::new_i32("age", vec![Some(30), Some(24)]),
/// );
///
/// let df = DataFrame::new(columns).unwrap();
/// println!("Initial DataFrame:\n{}", df);
/// ```
///
/// ## Basic DataFrame Operations
///
/// ```rust
/// use veloxx::dataframe::DataFrame;
/// use veloxx::series::Series;
/// use std::collections::BTreeMap;
///
/// let mut columns = BTreeMap::new();
/// columns.insert("A".to_string(), Series::new_i32("A", vec![Some(1), Some(2)]));
/// columns.insert("B".to_string(), Series::new_f64("B", vec![Some(1.1), Some(2.2)]));
/// let df = DataFrame::new(columns).unwrap();
///
/// assert_eq!(df.row_count(), 2);
/// assert_eq!(df.column_count(), 2);
/// assert!(df.column_names().contains(&&"A".to_string()));
/// ```
#[derive(Debug, Clone)]
pub struct DataFrame {
    pub(crate) columns: BTreeMap<String, Series>,
    pub(crate) row_count: usize,
}

impl DataFrame {
    /// Creates a new `DataFrame` from a `BTreeMap` of column names to `Series`.
    ///
    /// All `Series` in the map must have the same length, and their internal names
    /// must match the keys in the `BTreeMap`.
    ///
    /// # Arguments
    ///
    /// * `columns` - A `BTreeMap` where keys are column names (`String`) and values are `Series`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(DataFrame)` if the DataFrame is successfully created,
    /// or `Err(VeloxxError::InvalidOperation)` if there are inconsistent series lengths
    /// or name mismatches.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2)]));
    /// columns.insert("value".to_string(), Series::new_f64("value", vec![Some(10.0), Some(20.0)]));
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// assert_eq!(df.row_count(), 2);
    /// ```
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
                return Err(VeloxxError::InvalidOperation(
                    "All series in a DataFrame must have the same length.".to_string(),
                ));
            }
        }

        Ok(DataFrame { columns, row_count })
    }

    /// Returns the number of rows in the `DataFrame`.
    ///
    /// # Returns
    ///
    /// A `usize` representing the number of rows.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("data".to_string(), Series::new_i32("data", vec![Some(1), Some(2), Some(3)]));
    /// let df = DataFrame::new(columns).unwrap();
    /// assert_eq!(df.row_count(), 3);
    /// ```
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    /// Returns the number of columns in the `DataFrame`.
    ///
    /// # Returns
    ///
    /// A `usize` representing the number of columns.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("col1".to_string(), Series::new_i32("col1", vec![Some(1)]));
    /// columns.insert("col2".to_string(), Series::new_f64("col2", vec![Some(1.0)]));
    /// let df = DataFrame::new(columns).unwrap();
    /// assert_eq!(df.column_count(), 2);
    /// ```
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Returns a vector containing the names of all columns in the `DataFrame`.
    ///
    /// The order of column names is determined by the internal `BTreeMap` (lexicographical).
    ///
    /// # Returns
    ///
    /// A `Vec<&String>` containing references to the column names.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("B".to_string(), Series::new_i32("B", vec![Some(1)]));
    /// columns.insert("A".to_string(), Series::new_f64("A", vec![Some(1.0)]));
    /// let df = DataFrame::new(columns).unwrap();
    /// let mut column_names = df.column_names();
    /// column_names.sort(); // Sort for consistent testing
    /// assert_eq!(column_names, vec![&"A".to_string(), &"B".to_string()]);
    /// ```
    pub fn column_names(&self) -> Vec<&String> {
        self.columns.keys().collect()
    }

    /// Returns a reference to the `Series` with the given name, if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column (Series) to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<&Series>`: `Some(&Series)` if a column with the given name is found,
    /// otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert("data".to_string(), Series::new_i32("data", vec![Some(1), Some(2)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let series_ref = df.get_column("data").unwrap();
    /// assert_eq!(series_ref.len(), 2);
    ///
    /// assert!(df.get_column("non_existent").is_none());
    /// ```
    pub fn get_column(&self, name: &str) -> Option<&Series> {
        self.columns.get(name)
    }
}
