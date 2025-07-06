use crate::types::Value;
use crate::error::VeloxxError;

/// Represents an expression that can be evaluated against a DataFrame row.
///
/// Expressions are used to define computations, transformations, or logical conditions
/// that can be applied to data within a `DataFrame`. They can refer to columns,
/// literal values, and combine these using various arithmetic, comparison, and logical operators.
///
/// # Examples
///
/// ## Column Expression
///
/// Refers to the value of the "price" column:
///
/// ```rust
/// use veloxx::expressions::Expr;
///
/// let expr = Expr::Column("price".to_string());
/// ```
///
/// ## Literal Expression
///
/// Represents a fixed integer value of 100:
///
/// ```rust
/// use veloxx::expressions::Expr;
/// use veloxx::types::Value;
///
/// let expr = Expr::Literal(Value::I32(100));
/// ```
///
/// ## Arithmetic Expressions
///
/// Calculate "price" + "tax":
///
/// ```rust
/// use veloxx::expressions::Expr;
///
/// let expr = Expr::Add(
///     Box::new(Expr::Column("price".to_string())),
///     Box::new(Expr::Column("tax".to_string())),
/// );
/// ```
///
/// Calculate ("quantity" * 10) - "discount":
///
/// ```rust
/// use veloxx::expressions::Expr;
/// use veloxx::types::Value;
///
/// let expr = Expr::Subtract(
///     Box::new(Expr::Multiply(
///         Box::new(Expr::Column("quantity".to_string())),
///         Box::new(Expr::Literal(Value::I32(10))),
///     )),
///     Box::new(Expr::Column("discount".to_string())),
/// );
/// ```
///
/// ## Comparison Expressions
///
/// Check if "age" is greater than or equal to 18:
///
/// ```rust
/// use veloxx::expressions::Expr;
/// use veloxx::types::Value;
///
/// let expr = Expr::GreaterThanOrEqual(
///     Box::new(Expr::Column("age".to_string())),
///     Box::new(Expr::Literal(Value::I32(18))),
/// );
/// ```
///
/// ## Logical Expressions
///
/// Check if ("is_active" AND NOT "is_suspended"):
///
/// ```rust
/// use veloxx::expressions::Expr;
///
/// let expr = Expr::And(
///     Box::new(Expr::Column("is_active".to_string())),
///     Box::new(Expr::Not(Box::new(Expr::Column("is_suspended".to_string())))),
/// );
/// ```
#[derive(Debug)]
pub enum Expr {
    /// Refers to a column by its name.
    ///
    /// # Arguments
    /// - `String`: The name of the column.
    Column(String),
    /// Represents a literal value.
    ///
    /// # Arguments
    /// - `Value`: The literal value.
    Literal(Value),
    /// Represents an addition operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    Add(Box<Expr>, Box<Expr>),
    /// Represents a subtraction operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    Subtract(Box<Expr>, Box<Expr>),
    /// Represents a multiplication operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    Multiply(Box<Expr>, Box<Expr>),
    /// Represents a division operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    Divide(Box<Expr>, Box<Expr>),
    /// Represents an equality comparison operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    Equals(Box<Expr>, Box<Expr>),
    /// Represents a not-equals comparison operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    NotEquals(Box<Expr>, Box<Expr>),
    /// Represents a greater-than comparison operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    GreaterThan(Box<Expr>, Box<Expr>),
    /// Represents a less-than comparison operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    LessThan(Box<Expr>, Box<Expr>),
    /// Represents a greater-than-or-equal comparison operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    GreaterThanOrEqual(Box<Expr>, Box<Expr>),
    /// Represents a less-than-or-equal comparison operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    LessThanOrEqual(Box<Expr>, Box<Expr>),
    /// Represents a logical AND operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    And(Box<Expr>, Box<Expr>),
    /// Represents a logical OR operation between two expressions.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The left-hand side expression.
    /// - `Box<Expr>`: The right-hand side expression.
    Or(Box<Expr>, Box<Expr>),
    /// Represents a logical NOT operation on an expression.
    ///
    /// # Arguments
    /// - `Box<Expr>`: The expression to negate.
    Not(Box<Expr>),
}

impl Expr {
    /// Evaluates the expression for a specific row in the DataFrame.
    ///
    /// Returns the computed `Value` or an error if the expression cannot be evaluated.
    pub fn evaluate(
        &self,
        df: &crate::dataframe::DataFrame,
        row_index: usize,
    ) -> Result<Value, VeloxxError> {
        match self {
            Expr::Column(col_name) => {
                let series = df
                    .get_column(col_name)
                    .ok_or(VeloxxError::ColumnNotFound(col_name.to_string()))?;
                series.get_value(row_index).ok_or(VeloxxError::InvalidOperation(format!(
                    "Null value at row {row_index} in column {col_name}"
                )))
            }
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Add(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::I32(l + r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::F64(l + r)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for addition".to_string())),
                }
            }
            Expr::Subtract(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::I32(l - r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::F64(l - r)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for subtraction".to_string())),
                }
            }
            Expr::Multiply(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::I32(l * r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::F64(l * r)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for multiplication".to_string())),
                }
            }
            Expr::Divide(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => {
                        if r == 0 {
                            return Err(VeloxxError::InvalidOperation("Division by zero".to_string()));
                        }
                        Ok(Value::I32(l / r))
                    }
                    (Value::F64(l), Value::F64(r)) => {
                        if r == 0.0 {
                            return Err(VeloxxError::InvalidOperation("Division by zero".to_string()));
                        }
                        Ok(Value::F64(l / r))
                    }
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for division".to_string())),
                }
            }
            Expr::Equals(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                Ok(Value::Bool(left_val == right_val))
            }
            Expr::NotEquals(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                Ok(Value::Bool(left_val != right_val))
            }
            Expr::GreaterThan(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::Bool(l > r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::Bool(l > r)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for comparison".to_string())),
                }
            }
            Expr::LessThan(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::Bool(l < r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::Bool(l < r)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for comparison".to_string())),
                }
            }
            Expr::GreaterThanOrEqual(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::Bool(l >= r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::Bool(l >= r)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for comparison".to_string())),
                }
            }
            Expr::LessThanOrEqual(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::I32(l), Value::I32(r)) => Ok(Value::Bool(l <= r)),
                    (Value::F64(l), Value::F64(r)) => Ok(Value::Bool(l <= r)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for comparison".to_string())),
                }
            }
            Expr::And(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l && r)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for logical AND".to_string())),
                }
            }
            Expr::Or(left, right) => {
                let left_val = left.evaluate(df, row_index)?;
                let right_val = right.evaluate(df, row_index)?;
                match (left_val, right_val) {
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l || r)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported types for logical OR".to_string())),
                }
            }
            Expr::Not(expr) => {
                let val = expr.evaluate(df, row_index)?;
                match val {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    _ => Err(VeloxxError::InvalidOperation("Unsupported type for logical NOT".to_string())),
                }
            }
        }
    }
}
