# Rust API Reference

Complete API reference for the Veloxx Rust library.

## Core Modules

### `veloxx::dataframe`

The `DataFrame` is the primary data structure for working with tabular data.

#### `DataFrame`

```rust
pub struct DataFrame {
    // Internal implementation
}
```

##### Constructors

<div className="api-section">
<div className="api-method">DataFrame::new(columns: BTreeMap&lt;String, Series&gt;) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Creates a new DataFrame from a map of column names to Series.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">columns</span>: <span className="parameter-type">BTreeMap&lt;String, Series&gt;</span> - Map of column names to Series objects
</div>
</div>

**Example:**
```rust
use std::collections::BTreeMap;
use veloxx::{DataFrame, Series};

let mut columns = BTreeMap::new();
columns.insert("name".to_string(), Series::new_string("name", vec![
    Some("Alice".to_string()), 
    Some("Bob".to_string())
]));
columns.insert("age".to_string(), Series::new_i32("age", vec![Some(30), Some(25)]));

let df = DataFrame::new(columns)?;
```
</div>

<div className="api-section">
<div className="api-method">DataFrame::from_csv(path: &str) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Loads a DataFrame from a CSV file with automatic type inference.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">path</span>: <span className="parameter-type">&str</span> - Path to the CSV file
</div>
</div>

**Example:**
```rust
let df = DataFrame::from_csv("data/sales.csv")?;
println!("Loaded {} rows", df.row_count());
```
</div>

<div className="api-section">
<div className="api-method">DataFrame::from_json(path: &str) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Loads a DataFrame from a JSON file.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">path</span>: <span className="parameter-type">&str</span> - Path to the JSON file
</div>
</div>

**Example:**
```rust
let df = DataFrame::from_json("data/users.json")?;
```
</div>

<div className="api-section">
<div className="api-method">DataFrame::from_vec_of_vec(data: Vec&lt;Vec&lt;String&gt;&gt;, column_names: Vec&lt;String&gt;) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Creates a DataFrame from a vector of vectors with automatic type inference.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">data</span>: <span className="parameter-type">Vec&lt;Vec&lt;String&gt;&gt;</span> - 2D vector containing the data
</div>
<div className="api-parameter">
<span className="parameter-name">column_names</span>: <span className="parameter-type">Vec&lt;String&gt;</span> - Names for the columns
</div>
</div>

**Example:**
```rust
let data = vec![
    vec!["Alice".to_string(), "30".to_string(), "Engineer".to_string()],
    vec!["Bob".to_string(), "25".to_string(), "Designer".to_string()],
];
let columns = vec!["name".to_string(), "age".to_string(), "role".to_string()];
let df = DataFrame::from_vec_of_vec(data, columns)?;
```
</div>

##### Core Methods

<div className="api-section">
<div className="api-method">fn row_count(&self) -&gt; usize</div>

Returns the number of rows in the DataFrame.

**Example:**
```rust
println!("DataFrame has {} rows", df.row_count());
```
</div>

<div className="api-section">
<div className="api-method">fn column_count(&self) -&gt; usize</div>

Returns the number of columns in the DataFrame.

**Example:**
```rust
println!("DataFrame has {} columns", df.column_count());
```
</div>

<div className="api-section">
<div className="api-method">fn column_names(&self) -&gt; Vec&lt;&String&gt;</div>

Returns a vector of column names.

**Example:**
```rust
let names = df.column_names();
for name in names {
    println!("Column: {}", name);
}
```
</div>

<div className="api-section">
<div className="api-method">fn get_column(&self, name: &str) -&gt; Option&lt;&Series&gt;</div>

Gets a reference to a column by name.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">&str</span> - Name of the column to retrieve
</div>
</div>

**Example:**
```rust
if let Some(age_column) = df.get_column("age") {
    println!("Age column has {} values", age_column.len());
}
```
</div>

##### Data Manipulation

<div className="api-section">
<div className="api-method">fn filter(&self, condition: &Condition) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Filters rows based on a condition.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">condition</span>: <span className="parameter-type">&Condition</span> - The filtering condition
</div>
</div>

**Example:**
```rust
use veloxx::{Condition, Value};

let condition = Condition::Gt("age".to_string(), Value::I32(25));
let filtered_df = df.filter(&condition)?;
```
</div>

<div className="api-section">
<div className="api-method">fn select_columns(&self, names: Vec&lt;String&gt;) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Selects specific columns from the DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">names</span>: <span className="parameter-type">Vec&lt;String&gt;</span> - Names of columns to select
</div>
</div>

**Example:**
```rust
let selected = df.select_columns(vec!["name".to_string(), "age".to_string()])?;
```
</div>

<div className="api-section">
<div className="api-method">fn drop_columns(&self, names: Vec&lt;String&gt;) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Removes specified columns from the DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">names</span>: <span className="parameter-type">Vec&lt;String&gt;</span> - Names of columns to drop
</div>
</div>

**Example:**
```rust
let without_id = df.drop_columns(vec!["id".to_string()])?;
```
</div>

<div className="api-section">
<div className="api-method">fn rename_column(&self, old_name: &str, new_name: &str) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Renames a column in the DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">old_name</span>: <span className="parameter-type">&str</span> - Current name of the column
</div>
<div className="api-parameter">
<span className="parameter-name">new_name</span>: <span className="parameter-type">&str</span> - New name for the column
</div>
</div>

**Example:**
```rust
let renamed = df.rename_column("age", "years")?;
```
</div>

<div className="api-section">
<div className="api-method">fn with_column(&self, name: &str, expr: &Expr) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Adds a new column or replaces an existing one using an expression.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">&str</span> - Name of the new column
</div>
<div className="api-parameter">
<span className="parameter-name">expr</span>: <span className="parameter-type">&Expr</span> - Expression to compute the column values
</div>
</div>

**Example:**
```rust
use veloxx::Expr;

let expr = Expr::Add(
    Box::new(Expr::Column("salary".to_string())),
    Box::new(Expr::Literal(Value::F64(1000.0)))
);
let with_bonus = df.with_column("salary_with_bonus", &expr)?;
```
</div>

##### Aggregation and Grouping

<div className="api-section">
<div className="api-method">fn group_by(&self, by_columns: Vec&lt;String&gt;) -&gt; Result&lt;GroupedDataFrame, VeloxxError&gt;</div>

Groups the DataFrame by specified columns.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">by_columns</span>: <span className="parameter-type">Vec&lt;String&gt;</span> - Columns to group by
</div>
</div>

**Example:**
```rust
let grouped = df.group_by(vec!["department".to_string()])?;
let result = grouped.agg(vec![("salary", "mean"), ("age", "count")])?;
```
</div>

<div className="api-section">
<div className="api-method">fn describe(&self) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Generates descriptive statistics for numeric columns.

**Example:**
```rust
let stats = df.describe()?;
println!("{}", stats);
```
</div>

##### Joining

<div className="api-section">
<div className="api-method">fn join(&self, other: &DataFrame, on_column: &str, join_type: JoinType) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Joins this DataFrame with another DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">other</span>: <span className="parameter-type">&DataFrame</span> - DataFrame to join with
</div>
<div className="api-parameter">
<span className="parameter-name">on_column</span>: <span className="parameter-type">&str</span> - Column name to join on
</div>
<div className="api-parameter">
<span className="parameter-name">join_type</span>: <span className="parameter-type">JoinType</span> - Type of join (Inner, Left, Right)
</div>
</div>

**Example:**
```rust
use veloxx::JoinType;

let joined = df1.join(&df2, "user_id", JoinType::Inner)?;
```
</div>

##### Sorting and Ordering

<div className="api-section">
<div className="api-method">fn sort(&self, by_columns: Vec&lt;String&gt;, ascending: bool) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Sorts the DataFrame by specified columns.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">by_columns</span>: <span className="parameter-type">Vec&lt;String&gt;</span> - Columns to sort by
</div>
<div className="api-parameter">
<span className="parameter-name">ascending</span>: <span className="parameter-type">bool</span> - Sort order (true for ascending, false for descending)
</div>
</div>

**Example:**
```rust
let sorted = df.sort(vec!["age".to_string(), "name".to_string()], true)?;
```
</div>

##### Data Cleaning

<div className="api-section">
<div className="api-method">fn drop_nulls(&self) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Removes rows containing any null values.

**Example:**
```rust
let clean_df = df.drop_nulls()?;
```
</div>

<div className="api-section">
<div className="api-method">fn fill_nulls(&self, value: Value) -&gt; Result&lt;DataFrame, VeloxxError&gt;</div>

Fills null values with a specified value.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">value</span>: <span className="parameter-type">Value</span> - Value to use for filling nulls
</div>
</div>

**Example:**
```rust
let filled = df.fill_nulls(Value::I32(0))?;
```
</div>

##### I/O Operations

<div className="api-section">
<div className="api-method">fn to_csv(&self, path: &str) -&gt; Result&lt;(), VeloxxError&gt;</div>

Writes the DataFrame to a CSV file.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">path</span>: <span className="parameter-type">&str</span> - Output file path
</div>
</div>

**Example:**
```rust
df.to_csv("output/results.csv")?;
```
</div>

### `veloxx::series`

The `Series` represents a single column of data with a specific type.

#### `Series`

```rust
pub enum Series {
    I32(String, Vec<Option<i32>>),
    F64(String, Vec<Option<f64>>),
    Bool(String, Vec<Option<bool>>),
    String(String, Vec<Option<String>>),
    DateTime(String, Vec<Option<i64>>),
}
```

##### Constructors

<div className="api-section">
<div className="api-method">Series::new_i32(name: &str, data: Vec&lt;Option&lt;i32&gt;&gt;) -&gt; Series</div>

Creates a new integer Series.

**Example:**
```rust
let ages = Series::new_i32("age", vec![Some(25), Some(30), None, Some(35)]);
```
</div>

<div className="api-section">
<div className="api-method">Series::new_f64(name: &str, data: Vec&lt;Option&lt;f64&gt;&gt;) -&gt; Series</div>

Creates a new floating-point Series.

**Example:**
```rust
let salaries = Series::new_f64("salary", vec![Some(50000.0), Some(75000.0), Some(60000.0)]);
```
</div>

<div className="api-section">
<div className="api-method">Series::new_string(name: &str, data: Vec&lt;Option&lt;String&gt;&gt;) -&gt; Series</div>

Creates a new string Series.

**Example:**
```rust
let names = Series::new_string("name", vec![
    Some("Alice".to_string()), 
    Some("Bob".to_string()),
    None
]);
```
</div>

<div className="api-section">
<div className="api-method">Series::new_bool(name: &str, data: Vec&lt;Option&lt;bool&gt;&gt;) -&gt; Series</div>

Creates a new boolean Series.

**Example:**
```rust
let active = Series::new_bool("is_active", vec![Some(true), Some(false), Some(true)]);
```
</div>

<div className="api-section">
<div className="api-method">Series::new_datetime(name: &str, data: Vec&lt;Option&lt;i64&gt;&gt;) -&gt; Series</div>

Creates a new datetime Series (timestamps as i64).

**Example:**
```rust
let timestamps = Series::new_datetime("created_at", vec![
    Some(1678886400), 
    Some(1678972800), 
    None
]);
```
</div>

##### Core Methods

<div className="api-section">
<div className="api-method">fn name(&self) -&gt; &str</div>

Returns the name of the Series.

**Example:**
```rust
println!("Series name: {}", series.name());
```
</div>

<div className="api-section">
<div className="api-method">fn len(&self) -&gt; usize</div>

Returns the length of the Series.

**Example:**
```rust
println!("Series has {} values", series.len());
```
</div>

<div className="api-section">
<div className="api-method">fn is_empty(&self) -&gt; bool</div>

Checks if the Series is empty.

**Example:**
```rust
if series.is_empty() {
    println!("Series is empty");
}
```
</div>

<div className="api-section">
<div className="api-method">fn data_type(&self) -&gt; DataType</div>

Returns the data type of the Series.

**Example:**
```rust
match series.data_type() {
    DataType::I32 => println!("Integer series"),
    DataType::F64 => println!("Float series"),
    DataType::String => println!("String series"),
    _ => println!("Other type"),
}
```
</div>

<div className="api-section">
<div className="api-method">fn get_value(&self, index: usize) -&gt; Option&lt;Value&gt;</div>

Gets the value at a specific index.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">index</span>: <span className="parameter-type">usize</span> - Index of the value to retrieve
</div>
</div>

**Example:**
```rust
if let Some(value) = series.get_value(0) {
    println!("First value: {:?}", value);
}
```
</div>

##### Statistical Methods

<div className="api-section">
<div className="api-method">fn sum(&self) -&gt; Result&lt;Option&lt;Value&gt;, VeloxxError&gt;</div>

Calculates the sum of numeric values.

**Example:**
```rust
if let Ok(Some(Value::F64(total))) = series.sum() {
    println!("Sum: {}", total);
}
```
</div>

<div className="api-section">
<div className="api-method">fn mean(&self) -&gt; Result&lt;Option&lt;Value&gt;, VeloxxError&gt;</div>

Calculates the mean of numeric values.

**Example:**
```rust
if let Ok(Some(Value::F64(avg))) = series.mean() {
    println!("Average: {}", avg);
}
```
</div>

<div className="api-section">
<div className="api-method">fn median(&self) -&gt; Result&lt;Option&lt;Value&gt;, VeloxxError&gt;</div>

Calculates the median of numeric values.

**Example:**
```rust
if let Ok(Some(Value::F64(med))) = series.median() {
    println!("Median: {}", med);
}
```
</div>

<div className="api-section">
<div className="api-method">fn min(&self) -&gt; Result&lt;Option&lt;Value&gt;, VeloxxError&gt;</div>

Finds the minimum value.

**Example:**
```rust
if let Ok(Some(min_val)) = series.min() {
    println!("Minimum: {:?}", min_val);
}
```
</div>

<div className="api-section">
<div className="api-method">fn max(&self) -&gt; Result&lt;Option&lt;Value&gt;, VeloxxError&gt;</div>

Finds the maximum value.

**Example:**
```rust
if let Ok(Some(max_val)) = series.max() {
    println!("Maximum: {:?}", max_val);
}
```
</div>

<div className="api-section">
<div className="api-method">fn std_dev(&self) -&gt; Result&lt;Option&lt;Value&gt;, VeloxxError&gt;</div>

Calculates the standard deviation.

**Example:**
```rust
if let Ok(Some(Value::F64(std))) = series.std_dev() {
    println!("Standard deviation: {}", std);
}
```
</div>

<div className="api-section">
<div className="api-method">fn count(&self) -&gt; usize</div>

Counts non-null values.

**Example:**
```rust
println!("Non-null values: {}", series.count());
```
</div>

<div className="api-section">
<div className="api-method">fn unique(&self) -&gt; Result&lt;Series, VeloxxError&gt;</div>

Returns a Series with unique values.

**Example:**
```rust
let unique_values = series.unique()?;
println!("Unique values: {}", unique_values.len());
```
</div>

### `veloxx::conditions`

Conditions are used for filtering DataFrames.

#### `Condition`

```rust
pub enum Condition {
    Eq(String, Value),
    Ne(String, Value),
    Lt(String, Value),
    Le(String, Value),
    Gt(String, Value),
    Ge(String, Value),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}
```

##### Comparison Conditions

<div className="api-section">
<div className="api-method">Condition::Eq(column: String, value: Value)</div>

Equal to condition.

**Example:**
```rust
let condition = Condition::Eq("status".to_string(), Value::String("active".to_string()));
```
</div>

<div className="api-section">
<div className="api-method">Condition::Ne(column: String, value: Value)</div>

Not equal to condition.

**Example:**
```rust
let condition = Condition::Ne("age".to_string(), Value::I32(0));
```
</div>

<div className="api-section">
<div className="api-method">Condition::Lt(column: String, value: Value)</div>

Less than condition.

**Example:**
```rust
let condition = Condition::Lt("price".to_string(), Value::F64(100.0));
```
</div>

<div className="api-section">
<div className="api-method">Condition::Gt(column: String, value: Value)</div>

Greater than condition.

**Example:**
```rust
let condition = Condition::Gt("score".to_string(), Value::I32(80));
```
</div>

##### Logical Conditions

<div className="api-section">
<div className="api-method">Condition::And(left: Box&lt;Condition&gt;, right: Box&lt;Condition&gt;)</div>

Logical AND condition.

**Example:**
```rust
let condition = Condition::And(
    Box::new(Condition::Gt("age".to_string(), Value::I32(18))),
    Box::new(Condition::Lt("age".to_string(), Value::I32(65)))
);
```
</div>

<div className="api-section">
<div className="api-method">Condition::Or(left: Box&lt;Condition&gt;, right: Box&lt;Condition&gt;)</div>

Logical OR condition.

**Example:**
```rust
let condition = Condition::Or(
    Box::new(Condition::Eq("role".to_string(), Value::String("admin".to_string()))),
    Box::new(Condition::Eq("role".to_string(), Value::String("manager".to_string())))
);
```
</div>

### `veloxx::expressions`

Expressions are used for creating computed columns.

#### `Expr`

```rust
pub enum Expr {
    Column(String),
    Literal(Value),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    // ... more expression types
}
```

##### Basic Expressions

<div className="api-section">
<div className="api-method">Expr::Column(name: String)</div>

References a column by name.

**Example:**
```rust
let expr = Expr::Column("salary".to_string());
```
</div>

<div className="api-section">
<div className="api-method">Expr::Literal(value: Value)</div>

A literal value.

**Example:**
```rust
let expr = Expr::Literal(Value::F64(1000.0));
```
</div>

##### Arithmetic Expressions

<div className="api-section">
<div className="api-method">Expr::Add(left: Box&lt;Expr&gt;, right: Box&lt;Expr&gt;)</div>

Addition expression.

**Example:**
```rust
let expr = Expr::Add(
    Box::new(Expr::Column("base_salary".to_string())),
    Box::new(Expr::Column("bonus".to_string()))
);
```
</div>

### `veloxx::types`

Core data types and values.

#### `DataType`

```rust
pub enum DataType {
    I32,
    F64,
    Bool,
    String,
    DateTime,
}
```

#### `Value`

```rust
pub enum Value {
    I32(i32),
    F64(f64),
    Bool(bool),
    String(String),
    DateTime(i64),
    Null,
}
```

### `veloxx::error`

Error handling for the library.

#### `VeloxxError`

```rust
pub enum VeloxxError {
    ColumnNotFound(String),
    TypeMismatch(String),
    InvalidOperation(String),
    IoError(String),
    ParseError(String),
}
```

## Usage Patterns

### Basic DataFrame Operations

```rust
use veloxx::prelude::*;
use std::collections::BTreeMap;

fn main() -> Result<(), VeloxxError> {
    // Create DataFrame
    let mut columns = BTreeMap::new();
    columns.insert("name".to_string(), Series::new_string("name", vec![
        Some("Alice".to_string()), Some("Bob".to_string())
    ]));
    columns.insert("age".to_string(), Series::new_i32("age", vec![Some(30), Some(25)]));
    
    let df = DataFrame::new(columns)?;
    
    // Filter
    let filtered = df.filter(&Condition::Gt("age".to_string(), Value::I32(25)))?;
    
    // Add computed column
    let expr = Expr::Add(
        Box::new(Expr::Column("age".to_string())),
        Box::new(Expr::Literal(Value::I32(10)))
    );
    let with_future_age = df.with_column("age_in_10_years", &expr)?;
    
    Ok(())
}
```

### Advanced Analytics

```rust
use veloxx::prelude::*;

fn analyze_sales_data() -> Result<(), VeloxxError> {
    // Load data
    let df = DataFrame::from_csv("sales_data.csv")?;
    
    // Complex filtering
    let condition = Condition::And(
        Box::new(Condition::Gt("amount".to_string(), Value::F64(1000.0))),
        Box::new(Condition::Eq("status".to_string(), Value::String("completed".to_string())))
    );
    let high_value_sales = df.filter(&condition)?;
    
    // Group by and aggregate
    let summary = high_value_sales
        .group_by(vec!["region".to_string(), "product_category".to_string()])?
        .agg(vec![
            ("amount", "sum"),
            ("amount", "mean"),
            ("customer_id", "count")
        ])?;
    
    // Export results
    summary.to_csv("sales_summary.csv")?;
    
    Ok(())
}
```

## Performance Tips

1. **Use appropriate data types**: Choose the most specific type for your data
2. **Leverage lazy evaluation**: Chain operations for better optimization
3. **Minimize data copying**: Use references where possible
4. **Process in chunks**: For very large datasets, process in smaller chunks
5. **Use parallel operations**: Enable parallel processing for CPU-intensive tasks

## Error Handling

All operations that can fail return `Result<T, VeloxxError>`. Always handle errors appropriately:

```rust
match df.filter(&condition) {
    Ok(filtered_df) => {
        // Process the filtered DataFrame
        println!("Filtered to {} rows", filtered_df.row_count());
    }
    Err(VeloxxError::ColumnNotFound(col)) => {
        eprintln!("Column '{}' not found", col);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```