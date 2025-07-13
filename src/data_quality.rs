//! Data Quality & Validation module for Velox.
//!
//! This module provides comprehensive data quality assessment and validation capabilities including:
//! - Schema validation and enforcement
//! - Data profiling and statistics
//! - Anomaly detection algorithms
//! - Data consistency checks
//! - Quality metrics and reporting
//!
//! # Features
//!
//! - Schema validation with custom rules
//! - Statistical profiling for data understanding
//! - Outlier and anomaly detection
//! - Data type validation and coercion
//! - Quality score calculation
//!
//! # Examples
//!
//! ```rust
//! use veloxx::dataframe::DataFrame;
//! use veloxx::series::Series;
//! use std::collections::BTreeMap;
//!
//! # #[cfg(feature = "data_quality")]
//! # {
//! use veloxx::data_quality::{SchemaValidator, DataProfiler, AnomalyDetector};
//!
//! let mut columns = BTreeMap::new();
//! columns.insert(
//!     "age".to_string(),
//!     Series::new_i32("age", vec![Some(25), Some(30), Some(35), Some(1000)]), // 1000 is an outlier
//! );
//! columns.insert(
//!     "email".to_string(),
//!     Series::new_string("email", vec![
//!         Some("user@example.com".to_string()),
//!         Some("invalid-email".to_string()),
//!         Some("test@test.com".to_string()),
//!         Some("another@domain.org".to_string()),
//!     ]),
//! );
//!
//! let df = DataFrame::new(columns).unwrap();
//!
//! // Profile the data
//! let profiler = DataProfiler::new();
//! let profile = profiler.profile_dataframe(&df).unwrap();
//! println!("Data profile: {:?}", profile);
//!
//! // Detect anomalies
//! let detector = AnomalyDetector::new();
//! let anomalies = detector.detect_outliers(&df, "age").unwrap();
//! println!("Anomalies detected: {:?}", anomalies);
//! # }
//! ```

use crate::dataframe::DataFrame;
use crate::error::VeloxxError;
use crate::series::Series;
use crate::types::{DataType, Value};
use std::collections::BTreeMap;

#[cfg(feature = "data_quality")]
use regex::Regex;

/// Schema definition for data validation
#[derive(Debug, Clone)]
pub struct Schema {
    pub columns: BTreeMap<String, ColumnSchema>,
}

/// Schema definition for a single column
#[derive(Debug, Clone)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub constraints: Vec<Constraint>,
}

/// Data validation constraints
#[derive(Debug, Clone)]
pub enum Constraint {
    MinValue(Value),
    MaxValue(Value),
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    UniqueValues,
    NotNull,
    InSet(Vec<Value>),
}

/// Schema validator for enforcing data structure and constraints
pub struct SchemaValidator {
    #[cfg(not(feature = "data_quality"))]
    _phantom: std::marker::PhantomData<()>,
}

impl SchemaValidator {
    /// Create a new schema validator
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::data_quality::SchemaValidator;
    ///
    /// let validator = SchemaValidator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "data_quality"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Validate a DataFrame against a schema
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to validate
    /// * `schema` - Schema to validate against
    ///
    /// # Returns
    ///
    /// Validation result with any errors found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::data_quality::{SchemaValidator, Schema, ColumnSchema, Constraint};
    /// use veloxx::types::DataType;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "age".to_string(),
    ///     Series::new_i32("age", vec![Some(25), Some(30)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    ///
    /// let mut schema_columns = BTreeMap::new();
    /// schema_columns.insert(
    ///     "age".to_string(),
    ///     ColumnSchema {
    ///         name: "age".to_string(),
    ///         data_type: DataType::I32,
    ///         nullable: false,
    ///         constraints: vec![Constraint::MinValue(veloxx::types::Value::I32(0))],
    ///     },
    /// );
    ///
    /// let schema = Schema { columns: schema_columns };
    /// let validator = SchemaValidator::new();
    /// let result = validator.validate(&df, &schema).unwrap();
    /// ```
    pub fn validate(
        &self,
        dataframe: &DataFrame,
        schema: &Schema,
    ) -> Result<ValidationResult, VeloxxError> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if all required columns are present
        for (column_name, column_schema) in &schema.columns {
            if dataframe.get_column(column_name).is_none() {
                errors.push(ValidationError {
                    column: column_name.clone(),
                    row: None,
                    error_type: ValidationErrorType::MissingColumn,
                    message: format!("Required column '{}' is missing", column_name),
                });
                continue;
            }

            let series = dataframe.get_column(column_name).unwrap();

            // Check data type
            if series.data_type() != column_schema.data_type {
                warnings.push(ValidationError {
                    column: column_name.clone(),
                    row: None,
                    error_type: ValidationErrorType::TypeMismatch,
                    message: format!(
                        "Column '{}' has type {:?}, expected {:?}",
                        column_name,
                        series.data_type(),
                        column_schema.data_type
                    ),
                });
            }

            // Validate constraints
            self.validate_constraints(series, column_schema, &mut errors, &mut warnings)?;
        }

        // Check for unexpected columns
        for column_name in dataframe.column_names() {
            if !schema.columns.contains_key(column_name) {
                warnings.push(ValidationError {
                    column: column_name.clone(),
                    row: None,
                    error_type: ValidationErrorType::UnexpectedColumn,
                    message: format!("Unexpected column '{}' found", column_name),
                });
            }
        }

        Ok(ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        })
    }

    fn validate_constraints(
        &self,
        series: &Series,
        column_schema: &ColumnSchema,
        errors: &mut Vec<ValidationError>,
        _warnings: &mut [ValidationError],
    ) -> Result<(), VeloxxError> {
        for constraint in &column_schema.constraints {
            match constraint {
                Constraint::NotNull => {
                    for i in 0..series.len() {
                        if series.get_value(i).is_none() {
                            errors.push(ValidationError {
                                column: column_schema.name.clone(),
                                row: Some(i),
                                error_type: ValidationErrorType::NullValue,
                                message: format!(
                                    "Null value found in non-nullable column '{}'",
                                    column_schema.name
                                ),
                            });
                        }
                    }
                }
                Constraint::MinValue(min_val) => {
                    for i in 0..series.len() {
                        if let Some(value) = series.get_value(i) {
                            if value < *min_val {
                                errors.push(ValidationError {
                                    column: column_schema.name.clone(),
                                    row: Some(i),
                                    error_type: ValidationErrorType::ConstraintViolation,
                                    message: format!(
                                        "Value {:?} is below minimum {:?}",
                                        value, min_val
                                    ),
                                });
                            }
                        }
                    }
                }
                Constraint::MaxValue(max_val) => {
                    for i in 0..series.len() {
                        if let Some(value) = series.get_value(i) {
                            if value > *max_val {
                                errors.push(ValidationError {
                                    column: column_schema.name.clone(),
                                    row: Some(i),
                                    error_type: ValidationErrorType::ConstraintViolation,
                                    message: format!(
                                        "Value {:?} is above maximum {:?}",
                                        value, max_val
                                    ),
                                });
                            }
                        }
                    }
                }
                Constraint::Pattern(pattern) => {
                    #[cfg(feature = "data_quality")]
                    {
                        let regex = Regex::new(pattern).map_err(|e| {
                            VeloxxError::InvalidOperation(format!("Invalid regex pattern: {}", e))
                        })?;

                        for i in 0..series.len() {
                            if let Some(Value::String(s)) = series.get_value(i) {
                                if !regex.is_match(&s) {
                                    errors.push(ValidationError {
                                        column: column_schema.name.clone(),
                                        row: Some(i),
                                        error_type: ValidationErrorType::PatternMismatch,
                                        message: format!(
                                            "Value '{}' does not match pattern '{}'",
                                            s, pattern
                                        ),
                                    });
                                }
                            }
                        }
                    }
                    #[cfg(not(feature = "data_quality"))]
                    {
                        warnings.push(ValidationError {
                            column: column_schema.name.clone(),
                            row: None,
                            error_type: ValidationErrorType::FeatureNotEnabled,
                            message: "Pattern validation requires data_quality feature".to_string(),
                        });
                    }
                }
                Constraint::UniqueValues => {
                    let mut seen_values = std::collections::HashSet::new();
                    for i in 0..series.len() {
                        if let Some(value) = series.get_value(i) {
                            if !seen_values.insert(value.clone()) {
                                errors.push(ValidationError {
                                    column: column_schema.name.clone(),
                                    row: Some(i),
                                    error_type: ValidationErrorType::DuplicateValue,
                                    message: format!("Duplicate value {:?} found", value),
                                });
                            }
                        }
                    }
                }
                _ => {} // Other constraints not implemented yet
            }
        }
        Ok(())
    }

    /// Create a schema from an existing DataFrame
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to infer schema from
    /// * `nullable` - Whether columns should be nullable by default
    ///
    /// # Returns
    ///
    /// Inferred schema
    pub fn infer_schema(&self, dataframe: &DataFrame, nullable: bool) -> Schema {
        let mut columns = BTreeMap::new();

        for column_name in dataframe.column_names() {
            if let Some(series) = dataframe.get_column(column_name) {
                let column_schema = ColumnSchema {
                    name: column_name.clone(),
                    data_type: series.data_type(),
                    nullable,
                    constraints: Vec::new(),
                };
                columns.insert(column_name.clone(), column_schema);
            }
        }

        Schema { columns }
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of schema validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
}

/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub column: String,
    pub row: Option<usize>,
    pub error_type: ValidationErrorType,
    pub message: String,
}

/// Types of validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationErrorType {
    MissingColumn,
    UnexpectedColumn,
    TypeMismatch,
    NullValue,
    ConstraintViolation,
    PatternMismatch,
    DuplicateValue,
    FeatureNotEnabled,
}

/// Data profiler for generating comprehensive data statistics
pub struct DataProfiler {
    #[cfg(not(feature = "data_quality"))]
    _phantom: std::marker::PhantomData<()>,
}

impl DataProfiler {
    /// Create a new data profiler
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::data_quality::DataProfiler;
    ///
    /// let profiler = DataProfiler::new();
    /// ```
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "data_quality"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Generate a comprehensive profile of a DataFrame
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to profile
    ///
    /// # Returns
    ///
    /// Data profile with statistics and insights
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::data_quality::DataProfiler;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "age".to_string(),
    ///     Series::new_i32("age", vec![Some(25), Some(30), Some(35)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let profiler = DataProfiler::new();
    /// let profile = profiler.profile_dataframe(&df).unwrap();
    /// ```
    pub fn profile_dataframe(&self, dataframe: &DataFrame) -> Result<DataProfile, VeloxxError> {
        let mut column_profiles = BTreeMap::new();

        for column_name in dataframe.column_names() {
            if let Some(series) = dataframe.get_column(column_name) {
                let column_profile = self.profile_series(series)?;
                column_profiles.insert(column_name.clone(), column_profile);
            }
        }

        Ok(DataProfile {
            row_count: dataframe.row_count(),
            column_count: dataframe.column_count(),
            column_profiles,
            quality_score: self.calculate_quality_score(dataframe)?,
        })
    }

    /// Profile a single series
    ///
    /// # Arguments
    ///
    /// * `series` - Series to profile
    ///
    /// # Returns
    ///
    /// Column profile with statistics
    pub fn profile_series(&self, series: &Series) -> Result<ColumnProfile, VeloxxError> {
        let null_count = (0..series.len())
            .filter(|&i| series.get_value(i).is_none())
            .count();

        let null_percentage = if !series.is_empty() {
            (null_count as f64 / series.len() as f64) * 100.0
        } else {
            0.0
        };

        let unique_count = series.unique()?.len();
        let unique_percentage = if !series.is_empty() {
            (unique_count as f64 / series.len() as f64) * 100.0
        } else {
            0.0
        };

        Ok(ColumnProfile {
            name: series.name().to_string(),
            data_type: series.data_type(),
            null_count,
            null_percentage,
            unique_count,
            unique_percentage,
            min_value: series.min()?,
            max_value: series.max()?,
            mean_value: series.mean()?,
            std_dev: series.std_dev()?,
            median_value: series.median()?,
        })
    }

    fn calculate_quality_score(&self, dataframe: &DataFrame) -> Result<f64, VeloxxError> {
        let mut total_score = 0.0;
        let mut column_count = 0;

        for column_name in dataframe.column_names() {
            if let Some(series) = dataframe.get_column(column_name) {
                let null_count = (0..series.len())
                    .filter(|&i| series.get_value(i).is_none())
                    .count();

                let completeness = if !series.is_empty() {
                    1.0 - (null_count as f64 / series.len() as f64)
                } else {
                    1.0
                };

                // Simple quality score based on completeness
                // In a real implementation, this would consider more factors
                total_score += completeness;
                column_count += 1;
            }
        }

        Ok(if column_count > 0 {
            (total_score / column_count as f64) * 100.0
        } else {
            100.0
        })
    }
}

impl Default for DataProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive data profile
#[derive(Debug, Clone)]
pub struct DataProfile {
    pub row_count: usize,
    pub column_count: usize,
    pub column_profiles: BTreeMap<String, ColumnProfile>,
    pub quality_score: f64,
}

/// Profile for a single column
#[derive(Debug, Clone)]
pub struct ColumnProfile {
    pub name: String,
    pub data_type: DataType,
    pub null_count: usize,
    pub null_percentage: f64,
    pub unique_count: usize,
    pub unique_percentage: f64,
    pub min_value: Option<Value>,
    pub max_value: Option<Value>,
    pub mean_value: Option<Value>,
    pub std_dev: Option<Value>,
    pub median_value: Option<Value>,
}

/// Anomaly detector for identifying outliers and unusual patterns
pub struct AnomalyDetector {
    #[cfg(not(feature = "data_quality"))]
    _phantom: std::marker::PhantomData<()>,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::data_quality::AnomalyDetector;
    ///
    /// let detector = AnomalyDetector::new();
    /// ```
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "data_quality"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Detect outliers in a numeric column using IQR method
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame containing the data
    /// * `column_name` - Name of the column to analyze
    ///
    /// # Returns
    ///
    /// Vector of row indices where outliers were detected
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::data_quality::AnomalyDetector;
    /// use std::collections::BTreeMap;
    ///
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "values".to_string(),
    ///     Series::new_f64("values", vec![Some(1.0), Some(2.0), Some(3.0), Some(100.0)]), // 100.0 is an outlier
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let detector = AnomalyDetector::new();
    /// let outliers = detector.detect_outliers(&df, "values").unwrap();
    /// ```
    pub fn detect_outliers(
        &self,
        dataframe: &DataFrame,
        column_name: &str,
    ) -> Result<Vec<usize>, VeloxxError> {
        let series = dataframe
            .get_column(column_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.to_string()))?;

        let mut values = Vec::new();
        for i in 0..series.len() {
            if let Some(value) = series.get_value(i) {
                match value {
                    Value::F64(f) => values.push((i, f)),
                    Value::I32(n) => values.push((i, n as f64)),
                    _ => continue, // Skip non-numeric values
                }
            }
        }

        if values.len() < 4 {
            return Ok(Vec::new()); // Need at least 4 values for IQR
        }

        // Sort values for quantile calculation
        let mut sorted_values: Vec<f64> = values.iter().map(|(_, v)| *v).collect();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Calculate quartiles using proper percentile calculation
        let n = sorted_values.len();
        let q1_pos = (n - 1) as f64 * 0.25;
        let q3_pos = (n - 1) as f64 * 0.75;

        let q1 = if q1_pos.fract() == 0.0 {
            sorted_values[q1_pos as usize]
        } else {
            let lower = sorted_values[q1_pos.floor() as usize];
            let upper = sorted_values[q1_pos.ceil() as usize];
            lower + (upper - lower) * q1_pos.fract()
        };

        let q3 = if q3_pos.fract() == 0.0 {
            sorted_values[q3_pos as usize]
        } else {
            let lower = sorted_values[q3_pos.floor() as usize];
            let upper = sorted_values[q3_pos.ceil() as usize];
            lower + (upper - lower) * q3_pos.fract()
        };

        let iqr = q3 - q1;

        // Calculate outlier bounds
        let lower_bound = q1 - 1.5 * iqr;
        let upper_bound = q3 + 1.5 * iqr;

        // Find outliers
        let outliers: Vec<usize> = values
            .into_iter()
            .filter_map(|(idx, val)| {
                if val < lower_bound || val > upper_bound {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        Ok(outliers)
    }

    /// Detect anomalies using Z-score method
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame containing the data
    /// * `column_name` - Name of the column to analyze
    /// * `threshold` - Z-score threshold (typically 2.0 or 3.0)
    ///
    /// # Returns
    ///
    /// Vector of row indices where anomalies were detected
    pub fn detect_anomalies_zscore(
        &self,
        dataframe: &DataFrame,
        column_name: &str,
        threshold: f64,
    ) -> Result<Vec<usize>, VeloxxError> {
        let series = dataframe
            .get_column(column_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(column_name.to_string()))?;

        let mean = match series.mean()? {
            Some(Value::F64(m)) => m,
            Some(Value::I32(m)) => m as f64,
            _ => {
                return Err(VeloxxError::InvalidOperation(
                    "Cannot calculate mean for Z-score".to_string(),
                ))
            }
        };

        let std_dev = match series.std_dev()? {
            Some(Value::F64(s)) => s,
            Some(Value::I32(s)) => s as f64,
            _ => {
                return Err(VeloxxError::InvalidOperation(
                    "Cannot calculate std dev for Z-score".to_string(),
                ))
            }
        };

        if std_dev == 0.0 {
            return Ok(Vec::new()); // No variation, no anomalies
        }

        let mut anomalies = Vec::new();
        for i in 0..series.len() {
            if let Some(value) = series.get_value(i) {
                let val = match value {
                    Value::F64(f) => f,
                    Value::I32(n) => n as f64,
                    _ => continue,
                };

                let z_score = (val - mean).abs() / std_dev;
                if z_score > threshold {
                    anomalies.push(i);
                }
            }
        }

        Ok(anomalies)
    }

    /// Detect duplicate rows in a DataFrame
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to analyze
    ///
    /// # Returns
    ///
    /// Vector of row indices that are duplicates
    pub fn detect_duplicate_rows(&self, dataframe: &DataFrame) -> Result<Vec<usize>, VeloxxError> {
        let mut seen_rows = std::collections::HashSet::new();
        let mut duplicates = Vec::new();

        for i in 0..dataframe.row_count() {
            let mut row_values = Vec::new();
            for column_name in dataframe.column_names() {
                if let Some(series) = dataframe.get_column(column_name) {
                    row_values.push(series.get_value(i));
                }
            }

            if !seen_rows.insert(row_values) {
                duplicates.push(i);
            }
        }

        Ok(duplicates)
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Data consistency checker
pub struct ConsistencyChecker;

impl ConsistencyChecker {
    /// Check for referential integrity between two DataFrames
    ///
    /// # Arguments
    ///
    /// * `primary_df` - Primary DataFrame
    /// * `foreign_df` - Foreign DataFrame
    /// * `primary_key` - Primary key column name
    /// * `foreign_key` - Foreign key column name
    ///
    /// # Returns
    ///
    /// Vector of foreign key values that don't have corresponding primary keys
    pub fn check_referential_integrity(
        primary_df: &DataFrame,
        foreign_df: &DataFrame,
        primary_key: &str,
        foreign_key: &str,
    ) -> Result<Vec<Value>, VeloxxError> {
        let primary_series = primary_df
            .get_column(primary_key)
            .ok_or_else(|| VeloxxError::ColumnNotFound(primary_key.to_string()))?;

        let foreign_series = foreign_df
            .get_column(foreign_key)
            .ok_or_else(|| VeloxxError::ColumnNotFound(foreign_key.to_string()))?;

        // Collect all primary key values
        let mut primary_values = std::collections::HashSet::new();
        for i in 0..primary_series.len() {
            if let Some(value) = primary_series.get_value(i) {
                primary_values.insert(value);
            }
        }

        // Check foreign key values
        let mut orphaned_values = Vec::new();
        for i in 0..foreign_series.len() {
            if let Some(value) = foreign_series.get_value(i) {
                if !primary_values.contains(&value) {
                    orphaned_values.push(value);
                }
            }
        }

        Ok(orphaned_values)
    }

    /// Check for data type consistency across columns
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to check
    ///
    /// # Returns
    ///
    /// Map of column names to inconsistent value indices
    pub fn check_type_consistency(
        dataframe: &DataFrame,
    ) -> Result<BTreeMap<String, Vec<usize>>, VeloxxError> {
        let mut inconsistencies = BTreeMap::new();

        for column_name in dataframe.column_names() {
            if let Some(series) = dataframe.get_column(column_name) {
                let expected_type = series.data_type();
                let mut inconsistent_rows = Vec::new();

                // This is a simplified check - in reality, we'd need more sophisticated type checking
                for i in 0..series.len() {
                    if let Some(value) = series.get_value(i) {
                        if value.data_type() != expected_type {
                            inconsistent_rows.push(i);
                        }
                    }
                }

                if !inconsistent_rows.is_empty() {
                    inconsistencies.insert(column_name.clone(), inconsistent_rows);
                }
            }
        }

        Ok(inconsistencies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::Series;
    use std::collections::BTreeMap;

    #[test]
    fn test_schema_validator_creation() {
        let validator = SchemaValidator::new();
        assert_eq!(
            std::mem::size_of_val(&validator),
            std::mem::size_of::<SchemaValidator>()
        );
    }

    #[test]
    fn test_data_profiler_creation() {
        let profiler = DataProfiler::new();
        assert_eq!(
            std::mem::size_of_val(&profiler),
            std::mem::size_of::<DataProfiler>()
        );
    }

    #[test]
    fn test_anomaly_detector_creation() {
        let detector = AnomalyDetector::new();
        assert_eq!(
            std::mem::size_of_val(&detector),
            std::mem::size_of::<AnomalyDetector>()
        );
    }

    #[test]
    fn test_schema_inference() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "age".to_string(),
            Series::new_i32("age", vec![Some(25), Some(30)]),
        );
        columns.insert(
            "name".to_string(),
            Series::new_string(
                "name",
                vec![Some("Alice".to_string()), Some("Bob".to_string())],
            ),
        );

        let df = DataFrame::new(columns).unwrap();
        let validator = SchemaValidator::new();
        let schema = validator.infer_schema(&df, true);

        assert_eq!(schema.columns.len(), 2);
        assert!(schema.columns.contains_key("age"));
        assert!(schema.columns.contains_key("name"));
    }

    #[test]
    fn test_data_profiling() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "values".to_string(),
            Series::new_i32("values", vec![Some(1), Some(2), None, Some(4)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let profiler = DataProfiler::new();
        let profile = profiler.profile_dataframe(&df).unwrap();

        assert_eq!(profile.row_count, 4);
        assert_eq!(profile.column_count, 1);
        assert!(profile.column_profiles.contains_key("values"));

        let column_profile = &profile.column_profiles["values"];
        assert_eq!(column_profile.null_count, 1);
        assert_eq!(column_profile.null_percentage, 25.0);
    }

    #[test]
    fn test_outlier_detection() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "values".to_string(),
            Series::new_f64("values", vec![Some(1.0), Some(2.0), Some(3.0), Some(100.0)]),
        );

        let df = DataFrame::new(columns).unwrap();
        let detector = AnomalyDetector::new();
        let outliers = detector.detect_outliers(&df, "values").unwrap();

        // 100.0 should be detected as an outlier
        assert!(!outliers.is_empty());
        assert!(outliers.contains(&3)); // Index 3 contains the outlier
    }

    #[test]
    fn test_duplicate_detection() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "id".to_string(),
            Series::new_i32("id", vec![Some(1), Some(2), Some(1)]), // Duplicate: 1
        );
        columns.insert(
            "name".to_string(),
            Series::new_string(
                "name",
                vec![
                    Some("Alice".to_string()),
                    Some("Bob".to_string()),
                    Some("Alice".to_string()),
                ],
            ),
        );

        let df = DataFrame::new(columns).unwrap();
        let detector = AnomalyDetector::new();
        let duplicates = detector.detect_duplicate_rows(&df).unwrap();

        // Row 2 should be detected as a duplicate of row 0
        assert!(!duplicates.is_empty());
        assert!(duplicates.contains(&2));
    }
}
