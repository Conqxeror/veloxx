use crate::types::Value;

/// Represents an expression that can be evaluated against a DataFrame row.
pub enum Expr {
    /// Refers to a column by its name.
    Column(String),
    /// Represents a literal value.
    Literal(Value),
    /// Represents an addition operation between two expressions.
    Add(Box<Expr>, Box<Expr>),
    /// Represents a subtraction operation between two expressions.
    Subtract(Box<Expr>, Box<Expr>),
    /// Represents a multiplication operation between two expressions.
    Multiply(Box<Expr>, Box<Expr>),
    /// Represents a division operation between two expressions.
    Divide(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Evaluates the expression for a specific row in the DataFrame.
    ///
    /// Returns the computed `Value` or an error if the expression cannot be evaluated.
    pub fn evaluate(&self, df: &crate::dataframe::DataFrame, row_index: usize) -> Result<Value, String> {
        match self {
            Expr::Column(col_name) => {
                let series = df.get_column(col_name).ok_or(format!("Column '{col_name}' not found."))?;
                series.get_value(row_index).ok_or(format!("Null value at row {row_index} in column {col_name}"))
            }
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Add(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::I32(l + r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::F64(l + r)),
                    _ => Err("Unsupported types for addition".to_string()),
                }
            }
            Expr::Subtract(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::I32(l - r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::F64(l - r)),
                    _ => Err("Unsupported types for subtraction".to_string()),
                }
            }
            Expr::Multiply(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::I32(l * r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::F64(l * r)),
                    _ => Err("Unsupported types for multiplication".to_string()),
                }
            }
            Expr::Divide(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => {
                        if r == 0 { return Err("Division by zero".to_string()); }
                        Ok(Value::I32(l / r))
                    },
                    (Value::F64(l), Value::F64(r)) => {
                        if r == 0.0 { return Err("Division by zero".to_string()); }
                        Ok(Value::F64(l / r))
                    },
                    _ => Err("Unsupported types for division".to_string()),
                }
            }
        }
    }
}