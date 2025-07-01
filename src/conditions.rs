use crate::dataframe::DataFrame;
use crate::types::Value;

/// Defines conditions that can be used to filter rows in a `DataFrame`.
#[derive(Debug)]
pub enum Condition {
    /// Represents an equality comparison (column == value).
    Eq(String, Value),
    /// Represents a greater than comparison (column > value).
    Gt(String, Value),
    /// Represents a less than comparison (column < value).
    Lt(String, Value),
    /// Represents a logical AND operation between two conditions.
    And(Box<Condition>, Box<Condition>),
    /// Represents a logical OR operation between two conditions.
    Or(Box<Condition>, Box<Condition>),
    /// Represents a logical NOT operation on a condition.
    Not(Box<Condition>),
}

impl Condition {
    /// Evaluates the condition for a specific row in the `DataFrame`.
    ///
    /// Returns `true` if the condition is met, `false` otherwise.
    /// Returns an error if a specified column is not found or if types are incomparable.
    pub fn evaluate(&self, df: &DataFrame, row_index: usize) -> Result<bool, String> {
        match self {
            Condition::Eq(col_name, value) => {
                let series = df.get_column(col_name).ok_or(format!("Column '{col_name}' not found."))?;
                let cell_value = series.get_value(row_index);
                Ok(cell_value.as_ref() == Some(value))
            }
            Condition::Gt(col_name, value) => {
                let series = df.get_column(col_name).ok_or(format!("Column '{col_name}' not found."))?;
                let cell_value = series.get_value(row_index);
                match (cell_value.clone(), value) {
                    (Some(Value::I32(a)), Value::I32(b)) => Ok(a > *b),
                    (Some(Value::F64(a)), Value::F64(b)) => Ok(a > *b),
                    _ => Err(format!("Cannot compare {cell_value:?} and {value:?}")),
                }
            }
            Condition::Lt(col_name, value) => {
                let series = df.get_column(col_name).ok_or(format!("Column '{col_name}' not found."))?;
                let cell_value = series.get_value(row_index);
                match (cell_value.clone(), value) {
                    (Some(Value::I32(a)), Value::I32(b)) => Ok(a < *b),
                    (Some(Value::F64(a)), Value::F64(b)) => Ok(a < *b),
                    _ => Err(format!("Cannot compare {cell_value:?} and {value:?}")),
                }
            }
            Condition::And(left, right) => {
                Ok(left.evaluate(df, row_index)? && right.evaluate(df, row_index)?)
            }
            Condition::Or(left, right) => {
                Ok(left.evaluate(df, row_index)? || right.evaluate(df, row_index)?)
            }
            Condition::Not(cond) => {
                Ok(!cond.evaluate(df, row_index)?)
            }
        }
    }
}