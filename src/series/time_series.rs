use crate::series::Series;
use crate::VeloxxError;

impl Series {
    /// Calculates a rolling mean (moving average) over a specified window size.
    ///
    /// This function computes the mean of values within a sliding window of the specified size.
    /// For numeric series (I32, F64), it returns a new F64 series with the rolling means.
    /// For non-numeric series, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `window_size` - The size of the rolling window. Must be greater than 0.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Series` with rolling means, or a `VeloxxError` if:
    /// - The window size is 0 or greater than the series length
    /// - The series contains non-numeric data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_f64("values", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)]);
    /// let rolling_mean = series.rolling_mean(3).unwrap();
    /// // Result: [None, None, Some(2.0), Some(3.0), Some(4.0)]
    /// ```
    pub fn rolling_mean(&self, window_size: usize) -> Result<Series, VeloxxError> {
        if window_size == 0 {
            return Err(VeloxxError::InvalidOperation(
                "Window size must be greater than 0".to_string(),
            ));
        }

        if window_size > self.len() {
            return Err(VeloxxError::InvalidOperation(
                "Window size cannot be greater than series length".to_string(),
            ));
        }

        let new_name = format!("{}_rolling_mean_{}", self.name(), window_size);

        match self {
            Series::I32(_name, data, validity) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        // Collect only valid values in the window
                        let mut valid_values = Vec::new();
                        for j in window_start..=i {
                            if validity[j] {
                                valid_values.push(data[j]);
                            }
                        }

                        if valid_values.is_empty() {
                            result.push(None);
                        } else {
                            let sum: i32 = valid_values.iter().sum();
                            let mean = sum as f64 / valid_values.len() as f64;
                            result.push(Some(mean));
                        }
                    }
                }

                let result_validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let result_values: Vec<f64> =
                    result.into_iter().map(|x| x.unwrap_or(0.0)).collect();
                Ok(Series::F64(
                    new_name.clone(),
                    result_values,
                    result_validity,
                ))
            }
            Series::F64(_name, data, validity) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        // Collect only valid values in the window
                        let mut valid_values = Vec::new();
                        for j in window_start..=i {
                            if validity[j] {
                                valid_values.push(data[j]);
                            }
                        }

                        if valid_values.is_empty() {
                            result.push(None);
                        } else {
                            let sum: f64 = valid_values.iter().sum();
                            let mean = sum / valid_values.len() as f64;
                            result.push(Some(mean));
                        }
                    }
                }

                let result_validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let result_values: Vec<f64> =
                    result.into_iter().map(|x| x.unwrap_or(0.0)).collect();
                Ok(Series::F64(new_name, result_values, result_validity))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Rolling mean is only supported for numeric series (I32, F64)".to_string(),
            )),
        }
    }

    /// Calculates a rolling sum over a specified window size.
    ///
    /// This function computes the sum of values within a sliding window of the specified size.
    /// For numeric series (I32, F64), it returns a new series of the same type with the rolling sums.
    /// For non-numeric series, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `window_size` - The size of the rolling window. Must be greater than 0.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Series` with rolling sums, or a `VeloxxError` if:
    /// - The window size is 0 or greater than the series length
    /// - The series contains non-numeric data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_i32("values", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
    /// let rolling_sum = series.rolling_sum(3).unwrap();
    /// // Result: [None, None, Some(6), Some(9), Some(12)]
    /// ```
    pub fn rolling_sum(&self, window_size: usize) -> Result<Series, VeloxxError> {
        if window_size == 0 {
            return Err(VeloxxError::InvalidOperation(
                "Window size must be greater than 0".to_string(),
            ));
        }

        if window_size > self.len() {
            return Err(VeloxxError::InvalidOperation(
                "Window size cannot be greater than series length".to_string(),
            ));
        }

        let name = format!("{}_rolling_sum_{}", self.name(), window_size);

        match self {
            Series::I32(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        let window_data: Vec<i32> = data[window_start..=i].to_vec();

                        if window_data.is_empty() {
                            result.push(None);
                        } else {
                            let sum: i32 = window_data.iter().sum();
                            result.push(Some(sum));
                        }
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<i32> = result.into_iter().map(|x| x.unwrap_or(0)).collect();
                Ok(Series::I32(name, values, validity))
            }
            Series::F64(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        let window_data: Vec<f64> = data[window_start..=i].to_vec();

                        if window_data.is_empty() {
                            result.push(None);
                        } else {
                            let sum: f64 = window_data.iter().sum();
                            result.push(Some(sum));
                        }
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<f64> = result.into_iter().map(|x| x.unwrap_or(0.0)).collect();
                Ok(Series::F64(name, values, validity))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Rolling sum is only supported for numeric series (I32, F64)".to_string(),
            )),
        }
    }

    /// Calculates a rolling minimum over a specified window size.
    ///
    /// This function finds the minimum value within a sliding window of the specified size.
    /// For numeric series (I32, F64), it returns a new series of the same type with the rolling minimums.
    /// For non-numeric series, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `window_size` - The size of the rolling window. Must be greater than 0.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Series` with rolling minimums, or a `VeloxxError` if:
    /// - The window size is 0 or greater than the series length
    /// - The series contains non-numeric data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_i32("values", vec![Some(5), Some(2), Some(8), Some(1), Some(9)]);
    /// let rolling_min = series.rolling_min(3).unwrap();
    /// // Result: [None, None, Some(2), Some(1), Some(1)]
    /// ```
    pub fn rolling_min(&self, window_size: usize) -> Result<Series, VeloxxError> {
        if window_size == 0 {
            return Err(VeloxxError::InvalidOperation(
                "Window size must be greater than 0".to_string(),
            ));
        }

        if window_size > self.len() {
            return Err(VeloxxError::InvalidOperation(
                "Window size cannot be greater than series length".to_string(),
            ));
        }

        let name = format!("{}_rolling_min_{}", self.name(), window_size);

        match self {
            Series::I32(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        let window_data: Vec<i32> = data[window_start..=i].to_vec();

                        if window_data.is_empty() {
                            result.push(None);
                        } else {
                            let min = *window_data.iter().min().unwrap();
                            result.push(Some(min));
                        }
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<i32> = result.into_iter().map(|x| x.unwrap_or(0)).collect();
                Ok(Series::I32(name, values, validity))
            }
            Series::F64(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        let window_data: Vec<f64> = data[window_start..=i].to_vec();

                        if window_data.is_empty() {
                            result.push(None);
                        } else {
                            let min = window_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                            result.push(Some(min));
                        }
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<f64> = result.into_iter().map(|x| x.unwrap_or(0.0)).collect();
                Ok(Series::F64(name, values, validity))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Rolling min is only supported for numeric series (I32, F64)".to_string(),
            )),
        }
    }

    /// Calculates a rolling maximum over a specified window size.
    ///
    /// This function finds the maximum value within a sliding window of the specified size.
    /// For numeric series (I32, F64), it returns a new series of the same type with the rolling maximums.
    /// For non-numeric series, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `window_size` - The size of the rolling window. Must be greater than 0.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Series` with rolling maximums, or a `VeloxxError` if:
    /// - The window size is 0 or greater than the series length
    /// - The series contains non-numeric data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_i32("values", vec![Some(5), Some(2), Some(8), Some(1), Some(9)]);
    /// let rolling_max = series.rolling_max(3).unwrap();
    /// // Result: [None, None, Some(8), Some(8), Some(9)]
    /// ```
    pub fn rolling_max(&self, window_size: usize) -> Result<Series, VeloxxError> {
        if window_size == 0 {
            return Err(VeloxxError::InvalidOperation(
                "Window size must be greater than 0".to_string(),
            ));
        }

        if window_size > self.len() {
            return Err(VeloxxError::InvalidOperation(
                "Window size cannot be greater than series length".to_string(),
            ));
        }

        let name = format!("{}_rolling_max_{}", self.name(), window_size);

        match self {
            Series::I32(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        let window_data: Vec<i32> = data[window_start..=i].to_vec();

                        if window_data.is_empty() {
                            result.push(None);
                        } else {
                            let max = *window_data.iter().max().unwrap();
                            result.push(Some(max));
                        }
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<i32> = result.into_iter().map(|x| x.unwrap_or(0)).collect();
                Ok(Series::I32(name, values, validity))
            }
            Series::F64(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        let window_data: Vec<f64> = data[window_start..=i].to_vec();

                        if window_data.is_empty() {
                            result.push(None);
                        } else {
                            let max = window_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                            result.push(Some(max));
                        }
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<f64> = result.into_iter().map(|x| x.unwrap_or(0.0)).collect();
                Ok(Series::F64(name, values, validity))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Rolling max is only supported for numeric series (I32, F64)".to_string(),
            )),
        }
    }

    /// Calculates a rolling standard deviation over a specified window size.
    ///
    /// This function computes the standard deviation of values within a sliding window of the specified size.
    /// For numeric series (I32, F64), it returns a new F64 series with the rolling standard deviations.
    /// For non-numeric series, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `window_size` - The size of the rolling window. Must be greater than 1.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Series` with rolling standard deviations, or a `VeloxxError` if:
    /// - The window size is less than 2 or greater than the series length
    /// - The series contains non-numeric data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_f64("values", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)]);
    /// let rolling_std = series.rolling_std(3).unwrap();
    /// ```
    pub fn rolling_std(&self, window_size: usize) -> Result<Series, VeloxxError> {
        if window_size < 2 {
            return Err(VeloxxError::InvalidOperation(
                "Window size must be at least 2 for standard deviation".to_string(),
            ));
        }

        if window_size > self.len() {
            return Err(VeloxxError::InvalidOperation(
                "Window size cannot be greater than series length".to_string(),
            ));
        }

        let name = format!("{}_rolling_std_{}", self.name(), window_size);

        match self {
            Series::I32(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        let window_data: Vec<f64> = data[window_start..=i]
                            .iter()
                            .copied()
                            .map(|v| v as f64)
                            .collect();

                        if window_data.len() < 2 {
                            result.push(None);
                        } else {
                            let mean = window_data.iter().sum::<f64>() / window_data.len() as f64;
                            let variance =
                                window_data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>()
                                    / (window_data.len() - 1) as f64;
                            let std_dev = variance.sqrt();
                            result.push(Some(std_dev));
                        }
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<f64> = result.into_iter().map(|x| x.unwrap_or(0.0)).collect();
                Ok(Series::F64(name, values, validity))
            }
            Series::F64(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    if i < window_size - 1 {
                        result.push(None);
                    } else {
                        let window_start = i + 1 - window_size;
                        let window_data: Vec<f64> = data[window_start..=i].to_vec();

                        if window_data.len() < 2 {
                            result.push(None);
                        } else {
                            let mean = window_data.iter().sum::<f64>() / window_data.len() as f64;
                            let variance =
                                window_data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>()
                                    / (window_data.len() - 1) as f64;
                            let std_dev = variance.sqrt();
                            result.push(Some(std_dev));
                        }
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<f64> = result.into_iter().map(|x| x.unwrap_or(0.0)).collect();
                Ok(Series::F64(name, values, validity))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Rolling standard deviation is only supported for numeric series (I32, F64)"
                    .to_string(),
            )),
        }
    }

    /// Calculates percentage change between consecutive values.
    ///
    /// This function computes the percentage change from one value to the next.
    /// For numeric series (I32, F64), it returns a new F64 series with the percentage changes.
    /// The first value is always None since there's no previous value to compare to.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Series` with percentage changes, or a `VeloxxError` if
    /// the series contains non-numeric data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_f64("price", vec![Some(100.0), Some(110.0), Some(99.0)]);
    /// let pct_change = series.pct_change().unwrap();
    /// // Result: [None, Some(0.1), Some(-0.1)]
    /// ```
    pub fn pct_change(&self) -> Result<Series, VeloxxError> {
        let name = format!("{}_pct_change", self.name());

        match self {
            Series::I32(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());
                result.push(None); // First value is always None

                for i in 1..data.len() {
                    match (data[i - 1], data[i]) {
                        (prev, curr) if prev != 0 => {
                            let pct = (curr - prev) as f64 / prev as f64;
                            result.push(Some(pct));
                        }
                        _ => result.push(None),
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<f64> = result.into_iter().map(|x| x.unwrap_or(0.0)).collect();
                Ok(Series::F64(name, values, validity))
            }
            Series::F64(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());
                result.push(None); // First value is always None

                for i in 1..data.len() {
                    match (data[i - 1], data[i]) {
                        (prev, curr) if prev != 0.0 => {
                            let pct = (curr - prev) / prev;
                            result.push(Some(pct));
                        }
                        _ => result.push(None),
                    }
                }

                let validity: Vec<bool> = result.iter().map(|x| x.is_some()).collect();
                let values: Vec<f64> = result.into_iter().map(|x| x.unwrap_or(0.0)).collect();
                Ok(Series::F64(name, values, validity))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Percentage change is only supported for numeric series (I32, F64)".to_string(),
            )),
        }
    }

    /// Calculates cumulative sum of the series.
    ///
    /// This function computes the running total of values in the series.
    /// For numeric series (I32, F64), it returns a new series of the same type with cumulative sums.
    /// Null values are treated as 0 for the calculation but preserved in the result.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Series` with cumulative sums, or a `VeloxxError` if
    /// the series contains non-numeric data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::series::Series;
    ///
    /// let series = Series::new_i32("values", vec![Some(1), Some(2), Some(3), Some(4)]);
    /// let cumsum = series.cumsum().unwrap();
    /// // Result: [Some(1), Some(3), Some(6), Some(10)]
    /// ```
    pub fn cumsum(&self) -> Result<Series, VeloxxError> {
        let name = format!("{}_cumsum", self.name());

        match self {
            Series::I32(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());
                let mut running_sum = 0i32;

                for &value in data.iter() {
                    running_sum += value;
                    result.push(running_sum);
                }

                Ok(Series::I32(name, result, vec![true; data.len()]))
            }
            Series::F64(_, data, _) => {
                let mut result = Vec::with_capacity(data.len());
                let mut running_sum = 0.0f64;

                for &value in data.iter() {
                    running_sum += value;
                    result.push(running_sum);
                }

                Ok(Series::F64(name, result, vec![true; data.len()]))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Cumulative sum is only supported for numeric series (I32, F64)".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_mean_i32() {
        let series = Series::new_i32("test", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let result = series.rolling_mean(3).unwrap();

        match result {
            Series::F64(_, values, _) => {
                assert!((values[2] - 2.0).abs() < 1e-9);
                assert!((values[3] - 3.0).abs() < 1e-9);
                assert!((values[4] - 4.0).abs() < 1e-9);
            }
            _ => panic!("Expected F64 series"),
        }
    }

    #[test]
    fn test_rolling_sum_f64() {
        let series = Series::new_f64("test", vec![Some(1.5), Some(2.5), Some(3.5), Some(4.5)]);
        let result = series.rolling_sum(2).unwrap();

        match result {
            Series::F64(_, values, _) => {
                assert!((values[1] - 4.0).abs() < 1e-9);
                assert!((values[2] - 6.0).abs() < 1e-9);
                assert!((values[3] - 8.0).abs() < 1e-9);
            }
            _ => panic!("Expected F64 series"),
        }
    }

    #[test]
    fn test_rolling_min_max() {
        let series = Series::new_i32("test", vec![Some(5), Some(2), Some(8), Some(1), Some(9)]);

        let min_result = series.rolling_min(3).unwrap();
        let max_result = series.rolling_max(3).unwrap();

        match (min_result, max_result) {
            (Series::I32(_, min_values, _), Series::I32(_, max_values, _)) => {
                assert_eq!(min_values[2], 2);
                assert_eq!(min_values[3], 1);
                assert_eq!(min_values[4], 1);

                assert_eq!(max_values[2], 8);
                assert_eq!(max_values[3], 8);
                assert_eq!(max_values[4], 9);
            }
            _ => panic!("Expected I32 series"),
        }
    }

    #[test]
    fn test_rolling_std() {
        let series = Series::new_f64("test", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]);
        let result = series.rolling_std(3).unwrap();

        match result {
            Series::F64(_, values, _) => {
                assert!((values[2] - 1.0).abs() < 1e-10);
            }
            _ => panic!("Expected F64 series"),
        }
    }

    #[test]
    fn test_pct_change() {
        let series = Series::new_f64("test", vec![Some(100.0), Some(110.0), Some(99.0)]);
        let result = series.pct_change().unwrap();

        match result {
            Series::F64(_, values, _) => {
                assert!((values[1] - 0.1).abs() < 1e-10);
                assert!((values[2] - (-0.1)).abs() < 1e-10);
            }
            _ => panic!("Expected F64 series"),
        }
    }

    #[test]
    fn test_cumsum() {
        let series = Series::new_i32("test", vec![Some(1), Some(2), Some(3), Some(4)]);
        let result = series.cumsum().unwrap();

        match result {
            Series::I32(_, values, _) => {
                assert_eq!(values[0], 1);
                assert_eq!(values[1], 3);
                assert_eq!(values[2], 6);
                assert_eq!(values[3], 10);
            }
            _ => panic!("Expected I32 series"),
        }
    }

    #[test]
    fn test_rolling_operations_with_nulls() {
        let series = Series::new_i32("test", vec![Some(1), None, Some(3), Some(4), None]);
        let result = series.rolling_mean(3).unwrap();

        match result {
            Series::F64(_, values, _) => {
                assert!((values[2] - 2.0).abs() < 1e-9);
                assert!((values[3] - 3.5).abs() < 1e-9);
                assert!((values[4] - 3.5).abs() < 1e-9);
            }
            _ => panic!("Expected F64 series"),
        }
    }

    #[test]
    fn test_rolling_operations_errors() {
        let series = Series::new_i32("test", vec![Some(1), Some(2), Some(3)]);

        // Test zero window size
        assert!(series.rolling_mean(0).is_err());

        // Test window size greater than series length
        assert!(series.rolling_mean(5).is_err());

        // Test non-numeric series
        let string_series =
            Series::new_string("test", vec![Some("a".to_string()), Some("b".to_string())]);
        assert!(string_series.rolling_mean(2).is_err());
    }
}
