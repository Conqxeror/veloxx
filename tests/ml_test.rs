
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::ml::{LinearRegression};
use std::collections::BTreeMap;

#[test]
fn test_linear_regression() {
    let mut columns = BTreeMap::new();
    columns.insert("feature1".to_string(), Series::new_f64("feature1", vec![Some(1.0), Some(2.0), Some(3.0)]));
    columns.insert("feature2".to_string(), Series::new_f64("feature2", vec![Some(2.0), Some(3.0), Some(4.0)]));
    columns.insert("target".to_string(), Series::new_f64("target", vec![Some(3.0), Some(5.0), Some(7.0)]));
    let df = DataFrame::new(columns).unwrap();

    let mut model = LinearRegression::new();
    let fitted_model = model.fit(&df, "target", &["feature1", "feature2"]).unwrap();

    let predictions = fitted_model.predict(&df, &["feature1", "feature2"]).unwrap();
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
