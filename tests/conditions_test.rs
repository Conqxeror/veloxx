use veloxx::conditions::Condition;
use veloxx::types::Value;

#[test]
fn test_conditions() {
    let val1 = Value::String("hello".to_string());
    let val2 = Value::I32(42);

    let eq_cond = Condition::Eq("col1".to_string(), val1.clone());
    assert_eq!(format!("{:?}", eq_cond), "Eq(\"col1\", String(\"hello\"))");

    let gt_cond = Condition::Gt("col2".to_string(), val2.clone());
    assert_eq!(format!("{:?}", gt_cond), "Gt(\"col2\", I32(42))");

    let lt_cond = Condition::Lt("col2".to_string(), val2.clone());
    assert_eq!(format!("{:?}", lt_cond), "Lt(\"col2\", I32(42))");

    let and_cond = Condition::And(Box::new(eq_cond), Box::new(gt_cond));
    assert_eq!(
        format!("{:?}", and_cond),
        "And(Eq(\"col1\", String(\"hello\")), Gt(\"col2\", I32(42)))"
    );

    let or_cond = Condition::Or(
        Box::new(lt_cond),
        Box::new(Condition::Gt("col2".to_string(), val2.clone())),
    );
    assert_eq!(
        format!("{:?}", or_cond),
        "Or(Lt(\"col2\", I32(42)), Gt(\"col2\", I32(42)))"
    );
}
