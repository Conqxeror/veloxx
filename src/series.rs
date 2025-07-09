use crate::error::VeloxxError;
use crate::types::{DataType, Value};
use std::collections::HashSet;

/// Represents a single-typed, named column of data within a DataFrame.
///
/// A `Series` is a fundamental data structure in Veloxx, analogous to a column in a spreadsheet
/// or a database table. It holds a sequence of values of a single data type (e.g., all integers,
/// all floating-point numbers, all strings, or all booleans), and can contain null values.
/// Each `Series` has a name, which typically corresponds to the column name in a `DataFrame`.
///
/// # Variants
///
/// - `I32(String, Vec<Option<i32>>)`: A series of 32-bit signed integers.
/// - `F64(String, Vec<Option<f64>>)`: A series of 64-bit floating-point numbers.
/// - `Bool(String, Vec<Option<bool>>)`: A series of boolean values.
/// - `String(String, Vec<Option<String>>)`: A series of string values.
/// - `DateTime(String, Vec<Option<i64>>)`: A series of DateTime values, represented as Unix timestamps (i64).
/// # Examples
///
/// ```rust
/// use veloxx::series::Series;
///
/// let age_series = Series::new_i32("age", vec![Some(25), Some(30), None, Some(40)]);
/// let name_series = Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string()), None]);
/// let is_active_series = Series::new_bool("is_active", vec![Some(true), None, Some(false), Some(true)]);
/// ```
///
/// ## Getting Series Information
///
/// ```rust
/// use veloxx::series::Series;
/// use veloxx::types::DataType;
///
/// let series = Series::new_i32("data", vec![Some(1), Some(2), Some(3)]);
///
/// assert_eq!(series.name(), "data");
/// assert_eq!(series.len(), 3);
/// assert_eq!(series.data_type(), DataType::I32);
/// assert!(!series.is_empty());
/// ```
///
/// ## Accessing Values
///
/// ```rust
/// use veloxx::series::Series;
/// use veloxx::types::Value;
///
/// let series = Series::new_f64("values", vec![Some(1.1), None, Some(3.3)]);
///
/// assert_eq!(series.get_value(0), Some(Value::F64(1.1)));
/// assert_eq!(series.get_value(1), None);
/// assert_eq!(series.get_value(2), Some(Value::F64(3.3)));
/// assert_eq!(series.get_value(3), None); // Index out of bounds
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum Series {
    /// A series containing 32-bit signed integers.
    I32(String, Vec<Option<i32>>),
    /// A series containing 64-bit floating-point numbers.
    F64(String, Vec<Option<f64>>),
    /// A series containing boolean values.
    Bool(String, Vec<Option<bool>>),
    /// A series containing string values.
    String(String, Vec<Option<String>>),
    /// A series containing DateTime values (Unix timestamp i64).
    DateTime(String, Vec<Option<i64>>),
}

impl Series {
    /// Creates a new `Series` of 32-bit signed integers.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the series.
    /// * `data` - A `Vec` of `Option<i32>` containing the integer values. `None` represents a null value.
    ///
    /// # Returns
    ///
    /// A new `Series::I32` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_i32("age", vec![Some(25), Some(30), None]);
    /// ```
    pub fn new_i32(name: &str, data: Vec<Option<i32>>) -> Self {
        Series::I32(name.to_string(), data)
    }

    /// Creates a new `Series` of 64-bit floating-point numbers.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the series.
    /// * `data` - A `Vec` of `Option<f64>` containing the float values. `None` represents a null value.
    ///
    /// # Returns
    ///
    /// A new `Series::F64` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_f64("price", vec![Some(9.99), Some(19.99), None]);
    /// ```
    pub fn new_f64(name: &str, data: Vec<Option<f64>>) -> Self {
        Series::F64(name.to_string(), data)
    }

    /// Creates a new `Series` of boolean values.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the series.
    /// * `data` - A `Vec` of `Option<bool>` containing the boolean values. `None` represents a null value.
    ///
    /// # Returns
    ///
    /// A new `Series::Bool` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_bool("is_active", vec![Some(true), None, Some(false)]);
    /// ```
    pub fn new_bool(name: &str, data: Vec<Option<bool>>) -> Self {
        Series::Bool(name.to_string(), data)
    }

    /// Creates a new `Series` of string values.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the series.
    /// * `data` - A `Vec` of `Option<String>` containing the string values. `None` represents a null value.
    ///
    /// # Returns
    ///
    /// A new `Series::String` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_string("city", vec![Some("New York".to_string()), None, Some("London".to_string())]);
    /// ```
    pub fn new_string(name: &str, data: Vec<Option<String>>) -> Self {
        Series::String(name.to_string(), data)
    }

    /// Creates a new `Series` of DateTime values.
    ///
    /// DateTime values are represented as Unix timestamps (i64).
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the series.
    /// * `data` - A `Vec` of `Option<i64>` containing the DateTime values. `None` represents a null value.
    ///
    /// # Returns
    ///
    /// A new `Series::DateTime` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// // Example: Unix timestamps for Jan 1, 2023 and Jan 2, 2023
    /// let series = Series::new_datetime("timestamp", vec![Some(1672531200), Some(1672617600), None]);
    /// ```
    pub fn new_datetime(name: &str, data: Vec<Option<i64>>) -> Self {
        Series::DateTime(name.to_string(), data)
    }

    /// Returns the name of the series.
    ///
    /// # Returns
    ///
    /// A string slice (`&str`) representing the name of the series.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_i32("my_column", vec![Some(1), Some(2)]);
    /// assert_eq!(series.name(), "my_column");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Series::I32(name, _) => name,
            Series::F64(name, _) => name,
            Series::Bool(name, _) => name,
            Series::String(name, _) => name,
            Series::DateTime(name, _) => name,
        }
    }

    /// Returns the number of elements in the series.
    ///
    /// This count includes both non-null and null values.
    ///
    /// # Returns
    ///
    /// A `usize` representing the total number of elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_i32("data", vec![Some(1), None, Some(3)]);
    /// assert_eq!(series.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        match self {
            Series::I32(_, v) => v.len(),
            Series::F64(_, v) => v.len(),
            Series::Bool(_, v) => v.len(),
            Series::String(_, v) => v.len(),
            Series::DateTime(_, v) => v.len(),
        }
    }

    /// Returns `true` if the series contains no elements.
    ///
    /// # Returns
    ///
    /// A `bool` indicating whether the series is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let empty_series = Series::new_i32("empty", vec![]);
    /// assert!(empty_series.is_empty());
    ///
    /// let non_empty_series = Series::new_i32("non_empty", vec![Some(1)]);
    /// assert!(!non_empty_series.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the `DataType` of the series.
    ///
    /// # Returns
    ///
    /// A `DataType` enum variant representing the underlying type of the series.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::DataType;
    ///
    /// let series = Series::new_f64("values", vec![Some(1.0)]);
    /// assert_eq!(series.data_type(), DataType::F64);
    /// ```
    pub fn data_type(&self) -> DataType {
        match self {
            Series::I32(_, _) => DataType::I32,
            Series::F64(_, _) => DataType::F64,
            Series::Bool(_, _) => DataType::Bool,
            Series::String(_, _) => DataType::String,
            Series::DateTime(_, _) => DataType::DateTime,
        }
    }

    /// Sets the name of the series.
    ///
    /// # Arguments
    ///
    /// * `new_name` - The new name for the series as a string slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let mut series = Series::new_i32("old_name", vec![Some(1)]);
    /// series.set_name("new_name");
    /// assert_eq!(series.name(), "new_name");
    /// ```
    pub fn set_name(&mut self, new_name: &str) {
        match self {
            Series::I32(name, _) => *name = new_name.to_string(),
            Series::F64(name, _) => *name = new_name.to_string(),
            Series::Bool(name, _) => *name = new_name.to_string(),
            Series::String(name, _) => *name = new_name.to_string(),
            Series::DateTime(name, _) => *name = new_name.to_string(),
        }
    }

    /// Returns the `Value` at the given index, if it exists.
    ///
    /// This method provides a generic way to access values, converting the underlying
    /// type into the `Value` enum.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<Value>`: `Some(Value)` if the index is valid and the value is not null,
    /// `None` if the index is out of bounds or the value at the index is null.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("numbers", vec![Some(10), None, Some(30)]);
    ///
    /// assert_eq!(series.get_value(0), Some(Value::I32(10)));
    /// assert_eq!(series.get_value(1), None);
    /// assert_eq!(series.get_value(2), Some(Value::I32(30)));
    /// assert_eq!(series.get_value(3), None); // Index out of bounds
    /// ```
    pub fn get_value(&self, index: usize) -> Option<Value> {
        match self {
            Series::I32(_, v) => v.get(index).and_then(|&val| val.map(Value::I32)),
            Series::F64(_, v) => v.get(index).and_then(|&val| val.map(Value::F64)),
            Series::Bool(_, v) => v.get(index).and_then(|&val| val.map(Value::Bool)),
            Series::String(_, v) => v
                .get(index)
                .and_then(|val| val.as_ref().map(|s| Value::String(s.clone()))),
            Series::DateTime(_, v) => v.get(index).and_then(|&val| val.map(Value::DateTime)),
        }
    }

    /// Filters the series based on the provided row indices.
    ///
    /// This method creates a new `Series` containing only the elements at the specified indices.
    /// The order of elements in the new series will match the order of `row_indices`.
    ///
    /// # Arguments
    ///
    /// * `row_indices` - A slice of `usize` representing the indices of the rows to keep.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new filtered series, or `Err(VeloxxError)`
    /// if an invalid operation occurs (e.g., an index is out of bounds).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("data", vec![Some(10), Some(20), Some(30), Some(40)]);
    /// let indices = vec![0, 2, 3];
    /// let filtered_series = series.filter(&indices).unwrap();
    ///
    /// assert_eq!(filtered_series.len(), 3);
    /// assert_eq!(filtered_series.get_value(0), Some(Value::I32(10)));
    /// assert_eq!(filtered_series.get_value(1), Some(Value::I32(30)));
    /// assert_eq!(filtered_series.get_value(2), Some(Value::I32(40)));
    /// ```
    pub fn filter(&self, row_indices: &[usize]) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                let filtered_data: Vec<Option<i32>> =
                    row_indices.iter().map(|&i| data[i]).collect();
                Ok(Series::I32(name, filtered_data))
            }
            Series::F64(_, data) => {
                let filtered_data: Vec<Option<f64>> =
                    row_indices.iter().map(|&i| data[i]).collect();
                Ok(Series::F64(name, filtered_data))
            }
            Series::Bool(_, data) => {
                let filtered_data: Vec<Option<bool>> =
                    row_indices.iter().map(|&i| data[i]).collect();
                Ok(Series::Bool(name, filtered_data))
            }
            Series::String(_, data) => {
                let filtered_data: Vec<Option<String>> =
                    row_indices.iter().map(|&i| data[i].clone()).collect();
                Ok(Series::String(name, filtered_data))
            }
            Series::DateTime(_, data) => {
                let filtered_data: Vec<Option<i64>> =
                    row_indices.iter().map(|&i| data[i]).collect();
                Ok(Series::DateTime(name, filtered_data))
            }
        }
    }

    /// Fills null values in the series with a specified value.
    ///
    /// This method creates a new `Series` where all `None` (null) values are replaced
    /// by the provided `value`. The type of the `value` must match the `Series`'s
    /// underlying data type.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to a `Value` enum representing the fill value.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new series with nulls filled,
    /// or `Err(VeloxxError::DataTypeMismatch)` if the fill value's type does not match
    /// the series' data type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("data", vec![Some(1), None, Some(3)]);
    /// let filled_series = series.fill_nulls(&Value::I32(99)).unwrap();
    ///
    /// assert_eq!(filled_series.get_value(0), Some(Value::I32(1)));
    /// assert_eq!(filled_series.get_value(1), Some(Value::I32(99)));
    /// assert_eq!(filled_series.get_value(2), Some(Value::I32(3)));
    ///
    /// let string_series = Series::new_string("names", vec![Some("Alice".to_string()), None]);
    /// let filled_string_series = string_series.fill_nulls(&Value::String("Unknown".to_string())).unwrap();
    /// assert_eq!(filled_string_series.get_value(1), Some(Value::String("Unknown".to_string())));
    /// ```
    pub fn fill_nulls(&self, value: &Value) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                if let Value::I32(fill_val) = value {
                    let filled_data: Vec<Option<i32>> =
                        data.iter().map(|&x| x.or(Some(*fill_val))).collect();
                    Ok(Series::I32(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill I32 series with {value:?}"
                    )))
                }
            }
            Series::F64(_, data) => {
                if let Value::F64(fill_val) = value {
                    let filled_data: Vec<Option<f64>> =
                        data.iter().map(|&x| x.or(Some(*fill_val))).collect();
                    Ok(Series::F64(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill F64 series with {value:?}"
                    )))
                }
            }
            Series::Bool(_, data) => {
                if let Value::Bool(fill_val) = value {
                    let filled_data: Vec<Option<bool>> =
                        data.iter().map(|&x| x.or(Some(*fill_val))).collect();
                    Ok(Series::Bool(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill Bool series with {value:?}"
                    )))
                }
            }
            Series::String(_, data) => {
                if let Value::String(fill_val) = value {
                    let filled_data: Vec<Option<String>> = data
                        .iter()
                        .map(|x| x.clone().or(Some(fill_val.clone())))
                        .collect();
                    Ok(Series::String(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill String series with {value:?}"
                    )))
                }
            }
            Series::DateTime(_, data) => {
                if let Value::DateTime(fill_val) = value {
                    let filled_data: Vec<Option<i64>> =
                        data.iter().map(|&x| x.or(Some(*fill_val))).collect();
                    Ok(Series::DateTime(name, filled_data))
                } else {
                    Err(VeloxxError::DataTypeMismatch(format!(
                        "Type mismatch: Cannot fill DateTime series with {value:?}"
                    )))
                }
            }
        }
    }

    /// Casts the series to a new data type.
    ///
    /// This method attempts to convert the elements of the current series to the specified
    /// `to_type`. Supported conversions include numeric to numeric (e.g., I32 to F64),
    /// string to numeric/boolean/datetime, and datetime to string. Null values are preserved.
    ///
    /// # Arguments
    ///
    /// * `to_type` - The `DataType` to which the series should be cast.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new series with the casted data,
    /// or `Err(VeloxxError::Unsupported)` if the cast is not supported between the
    /// current and target data types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::{DataType, Value};
    ///
    /// let int_series = Series::new_i32("numbers", vec![Some(1), Some(2), None]);
    /// let float_series = int_series.cast(DataType::F64).unwrap();
    /// assert_eq!(float_series.get_value(0), Some(Value::F64(1.0)));
    /// assert_eq!(float_series.data_type(), DataType::F64);
    ///
    /// let string_series = Series::new_string("text_numbers", vec![Some("10".to_string()), Some("20.5".to_string()), None, Some("invalid".to_string())]);
    /// let casted_int_series = string_series.cast(DataType::I32).unwrap();
    /// assert_eq!(casted_int_series.get_value(0), Some(Value::I32(10)));
    /// assert_eq!(casted_int_series.get_value(1), None); // "20.5" cannot be parsed as i32
    /// assert_eq!(casted_int_series.get_value(3), None); // "invalid" cannot be parsed as i32
    /// ```
    pub fn cast(&self, to_type: DataType) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match (self, to_type) {
            (Series::I32(_, data), DataType::F64) => Ok(Series::F64(
                name,
                data.iter().map(|&x| x.map(|val| val as f64)).collect(),
            )),
            (Series::F64(_, data), DataType::I32) => Ok(Series::I32(
                name,
                data.iter().map(|&x| x.map(|val| val as i32)).collect(),
            )),
            (Series::String(_, data), DataType::I32) => Ok(Series::I32(
                name,
                data.iter()
                    .map(|x| x.as_ref().and_then(|s| s.parse::<i32>().ok()))
                    .collect::<Vec<Option<i32>>>(),
            )),
            (Series::String(_, data), DataType::F64) => Ok(Series::F64(
                name,
                data.iter()
                    .map(|x| x.as_ref().and_then(|s| s.parse::<f64>().ok()))
                    .collect::<Vec<Option<f64>>>(),
            )),
            (Series::String(_, data), DataType::Bool) => Ok(Series::Bool(
                name,
                data.iter()
                    .map(|x| x.as_ref().and_then(|s| s.parse::<bool>().ok()))
                    .collect(),
            )),
            (Series::I32(_, data), DataType::DateTime) => Ok(Series::DateTime(
                name,
                data.iter().map(|&x| x.map(|val| val as i64)).collect(),
            )),
            (Series::String(_, data), DataType::DateTime) => Ok(Series::DateTime(
                name,
                data.iter()
                    .map(|x| x.as_ref().and_then(|s| s.parse::<i64>().ok()))
                    .collect(),
            )),
            (Series::DateTime(_, data), DataType::String) => Ok(Series::String(
                name,
                data.iter().map(|&x| x.map(|val| val.to_string())).collect(),
            )),
            (s, t) if s.data_type() == t => Ok(s.clone()),
            (_, to_type) => Err(VeloxxError::Unsupported(format!(
                "Unsupported cast from {:?} to {:?}",
                self.data_type(),
                to_type
            ))),
        }
    }

    /// Appends another series to the end of this series.
    ///
    /// This method concatenates the data from `other` series to the end of the current series.
    /// Both series must have the same `DataType`.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the `Series` to append.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new combined series,
    /// or `Err(VeloxxError::DataTypeMismatch)` if the series have different data types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series1 = Series::new_i32("numbers", vec![Some(1), Some(2)]);
    /// let series2 = Series::new_i32("numbers", vec![Some(3), None]);
    ///
    /// let combined_series = series1.append(&series2).unwrap();
    /// assert_eq!(combined_series.len(), 4);
    /// assert_eq!(combined_series.get_value(2), Some(Value::I32(3)));
    /// assert_eq!(combined_series.get_value(3), None);
    /// ```
    pub fn append(&self, other: &Series) -> Result<Self, VeloxxError> {
        if self.data_type() != other.data_type() {
            return Err(VeloxxError::DataTypeMismatch(format!(
                "Cannot append Series of different types: {:?} and {:?}",
                self.data_type(),
                other.data_type()
            )));
        }
        let new_name = self.name().to_string();
        match (self, other) {
            (Series::I32(_, data1), Series::I32(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::I32(new_name, new_data))
            }
            (Series::F64(_, data1), Series::F64(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::F64(new_name, new_data))
            }
            (Series::Bool(_, data1), Series::Bool(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::Bool(new_name, new_data))
            }
            (Series::String(_, data1), Series::String(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::String(new_name, new_data))
            }
            (Series::DateTime(_, data1), Series::DateTime(_, data2)) => {
                let mut new_data = data1.to_vec();
                new_data.extend(data2.iter().cloned());
                Ok(Series::DateTime(new_name, new_data))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Mismatched series types during append (should be caught by data_type check)."
                    .to_string(),
            )),
        }
    }

    /// Calculates the sum of all non-null values in the series.
    ///
    /// This operation is supported for `I32`, `F64`, and `DateTime` series.
    /// Null values are ignored in the sum calculation.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value))` containing the sum, `Ok(None)` if the series
    /// contains no non-null values, or `Err(VeloxxError::Unsupported)` if the operation
    /// is not supported for the series' data type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("numbers", vec![Some(1), None, Some(3), Some(5)]);
    /// assert_eq!(series.sum().unwrap(), Some(Value::I32(9)));
    ///
    /// let float_series = Series::new_f64("prices", vec![Some(10.5), Some(20.0), None]);
    /// assert_eq!(float_series.sum().unwrap(), Some(Value::F64(30.5)));
    ///
    /// let empty_series = Series::new_i32("empty", vec![]);
    /// assert_eq!(empty_series.sum().unwrap(), None);
    /// ```
    pub fn sum(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let sum_val = data.iter().fold(None, |acc, &x| match (acc, x) {
                    (Some(current_sum), Some(val)) => Some(current_sum + val),
                    (None, Some(val)) => Some(val),
                    (acc, None) => acc,
                });
                Ok(sum_val.map(Value::I32))
            }
            Series::F64(_, data) => {
                let sum_val = data.iter().fold(None, |acc, &x| match (acc, x) {
                    (Some(current_sum), Some(val)) => Some(current_sum + val),
                    (None, Some(val)) => Some(val),
                    (acc, None) => acc,
                });
                Ok(sum_val.map(Value::F64))
            }
            Series::DateTime(_, data) => {
                let sum_val = data.iter().fold(None, |acc, &x| match (acc, x) {
                    (Some(current_sum), Some(val)) => Some(current_sum + val),
                    (None, Some(val)) => Some(val),
                    (acc, None) => acc,
                });
                Ok(sum_val.map(Value::DateTime))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Sum operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Counts the number of non-null values in the series.
    ///
    /// # Returns
    ///
    /// A `usize` representing the count of non-null elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_i32("data", vec![Some(1), None, Some(3), None, Some(5)]);
    /// assert_eq!(series.count(), 3);
    ///
    /// let empty_series = Series::new_string("empty", vec![]);
    /// assert_eq!(empty_series.count(), 0);
    /// ```
    pub fn count(&self) -> usize {
        match self {
            Series::I32(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::F64(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::Bool(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::String(_, data) => data.iter().filter(|x| x.is_some()).count(),
            Series::DateTime(_, data) => data.iter().filter(|x| x.is_some()).count(),
        }
    }

    /// Finds the minimum non-null value in the series.
    ///
    /// This operation is supported for `I32`, `F64`, `DateTime`, and `String` series.
    /// Null values are ignored. For `F64`, `partial_cmp` is used for comparison.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value))` containing the minimum value, `Ok(None)` if the series
    /// contains no non-null values, or `Err(VeloxxError::Unsupported)` if the operation
    /// is not supported for the series' data type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("numbers", vec![Some(5), Some(1), None, Some(3)]);
    /// assert_eq!(series.min().unwrap(), Some(Value::I32(1)));
    ///
    /// let string_series = Series::new_string("words", vec![Some("banana".to_string()), Some("apple".to_string())]);
    /// assert_eq!(string_series.min().unwrap(), Some(Value::String("apple".to_string())));
    /// ```
    pub fn min(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let min_val = data.iter().filter_map(|&x| x).min();
                Ok(min_val.map(Value::I32))
            }
            Series::F64(_, data) => {
                let min_val = data
                    .iter()
                    .filter_map(|&x| x)
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                Ok(min_val.map(Value::F64))
            }
            Series::DateTime(_, data) => {
                let min_val = data.iter().filter_map(|&x| x).min();
                Ok(min_val.map(Value::DateTime))
            }
            Series::String(_, data) => {
                let min_val = data
                    .iter()
                    .filter_map(|x| x.as_ref())
                    .min_by(|a, b| a.cmp(b));
                Ok(min_val.map(|s| Value::String(s.clone())))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Min operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Finds the maximum non-null value in the series.
    ///
    /// This operation is supported for `I32`, `F64`, `DateTime`, and `String` series.
    /// Null values are ignored. For `F64`, `partial_cmp` is used for comparison.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value))` containing the maximum value, `Ok(None)` if the series
    /// contains no non-null values, or `Err(VeloxxError::Unsupported)` if the operation
    /// is not supported for the series' data type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("numbers", vec![Some(5), Some(1), None, Some(3)]);
    /// assert_eq!(series.max().unwrap(), Some(Value::I32(5)));
    ///
    /// let string_series = Series::new_string("words", vec![Some("banana".to_string()), Some("apple".to_string())]);
    /// assert_eq!(string_series.max().unwrap(), Some(Value::String("banana".to_string())));
    /// ```
    pub fn max(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let max_val = data.iter().filter_map(|&x| x).max();
                Ok(max_val.map(Value::I32))
            }
            Series::F64(_, data) => {
                let max_val = data
                    .iter()
                    .filter_map(|&x| x)
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                Ok(max_val.map(Value::F64))
            }
            Series::DateTime(_, data) => {
                let max_val = data.iter().filter_map(|&x| x).max();
                Ok(max_val.map(Value::DateTime))
            }
            Series::String(_, data) => {
                let max_val = data
                    .iter()
                    .filter_map(|x| x.as_ref())
                    .max_by(|a, b| a.cmp(b));
                Ok(max_val.map(|s| Value::String(s.clone())))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Max operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Calculates the mean (average) of all non-null values in the series.
    ///
    /// This operation is supported for `I32`, `F64`, and `DateTime` series.
    /// Null values are ignored. The result is always an `F64` `Value`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value::F64))` containing the mean, `Ok(None)` if the series
    /// contains no non-null values, or `Err(VeloxxError::Unsupported)` if the operation
    /// is not supported for the series' data type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("numbers", vec![Some(1), Some(2), Some(3)]);
    /// assert_eq!(series.mean().unwrap(), Some(Value::F64(2.0)));
    ///
    /// let float_series = Series::new_f64("prices", vec![Some(10.0), Some(20.0), Some(30.0)]);
    /// assert_eq!(float_series.mean().unwrap(), Some(Value::F64(20.0)));
    ///
    /// let empty_series = Series::new_i32("empty", vec![]);
    /// assert_eq!(empty_series.mean().unwrap(), None);
    /// ```
    pub fn mean(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let sum_val: i64 = data.iter().filter_map(|&x| x.map(|v| v as i64)).sum();
                let count_val = data.iter().filter(|&x| x.is_some()).count() as i64;
                if count_val == 0 {
                    Ok(None)
                } else {
                    Ok(Some(Value::F64(sum_val as f64 / count_val as f64)))
                }
            }
            Series::F64(_, data) => {
                let sum_val: f64 = data.iter().filter_map(|&x| x).sum();
                let count_val = data.iter().filter(|&x| x.is_some()).count() as f64;
                if count_val == 0.0 {
                    Ok(None)
                } else {
                    Ok(Some(Value::F64(sum_val / count_val)))
                }
            }
            Series::DateTime(_, data) => {
                let sum_val: i64 = data.iter().filter_map(|&x| x).sum();
                let count_val = data.iter().filter(|&x| x.is_some()).count() as i64;
                if count_val == 0 {
                    Ok(None)
                } else {
                    Ok(Some(Value::F64(sum_val as f64 / count_val as f64)))
                }
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Mean operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Calculates the median of all non-null values in the series.
    ///
    /// This operation is supported for `I32`, `F64`, and `DateTime` series.
    /// Null values are ignored. The data is sorted internally to find the median.
    /// For even-sized datasets, the median is the average of the two middle values.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value))` containing the median, `Ok(None)` if the series
    /// contains no non-null values, or `Err(VeloxxError::Unsupported)` if the operation
    /// is not supported for the series' data type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series_odd = Series::new_i32("numbers", vec![Some(1), Some(5), Some(2), Some(4), Some(3)]);
    /// assert_eq!(series_odd.median().unwrap(), Some(Value::I32(3)));
    ///
    /// let series_even = Series::new_f64("prices", vec![Some(10.0), Some(40.0), Some(20.0), Some(30.0)]);
    /// assert_eq!(series_even.median().unwrap(), Some(Value::F64(25.0)));
    /// ```
    pub fn median(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let mut non_null_data: Vec<i32> = data.iter().filter_map(|&x| x).collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data.sort_unstable();
                let mid = non_null_data.len() / 2;
                if non_null_data.len().is_multiple_of(2) {
                    // Even number of elements
                    let median_val = (non_null_data[mid - 1] + non_null_data[mid]) as f64 / 2.0;
                    Ok(Some(Value::F64(median_val)))
                } else {
                    // Odd number of elements
                    Ok(Some(Value::I32(non_null_data[mid])))
                }
            }
            Series::F64(_, data) => {
                let mut non_null_data: Vec<f64> = data.iter().filter_map(|&x| x).collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let mid = non_null_data.len() / 2;
                if non_null_data.len().is_multiple_of(2) {
                    // Even number of elements
                    let median_val = (non_null_data[mid - 1] + non_null_data[mid]) / 2.0;
                    Ok(Some(Value::F64(median_val)))
                } else {
                    // Odd number of elements
                    Ok(Some(Value::F64(non_null_data[mid])))
                }
            }
            Series::DateTime(_, data) => {
                let mut non_null_data: Vec<i64> = data.iter().filter_map(|&x| x).collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data.sort_unstable();
                let mid = non_null_data.len() / 2;
                if non_null_data.len().is_multiple_of(2) {
                    // Even number of elements
                    let median_val = (non_null_data[mid - 1] + non_null_data[mid]) as f64 / 2.0;
                    Ok(Some(Value::F64(median_val)))
                } else {
                    // Odd number of elements
                    Ok(Some(Value::F64(non_null_data[mid] as f64)))
                }
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Median operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Calculates the standard deviation of all non-null values in the series.
    ///
    /// This operation is supported for `I32` and `F64` series. Null values are ignored.
    /// The standard deviation is calculated using the sample standard deviation formula (N-1 denominator).
    /// Requires at least two non-null values to compute.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value::F64))` containing the standard deviation, `Ok(None)` if the series
    /// has fewer than two non-null values, or `Err(VeloxxError::Unsupported)` if the operation
    /// is not supported for the series' data type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("numbers", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
    /// // Expected standard deviation for [1, 2, 3, 4, 5] is approx 1.5811
    /// let std_dev = series.std_dev().unwrap().unwrap().as_f64().unwrap();
    /// assert!((std_dev - 1.5811).abs() < 0.0001);
    ///
    /// let single_value_series = Series::new_f64("single", vec![Some(10.0)]);
    /// assert_eq!(single_value_series.std_dev().unwrap(), None);
    /// ```
    pub fn std_dev(&self) -> Result<Option<Value>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let non_null_data: Vec<f64> =
                    data.iter().filter_map(|&x| x.map(|v| v as f64)).collect();
                let n = non_null_data.len();
                if n < 2 {
                    return Ok(None);
                }
                let mean = non_null_data.iter().sum::<f64>() / n as f64;
                let variance = non_null_data
                    .iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>()
                    / (n - 1) as f64;
                Ok(Some(Value::F64(variance.sqrt())))
            }
            Series::F64(_, data) => {
                let non_null_data: Vec<f64> = data.iter().filter_map(|&x| x).collect();
                let n = non_null_data.len();
                if n < 2 {
                    return Ok(None);
                }
                let mean = non_null_data.iter().sum::<f64>() / n as f64;
                let variance = non_null_data
                    .iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>()
                    / (n - 1) as f64;
                Ok(Some(Value::F64(variance.sqrt())))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Standard deviation operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Calculates the Pearson correlation coefficient between this series and another series.
    ///
    /// This operation is supported for numeric (`I32`, `F64`) series. Both series must have
    /// the same length. Null values are handled by pairwise deletion (rows with nulls in
    /// either series are excluded from the calculation).
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the `Series` to correlate with.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value::F64))` containing the correlation coefficient,
    /// `Ok(None)` if there are fewer than two valid data points after handling nulls,
    /// `Err(VeloxxError::InvalidOperation)` if series lengths differ, or
    /// `Err(VeloxxError::Unsupported)` if the operation is not supported for the series' data types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let x = Series::new_i32("x", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
    /// let y = Series::new_f64("y", vec![Some(2.0), Some(4.0), Some(5.0), Some(4.0), Some(5.0)]);
    ///
    /// let correlation = x.correlation(&y).unwrap().unwrap().as_f64().unwrap();
    /// // Expected correlation for these values is approx 0.7746
    /// assert!((correlation - 0.7746).abs() < 0.0001);
    ///
    /// let x_with_null = Series::new_i32("x_null", vec![Some(1), None, Some(3)]);
    /// let y_with_null = Series::new_i32("y_null", vec![Some(10), Some(20), None]);
    /// // Only (1, 10) is a valid pair, so correlation cannot be computed (needs at least 2 pairs)
    /// assert!(x_with_null.correlation(&y_with_null).unwrap().is_none());
    /// ```
    pub fn correlation(&self, other: &Series) -> Result<Option<Value>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have the same length for correlation calculation.".to_string(),
            ));
        }

        match (self, other) {
            (Series::I32(_, data1), Series::I32(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| {
                        x.and_then(|x_val| y.map(|y_val| (x_val as f64, y_val as f64)))
                    })
                    .unzip();
                Self::calculate_correlation(&x_vals, &y_vals)
            }
            (Series::F64(_, data1), Series::F64(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val, y_val))))
                    .unzip();
                Self::calculate_correlation(&x_vals, &y_vals)
            }
            (Series::I32(_, data1), Series::F64(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val as f64, y_val))))
                    .unzip();
                Self::calculate_correlation(&x_vals, &y_vals)
            }
            (Series::F64(_, data1), Series::I32(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val, y_val as f64))))
                    .unzip();
                Self::calculate_correlation(&x_vals, &y_vals)
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Correlation not supported for series of types {:?} and {:?}",
                self.data_type(),
                other.data_type()
            ))),
        }
    }

    /// Helper function to calculate Pearson correlation coefficient.
    ///
    /// This private function performs the core calculation given two slices of `f64` values.
    /// It returns `None` if there are fewer than two data points.
    ///
    /// # Arguments
    ///
    /// * `x_vals` - A slice of `f64` values for the first variable.
    /// * `y_vals` - A slice of `f64` values for the second variable.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value::F64))` containing the correlation coefficient,
    /// or `Ok(None)` if `n < 2`.
    fn calculate_correlation(x_vals: &[f64], y_vals: &[f64]) -> Result<Option<Value>, VeloxxError> {
        let n = x_vals.len();
        if n < 2 {
            return Ok(None);
        }

        let sum_x: f64 = x_vals.iter().sum();
        let sum_y: f64 = y_vals.iter().sum();
        let mean_x = sum_x / n as f64;
        let mean_y = sum_y / n as f64;

        let numerator: f64 = x_vals
            .iter()
            .zip(y_vals.iter())
            .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
            .sum();

        let sum_sq_dev_x: f64 = x_vals.iter().map(|&x| (x - mean_x).powi(2)).sum();
        let sum_sq_dev_y: f64 = y_vals.iter().map(|&y| (y - mean_y).powi(2)).sum();

        let denominator = (sum_sq_dev_x * sum_sq_dev_y).sqrt();

        if denominator == 0.0 {
            Ok(Some(Value::F64(0.0))) // Or handle as an error, depending on desired behavior for zero variance
        } else {
            Ok(Some(Value::F64(numerator / denominator)))
        }
    }

    /// Calculates the covariance between this series and another series.
    ///
    /// This operation is supported for numeric (`I32`, `F64`) series. Both series must have
    /// the same length. Null values are handled by pairwise deletion.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the `Series` to calculate covariance with.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value::F64))` containing the covariance,
    /// `Ok(None)` if there are fewer than two valid data points after handling nulls,
    /// `Err(VeloxxError::InvalidOperation)` if series lengths differ, or
    /// `Err(VeloxxError::Unsupported)` if the operation is not supported for the series' data types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let x = Series::new_i32("x", vec![Some(1), Some(2), Some(3)]);
    /// let y = Series::new_f64("y", vec![Some(2.0), Some(3.0), Some(4.0)]);
    ///
    /// // Expected covariance for these values is 1.0
    /// let covariance = x.covariance(&y).unwrap().unwrap().as_f64().unwrap();
    /// assert!((covariance - 1.0).abs() < 0.0001);
    /// ```
    pub fn covariance(&self, other: &Series) -> Result<Option<Value>, VeloxxError> {
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have the same length for covariance calculation.".to_string(),
            ));
        }

        match (self, other) {
            (Series::I32(_, data1), Series::I32(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| {
                        x.and_then(|x_val| y.map(|y_val| (x_val as f64, y_val as f64)))
                    })
                    .unzip();
                Self::calculate_covariance(&x_vals, &y_vals)
            }
            (Series::F64(_, data1), Series::F64(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val, y_val))))
                    .unzip();
                Self::calculate_covariance(&x_vals, &y_vals)
            }
            (Series::I32(_, data1), Series::F64(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val as f64, y_val))))
                    .unzip();
                Self::calculate_covariance(&x_vals, &y_vals)
            }
            (Series::F64(_, data1), Series::I32(_, data2)) => {
                let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = data1
                    .iter()
                    .zip(data2.iter())
                    .filter_map(|(&x, &y)| x.and_then(|x_val| y.map(|y_val| (x_val, y_val as f64))))
                    .unzip();
                Self::calculate_covariance(&x_vals, &y_vals)
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Covariance not supported for series of types {:?} and {:?}",
                self.data_type(),
                other.data_type()
            ))),
        }
    }

    /// Helper function to calculate covariance.
    ///
    /// This private function performs the core calculation given two slices of `f64` values.
    /// It returns `None` if there are fewer than two data points.
    ///
    /// # Arguments
    ///
    /// * `x_vals` - A slice of `f64` values for the first variable.
    /// * `y_vals` - A slice of `f64` values for the second variable.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Some(Value::F64))` containing the covariance,
    /// or `Ok(None)` if `n < 2`.
    fn calculate_covariance(x_vals: &[f64], y_vals: &[f64]) -> Result<Option<Value>, VeloxxError> {
        let n = x_vals.len();
        if n < 2 {
            return Ok(None);
        }

        let sum_x: f64 = x_vals.iter().sum();
        let sum_y: f64 = y_vals.iter().sum();
        let mean_x = sum_x / n as f64;
        let mean_y = sum_y / n as f64;

        let numerator: f64 = x_vals
            .iter()
            .zip(y_vals.iter())
            .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
            .sum();

        Ok(Some(Value::F64(numerator / (n - 1) as f64)))
    }

    /// Returns a new series containing only the unique non-null values from this series.
    ///
    /// The order of unique values is not guaranteed. Null values are treated as a single unique value.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new series with unique values,
    /// or `Err(VeloxxError)` if an unexpected error occurs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("data", vec![Some(1), Some(2), Some(1), None, Some(3), None]);
    /// let unique_series = series.unique().unwrap();
    ///
    /// // The order of elements in unique_series is not guaranteed, but it should contain:
    /// // Some(1), Some(2), Some(3), None
    /// assert_eq!(unique_series.len(), 4);
    ///
    /// let string_series = Series::new_string("fruits", vec![Some("apple".to_string()), Some("banana".to_string()), Some("apple".to_string())]);
    /// let unique_string_series = string_series.unique().unwrap();
    /// assert_eq!(unique_string_series.len(), 2);
    /// ```
    pub fn unique(&self) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                let mut unique_set = HashSet::new();
                let unique_data: Vec<Option<i32>> = data
                    .iter()
                    .filter_map(|&x| if unique_set.insert(x) { Some(x) } else { None })
                    .collect();
                Ok(Series::I32(name, unique_data))
            }
            Series::F64(_, data) => {
                let mut unique_bits = HashSet::new();
                let mut unique_data: Vec<Option<f64>> = Vec::new();
                let mut has_null = false;

                for &val_opt in data.iter() {
                    match val_opt {
                        Some(f_val) => {
                            if unique_bits.insert(f_val.to_bits()) {
                                unique_data.push(Some(f_val));
                            }
                        }
                        None => {
                            if !has_null {
                                unique_data.push(None);
                                has_null = true;
                            }
                        }
                    }
                }
                Ok(Series::F64(name, unique_data))
            }
            Series::Bool(_, data) => {
                let mut unique_set = HashSet::new();
                let unique_data: Vec<Option<bool>> = data
                    .iter()
                    .filter_map(|&x| if unique_set.insert(x) { Some(x) } else { None })
                    .collect();
                Ok(Series::Bool(name, unique_data))
            }
            Series::String(_, data) => {
                let mut unique_set = HashSet::new();
                let unique_data: Vec<Option<String>> = data
                    .iter()
                    .filter_map(|x| {
                        if unique_set.insert(x.clone()) {
                            Some(x.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                Ok(Series::String(name, unique_data))
            }
            Series::DateTime(_, data) => {
                let mut unique_set = HashSet::new();
                let unique_data: Vec<Option<i64>> = data
                    .iter()
                    .filter_map(|&x| if unique_set.insert(x) { Some(x) } else { None })
                    .collect();
                Ok(Series::DateTime(name, unique_data))
            }
        }
    }

    /// Converts the series data to a `Vec<f64>`, ignoring null values.
    ///
    /// This method is useful for operations that require a flat `Vec<f64>` representation
    /// of the series' numeric data. Only `I32`, `F64`, and `DateTime` series can be converted.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Vec<f64>)` containing the non-null values as `f64`,
    /// or `Err(VeloxxError::Unsupported)` if the series' data type cannot be converted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_i32("numbers", vec![Some(1), None, Some(2), Some(3)]);
    /// let vec_f64 = series.to_vec_f64().unwrap();
    /// assert_eq!(vec_f64, vec![1.0, 2.0, 3.0]);
    ///
    /// let float_series = Series::new_f64("prices", vec![Some(10.5), None, Some(20.0)]);
    /// let vec_f64_float = float_series.to_vec_f64().unwrap();
    /// assert_eq!(vec_f64_float, vec![10.5, 20.0]);
    /// ```
    pub fn to_vec_f64(&self) -> Result<Vec<f64>, VeloxxError> {
        match self {
            Series::I32(_, data) => {
                let mut vec_f64 = Vec::with_capacity(data.len());
                for &val_opt in data {
                    if let Some(val) = val_opt {
                        vec_f64.push(val as f64);
                    }
                }
                Ok(vec_f64)
            }
            Series::F64(_, data) => {
                let mut vec_f64 = Vec::with_capacity(data.len());
                for &val_opt in data {
                    if let Some(val) = val_opt {
                        vec_f64.push(val);
                    }
                }
                Ok(vec_f64)
            }
            Series::DateTime(_, data) => {
                let mut vec_f64 = Vec::with_capacity(data.len());
                for &val_opt in data {
                    if let Some(val) = val_opt {
                        vec_f64.push(val as f64);
                    }
                }
                Ok(vec_f64)
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Cannot convert series of type {:?} to Vec<f64>.",
                self.data_type()
            ))),
        }
    }

    /// Interpolates null values in the series using linear interpolation.
    ///
    /// This operation is only supported for numeric (`I32`, `F64`) and `DateTime` series.
    /// Nulls at the beginning or end of the series, or consecutive nulls
    /// where no surrounding non-null values exist, will remain null.
    ///
    /// For `I32` and `DateTime` series, the interpolated values are cast back to their
    /// original integer types, which may involve truncation.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new series with interpolated values,
    /// or `Err(VeloxxError::Unsupported)` if the operation is not supported for the series' data type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("data", vec![Some(10), None, None, Some(40), None, Some(60)]);
    /// let interpolated_series = series.interpolate_nulls().unwrap();
    ///
    /// assert_eq!(interpolated_series.get_value(0), Some(Value::I32(10)));
    /// assert_eq!(interpolated_series.get_value(1), Some(Value::I32(20)));
    /// assert_eq!(interpolated_series.get_value(2), Some(Value::I32(30)));
    /// assert_eq!(interpolated_series.get_value(3), Some(Value::I32(40)));
    /// assert_eq!(interpolated_series.get_value(4), Some(Value::I32(50)));
    /// assert_eq!(interpolated_series.get_value(5), Some(Value::I32(60)));
    ///
    /// let series_leading_nulls = Series::new_f64("data", vec![None, None, Some(3.0), Some(4.0)]);
    /// let interpolated_leading_nulls = series_leading_nulls.interpolate_nulls().unwrap();
    /// assert_eq!(interpolated_leading_nulls.get_value(0), None);
    /// assert_eq!(interpolated_leading_nulls.get_value(1), None);
    /// assert_eq!(interpolated_leading_nulls.get_value(2), Some(Value::F64(3.0)));
    /// ```
    pub fn interpolate_nulls(&self) -> Result<Self, VeloxxError> {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                let mut interpolated_data = Vec::with_capacity(data.len());
                let mut last_known_idx: Option<usize> = None;
                let mut next_known_indices: Vec<Option<usize>> = vec![None; data.len()];
                let mut last_non_null_idx: Option<usize> = None;

                // Pre-calculate next known indices
                for i in (0..data.len()).rev() {
                    if data[i].is_some() {
                        last_non_null_idx = Some(i);
                    }
                    next_known_indices[i] = last_non_null_idx;
                }

                // Forward pass for interpolation
                for i in 0..data.len() {
                    if data[i].is_some() {
                        interpolated_data.push(data[i]);
                        last_known_idx = Some(i);
                    } else if let Some(prev_idx) = last_known_idx {
                        if let Some(next_idx) = next_known_indices[i] {
                            let prev_val = data[prev_idx].unwrap() as f64;
                            let next_val = data[next_idx].unwrap() as f64;
                            let interpolated_val = prev_val
                                + (next_val - prev_val)
                                    * ((i - prev_idx) as f64 / (next_idx - prev_idx) as f64);
                            interpolated_data.push(Some(interpolated_val as i32));
                        } else {
                            interpolated_data.push(None);
                        }
                    } else {
                        interpolated_data.push(None);
                    }
                }
                Ok(Series::I32(name, interpolated_data))
            }
            Series::F64(_, data) => {
                let mut interpolated_data = Vec::with_capacity(data.len());
                let mut last_known_idx: Option<usize> = None;
                let mut next_known_indices: Vec<Option<usize>> = vec![None; data.len()];
                let mut last_non_null_idx: Option<usize> = None;

                // Pre-calculate next known indices
                for i in (0..data.len()).rev() {
                    if data[i].is_some() {
                        last_non_null_idx = Some(i);
                    }
                    next_known_indices[i] = last_non_null_idx;
                }

                // Forward pass for interpolation
                for i in 0..data.len() {
                    if data[i].is_some() {
                        interpolated_data.push(data[i]);
                        last_known_idx = Some(i);
                    } else if let Some(prev_idx) = last_known_idx {
                        if let Some(next_idx) = next_known_indices[i] {
                            let prev_val = data[prev_idx].unwrap();
                            let next_val = data[next_idx].unwrap();
                            let interpolated_val = prev_val
                                + (next_val - prev_val)
                                    * ((i - prev_idx) as f64 / (next_idx - prev_idx) as f64);
                            interpolated_data.push(Some(interpolated_val));
                        } else {
                            interpolated_data.push(None);
                        }
                    } else {
                        interpolated_data.push(None);
                    }
                }
                Ok(Series::F64(name, interpolated_data))
            }
            Series::DateTime(_, data) => {
                let mut interpolated_data = data.clone();
                let mut last_known_idx: Option<usize> = None;

                // Forward pass
                for i in 0..interpolated_data.len() {
                    if interpolated_data[i].is_some() {
                        last_known_idx = Some(i);
                    } else if let Some(prev_idx) = last_known_idx {
                        // Find next non-null value
                        let next_known_idx =
                            (i..interpolated_data.len()).find(|&j| interpolated_data[j].is_some());

                        if let Some(next_idx) = next_known_idx {
                            let prev_val = interpolated_data[prev_idx].unwrap() as f64;
                            let next_val = interpolated_data[next_idx].unwrap() as f64;
                            let interpolated_val = prev_val
                                + (next_val - prev_val)
                                    * ((i - prev_idx) as f64 / (next_idx - prev_idx) as f64);
                            interpolated_data[i] = Some(interpolated_val as i64);
                        }
                    }
                }
                Ok(Series::DateTime(name, interpolated_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Interpolate nulls operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of an `I32` series, returning a new `I32` series.
    ///
    /// This method provides a type-specific way to transform each element of an `I32` series.
    /// The provided closure `f` takes an `Option<i32>` and returns an `Option<i32>`,
    /// allowing for handling of null values and producing new nulls.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes an `Option<i32>` and returns an `Option<i32>`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new transformed series,
    /// or `Err(VeloxxError::Unsupported)` if the series is not of type `I32`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_i32("numbers", vec![Some(1), Some(2), None, Some(4)]);
    /// let doubled_series = series.apply_i32(|x| x.map(|val| val * 2)).unwrap();
    ///
    /// assert_eq!(doubled_series.get_value(0), Some(Value::I32(2)));
    /// assert_eq!(doubled_series.get_value(1), Some(Value::I32(4)));
    /// assert_eq!(doubled_series.get_value(2), None);
    /// assert_eq!(doubled_series.get_value(3), Some(Value::I32(8)));
    /// ```
    pub fn apply_i32<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<i32>) -> Option<i32>,
    {
        let name = self.name().to_string();
        match self {
            Series::I32(_, data) => {
                let new_data = data.iter().map(|&x| f(x)).collect();
                Ok(Series::I32(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_i32.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of an `F64` series, returning a new `F64` series.
    ///
    /// This method provides a type-specific way to transform each element of an `F64` series.
    /// The provided closure `f` takes an `Option<f64>` and returns an `Option<f64>`,
    /// allowing for handling of null values and producing new nulls.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes an `Option<f64>` and returns an `Option<f64>`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new transformed series,
    /// or `Err(VeloxxError::Unsupported)` if the series is not of type `F64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_f64("prices", vec![Some(10.0), Some(20.5), None, Some(30.0)]);
    /// let half_prices_series = series.apply_f64(|x| x.map(|val| val / 2.0)).unwrap();
    ///
    /// assert_eq!(half_prices_series.get_value(0), Some(Value::F64(5.0)));
    /// assert_eq!(half_prices_series.get_value(1), Some(Value::F64(10.25)));
    /// assert_eq!(half_prices_series.get_value(2), None);
    /// ```
    pub fn apply_f64<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<f64>) -> Option<f64>,
    {
        let name = self.name().to_string();
        match self {
            Series::F64(_, data) => {
                let new_data = data.iter().map(|&x| f(x)).collect();
                Ok(Series::F64(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_f64.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of a `Bool` series, returning a new `Bool` series.
    ///
    /// This method provides a type-specific way to transform each element of a `Bool` series.
    /// The provided closure `f` takes an `Option<bool>` and returns an `Option<bool>`,
    /// allowing for handling of null values and producing new nulls.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes an `Option<bool>` and returns an `Option<bool>`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new transformed series,
    /// or `Err(VeloxxError::Unsupported)` if the series is not of type `Bool`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_bool("flags", vec![Some(true), Some(false), None]);
    /// let inverted_series = series.apply_bool(|x| x.map(|val| !val)).unwrap();
    ///
    /// assert_eq!(inverted_series.get_value(0), Some(Value::Bool(false)));
    /// assert_eq!(inverted_series.get_value(1), Some(Value::Bool(true)));
    /// assert_eq!(inverted_series.get_value(2), None);
    /// ```
    pub fn apply_bool<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<bool>) -> Option<bool>,
    {
        let name = self.name().to_string();
        match self {
            Series::Bool(_, data) => {
                let new_data = data.iter().map(|&x| f(x)).collect();
                Ok(Series::Bool(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_bool.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of a `String` series, returning a new `String` series.
    ///
    /// This method provides a type-specific way to transform each element of a `String` series.
    /// The provided closure `f` takes an `Option<&String>` and returns an `Option<String>`,
    /// allowing for handling of null values and producing new nulls.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes an `Option<&String>` and returns an `Option<String>`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new transformed series,
    /// or `Err(VeloxxError::Unsupported)` if the series is not of type `String`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// let series = Series::new_string("names", vec![Some("Alice".to_string()), Some("Bob".to_string()), None]);
    /// let upper_case_series = series.apply_string(|x| x.map(|s| s.to_uppercase())).unwrap();
    ///
    /// assert_eq!(upper_case_series.get_value(0), Some(Value::String("ALICE".to_string())));
    /// assert_eq!(upper_case_series.get_value(1), Some(Value::String("BOB".to_string())));
    /// assert_eq!(upper_case_series.get_value(2), None);
    /// ```
    pub fn apply_string<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<&String>) -> Option<String>,
    {
        let name = self.name().to_string();
        match self {
            Series::String(_, data) => {
                let new_data = data.iter().map(|x| f(x.as_ref())).collect();
                Ok(Series::String(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_string.",
                self.data_type()
            ))),
        }
    }

    /// Applies a function to each element of a `DateTime` series, returning a new `DateTime` series.
    ///
    /// This method provides a type-specific way to transform each element of a `DateTime` series.
    /// The provided closure `f` takes an `Option<i64>` (Unix timestamp) and returns an `Option<i64>`,
    /// allowing for handling of null values and producing new nulls.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes an `Option<i64>` and returns an `Option<i64>`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Self)` containing the new transformed series,
    /// or `Err(VeloxxError::Unsupported)` if the series is not of type `DateTime`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    /// use veloxx::types::Value;
    ///
    /// // Example: Add one day (86400 seconds) to each timestamp
    /// let series = Series::new_datetime("timestamps", vec![Some(1672531200), None, Some(1672617600)]);
    /// let next_day_series = series.apply_datetime(|x| x.map(|ts| ts + 86400)).unwrap();
    ///
    /// assert_eq!(next_day_series.get_value(0), Some(Value::DateTime(1672531200 + 86400)));
    /// assert_eq!(next_day_series.get_value(1), None);
    /// ```
    pub fn apply_datetime<F>(&self, f: F) -> Result<Self, VeloxxError>
    where
        F: Fn(Option<i64>) -> Option<i64>,
    {
        let name = self.name().to_string();
        match self {
            Series::DateTime(_, data) => {
                let new_data = data.iter().map(|&x| f(x)).collect();
                Ok(Series::DateTime(name, new_data))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Apply operation not supported for {:?} series with apply_datetime.",
                self.data_type()
            ))),
        }
    }
}
