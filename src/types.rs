use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Defines the possible data types for a `Series` or `Value`.
///
/// This enum is used to strongly type data within the Veloxx library, ensuring type safety
/// and enabling type-specific operations. It supports common primitive types as well as a
/// dedicated DateTime type.
///
/// # Examples
///
/// ```rust
/// use veloxx::types::DataType;
///
/// let int_type = DataType::I32;
/// let string_type = DataType::String;
///
/// assert_eq!(int_type, DataType::I32);
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    bincode::Encode,
    bincode::Decode,
)]
pub enum DataType {
    /// 32-bit signed integer type.
    I32,
    /// 64-bit floating-point number type.
    F64,
    /// Boolean type.
    Bool,
    /// String type.
    String,
    /// DateTime type, represented as a Unix timestamp (i64).
    DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
/// Represents a single data point within a `Series` or `DataFrame`.
///
/// This enum can hold various types of data, including integers, floats, booleans, and strings,
/// and also includes a `Null` variant to represent missing values.
/// `Value` implements `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` to allow for
/// comparisons, sorting, and use in hash-based collections.
///
/// # Examples
///
/// ```rust
/// use veloxx::types::Value;
///
/// let int_value = Value::I32(10);
/// let float_value = Value::F64(3.14);
/// let bool_value = Value::Bool(true);
/// let string_value = Value::String("hello".to_string());
/// let null_value = Value::Null;
///
/// assert_eq!(int_value, Value::I32(10));
/// assert_ne!(float_value, Value::F64(3.0));
/// assert!(null_value == Value::Null);
/// ```
pub enum Value {
    /// Represents a null or missing value.
    Null,
    /// A 32-bit signed integer value.
    I32(i32),
    /// A 64-bit floating-point number value.
    F64(f64),
    /// A boolean value.
    Bool(bool),
    /// A string value.
    String(String),
    /// A DateTime value, represented as a Unix timestamp (i64).
    DateTime(i64),
}

impl Value {
    /// Returns the `DataType` of the `Value`.
    ///
    /// # Panics
    /// Panics if called on a `Value::Null` variant, as `Null` does not have a concrete `DataType`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::{DataType, Value};
    ///
    /// let value = Value::I32(10);
    /// assert_eq!(value.data_type(), DataType::I32);
    ///
    /// // This would panic:
    /// // let null_value = Value::Null;
    /// // null_value.data_type();
    /// ```
    pub fn data_type(&self) -> DataType {
        match self {
            Value::I32(_) => DataType::I32,
            Value::F64(_) => DataType::F64,
            Value::Bool(_) => DataType::Bool,
            Value::String(_) => DataType::String,
            Value::DateTime(_) => DataType::DateTime,
            Value::Null => panic!("Cannot get data type of a Null value"),
        }
    }

    /// Attempts to convert the `Value` into an `i32`.
    /// Returns `Some(i32)` if the `Value` is `I32`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::Value;
    ///
    /// assert_eq!(Value::I32(10).as_i32(), Some(10));
    /// assert_eq!(Value::F64(10.0).as_i32(), None);
    /// ```
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::I32(v) => Some(*v),
            _ => None,
        }
    }

    /// Attempts to convert the `Value` into an `f64`.
    /// Returns `Some(f64)` if the `Value` is `F64`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::Value;
    ///
    /// assert_eq!(Value::F64(10.5).as_f64(), Some(10.5));
    /// assert_eq!(Value::I32(10).as_f64(), None);
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(v) => Some(*v),
            _ => None,
        }
    }

    /// Attempts to convert the `Value` into a `bool`.
    /// Returns `Some(bool)` if the `Value` is `Bool`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::Value;
    ///
    /// assert_eq!(Value::Bool(true).as_bool(), Some(true));
    /// assert_eq!(Value::I32(0).as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Attempts to convert the `Value` into a `String` reference.
    /// Returns `Some(&String)` if the `Value` is `String`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::Value;
    ///
    /// assert_eq!(Value::String("hello".to_string()).as_string(), Some(&"hello".to_string()));
    /// assert_eq!(Value::I32(0).as_string(), None);
    /// ```
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(v) => Some(v),
            _ => None,
        }
    }

    /// Attempts to convert the `Value` into an `i64` (for DateTime).
    /// Returns `Some(i64)` if the `Value` is `DateTime`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::Value;
    ///
    /// assert_eq!(Value::DateTime(1672531200).as_datetime(), Some(1672531200));
    /// assert_eq!(Value::I32(0).as_datetime(), None);
    /// ```
    pub fn as_datetime(&self) -> Option<i64> {
        match self {
            Value::DateTime(v) => Some(*v),
            _ => None,
        }
    }
}

impl PartialEq for Value {
    /// Compares two `Value` instances for equality.
    ///
    /// `Null` values are considered equal to other `Null` values.
    /// For `F64` values, a bitwise comparison is used to handle floating-point precision.
    /// Comparisons between different concrete types (e.g., `I32` and `F64`) will always return `false`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::Value;
    ///
    /// assert_eq!(Value::I32(5), Value::I32(5));
    /// assert_ne!(Value::I32(5), Value::I32(10));
    /// assert_eq!(Value::Null, Value::Null);
    /// assert_ne!(Value::I32(1), Value::Null);
    /// assert!((Value::F64(0.1 + 0.2).as_f64().unwrap() - 0.3).abs() < 1e-10);
    /// // Bitwise comparison is not reliable for floating-point values due to precision issues.
    /// // Use approximate comparison for f64 values.
    /// assert_eq!(Value::I32(5), Value::I32(5));
    /// assert_ne!(Value::I32(5), Value::I32(10));
    /// assert_eq!(Value::Null, Value::Null);
    /// assert_ne!(Value::I32(1), Value::Null);
    /// // assert_eq!(Value::F64(0.1 + 0.2), Value::F64(0.3)); // Not reliable, see above.
    ///
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::I32(l), Value::I32(r)) => l == r,
            (Value::F64(l), Value::F64(r)) => l.to_bits() == r.to_bits(), // Compare bitwise for f64
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::DateTime(l), Value::DateTime(r)) => l == r,
            _ => false,
        }
    }
}

/// Implements the `Eq` trait for `Value`.
///
/// This is a marker trait that indicates that `PartialEq` implies a total equivalence relation.
/// It has no methods and simply inherits `PartialEq`'s requirements.
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::I32(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::DateTime(v) => write!(f, "{}", v),
        }
    }
}

impl Eq for Value {}

impl Value {
    // Helper to get a discriminant for ordering incomparable types
    fn discriminant(&self) -> u8 {
        match self {
            Value::Null => 0,
            Value::I32(_) => 1,
            Value::F64(_) => 2,
            Value::Bool(_) => 3,
            Value::String(_) => 4,
            Value::DateTime(_) => 5,
        }
    }
}

impl Hash for Value {
    /// Implements the `Hash` trait for `Value`.
    ///
    /// This allows `Value` instances to be used as keys in hash maps.
    /// For `F64` values, the bit representation is hashed to ensure consistency.
    /// `Null` values hash to a fixed value (0).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexmap::IndexMap;
    /// use veloxx::types::Value;
    ///
    /// let mut map = IndexMap::new();
    /// map.insert(Value::I32(10), "ten");
    /// map.insert(Value::String("hello".to_string()), "greeting");
    ///
    /// assert_eq!(map.get(&Value::I32(10)), Some(&"ten"));
    /// ```
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Null => 0.hash(state),
            Value::I32(v) => v.hash(state),
            Value::F64(v) => v.to_bits().hash(state), // Hash bitwise for f64
            Value::Bool(v) => v.hash(state),
            Value::String(v) => v.hash(state),
            Value::DateTime(v) => v.hash(state),
        }
    }
}

impl PartialOrd for Value {
    /// Compares two `Value` instances for partial ordering.
    ///
    /// Numeric and boolean values are compared directly. Strings are compared lexicographically.
    /// `Null` values are considered less than any non-`Null` value.
    /// Returns `None` for comparisons between incomparable types (e.g., `I32` and `String`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::Value;
    ///
    /// assert!(Value::I32(5) < Value::I32(10));
    /// assert!(Value::F64(3.0) <= Value::F64(3.14));
    /// assert!(Value::Null < Value::I32(1));
    /// assert_eq!(Value::I32(1).partial_cmp(&Value::String("a".to_string())), None);
    /// ```
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (self, other) {
            // Null comparisons - Null is less than everything except Null
            (Value::Null, Value::Null) => Some(Ordering::Equal),
            (Value::Null, _) => Some(Ordering::Less),
            (_, Value::Null) => Some(Ordering::Greater),

            // Same type comparisons
            (Value::I32(a), Value::I32(b)) => a.partial_cmp(b),
            (Value::F64(a), Value::F64(b)) => a.partial_cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::DateTime(a), Value::DateTime(b)) => a.partial_cmp(b),

            // Cross-type numeric comparisons
            (Value::I32(a), Value::F64(b)) => (*a as f64).partial_cmp(b),
            (Value::F64(a), Value::I32(b)) => a.partial_cmp(&(*b as f64)),

            // Different types - return None for incomparable types
            _ => None,
        }
    }
}

impl Ord for Value {
    /// Compares two `Value` instances for total ordering.
    ///
    /// This implementation provides a consistent ordering for all `Value` variants.
    /// Numeric and boolean values are compared directly. Strings are compared lexicographically.
    /// `Null` values are ordered before all other values.
    /// When comparing values of different concrete types, a fixed discriminant is used to establish an order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::Value;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Value::I32(5).cmp(&Value::I32(10)), Ordering::Less);
    /// assert_eq!(Value::Null.cmp(&Value::I32(1)), Ordering::Less);
    /// assert_eq!(Value::String("apple".to_string()).cmp(&Value::String("banana".to_string())), Ordering::Less);
    /// ```
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(ord) = self.partial_cmp(other) {
            ord
        } else {
            self.discriminant().cmp(&other.discriminant())
        }
    }
}

/// A flattened representation of `Value` for efficient serialization and deserialization.
///
/// This enum stores `F64` values as their bit representation (`u64`) and `String` values
/// as byte vectors (`Vec<u8>`) to facilitate direct binary encoding/decoding.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub enum FlatValue {
    /// Represents a null or missing value.
    Null,
    /// A 32-bit signed integer value.
    I32(i32),
    /// A 64-bit floating-point number value, stored as its bit representation.
    F64(u64), // Store bit representation
    /// A boolean value.
    Bool(bool),
    /// A string value, stored as its UTF-8 byte representation.
    String(Vec<u8>), // Store byte representation
    /// A DateTime value, represented as a Unix timestamp (i64).
    DateTime(i64),
}

impl From<Value> for FlatValue {
    /// Converts a `Value` into a `FlatValue`.
    ///
    /// This conversion is used for serialization purposes, transforming `f64` to `u64` bits
    /// and `String` to `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::{Value, FlatValue};
    ///
    /// let value = Value::I32(123);
    /// let flat_value: FlatValue = value.into();
    /// assert_eq!(flat_value, FlatValue::I32(123));
    ///
    /// let string_value = Value::String("test".to_string());
    /// let flat_string_value: FlatValue = string_value.into();
    /// assert_eq!(flat_string_value, FlatValue::String(vec![116, 101, 115, 116]));
    /// ```
    fn from(value: Value) -> Self {
        match value {
            Value::Null => FlatValue::Null,
            Value::I32(v) => FlatValue::I32(v),
            Value::F64(v) => FlatValue::F64(v.to_bits()),
            Value::Bool(v) => FlatValue::Bool(v),
            Value::String(v) => FlatValue::String(v.into_bytes()),
            Value::DateTime(v) => FlatValue::DateTime(v),
        }
    }
}

impl From<FlatValue> for Value {
    /// Converts a `FlatValue` back into a `Value`.
    ///
    /// This conversion is used for deserialization, reconstructing the original `Value`
    /// from its flattened representation. Handles potential UTF-8 errors during string conversion.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::types::{Value, FlatValue};
    ///
    /// let flat_value = FlatValue::I32(123);
    /// let value: Value = flat_value.into();
    /// assert_eq!(value, Value::I32(123));
    ///
    /// let flat_string_value = FlatValue::String(vec![116, 101, 115, 116]);
    /// let string_value: Value = flat_string_value.into();
    /// assert_eq!(string_value, Value::String("test".to_string()));
    /// ```
    fn from(flat_value: FlatValue) -> Self {
        match flat_value {
            FlatValue::Null => Value::Null,
            FlatValue::I32(v) => Value::I32(v),
            FlatValue::F64(v) => Value::F64(f64::from_bits(v)),
            FlatValue::Bool(v) => Value::Bool(v),
            FlatValue::String(v) => Value::String(String::from_utf8(v).unwrap_or_default()), // Handle potential UTF-8 errors
            FlatValue::DateTime(v) => Value::DateTime(v),
        }
    }
}
