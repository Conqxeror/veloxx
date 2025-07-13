//! Example demonstrating machine learning capabilities in Velox
//!
//! This example shows how to use linear regression and data preprocessing
//! features with DataFrame data.

use std::collections::BTreeMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::types::Value;

#[cfg(feature = "ml")]
use veloxx::ml::{LinearRegression, Preprocessing};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Velox Machine Learning Example");
    println!("==============================");

    // Create sample data for linear regression
    linear_regression_example()?;

    // Demonstrate data preprocessing
    preprocessing_example()?;

    println!("\nMachine learning examples completed!");
    println!("Note: Enable the 'ml' feature to use actual ML algorithms:");
    println!("cargo run --example machine_learning --features ml");

    Ok(())
}

fn linear_regression_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. Linear Regression Example");
    println!("----------------------------");

    // Create training data: y = 2x + 1 + noise
    let mut columns = BTreeMap::new();
    columns.insert(
        "x".to_string(),
        Series::new_f64(
            "x",
            vec![
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
                Some(9.0),
                Some(10.0),
            ],
        ),
    );
    columns.insert(
        "y".to_string(),
        Series::new_f64(
            "y",
            vec![
                Some(3.1),
                Some(4.9),
                Some(7.2),
                Some(8.8),
                Some(11.1),
                Some(12.9),
                Some(15.2),
                Some(16.8),
                Some(19.1),
                Some(20.9),
            ],
        ),
    );

    let training_df = DataFrame::new(columns)?;
    println!("Training data (y = 2x + 1 + noise):");
    println!("{}", training_df);

    #[cfg(feature = "ml")]
    {
        // Train linear regression model
        let model = LinearRegression::new();
        let fitted_model = model.fit(&training_df, "y", &["x"])?;

        println!("Model coefficients: {:?}", fitted_model.coefficients());
        println!("Model intercept: {:.4}", fitted_model.intercept());

        // Create test data
        let mut test_columns = BTreeMap::new();
        test_columns.insert(
            "x".to_string(),
            Series::new_f64("x", vec![Some(11.0), Some(12.0), Some(13.0)]),
        );
        let test_df = DataFrame::new(test_columns)?;

        // Make predictions
        let predictions = fitted_model.predict(&test_df, &["x"])?;
        println!("Predictions for x=[11, 12, 13]: {:?}", predictions);
        println!("✓ Linear regression completed successfully");
    }

    #[cfg(not(feature = "ml"))]
    {
        println!("✗ ML feature not enabled - linear regression not available");
    }

    Ok(())
}

fn kmeans_clustering_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. K-Means Clustering Example");
    println!("-----------------------------");

    println!("K-means clustering is not implemented in this simplified version.");
    println!("This feature would require additional clustering dependencies.");
    println!("✓ K-means clustering placeholder completed");

    Ok(())
}

fn preprocessing_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. Data Preprocessing Example");
    println!("-----------------------------");

    // Create data with different scales
    let mut columns = BTreeMap::new();
    columns.insert(
        "feature1".to_string(),
        Series::new_f64(
            "feature1",
            vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)],
        ),
    );
    columns.insert(
        "feature2".to_string(),
        Series::new_f64(
            "feature2",
            vec![
                Some(100.0),
                Some(200.0),
                Some(300.0),
                Some(400.0),
                Some(500.0),
            ],
        ),
    );
    columns.insert(
        "label".to_string(),
        Series::new_string(
            "label",
            vec![
                Some("A".to_string()),
                Some("B".to_string()),
                Some("C".to_string()),
                Some("D".to_string()),
                Some("E".to_string()),
            ],
        ),
    );

    let original_df = DataFrame::new(columns)?;
    println!("Original data (different scales):");
    println!("{}", original_df);

    #[cfg(feature = "ml")]
    {
        // Standardization
        let standardized_df = Preprocessing::standardize(&original_df, &["feature1", "feature2"])?;
        println!("\nStandardized data (mean=0, std=1):");
        println!("{}", standardized_df);

        // Normalization
        let normalized_df = Preprocessing::normalize(&original_df, &["feature1", "feature2"])?;
        println!("\nNormalized data (range [0,1]):");
        println!("{}", normalized_df);
    }

    #[cfg(not(feature = "ml"))]
    {
        println!("\n✗ ML feature not enabled - preprocessing not available");
        println!("Enable with: cargo run --example machine_learning --features ml");
    }

    #[cfg(feature = "ml")]
    {
        // Verify standardization
        if let Some(std_feature1) = standardized_df.get_column("feature1") {
            let mean = match std_feature1.mean()? {
                Some(Value::F64(m)) => m,
                Some(Value::I32(m)) => m as f64,
                _ => 0.0,
            };
            let std_dev = match std_feature1.std_dev()? {
                Some(Value::F64(s)) => s,
                Some(Value::I32(s)) => s as f64,
                _ => 0.0,
            };
            println!(
                "\nStandardized feature1 - Mean: {:.6}, Std: {:.6}",
                mean, std_dev
            );
        }

        // Verify normalization
        if let Some(norm_feature1) = normalized_df.get_column("feature1") {
            let min_val = match norm_feature1.min()? {
                Some(Value::F64(m)) => m,
                Some(Value::I32(m)) => m as f64,
                _ => 0.0,
            };
            let max_val = match norm_feature1.max()? {
                Some(Value::F64(m)) => m,
                Some(Value::I32(m)) => m as f64,
                _ => 0.0,
            };
            println!(
                "Normalized feature1 - Min: {:.6}, Max: {:.6}",
                min_val, max_val
            );
        }

        println!("✓ Data preprocessing completed successfully");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data_creation() {
        // Test that we can create the example data without errors
        assert!(linear_regression_example().is_ok());
        assert!(kmeans_clustering_example().is_ok());
        assert!(preprocessing_example().is_ok());
    }
}
