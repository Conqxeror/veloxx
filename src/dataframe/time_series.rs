use crate::dataframe::DataFrame;
use crate::VeloxxError;

#[cfg(test)]
use crate::series::Series;
#[cfg(test)]
use crate::types::Value;
#[cfg(test)]
use std::collections::HashMap;

impl DataFrame {
    /// Applies rolling mean to specified numeric columns in the DataFrame.
    ///
    /// This method creates new columns with rolling mean calculations for the specified columns.
    /// The new columns are named with the pattern "{original_name}_rolling_mean_{window_size}".
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of column names to apply rolling mean to
    /// * `window_size` - The size of the rolling window
    ///
    /// # Returns
    ///
    /// A new `DataFrame` with the original columns plus the new rolling mean columns
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::HashMap;
    ///
    /// let mut columns = HashMap::new();
    /// columns.insert("price".to_string(), Series::new_f64("price", vec![Some(10.0), Some(15.0), Some(12.0), Some(18.0)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let result = df.rolling_mean(vec!["price".to_string()], 3).unwrap();
    /// ```
    pub fn rolling_mean(
        &self,
        columns: Vec<String>,
        window_size: usize,
    ) -> Result<DataFrame, VeloxxError> {
        let mut new_columns = self.columns.clone();

        for column_name in columns {
            let series = self
                .get_column(&column_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.clone()))?;

            let rolling_series = series.rolling_mean(window_size)?;
            new_columns.insert(rolling_series.name().to_string(), rolling_series);
        }

        DataFrame::new(new_columns)
    }

    /// Applies rolling sum to specified numeric columns in the DataFrame.
    ///
    /// This method creates new columns with rolling sum calculations for the specified columns.
    /// The new columns are named with the pattern "{original_name}_rolling_sum_{window_size}".
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of column names to apply rolling sum to
    /// * `window_size` - The size of the rolling window
    ///
    /// # Returns
    ///
    /// A new `DataFrame` with the original columns plus the new rolling sum columns
    pub fn rolling_sum(
        &self,
        columns: Vec<String>,
        window_size: usize,
    ) -> Result<DataFrame, VeloxxError> {
        let mut new_columns = self.columns.clone();

        for column_name in columns {
            let series = self
                .get_column(&column_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.clone()))?;

            let rolling_series = series.rolling_sum(window_size)?;
            new_columns.insert(rolling_series.name().to_string(), rolling_series);
        }

        DataFrame::new(new_columns)
    }

    /// Applies rolling minimum to specified numeric columns in the DataFrame.
    ///
    /// This method creates new columns with rolling minimum calculations for the specified columns.
    /// The new columns are named with the pattern "{original_name}_rolling_min_{window_size}".
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of column names to apply rolling minimum to
    /// * `window_size` - The size of the rolling window
    ///
    /// # Returns
    ///
    /// A new `DataFrame` with the original columns plus the new rolling minimum columns
    pub fn rolling_min(
        &self,
        columns: Vec<String>,
        window_size: usize,
    ) -> Result<DataFrame, VeloxxError> {
        let mut new_columns = self.columns.clone();

        for column_name in columns {
            let series = self
                .get_column(&column_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.clone()))?;

            let rolling_series = series.rolling_min(window_size)?;
            new_columns.insert(rolling_series.name().to_string(), rolling_series);
        }

        DataFrame::new(new_columns)
    }

    /// Applies rolling maximum to specified numeric columns in the DataFrame.
    ///
    /// This method creates new columns with rolling maximum calculations for the specified columns.
    /// The new columns are named with the pattern "{original_name}_rolling_max_{window_size}".
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of column names to apply rolling maximum to
    /// * `window_size` - The size of the rolling window
    ///
    /// # Returns
    ///
    /// A new `DataFrame` with the original columns plus the new rolling maximum columns
    pub fn rolling_max(
        &self,
        columns: Vec<String>,
        window_size: usize,
    ) -> Result<DataFrame, VeloxxError> {
        let mut new_columns = self.columns.clone();

        for column_name in columns {
            let series = self
                .get_column(&column_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.clone()))?;

            let rolling_series = series.rolling_max(window_size)?;
            new_columns.insert(rolling_series.name().to_string(), rolling_series);
        }

        DataFrame::new(new_columns)
    }

    /// Applies rolling standard deviation to specified numeric columns in the DataFrame.
    ///
    /// This method creates new columns with rolling standard deviation calculations for the specified columns.
    /// The new columns are named with the pattern "{original_name}_rolling_std_{window_size}".
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of column names to apply rolling standard deviation to
    /// * `window_size` - The size of the rolling window (must be at least 2)
    ///
    /// # Returns
    ///
    /// A new `DataFrame` with the original columns plus the new rolling standard deviation columns
    pub fn rolling_std(
        &self,
        columns: Vec<String>,
        window_size: usize,
    ) -> Result<DataFrame, VeloxxError> {
        let mut new_columns = self.columns.clone();

        for column_name in columns {
            let series = self
                .get_column(&column_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.clone()))?;

            let rolling_series = series.rolling_std(window_size)?;
            new_columns.insert(rolling_series.name().to_string(), rolling_series);
        }

        DataFrame::new(new_columns)
    }

    /// Calculates percentage change between consecutive values for specified numeric columns.
    ///
    /// This method creates new columns with percentage change calculations.
    /// The new columns are named with the pattern "{original_name}_pct_change".
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of column names to calculate percentage change for
    ///
    /// # Returns
    ///
    /// A new `DataFrame` with the original columns plus the new percentage change columns
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use std::collections::HashMap;
    ///
    /// let mut columns = HashMap::new();
    /// columns.insert("price".to_string(), Series::new_f64("price", vec![Some(100.0), Some(110.0), Some(99.0)]));
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let result = df.pct_change(vec!["price".to_string()]).unwrap();
    /// ```
    pub fn pct_change(&self, columns: Vec<String>) -> Result<DataFrame, VeloxxError> {
        let mut new_columns = self.columns.clone();

        for column_name in columns {
            let series = self
                .get_column(&column_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.clone()))?;

            let pct_change_series = series.pct_change()?;
            new_columns.insert(pct_change_series.name().to_string(), pct_change_series);
        }

        DataFrame::new(new_columns)
    }

    /// Calculates cumulative sum for specified numeric columns.
    ///
    /// This method creates new columns with cumulative sum calculations.
    /// The new columns are named with the pattern "{original_name}_cumsum".
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of column names to calculate cumulative sum for
    ///
    /// # Returns
    ///
    /// A new `DataFrame` with the original columns plus the new cumulative sum columns
    pub fn cumsum(&self, columns: Vec<String>) -> Result<DataFrame, VeloxxError> {
        let mut new_columns = self.columns.clone();

        for column_name in columns {
            let series = self
                .get_column(&column_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.clone()))?;

            let cumsum_series = series.cumsum()?;
            new_columns.insert(cumsum_series.name().to_string(), cumsum_series);
        }

        DataFrame::new(new_columns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataframe_rolling_mean() {
        let mut columns = HashMap::new();
        columns.insert(
            "price".to_string(),
            Series::new_f64(
                "price",
                vec![Some(10.0), Some(15.0), Some(12.0), Some(18.0), Some(20.0)],
            ),
        );
        columns.insert(
            "volume".to_string(),
            Series::new_i32(
                "volume",
                vec![Some(100), Some(150), Some(120), Some(180), Some(200)],
            ),
        );
        let df = DataFrame::new(columns).unwrap();

        let result = df
            .rolling_mean(vec!["price".to_string(), "volume".to_string()], 3)
            .unwrap();

        assert_eq!(result.column_count(), 4); // original 2 + 2 new rolling mean columns
        assert!(result
            .column_names()
            .contains(&&"price_rolling_mean_3".to_string()));
        assert!(result
            .column_names()
            .contains(&&"volume_rolling_mean_3".to_string()));

        let price_rolling = result.get_column("price_rolling_mean_3").unwrap();
        
        // Check rolling mean values using get_value method
        assert_eq!(price_rolling.get_value(0), None);
        assert_eq!(price_rolling.get_value(1), None);
        if let Some(val) = price_rolling.get_value(2) {
            if let Value::F64(v) = val {
                assert!((v - 12.333333333333334).abs() < 1e-10); // (10+15+12)/3
            } else {
                panic!("Expected F64 value");
            }
        } else {
            panic!("Expected Some value");
        }
    }

    #[test]
    fn test_dataframe_pct_change() {
        let mut columns = HashMap::new();
        columns.insert(
            "price".to_string(),
            Series::new_f64(
                "price",
                vec![Some(100.0), Some(110.0), Some(99.0), Some(108.9)],
            ),
        );
        let df = DataFrame::new(columns).unwrap();

        let result = df.pct_change(vec!["price".to_string()]).unwrap();

        assert_eq!(result.column_count(), 2); // original 1 + 1 new pct_change column
        assert!(result
            .column_names()
            .contains(&&"price_pct_change".to_string()));

        let pct_change = result.get_column("price_pct_change").unwrap();
        
        // Check pct_change values using get_value method
        assert_eq!(pct_change.get_value(0), None);
        
        if let Some(val) = pct_change.get_value(1) {
            if let Value::F64(v) = val {
                assert!((v - 0.1).abs() < 1e-10); // 10% increase
            } else {
                panic!("Expected F64 value");
            }
        } else {
            panic!("Expected Some value for index 1");
        }
        
        if let Some(val) = pct_change.get_value(2) {
            if let Value::F64(v) = val {
                assert!((v - (-0.1)).abs() < 1e-10); // 10% decrease
            } else {
                panic!("Expected F64 value");
            }
        } else {
            panic!("Expected Some value for index 2");
        }
    }

    #[test]
    fn test_dataframe_cumsum() {
        let mut columns = HashMap::new();
        columns.insert(
            "sales".to_string(),
            Series::new_i32("sales", vec![Some(10), Some(20), Some(15), Some(25)]),
        );
        let df = DataFrame::new(columns).unwrap();

        let result = df.cumsum(vec!["sales".to_string()]).unwrap();

        assert_eq!(result.column_count(), 2); // original 1 + 1 new cumsum column
        assert!(result.column_names().contains(&&"sales_cumsum".to_string()));

        let cumsum = result.get_column("sales_cumsum").unwrap();
        
        // Check cumsum values using get_value method
        if let Some(val) = cumsum.get_value(0) {
            if let Value::I32(v) = val {
                assert_eq!(v, 10);
            } else {
                panic!("Expected I32 value");
            }
        } else {
            panic!("Expected Some value for index 0");
        }
        
        if let Some(val) = cumsum.get_value(1) {
            if let Value::I32(v) = val {
                assert_eq!(v, 30);
            } else {
                panic!("Expected I32 value");
            }
        } else {
            panic!("Expected Some value for index 1");
        }
        
        if let Some(val) = cumsum.get_value(2) {
            if let Value::I32(v) = val {
                assert_eq!(v, 45);
            } else {
                panic!("Expected I32 value");
            }
        } else {
            panic!("Expected Some value for index 2");
        }
        
        if let Some(val) = cumsum.get_value(3) {
            if let Value::I32(v) = val {
                assert_eq!(v, 70);
            } else {
                panic!("Expected I32 value");
            }
        } else {
            panic!("Expected Some value for index 3");
        }
    }

    #[test]
    fn test_dataframe_rolling_operations_error() {
        let mut columns = HashMap::new();
        columns.insert(
            "price".to_string(),
            Series::new_f64("price", vec![Some(10.0), Some(15.0)]),
        );
        let df = DataFrame::new(columns).unwrap();

        // Test with non-existent column
        let result = df.rolling_mean(vec!["non_existent".to_string()], 2);
        assert!(result.is_err());

        // Test with window size larger than data
        let result = df.rolling_mean(vec!["price".to_string()], 5);
        assert!(result.is_err());
    }
}
