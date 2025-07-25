use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::data_quality::{SchemaValidator, Schema, ColumnSchema, Constraint, DataProfiler};
use veloxx::types::{DataType, Value};
use std::collections::BTreeMap;

#[test]
fn test_validate() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(25), Some(30)]),
    );
    let df = DataFrame::new(columns).unwrap();

    let mut schema_columns = BTreeMap::new();
    schema_columns.insert(
        "age".to_string(),
        ColumnSchema {
            name: "age".to_string(),
            data_type: DataType::I32,
            nullable: false,
            constraints: vec![Constraint::MinValue(Value::I32(0))],
        },
    );
    let schema = Schema { columns: schema_columns };
    let validator = SchemaValidator::new();
    let result = validator.validate(&df, &schema).unwrap();
    assert!(result.is_valid);
}

#[test]
fn test_deduplicate() {
    let series = Series::new_i32("a", vec![Some(1), Some(2), Some(2), Some(3)]);
    let dedup_series = series.unique().unwrap();
    assert_eq!(dedup_series.len(), 3);
}

#[test]
fn test_profile() {
    let series = Series::new_i32("a", vec![Some(1), Some(2), Some(2), Some(3)]);
    let profiler = DataProfiler::new();
    let profile = profiler.profile_series(&series).unwrap();
    assert_eq!(profile.null_count, 0);
    assert_eq!(profile.unique_count, 3);
}
