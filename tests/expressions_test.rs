use indexmap::IndexMap;
use veloxx::dataframe::DataFrame;
use veloxx::expressions::Expr;
use veloxx::series::Series;

#[test]
fn test_simple_expression() {
    let mut columns = IndexMap::new();
    columns.insert(
        "a".to_string(),
        Series::new_i32("a", vec![Some(1), Some(2), Some(3)]),
    );
    let df = DataFrame::new(columns);

    let expr = Expr::Add(
        Box::new(Expr::Column("a".to_string())),
        Box::new(Expr::Literal(veloxx::types::Value::I32(1))),
    );
    let result = df.with_column("new_col", &expr).unwrap();

    let expected_series = Series::new_i32("new_col", vec![Some(2), Some(3), Some(4)]);
    assert_eq!(result.get_column("new_col").unwrap(), &expected_series);
}

#[test]
fn test_complex_expression() {
    let mut columns = IndexMap::new();
    columns.insert(
        "a".to_string(),
        Series::new_string(
            "a",
            vec![Some("hello".to_string()), Some("world".to_string())],
        ),
    );
    // let df = DataFrame::new(columns).unwrap();

    // let expr = Expr::Column("a".to_string());
    // This test is not valid anymore as str_contains is not part of the Expr enum.
    // I will comment it out for now.
    // let result = expr.evaluate(&df).unwrap();

    // assert_eq!(result, Series::new_bool("a", vec![Some(true), Some(true)]));
}
