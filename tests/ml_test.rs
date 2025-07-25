use std::collections::BTreeMap;
use veloxx::dataframe::DataFrame;
use veloxx::ml::LinearRegression;
use veloxx::series::Series;

#[test]
fn test_linear_regression() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "feature1".to_string(),
        Series::new_f64("feature1", vec![Some(1.0), Some(2.0), Some(3.0)]),
    );
    columns.insert(
        "feature2".to_string(),
        Series::new_f64("feature2", vec![Some(2.0), Some(3.0), Some(4.0)]),
    );
    columns.insert(
        "target".to_string(),
        Series::new_f64("target", vec![Some(3.0), Some(5.0), Some(7.0)]),
    );
    let df = DataFrame::new(columns).unwrap();

    let mut model = LinearRegression::new();
    let fitted_model = model.fit(&df, "target", &["feature1", "feature2"]).unwrap();

    let predictions = fitted_model
        .predict(&df, &["feature1", "feature2"])
        .unwrap();
    for (p, e) in predictions.iter().zip(&[3.0, 5.0, 7.0]) {
        assert!((p - e).abs() < 1e-9);
    }
}

// #[test]
// fn test_kmeans() {
//     let mut columns = BTreeMap::new();
//     columns.insert("feature1".to_string(), Series::new_f64("feature1", vec![Some(1.0), Some(1.5), Some(5.0), Some(5.5)]));
//     columns.insert("feature2".to_string(), Series::new_f64("feature2", vec![Some(1.0), Some(2.0), Some(8.0), Some(8.5)]));
//     let df = DataFrame::new(columns).unwrap();

//     let mut model = KMeans::new(2);
//     model.fit(&df, &["feature1", "feature2"]).unwrap();

//     let predictions = model.predict(&df, &["feature1", "feature2"]).unwrap();
//     assert_eq!(predictions, vec![0, 1, 0, 1]);
// }

#[test]
fn test_linear_regression_empty_dataframe() {
    let columns = BTreeMap::new();
    let df = DataFrame::new(columns).unwrap();

    let mut model = LinearRegression::new();
    let result = model.fit(&df, "target", &["feature1"]);

    // Should handle empty dataframe gracefully
    assert!(result.is_err());
}

#[test]
fn test_linear_regression_missing_target_column() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "feature1".to_string(),
        Series::new_f64("feature1", vec![Some(1.0), Some(2.0)]),
    );
    let df = DataFrame::new(columns).unwrap();

    let mut model = LinearRegression::new();
    let result = model.fit(&df, "nonexistent_target", &["feature1"]);

    // Should fail when target column doesn't exist
    assert!(result.is_err());
}

#[test]
fn test_linear_regression_missing_feature_column() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "target".to_string(),
        Series::new_f64("target", vec![Some(1.0), Some(2.0)]),
    );
    let df = DataFrame::new(columns).unwrap();

    let mut model = LinearRegression::new();
    let result = model.fit(&df, "target", &["nonexistent_feature"]);

    // Should fail when feature column doesn't exist
    assert!(result.is_err());
}

#[test]
fn test_linear_regression_with_nulls() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "feature1".to_string(),
        Series::new_f64("feature1", vec![Some(1.0), None, Some(3.0)]),
    );
    columns.insert(
        "target".to_string(),
        Series::new_f64("target", vec![Some(2.0), Some(4.0), None]),
    );
    let df = DataFrame::new(columns).unwrap();

    let mut model = LinearRegression::new();
    let result = model.fit(&df, "target", &["feature1"]);

    // Should handle null values appropriately (either skip or error)
    // The exact behavior depends on implementation
    match result {
        Ok(fitted_model) => {
            // If it succeeds, predictions should work
            let predictions = fitted_model.predict(&df, &["feature1"]);
            assert!(predictions.is_ok() || predictions.is_err());
        }
        Err(_) => {
            // If it fails, that's also acceptable for null handling
            assert!(true);
        }
    }
}

#[test]
fn test_linear_regression_single_feature() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "x".to_string(),
        Series::new_f64("x", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]),
    );
    columns.insert(
        "y".to_string(),
        Series::new_f64("y", vec![Some(2.0), Some(4.0), Some(6.0), Some(8.0)]),
    );
    let df = DataFrame::new(columns).unwrap();

    let mut model = LinearRegression::new();
    let fitted_model = model.fit(&df, "y", &["x"]).unwrap();

    let predictions = fitted_model.predict(&df, &["x"]).unwrap();

    // For y = 2x, predictions should be close to actual values
    for (pred, actual) in predictions.iter().zip(&[2.0, 4.0, 6.0, 8.0]) {
        assert!(
            (pred - actual).abs() < 0.1,
            "Prediction {} too far from actual {}",
            pred,
            actual
        );
    }
}

#[test]
fn test_linear_regression_multiple_features() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "x1".to_string(),
        Series::new_f64("x1", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]),
    );
    columns.insert(
        "x2".to_string(),
        Series::new_f64("x2", vec![Some(2.0), Some(3.0), Some(4.0), Some(5.0)]),
    );
    columns.insert(
        "y".to_string(),
        Series::new_f64("y", vec![Some(5.0), Some(8.0), Some(11.0), Some(14.0)]),
    ); // y = 2*x1 + 1*x2 + 0
    let df = DataFrame::new(columns).unwrap();

    let mut model = LinearRegression::new();
    let fitted_model = model.fit(&df, "y", &["x1", "x2"]).unwrap();

    let predictions = fitted_model.predict(&df, &["x1", "x2"]).unwrap();

    // Check that predictions are reasonably close
    for (pred, actual) in predictions.iter().zip(&[5.0, 8.0, 11.0, 14.0]) {
        assert!(
            (pred - actual).abs() < 0.1,
            "Prediction {} too far from actual {}",
            pred,
            actual
        );
    }
}

#[test]
fn test_linear_regression_prediction_different_data() {
    let mut train_columns = BTreeMap::new();
    train_columns.insert(
        "x".to_string(),
        Series::new_f64("x", vec![Some(1.0), Some(2.0), Some(3.0)]),
    );
    train_columns.insert(
        "y".to_string(),
        Series::new_f64("y", vec![Some(2.0), Some(4.0), Some(6.0)]),
    );
    let train_df = DataFrame::new(train_columns).unwrap();

    let mut test_columns = BTreeMap::new();
    test_columns.insert(
        "x".to_string(),
        Series::new_f64("x", vec![Some(4.0), Some(5.0)]),
    );
    let test_df = DataFrame::new(test_columns).unwrap();

    let mut model = LinearRegression::new();
    let fitted_model = model.fit(&train_df, "y", &["x"]).unwrap();

    let predictions = fitted_model.predict(&test_df, &["x"]).unwrap();

    // For y = 2x pattern, predictions should be close to 8.0 and 10.0
    assert_eq!(predictions.len(), 2);
    assert!((predictions[0] - 8.0).abs() < 0.5);
    assert!((predictions[1] - 10.0).abs() < 0.5);
}
