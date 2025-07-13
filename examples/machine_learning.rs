use veloxx::{
    dataframe::DataFrame,
    series::Series,
    types::Value,
    error::VeloxxError,
    ml::{LinearRegression, KMeansClustering, Preprocessing},
};
use std::collections::BTreeMap;

fn main() -> Result<(), VeloxxError> {
    println!("🚀 Veloxx Machine Learning Examples");
    println!("====================================");

    linear_regression_example()?;
    kmeans_clustering_example()?;
    preprocessing_example()?;

    Ok(())
}

fn linear_regression_example() -> Result<(), VeloxxError> {
    println!("\n📊 Linear Regression Example");
    println!("-----------------------------");

    // Create sample data for linear regression
    let x_values: Vec<Option<Value>> = (1..=10).map(|i| Some(Value::F64(i as f64))).collect();
    let y_values: Vec<Option<Value>> = x_values.iter().map(|x| {
        if let Some(Value::F64(val)) = x {
            Some(Value::F64(2.0 * val + 1.0 + (rand::random::<f64>() - 0.5) * 2.0)) // y = 2x + 1 + noise
        } else {
            None
        }
    }).collect();

    let mut columns = BTreeMap::new();
    columns.insert("x".to_string(), Series::new("x", x_values));
    columns.insert("y".to_string(), Series::new("y", y_values));

    let df = DataFrame::new(columns)?;
    println!("Training data:");
    println!("{}", df);

    #[cfg(feature = "ml")]
    {
        let mut regression = LinearRegression::new();
        
        // Train the model
        regression.fit(&df, "x", "y")?;
        println!("✓ Linear regression model trained");

        // Make predictions
        let test_x = vec![11.0, 12.0, 13.0];
        for x in test_x {
            let prediction = regression.predict(x)?;
            println!("Prediction for x={}: {:.2}", x, prediction);
        }
    }

    #[cfg(not(feature = "ml"))]
    {
        println!("\n✗ ML feature not enabled - linear regression not available");
        println!("Enable with: cargo run --example machine_learning --features ml");
    }

    Ok(())
}

fn kmeans_clustering_example() -> Result<(), VeloxxError> {
    println!("\n🎯 K-Means Clustering Example");
    println!("------------------------------");

    // Create sample data for clustering
    let feature1: Vec<Option<Value>> = vec![
        Some(Value::F64(1.0)), Some(Value::F64(1.5)), Some(Value::F64(2.0)),
        Some(Value::F64(8.0)), Some(Value::F64(8.5)), Some(Value::F64(9.0)),
        Some(Value::F64(1.2)), Some(Value::F64(1.8)), Some(Value::F64(2.1)),
        Some(Value::F64(8.2)), Some(Value::F64(8.7)), Some(Value::F64(9.1)),
    ];

    let feature2: Vec<Option<Value>> = vec![
        Some(Value::F64(1.0)), Some(Value::F64(1.2)), Some(Value::F64(1.1)),
        Some(Value::F64(8.0)), Some(Value::F64(8.2)), Some(Value::F64(8.1)),
        Some(Value::F64(1.3)), Some(Value::F64(1.0)), Some(Value::F64(1.2)),
        Some(Value::F64(8.3)), Some(Value::F64(8.0)), Some(Value::F64(8.2)),
    ];

    let mut columns = BTreeMap::new();
    columns.insert("feature1".to_string(), Series::new("feature1", feature1));
    columns.insert("feature2".to_string(), Series::new("feature2", feature2));

    let df = DataFrame::new(columns)?;
    println!("Clustering data:");
    println!("{}", df);

    #[cfg(feature = "ml")]
    {
        let mut kmeans = KMeansClustering::new(2); // 2 clusters
        
        // Fit the model
        kmeans.fit(&df, &["feature1", "feature2"])?;
        println!("✓ K-means clustering completed");

        // Get cluster assignments
        let clusters = kmeans.predict(&df)?;
        println!("Cluster assignments: {:?}", clusters);

        // Get centroids
        let centroids = kmeans.get_centroids();
        println!("Cluster centroids: {:?}", centroids);
    }

    #[cfg(not(feature = "ml"))]
    {
        println!("\n✗ ML feature not enabled - clustering not available");
        println!("Enable with: cargo run --example machine_learning --features ml");
    }

    Ok(())
}

fn preprocessing_example() -> Result<(), VeloxxError> {
    println!("\n🔧 Data Preprocessing Example");
    println!("------------------------------");

    // Create sample data with different scales
    let feature1: Vec<Option<Value>> = vec![
        Some(Value::F64(100.0)), Some(Value::F64(200.0)), Some(Value::F64(300.0)),
        Some(Value::F64(400.0)), Some(Value::F64(500.0)),
    ];

    let feature2: Vec<Option<Value>> = vec![
        Some(Value::F64(1.0)), Some(Value::F64(2.0)), Some(Value::F64(3.0)),
        Some(Value::F64(4.0)), Some(Value::F64(5.0)),
    ];

    let mut columns = BTreeMap::new();
    columns.insert("feature1".to_string(), Series::new("feature1", feature1));
    columns.insert("feature2".to_string(), Series::new("feature2", feature2));

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

        println!("\n✓ Data preprocessing completed successfully");
    }

    #[cfg(not(feature = "ml"))]
    {
        println!("\n✗ ML feature not enabled - preprocessing not available");
        println!("Enable with: cargo run --example machine_learning --features ml");
    }

    println!("\n" + "=".repeat(50).as_str());
    println!("🧠 Machine Learning Models");
    println!("=".repeat(50));

    println!("\n✅ All machine learning examples completed!");
    println!("💡 Tip: Enable the 'ml' feature for full functionality:");
    println!("   cargo run --example machine_learning --features ml");

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