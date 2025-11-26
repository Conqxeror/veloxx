use indexmap::IndexMap;
use veloxx::{dataframe::DataFrame, error::VeloxxError, series::Series};

#[cfg(feature = "ml")]
use veloxx::ml::{LinearRegression, Preprocessing};

#[cfg(feature = "ml")]
use veloxx::types::Value;

fn main() -> Result<(), VeloxxError> {
    println!("🚀 Veloxx Machine Learning Examples");
    println!("====================================");

    linear_regression_example()?;
    preprocessing_example()?;

    Ok(())
}

fn linear_regression_example() -> Result<(), VeloxxError> {
    println!("\n📊 Linear Regression Example");
    println!("-----------------------------");

    // Create sample data for linear regression
    let x_values: Vec<Option<f64>> = (1..=10).map(|i| Some(i as f64)).collect();
    let y_values: Vec<Option<f64>> = x_values
        .iter()
        .map(|x| x.as_ref().map(|val| 2.0 * val + 1.0 + (val % 3.0 - 1.0)))
        .collect();

    let mut columns = IndexMap::new();
    columns.insert("x".to_string(), Series::new_f64("x", x_values));
    columns.insert("y".to_string(), Series::new_f64("y", y_values));

    let df = DataFrame::new(columns);
    println!("Training data:");
    println!("{}", df);

    #[cfg(feature = "ml")]
    {
        let mut regression = LinearRegression::new();

        // Train the model (note: fit returns a FittedLinearRegression)
        let fitted_model = regression.fit(&df, "y", &["x"])?;
        println!("✓ Linear regression model trained");

        // Make predictions
        for i in 11..=13 {
            let x_val = i as f64;
            let mut test_columns = IndexMap::new();
            test_columns.insert("x".to_string(), Series::new_f64("x", vec![Some(x_val)]));
            let test_df = DataFrame::new(test_columns);

            let predictions = fitted_model.predict(&test_df, &["x"])?;
            if let Some(prediction) = predictions.first() {
                println!("Prediction for x={}: {:.2}", x_val, prediction);
            }
        }
    }

    #[cfg(not(feature = "ml"))]
    {
        println!("⚠️ Machine learning features not enabled. Enable with --features ml");
    }

    Ok(())
}

fn preprocessing_example() -> Result<(), VeloxxError> {
    println!("\n🔧 Data Preprocessing Example");
    println!("------------------------------");

    // Create sample data with different scales
    let feature1: Vec<Option<f64>> = vec![
        Some(100.0),
        Some(200.0),
        Some(300.0),
        Some(400.0),
        Some(500.0),
    ];

    let feature2: Vec<Option<f64>> = vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];

    let mut columns = IndexMap::new();
    columns.insert(
        "feature1".to_string(),
        Series::new_f64("feature1", feature1),
    );
    columns.insert(
        "feature2".to_string(),
        Series::new_f64("feature2", feature2),
    );

    let df = DataFrame::new(columns);
    println!("Original data:");
    println!("{}", df);

    #[cfg(feature = "ml")]
    {
        // Standardization (z-score normalization)
        let standardized_df = Preprocessing::standardize(&df, &["feature1"])?;
        println!("\nStandardized feature1:");
        println!("{}", standardized_df);

        if let Some(std_feature1) = standardized_df.get_column("feature1") {
            if let Ok(mean_val) = std_feature1.mean() {
                let mean = match mean_val {
                    Value::F64(m) => m,
                    Value::I32(m) => m as f64,
                    _ => 0.0,
                };

                let std_dev = match std_feature1.std_dev()? {
                    Value::F64(s) => s,
                    Value::I32(s) => s as f64,
                    _ => 0.0,
                };

                println!(
                    "\nStandardized feature1 - Mean: {:.6}, Std: {:.6}",
                    mean, std_dev
                );
            }
        }

        // Min-Max normalization
        let normalized_df = Preprocessing::normalize(&df, &["feature1"])?;
        println!("\nNormalized feature1 (0-1 range):");
        println!("{}", normalized_df);

        if let Some(norm_feature1) = normalized_df.get_column("feature1") {
            let min_val = match norm_feature1.min()? {
                Value::F64(m) => m,
                Value::I32(m) => m as f64,
                _ => 0.0,
            };

            let max_val = match norm_feature1.max()? {
                Value::F64(m) => m,
                Value::I32(m) => m as f64,
                _ => 0.0,
            };

            println!(
                "Normalized feature1 - Min: {:.6}, Max: {:.6}",
                min_val, max_val
            );
        }
    }

    #[cfg(not(feature = "ml"))]
    {
        println!("⚠️ Machine learning features not enabled. Enable with --features ml");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_regression_example() {
        assert!(linear_regression_example().is_ok());
    }

    #[test]
    fn test_preprocessing_example() {
        assert!(preprocessing_example().is_ok());
    }
}
