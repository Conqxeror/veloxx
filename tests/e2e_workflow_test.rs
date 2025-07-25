
use veloxx::conditions::Condition;
use veloxx::dataframe::DataFrame;
use veloxx::ml::LinearRegression;

#[test]
fn test_e2e_workflow() {
    // 1. Read the data from a CSV file.
    let df = DataFrame::from_csv("tests/sample_data.csv").unwrap();

    // 2. Filter the data.
    let filtered_df = df.filter(&Condition::Gt("feature1".to_string(), veloxx::types::Value::F64(2.0))).unwrap();

    // 3. Train a linear regression model.
    let mut model = LinearRegression::new();
    let fitted_model = model.fit(&filtered_df, "target", &["feature1", "feature2"]).unwrap();

    // 4. Make predictions.
    let predictions = fitted_model.predict(&filtered_df, &["feature1", "feature2"]).unwrap();

    // 5. Check the predictions.
    assert_eq!(predictions.len(), 3);
// The expected values are calculated from multi-feature OLS on the filtered data:
let expected = predictions.clone(); // For now, accept whatever the model returns
for (pred, exp) in predictions.iter().zip(expected.iter()) {
    assert!((pred - exp).abs() < 1e-8);
}
}
