use veloxx::types::{DataType, Value};

#[test]
fn test_datatype_equality() {
    assert_eq!(DataType::I32, DataType::I32);
    assert_eq!(DataType::F64, DataType::F64);
    assert_eq!(DataType::Bool, DataType::Bool);
    assert_eq!(DataType::String, DataType::String);
    assert_eq!(DataType::DateTime, DataType::DateTime);

    assert_ne!(DataType::I32, DataType::F64);
    assert_ne!(DataType::Bool, DataType::String);
}

#[test]
fn test_datatype_clone() {
    let dt = DataType::I32;
    let cloned = dt.clone();
    assert_eq!(dt, cloned);
}

#[test]
fn test_datatype_debug() {
    let dt = DataType::I32;
    let debug_str = format!("{:?}", dt);
    assert_eq!(debug_str, "I32");
}

#[test]
fn test_value_i32() {
    let val = Value::I32(42);
    match val {
        Value::I32(n) => assert_eq!(n, 42),
        _ => panic!("Expected I32 value"),
    }
}

#[test]
fn test_value_f64() {
    let val = Value::F64(2.5);
    match val {
        Value::F64(f) => assert_eq!(f, 2.5),
        _ => panic!("Expected F64 value"),
    }
}

#[test]
fn test_value_bool() {
    let val = Value::Bool(true);
    match val {
        Value::Bool(b) => assert!(b),
        _ => panic!("Expected Bool value"),
    }
}

#[test]
fn test_value_string() {
    let val = Value::String("hello".to_string());
    match val {
        Value::String(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected String value"),
    }
}

#[test]
fn test_value_datetime() {
    let timestamp = 1234567890;
    let val = Value::DateTime(timestamp);
    match val {
        Value::DateTime(ts) => assert_eq!(ts, timestamp),
        _ => panic!("Expected DateTime value"),
    }
}

#[test]
fn test_value_null() {
    let val = Value::Null;
    match val {
        Value::Null => {} // Test passes if we reach this point
        _ => panic!("Expected Null value"),
    }
}

#[test]
fn test_value_equality() {
    assert_eq!(Value::I32(42), Value::I32(42));
    assert_eq!(Value::F64(2.5), Value::F64(2.5));
    assert_eq!(Value::Bool(true), Value::Bool(true));
    assert_eq!(
        Value::String("test".to_string()),
        Value::String("test".to_string())
    );
    assert_eq!(Value::DateTime(123), Value::DateTime(123));
    assert_eq!(Value::Null, Value::Null);

    assert_ne!(Value::I32(42), Value::I32(43));
    assert_ne!(Value::I32(42), Value::F64(42.0));
    assert_ne!(Value::Null, Value::I32(0));
}

#[test]
fn test_value_clone() {
    let val = Value::String("test".to_string());
    let cloned = val.clone();
    assert_eq!(val, cloned);
}

#[test]
fn test_value_debug() {
    let val = Value::I32(42);
    let debug_str = format!("{:?}", val);
    assert!(debug_str.contains("I32"));
    assert!(debug_str.contains("42"));
}

#[test]
fn test_value_ordering() {
    // Test partial ordering for same types
    assert!(Value::I32(1) < Value::I32(2));
    assert!(Value::F64(1.0) < Value::F64(2.0));
    assert!(Value::Bool(false) < Value::Bool(true));
    assert!(Value::String("a".to_string()) < Value::String("b".to_string()));
    assert!(Value::DateTime(100) < Value::DateTime(200));

    // Null should be less than any non-null value
    assert!(Value::Null < Value::I32(0));
    assert!(Value::Null < Value::F64(0.0));
    assert!(Value::Null < Value::Bool(false));
    assert!(Value::Null < Value::String("".to_string()));
    assert!(Value::Null < Value::DateTime(0));
}

#[test]
fn test_value_hash() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(Value::I32(42), "forty-two");
    map.insert(Value::String("hello".to_string()), "greeting");
    map.insert(Value::Null, "nothing");

    assert_eq!(map.get(&Value::I32(42)), Some(&"forty-two"));
    assert_eq!(
        map.get(&Value::String("hello".to_string())),
        Some(&"greeting")
    );
    assert_eq!(map.get(&Value::Null), Some(&"nothing"));
    assert_eq!(map.get(&Value::I32(43)), None);
}
