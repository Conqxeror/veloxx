use indexmap::IndexMap;
use veloxx::dataframe::join::JoinType;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

#[test]
fn test_inner_join() {
    // Create first DataFrame
    let mut columns1 = IndexMap::new();
    columns1.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(2), Some(3)]),
    );
    columns1.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
            ],
        ),
    );
    let df1 = DataFrame::new(columns1);

    // Create second DataFrame
    let mut columns2 = IndexMap::new();
    columns2.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(2), Some(4)]),
    );
    columns2.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(25), Some(30), Some(35)]),
    );
    let df2 = DataFrame::new(columns2);

    // Perform inner join
    let result = df1.join(&df2, "id", JoinType::Inner).unwrap();

    // Should have 2 rows (id 1 and 2 match)
    assert_eq!(result.row_count(), 2);
    assert_eq!(result.column_count(), 3); // id, name, age

    // Check column names
    let column_names = result.column_names();
    assert!(column_names.contains(&&"id".to_string()));
    assert!(column_names.contains(&&"name".to_string()));
    assert!(column_names.contains(&&"age".to_string()));
}

#[test]
fn test_left_join() {
    // Create first DataFrame
    let mut columns1 = IndexMap::new();
    columns1.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(2), Some(3)]),
    );
    columns1.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
            ],
        ),
    );
    let df1 = DataFrame::new(columns1);

    // Create second DataFrame
    let mut columns2 = IndexMap::new();
    columns2.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(2)]),
    );
    columns2.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(25), Some(30)]),
    );
    let df2 = DataFrame::new(columns2);

    // Perform left join
    let result = df1.join(&df2, "id", JoinType::Left).unwrap();

    // Should have 3 rows (all from left DataFrame)
    assert_eq!(result.row_count(), 3);
    assert_eq!(result.column_count(), 3); // id, name, age
}

#[test]
fn test_right_join() {
    // Create first DataFrame
    let mut columns1 = IndexMap::new();
    columns1.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(2)]),
    );
    columns1.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![Some("Alice".to_string()), Some("Bob".to_string())],
        ),
    );
    let df1 = DataFrame::new(columns1);

    // Create second DataFrame
    let mut columns2 = IndexMap::new();
    columns2.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(2), Some(3)]),
    );
    columns2.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(25), Some(30), Some(35)]),
    );
    let df2 = DataFrame::new(columns2);

    // Perform right join
    let result = df1.join(&df2, "id", JoinType::Right).unwrap();

    // Should have 3 rows (all from right DataFrame)
    assert_eq!(result.row_count(), 3);
    assert_eq!(result.column_count(), 3); // id, name, age
}

#[test]
fn test_join_with_no_matches() {
    // Create first DataFrame
    let mut columns1 = IndexMap::new();
    columns1.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(2)]),
    );
    columns1.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![Some("Alice".to_string()), Some("Bob".to_string())],
        ),
    );
    let df1 = DataFrame::new(columns1);

    // Create second DataFrame with no matching IDs
    let mut columns2 = IndexMap::new();
    columns2.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(3), Some(4)]),
    );
    columns2.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(25), Some(30)]),
    );
    let df2 = DataFrame::new(columns2);

    // Perform inner join - should result in empty DataFrame
    let result = df1.join(&df2, "id", JoinType::Inner).unwrap();
    assert_eq!(result.row_count(), 0);
    assert_eq!(result.column_count(), 3); // Still has the column structure
}

#[test]
fn test_join_with_nulls() {
    // Create first DataFrame with null values
    let mut columns1 = IndexMap::new();
    columns1.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), None, Some(3)]),
    );
    columns1.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
            ],
        ),
    );
    let df1 = DataFrame::new(columns1);

    // Create second DataFrame
    let mut columns2 = IndexMap::new();
    columns2.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(3)]),
    );
    columns2.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(25), Some(35)]),
    );
    let df2 = DataFrame::new(columns2);

    // Perform inner join
    let result = df1.join(&df2, "id", JoinType::Inner).unwrap();

    // Should have 2 rows (null values don't match)
    assert_eq!(result.row_count(), 2);
}

#[test]
fn test_join_nonexistent_column() {
    // Create DataFrames
    let mut columns1 = IndexMap::new();
    columns1.insert("id".to_string(), Series::new_i32("id", vec![Some(1)]));
    let df1 = DataFrame::new(columns1);

    let mut columns2 = IndexMap::new();
    columns2.insert("id".to_string(), Series::new_i32("id", vec![Some(1)]));
    let df2 = DataFrame::new(columns2);

    // Try to join on non-existent column
    let result = df1.join(&df2, "nonexistent", JoinType::Inner);
    assert!(result.is_err());
}
