use veloxx::conditions::Condition;
use veloxx::types::Value;

#[test]
fn test_eq_condition() {
    let condition = Condition::Eq("col1".to_string(), Value::I32(10));
    assert_eq!(format!("{:?}", condition), "Eq(\"col1\", I32(10))");
}

#[test]
fn test_gt_condition() {
    let condition = Condition::Gt("col1".to_string(), Value::I32(10));
    assert_eq!(format!("{:?}", condition), "Gt(\"col1\", I32(10))");
}

#[test]
fn test_lt_condition() {
    let condition = Condition::Lt("col1".to_string(), Value::I32(10));
    assert_eq!(format!("{:?}", condition), "Lt(\"col1\", I32(10))");
}

#[test]
fn test_and_condition() {
    let condition = Condition::And(
        Box::new(Condition::Eq("col1".to_string(), Value::I32(10))),
        Box::new(Condition::Gt("col2".to_string(), Value::F64(5.0))),
    );
    assert_eq!(
        format!("{:?}", condition),
        "And(Eq(\"col1\", I32(10)), Gt(\"col2\", F64(5.0)))"
    );
}

#[test]
fn test_or_condition() {
    let condition = Condition::Or(
        Box::new(Condition::Eq("col1".to_string(), Value::I32(10))),
        Box::new(Condition::Lt("col2".to_string(), Value::F64(5.0))),
    );
    assert_eq!(
        format!("{:?}", condition),
        "Or(Eq(\"col1\", I32(10)), Lt(\"col2\", F64(5.0)))"
    );
}
