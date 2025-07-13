//! Machine learning integration module for Velox.
//!
//! This module provides machine learning capabilities including linear regression,
//! data preprocessing utilities, and basic statistical modeling. It integrates
//! with external ML libraries when available.
//!
//! # Features
//!
//! - Linear regression for predictive modeling
//! - Data preprocessing and feature scaling
//! - Model evaluation metrics
//! - Statistical analysis utilities
//!
//! # Examples
//!
//! ```rust
//! use veloxx::dataframe::DataFrame;
//! use veloxx::series::Series;
//! use std::collections::BTreeMap;
//!
//! # #[cfg(feature = "ml")]
//! # {
//! use veloxx::ml::{LinearRegression, Preprocessing};
//!
//! let mut columns = BTreeMap::new();
//! columns.insert(
//!     "x".to_string(),
//!     Series::new_f64("x", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]),
//! );
//! columns.insert(
//!     "y".to_string(),
//!     Series::new_f64("y", vec![Some(2.0), Some(4.0), Some(6.0), Some(8.0)]),
//! );
//!
//! let df = DataFrame::new(columns).unwrap();
//! let model = LinearRegression::new();
//! // let trained_model = model.fit(&df, "y", &["x"]).unwrap();
//! # }
//! ```

#[cfg(feature = "ml")]
use linfa::prelude::*;
#[cfg(feature = "ml")]
use linfa_linear::LinearRegression as LinfaLinearRegression;
#[cfg(feature = "ml")]
use ndarray::{Array1, Array2};

use crate::dataframe::DataFrame;
use crate::error::VeloxxError;
use crate::series::Series;
use crate::types::Value;

/// Linear regression model for predictive analytics
#[derive(Debug, Clone)]
pub struct LinearRegression {
    #[cfg(feature = "ml")]
    model: Option<linfa_linear::FittedLinearRegression<f64>>,
    #[cfg(not(feature = "ml"))]
    _phantom: std::marker::PhantomData<()>,
}

impl LinearRegression {
    /// Create a new linear regression model
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::ml::LinearRegression;
    ///
    /// let model = LinearRegression::new();
    /// ```
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "ml")]
            model: None,
            #[cfg(not(feature = "ml"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Fit the linear regression model to the data
    ///
    /// # Arguments
    ///
    /// * `dataframe` - The DataFrame containing the training data
    /// * `target_column` - Name of the target column (dependent variable)
    /// * `feature_columns` - Names of the feature columns (independent variables)
    ///
    /// # Returns
    ///
    /// A fitted linear regression model
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::ml::LinearRegression;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "x".to_string(),
    ///     Series::new_f64("x", vec![Some(1.0), Some(2.0), Some(3.0)]),
    /// );
    /// columns.insert(
    ///     "y".to_string(),
    ///     Series::new_f64("y", vec![Some(2.0), Some(4.0), Some(6.0)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let model = LinearRegression::new();
    /// // let fitted_model = model.fit(&df, "y", &["x"]).unwrap();
    /// ```
    #[cfg(feature = "ml")]
    pub fn fit(
        &self,
        dataframe: &DataFrame,
        target_column: &str,
        feature_columns: &[&str],
    ) -> Result<FittedLinearRegression, VeloxxError> {
        let (features, targets) = self.prepare_data(dataframe, target_column, feature_columns)?;

        let dataset = Dataset::new(features, targets);
        let model = LinfaLinearRegression::default()
            .fit(&dataset)
            .map_err(|e| VeloxxError::InvalidOperation(format!("Failed to fit model: {}", e)))?;

        Ok(FittedLinearRegression { model })
    }

    #[cfg(not(feature = "ml"))]
    pub fn fit(
        &self,
        _dataframe: &DataFrame,
        _target_column: &str,
        _feature_columns: &[&str],
    ) -> Result<FittedLinearRegression, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "ML feature is not enabled. Enable with --features ml".to_string(),
        ))
    }

    #[cfg(feature = "ml")]
    fn prepare_data(
        &self,
        dataframe: &DataFrame,
        target_column: &str,
        feature_columns: &[&str],
    ) -> Result<(Array2<f64>, Array1<f64>), VeloxxError> {
        // Extract target data
        let target_series = dataframe
            .get_column(target_column)
            .ok_or_else(|| VeloxxError::ColumnNotFound(target_column.to_string()))?;
        let targets = self.series_to_array1(target_series)?;

        // Extract feature data
        let mut feature_data = Vec::new();
        for &col_name in feature_columns {
            let series = dataframe
                .get_column(col_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(col_name.to_string()))?;
            let col_data = self.series_to_vec(series)?;
            feature_data.push(col_data);
        }

        // Convert to ndarray format
        let n_samples = targets.len();
        let n_features = feature_columns.len();
        let mut features = Array2::zeros((n_samples, n_features));

        for (i, feature_col) in feature_data.iter().enumerate() {
            for (j, &value) in feature_col.iter().enumerate() {
                features[[j, i]] = value;
            }
        }

        Ok((features, targets))
    }

    #[cfg(feature = "ml")]
    fn series_to_array1(&self, series: &Series) -> Result<Array1<f64>, VeloxxError> {
        let vec_data = self.series_to_vec(series)?;
        Ok(Array1::from_vec(vec_data))
    }

    #[cfg(feature = "ml")]
    fn series_to_vec(&self, series: &Series) -> Result<Vec<f64>, VeloxxError> {
        let mut result = Vec::new();
        for i in 0..series.len() {
            if let Some(value) = series.get_value(i) {
                match value {
                    Value::F64(f) => result.push(f),
                    Value::I32(i) => result.push(i as f64),
                    _ => {
                        return Err(VeloxxError::InvalidOperation(
                            "Cannot convert non-numeric data to f64".to_string(),
                        ));
                    }
                }
            } else {
                return Err(VeloxxError::InvalidOperation(
                    "Cannot handle null values in ML operations".to_string(),
                ));
            }
        }
        Ok(result)
    }
}

impl Default for LinearRegression {
    fn default() -> Self {
        Self::new()
    }
}

/// A fitted linear regression model that can make predictions
#[derive(Debug, Clone)]
pub struct FittedLinearRegression {
    #[cfg(feature = "ml")]
    model: linfa_linear::FittedLinearRegression<f64>,
    #[cfg(not(feature = "ml"))]
    _phantom: std::marker::PhantomData<()>,
}

impl FittedLinearRegression {
    /// Make predictions using the fitted model
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame containing the features to predict on
    /// * `feature_columns` - Names of the feature columns
    ///
    /// # Returns
    ///
    /// Vector of predictions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::ml::LinearRegression;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "x".to_string(),
    ///     Series::new_f64("x", vec![Some(5.0), Some(6.0)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// // let predictions = fitted_model.predict(&df, &["x"]).unwrap();
    /// ```
    #[cfg(feature = "ml")]
    pub fn predict(
        &self,
        dataframe: &DataFrame,
        feature_columns: &[&str],
    ) -> Result<Vec<f64>, VeloxxError> {
        let features = self.prepare_features(dataframe, feature_columns)?;
        let predictions = self.model.predict(&features);
        Ok(predictions.to_vec())
    }

    #[cfg(not(feature = "ml"))]
    pub fn predict(
        &self,
        _dataframe: &DataFrame,
        _feature_columns: &[&str],
    ) -> Result<Vec<f64>, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "ML feature is not enabled. Enable with --features ml".to_string(),
        ))
    }

    #[cfg(feature = "ml")]
    fn prepare_features(
        &self,
        dataframe: &DataFrame,
        feature_columns: &[&str],
    ) -> Result<Array2<f64>, VeloxxError> {
        let mut feature_data = Vec::new();
        for &col_name in feature_columns {
            let series = dataframe
                .get_column(col_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(col_name.to_string()))?;
            let col_data = self.series_to_vec(series)?;
            feature_data.push(col_data);
        }

        let n_samples = feature_data[0].len();
        let n_features = feature_columns.len();
        let mut features = Array2::zeros((n_samples, n_features));

        for (i, feature_col) in feature_data.iter().enumerate() {
            for (j, &value) in feature_col.iter().enumerate() {
                features[[j, i]] = value;
            }
        }

        Ok(features)
    }

    #[cfg(feature = "ml")]
    fn series_to_vec(&self, series: &Series) -> Result<Vec<f64>, VeloxxError> {
        let mut result = Vec::new();
        for i in 0..series.len() {
            if let Some(value) = series.get_value(i) {
                match value {
                    Value::F64(f) => result.push(f),
                    Value::I32(i) => result.push(i as f64),
                    _ => {
                        return Err(VeloxxError::InvalidOperation(
                            "Cannot convert non-numeric data to f64".to_string(),
                        ));
                    }
                }
            } else {
                return Err(VeloxxError::InvalidOperation(
                    "Cannot handle null values in ML operations".to_string(),
                ));
            }
        }
        Ok(result)
    }

    /// Get the model coefficients
    ///
    /// # Returns
    ///
    /// Vector of model coefficients
    #[cfg(feature = "ml")]
    pub fn coefficients(&self) -> Vec<f64> {
        self.model.params().to_vec()
    }

    #[cfg(not(feature = "ml"))]
    pub fn coefficients(&self) -> Vec<f64> {
        Vec::new()
    }

    /// Get the model intercept
    ///
    /// # Returns
    ///
    /// Model intercept value
    #[cfg(feature = "ml")]
    pub fn intercept(&self) -> f64 {
        self.model.intercept()
    }

    #[cfg(not(feature = "ml"))]
    pub fn intercept(&self) -> f64 {
        0.0
    }
}

/// Data preprocessing utilities
pub struct Preprocessing;

impl Preprocessing {
    /// Standardize features to have zero mean and unit variance
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame containing the data
    /// * `columns` - Names of columns to standardize
    ///
    /// # Returns
    ///
    /// DataFrame with standardized features
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::ml::Preprocessing;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "feature".to_string(),
    ///     Series::new_f64("feature", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let standardized_df = Preprocessing::standardize(&df, &["feature"]).unwrap();
    /// ```
    pub fn standardize(dataframe: &DataFrame, columns: &[&str]) -> Result<DataFrame, VeloxxError> {
        let mut new_columns = std::collections::BTreeMap::new();

        // Copy non-standardized columns
        for (name, series) in dataframe.columns.iter() {
            if !columns.contains(&name.as_str()) {
                new_columns.insert(name.clone(), series.clone());
            }
        }

        // Standardize specified columns
        for &col_name in columns {
            let series = dataframe
                .get_column(col_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(col_name.to_string()))?;

            let standardized_series = Self::standardize_series(series)?;
            new_columns.insert(col_name.to_string(), standardized_series);
        }

        DataFrame::new(new_columns)
    }

    fn standardize_series(series: &Series) -> Result<Series, VeloxxError> {
        // Calculate mean and standard deviation
        let mean = match series.mean()? {
            Some(Value::F64(m)) => m,
            Some(Value::I32(m)) => m as f64,
            _ => {
                return Err(VeloxxError::InvalidOperation(
                    "Cannot calculate mean for standardization".to_string(),
                ))
            }
        };

        let std_dev = match series.std_dev()? {
            Some(Value::F64(s)) => s,
            Some(Value::I32(s)) => s as f64,
            _ => {
                return Err(VeloxxError::InvalidOperation(
                    "Cannot calculate std dev for standardization".to_string(),
                ))
            }
        };

        if std_dev == 0.0 {
            return Err(VeloxxError::InvalidOperation(
                "Cannot standardize column with zero variance".to_string(),
            ));
        }

        // Apply standardization
        match series {
            Series::F64(name, values) => {
                let standardized_values: Vec<Option<f64>> = values
                    .iter()
                    .map(|&val| val.map(|v| (v - mean) / std_dev))
                    .collect();
                Ok(Series::new_f64(name, standardized_values))
            }
            Series::I32(name, values) => {
                let standardized_values: Vec<Option<f64>> = values
                    .iter()
                    .map(|&val| val.map(|v| (v as f64 - mean) / std_dev))
                    .collect();
                Ok(Series::new_f64(name, standardized_values))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Can only standardize numeric columns".to_string(),
            )),
        }
    }

    /// Normalize features to a range [0, 1]
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame containing the data
    /// * `columns` - Names of columns to normalize
    ///
    /// # Returns
    ///
    /// DataFrame with normalized features
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::ml::Preprocessing;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "feature".to_string(),
    ///     Series::new_f64("feature", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let normalized_df = Preprocessing::normalize(&df, &["feature"]).unwrap();
    /// ```
    pub fn normalize(dataframe: &DataFrame, columns: &[&str]) -> Result<DataFrame, VeloxxError> {
        let mut new_columns = std::collections::BTreeMap::new();

        // Copy non-normalized columns
        for (name, series) in dataframe.columns.iter() {
            if !columns.contains(&name.as_str()) {
                new_columns.insert(name.clone(), series.clone());
            }
        }

        // Normalize specified columns
        for &col_name in columns {
            let series = dataframe
                .get_column(col_name)
                .ok_or_else(|| VeloxxError::ColumnNotFound(col_name.to_string()))?;

            let normalized_series = Self::normalize_series(series)?;
            new_columns.insert(col_name.to_string(), normalized_series);
        }

        DataFrame::new(new_columns)
    }

    fn normalize_series(series: &Series) -> Result<Series, VeloxxError> {
        // Calculate min and max
        let min_val = match series.min()? {
            Some(Value::F64(m)) => m,
            Some(Value::I32(m)) => m as f64,
            _ => {
                return Err(VeloxxError::InvalidOperation(
                    "Cannot calculate min for normalization".to_string(),
                ))
            }
        };

        let max_val = match series.max()? {
            Some(Value::F64(m)) => m,
            Some(Value::I32(m)) => m as f64,
            _ => {
                return Err(VeloxxError::InvalidOperation(
                    "Cannot calculate max for normalization".to_string(),
                ))
            }
        };

        let range = max_val - min_val;
        if range == 0.0 {
            return Err(VeloxxError::InvalidOperation(
                "Cannot normalize column with zero range".to_string(),
            ));
        }

        // Apply normalization
        match series {
            Series::F64(name, values) => {
                let normalized_values: Vec<Option<f64>> = values
                    .iter()
                    .map(|&val| val.map(|v| (v - min_val) / range))
                    .collect();
                Ok(Series::new_f64(name, normalized_values))
            }
            Series::I32(name, values) => {
                let normalized_values: Vec<Option<f64>> = values
                    .iter()
                    .map(|&val| val.map(|v| (v as f64 - min_val) / range))
                    .collect();
                Ok(Series::new_f64(name, normalized_values))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Can only normalize numeric columns".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::Series;
    use std::collections::BTreeMap;

    #[test]
    fn test_linear_regression_creation() {
        let model = LinearRegression::new();
        assert!(model.model.is_none() || cfg!(not(feature = "ml")));
    }

    #[test]
    fn test_standardization() -> Result<(), Box<dyn std::error::Error>> {
        let mut columns = BTreeMap::new();
        columns.insert(
            "feature".to_string(),
            Series::new_f64("feature", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]),
        );

        let df = DataFrame::new(columns)?;
        let standardized_df = Preprocessing::standardize(&df, &["feature"])?;

        let standardized_series = standardized_df.get_column("feature").unwrap();
        let mean = match standardized_series.mean()? {
            Some(Value::F64(m)) => m,
            Some(Value::I32(m)) => m as f64,
            _ => 0.0,
        };
        let std_dev = match standardized_series.std_dev()? {
            Some(Value::F64(s)) => s,
            Some(Value::I32(s)) => s as f64,
            _ => 0.0,
        };

        // Mean should be approximately 0
        assert!((mean.abs()) < 1e-10);
        // Standard deviation should be approximately 1
        assert!((std_dev - 1.0).abs() < 1e-10);
        Ok(())
    }

    #[test]
    fn test_normalization() -> Result<(), Box<dyn std::error::Error>> {
        let mut columns = BTreeMap::new();
        columns.insert(
            "feature".to_string(),
            Series::new_f64("feature", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]),
        );

        let df = DataFrame::new(columns)?;
        let normalized_df = Preprocessing::normalize(&df, &["feature"])?;

        let normalized_series = normalized_df.get_column("feature").unwrap();
        let min_val = match normalized_series.min()? {
            Some(Value::F64(m)) => m,
            Some(Value::I32(m)) => m as f64,
            _ => 0.0,
        };
        let max_val = match normalized_series.max()? {
            Some(Value::F64(m)) => m,
            Some(Value::I32(m)) => m as f64,
            _ => 0.0,
        };

        // Min should be 0, max should be 1
        assert!((min_val - 0.0).abs() < 1e-10);
        assert!((max_val - 1.0).abs() < 1e-10);
        Ok(())
    }

    #[test]
    #[cfg(not(feature = "ml"))]
    fn test_ml_operations_without_feature() {
        let columns = BTreeMap::new();
        let df = DataFrame::new(columns).unwrap();

        let model = LinearRegression::new();
        let result = model.fit(&df, "target", &["feature"]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("ML feature is not enabled"));
    }
}
