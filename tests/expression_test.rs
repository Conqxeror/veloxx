use std::collections::BTreeMap;
use veloxx::dataframe::DataFrame;
use veloxx::expressions::Expr;
use veloxx::series::Series;
use veloxx::types::Value;

#[test]
fn test_column_expression() {
    let expr = Expr::Column("test_col".to_string());

    match expr {
        Expr::Column(name) => assert_eq!(name, "test_col"),
        _ => panic!("Expected Column expression"),
    }
}

#[test]
fn test_literal_expression() {
    let expr = Expr::Literal(Value::I32(42));

    match expr {
        Expr::Literal(Value::I32(val)) => assert_eq!(val, 42),
        _ => panic!("Expected Literal expression with I32 value"),
    }
}

#[test]
fn test_add_expression() {
    let left = Expr::Column("col1".to_string());
    let right = Expr::Literal(Value::I32(10));
    let expr = Expr::Add(Box::new(left), Box::new(right));

    match expr {
        Expr::Add(_, _) => assert!(true),
        _ => panic!("Expected Add expression"),
    }
}

#[test]
fn test_multiply_expression() {
    let left = Expr::Column("col1".to_string());
    let right = Expr::Literal(Value::I32(5));
    let expr = Expr::Multiply(Box::new(left), Box::new(right));

    match expr {
        Expr::Multiply(_, _) => assert!(true),
        _ => panic!("Expected Multiply expression"),
    }
}

#[test]
fn test_equals_expression() {
    let left = Expr::Column("col1".to_string());
    let right = Expr::Literal(Value::I32(10));
    let expr = Expr::Equals(Box::new(left), Box::new(right));

    match expr {
        Expr::Equals(_, _) => assert!(true),
        _ => panic!("Expected Equals expression"),
    }
}

#[test]
fn test_greater_than_expression() {
    let left = Expr::Column("age".to_string());
    let right = Expr::Literal(Value::I32(18));
    let expr = Expr::GreaterThan(Box::new(left), Box::new(right));

    match expr {
        Expr::GreaterThan(_, _) => assert!(true),
        _ => panic!("Expected GreaterThan expression"),
    }
}

#[test]
fn test_and_expression() {
    let left = Expr::Column("is_active".to_string());
    let right = Expr::Column("is_verified".to_string());
    let expr = Expr::And(Box::new(left), Box::new(right));

    match expr {
        Expr::And(_, _) => assert!(true),
        _ => panic!("Expected And expression"),
    }
}

#[test]
fn test_not_expression() {
    let inner = Expr::Column("is_suspended".to_string());
    let expr = Expr::Not(Box::new(inner));

    match expr {
        Expr::Not(_) => {} // Test passes - we found the expected Not expression
        _ => panic!("Expected Not expression"),
    }
}

#[test]
fn test_expression_evaluation() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(25), Some(30), Some(35)]),
    );
    columns.insert(
        "score".to_string(),
        Series::new_f64("score", vec![Some(85.5), Some(92.0), Some(78.5)]),
    );
    let df = DataFrame::new(columns).unwrap();

    // Test column expression evaluation
    let col_expr = Expr::Column("age".to_string());
    let result = col_expr.evaluate(&df, 0).unwrap();
    assert_eq!(result, Value::I32(25));

    // Test literal expression evaluation
    let lit_expr = Expr::Literal(Value::I32(100));
    let result = lit_expr.evaluate(&df, 0).unwrap();
    assert_eq!(result, Value::I32(100));
}

#[test]
fn test_arithmetic_expression_evaluation() {
    let mut columns = BTreeMap::new();
    columns.insert(
        "a".to_string(),
        Series::new_i32("a", vec![Some(10), Some(20)]),
    );
    columns.insert(
        "b".to_string(),
        Series::new_i32("b", vec![Some(5), Some(3)]),
    );
    let df = DataFrame::new(columns).unwrap();

    // Test addition: a + b
    let add_expr = Expr::Add(
        Box::new(Expr::Column("a".to_string())),
        Box::new(Expr::Column("b".to_string())),
    );
    let result = add_expr.evaluate(&df, 0).unwrap();
    assert_eq!(result, Value::I32(15));

    // Test multiplication: a * b
    let mul_expr = Expr::Multiply(
        Box::new(Expr::Column("a".to_string())),
        Box::new(Expr::Column("b".to_string())),
    );
    let result = mul_expr.evaluate(&df, 0).unwrap();
    assert_eq!(result, Value::I32(50));
}
