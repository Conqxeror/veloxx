use crate::dataframe::DataFrame;
use crate::types::Value;
use crate::VeloxxError;

/// Defines conditions that can be used to filter rows in a `DataFrame`.
///
/// These conditions allow for flexible and powerful data filtering based on column values.
/// They can be combined using logical operators (`And`, `Or`, `Not`) to create complex
/// filtering criteria.
///
/// # Examples
///
/// ## Equality Condition
///
/// Filter rows where the "city" column is equal to "New York":
///
/// ```rust
/// use veloxx::conditions::Condition;
/// use veloxx::types::Value;
///
/// let condition = Condition::Eq("city".to_string(), Value::String("New York".to_string()));
/// // This condition can then be used with a DataFrame's filter method.
/// ```
///
/// ## Greater Than Condition
///
/// Filter rows where the "age" column is greater than 30:
///
/// ```rust
/// use veloxx::conditions::Condition;
/// use veloxx::types::Value;
///
/// let condition = Condition::Gt("age".to_string(), Value::I32(30));
/// // This condition can then be used with a DataFrame's filter method.
/// ```
///
/// ## Less Than Condition
///
/// Filter rows where the "sales" column is less than 100.5:
///
/// ```rust
/// use veloxx::conditions::Condition;
/// use veloxx::types::Value;
///
/// let condition = Condition::Lt("sales".to_string(), Value::F64(100.5));
/// // This condition can then be used with a DataFrame's filter method.
/// ```
///
/// ## Combined Conditions (AND, OR, NOT)
///
/// Filter rows where "age" is greater than 25 AND "city" is "London":
///
/// ```rust
/// use veloxx::conditions::Condition;
/// use veloxx::types::Value;
///
/// let condition = Condition::And(
///     Box::new(Condition::Gt("age".to_string(), Value::I32(25))),
///     Box::new(Condition::Eq("city".to_string(), Value::String("London".to_string()))),
/// );
/// // This condition can then be used with a DataFrame's filter method.
/// ```
///
/// Filter rows where "status" is "active" OR "last_login" is null:
///
/// ```rust
/// use veloxx::conditions::Condition;
/// use veloxx::types::Value;
///
/// let condition = Condition::Or(
///     Box::new(Condition::Eq("status".to_string(), Value::String("active".to_string()))),
///     Box::new(Condition::Eq("last_login".to_string(), Value::Null)), // Assuming Value::Null exists for null checks
/// );
/// // This condition can then be used with a DataFrame's filter method.
/// ```
///
/// Filter rows where "is_admin" is NOT true:
///
/// ```rust
/// use veloxx::conditions::Condition;
/// use veloxx::types::Value;
///
/// let condition = Condition::Not(Box::new(Condition::Eq("is_admin".to_string(), Value::Bool(true))));
/// // This condition can then be used with a DataFrame's filter method.
/// ```
#[derive(Debug, Clone)]
pub enum Condition {
    /// Represents an equality comparison (column == value).
    ///
    /// # Arguments
    /// - `String`: The name of the column to compare.
    /// - `Value`: The value to compare against.
    Eq(String, Value),
    /// Represents a greater than comparison (column > value).
    ///
    /// # Arguments
    /// - `String`: The name of the column to compare.
    /// - `Value`: The value to compare against.
    Gt(String, Value),
    /// Represents a less than comparison (column < value).
    ///
    /// # Arguments
    /// - `String`: The name of the column to compare.
    /// - `Value`: The value to compare against.
    Lt(String, Value),
    /// Represents a logical AND operation between two conditions.
    ///
    /// Both sub-conditions must evaluate to `true` for the `And` condition to be `true`.
    ///
    /// # Arguments
    /// - `Box<Condition>`: The left-hand side condition.
    /// - `Box<Condition>`: The right-hand side condition.
    And(Box<Condition>, Box<Condition>),
    /// Represents a logical OR operation between two conditions.
    ///
    /// At least one sub-condition must evaluate to `true` for the `Or` condition to be `true`.
    ///
    /// # Arguments
    /// - `Box<Condition>`: The left-hand side condition.
    /// - `Box<Condition>`: The right-hand side condition.
    Or(Box<Condition>, Box<Condition>),
    /// Represents a logical NOT operation on a condition.
    ///
    /// Inverts the boolean result of the wrapped condition.
    ///
    /// # Arguments
    /// - `Box<Condition>`: The condition to negate.
    Not(Box<Condition>),
}

impl Condition {
    /// Evaluates the condition for a specific row in the `DataFrame`.
    ///
    /// Returns `true` if the condition is met, `false` otherwise.
    /// Returns an error if a specified column is not found or if types are incomparable.
    pub fn evaluate(&self, df: &DataFrame, row_index: usize) -> Result<bool, VeloxxError> {
        match self {
            Condition::Eq(col_name, value) => {
                let series = df
                    .get_column(col_name)
                    .ok_or(VeloxxError::ColumnNotFound(col_name.to_string()))?;
                let cell_value = series.get_value(row_index);
                Ok(cell_value.as_ref() == Some(value))
            }
            Condition::Gt(col_name, value) => {
                let series = df
                    .get_column(col_name)
                    .ok_or(VeloxxError::ColumnNotFound(col_name.to_string()))?;
                let cell_value = series.get_value(row_index);
                match (cell_value.as_ref(), value) {
                    (Some(Value::I32(a)), Value::I32(b)) => Ok(a > b),
                    (Some(Value::F64(a)), Value::F64(b)) => Ok(a > b),
                    _ => Err(VeloxxError::InvalidOperation(format!(
                        "Cannot compare {cell_value:?} and {value:?}"
                    ))),
                }
            }
            Condition::Lt(col_name, value) => {
                let series = df
                    .get_column(col_name)
                    .ok_or(VeloxxError::ColumnNotFound(col_name.to_string()))?;
                let cell_value = series.get_value(row_index);
                match (cell_value.as_ref(), value) {
                    (Some(Value::I32(a)), Value::I32(b)) => Ok(a < b),
                    (Some(Value::F64(a)), Value::F64(b)) => Ok(a < b),
                    _ => Err(VeloxxError::InvalidOperation(format!(
                        "Cannot compare {cell_value:?} and {value:?}"
                    ))),
                }
            }
            Condition::And(left, right) => {
                Ok(left.evaluate(df, row_index)? && right.evaluate(df, row_index)?)
            }
            Condition::Or(left, right) => {
                Ok(left.evaluate(df, row_index)? || right.evaluate(df, row_index)?)
            }
            Condition::Not(cond) => Ok(!cond.evaluate(df, row_index)?),
        }
    }
}
