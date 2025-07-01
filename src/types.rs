use std::hash::{Hash, Hasher};

/// Defines the possible data types for a `Series` or `Value`.
#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    /// 32-bit signed integer type.
    I32,
    /// 64-bit floating-point number type.
    F64,
    /// Boolean type.
    Bool,
    /// String type.
    String,
}

#[derive(Debug, Clone)]
/// Represents a single data point within a `Series` or `DataFrame`.
///
/// This enum can hold various types of data, including integers, floats, booleans, and strings,
/// and also includes a `Null` variant to represent missing values.
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
}

impl Value {
    /// Returns the `DataType` of the `Value`.
    ///
    /// # Panics
    /// Panics if called on a `Value::Null` variant, as `Null` does not have a concrete `DataType`.
    pub fn data_type(&self) -> DataType {
        match self {
            Value::I32(_) => DataType::I32,
            Value::F64(_) => DataType::F64,
            Value::Bool(_) => DataType::Bool,
            Value::String(_) => DataType::String,
            Value::Null => panic!("Cannot get data type of a Null value"),
        }
    }
}

impl PartialEq for Value {
    /// Compares two `Value` instances for equality.
    ///
    /// `Null` values are considered equal to other `Null` values.
    /// For `F64` values, a bitwise comparison is used to handle floating-point precision.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::I32(l), Value::I32(r)) => l == r,
            (Value::F64(l), Value::F64(r)) => l.to_bits() == r.to_bits(), // Compare bitwise for f64
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            _ => false,
        }
    }
}

/// Implements the `Eq` trait for `Value`.
///
/// This is a marker trait that indicates that `PartialEq` implies a total equivalence relation.
/// It has no methods and simply inherits `PartialEq`'s requirements.
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
        }
    }
}

impl Hash for Value {
    /// Implements the `Hash` trait for `Value`.
    ///
    /// This allows `Value` instances to be used as keys in hash maps.
    /// For `F64` values, the bit representation is hashed to ensure consistency.
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Null => 0.hash(state),
            Value::I32(v) => v.hash(state),
            Value::F64(v) => v.to_bits().hash(state), // Hash bitwise for f64
            Value::Bool(v) => v.hash(state),
            Value::String(v) => v.hash(state),
        }
    }
}

impl PartialOrd for Value {
    /// Compares two `Value` instances for partial ordering.
    ///
    /// Numeric and boolean values are compared directly. Strings are compared lexicographically.
    /// `Null` values are considered less than any non-`Null` value.
    /// Returns `None` for comparisons between incomparable types (e.g., `I32` and `String`).
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    /// Implements the `Ord` trait for `Value`.
    ///
    /// This provides a total ordering for `Value` instances, which is necessary for sorting.
    /// For incomparable types (e.g., `I32` and `String`), a consistent but arbitrary ordering is defined
    /// based on the discriminant of the `Value` enum variants.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or_else(|| {
            // Define a consistent ordering for incomparable types (e.g., different variants)
            // This is arbitrary but necessary for a total order.
            // For example, order by variant discriminant, then by value.
            self.discriminant().cmp(&other.discriminant())
        })
    }
}