use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::types::Value;

use indexmap::IndexMap;

#[test]
fn test_to_csv() {
    let mut columns = IndexMap::new();
    columns.insert(
        "col1".to_string(),
        Series::new_i32("col1", vec![Some(1), Some(2), Some(3)]),
    );
    columns.insert(
        "col2".to_string(),
        Series::new_string(
            "col2",
            vec![
                Some("a".to_string()),
                Some("b".to_string()),
                Some("c".to_string()),
            ],
        ),
    );
    let df = DataFrame::new(columns);

    let path = "test.csv";
    df.to_csv(path).unwrap();

    let content = std::fs::read_to_string(path).unwrap();
    let expected = "col1,col2\n1,a\n2,b\n3,c\n";
    assert_eq!(content, expected);

    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_from_csv() {
    let csv_data = "col1,col2\n1,a\n2,b\n3,c\n";
    let path = "test_from.csv";
    std::fs::write(path, csv_data).unwrap();

    let df = DataFrame::from_csv(path).unwrap();
    println!("{:?}", df);

    let col1 = df.get_column("col1").unwrap();
    let col2 = df.get_column("col2").unwrap();

    let col1_values: Vec<i32> = (0..col1.len())
        .map(|i| match col1.get_value(i) {
            Some(Value::I32(v)) => v,
            _ => panic!("Expected I32 value"),
        })
        .collect();
    let col2_values: Vec<String> = (0..col2.len())
        .map(|i| match col2.get_value(i) {
            Some(Value::String(v)) => v,
            _ => panic!("Expected String value"),
        })
        .collect();

    assert_eq!(col1_values, vec![1, 2, 3]);
    assert_eq!(
        col2_values,
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );

    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_from_csv_nonexistent_file() {
    let result = DataFrame::from_csv("nonexistent.csv");
    assert!(result.is_err());
}

#[test]
fn test_from_csv_malformed() {
    let path = "malformed.csv";
    std::fs::write(path, "col1,col2\n1,a\n2").unwrap(); // malformed: missing value in row 2
    let result = DataFrame::from_csv(path);
    println!("{:?}", result);
    assert!(result.is_err());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_empty_dataframe_to_from_csv() {
    let columns = IndexMap::new();
    let df = DataFrame::new(columns);
    let path = "empty.csv";
    df.to_csv(path).unwrap();
    let df2 = DataFrame::from_csv(path).unwrap();
    assert_eq!(df2.column_names().len(), 0);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_get_nonexistent_column() {
    let mut columns = IndexMap::new();
    columns.insert("col1".to_string(), Series::new_i32("col1", vec![Some(1)]));
    let df = DataFrame::new(columns);
    assert!(df.get_column("colX").is_none());
}
