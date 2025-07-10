# <img width="180" height="180" alt="logo2_png" src="https://github.com/user-attachments/assets/7b02676c-c897-4a20-adaf-c8077504ff58" />

# Veloxx: Lightweight Rust-Powered Data Processing & Analytics Library

[![crates.io](https://img.shields.io/crates/v/veloxx.svg)](https://crates.io/crates/veloxx)

> **New in 0.2.1:** Major performance improvements across all core operations. See CHANGELOG for details.

Veloxx is a new Rust library designed for highly performant and **extremely lightweight** in-memory data processing and analytics. It prioritizes minimal dependencies, optimal memory footprint, and compile-time guarantees, making it an ideal choice for resource-constrained environments, high-performance computing, and applications where every byte and cycle counts.

## Core Principles & Design Goals

- **Extreme Lightweighting:** Strives for zero or very few, carefully selected external crates. Focuses on minimal overhead and small binary size.
- **Performance First:** Leverages Rust's zero-cost abstractions, with potential for SIMD and parallelism. Data structures are optimized for cache efficiency.
- **Safety & Reliability:** Fully utilizes Rust's ownership and borrowing system to ensure memory safety and prevent common data manipulation errors. Unsafe code is minimized and thoroughly audited.
- **Ergonomics & Idiomatic Rust API:** Designed for a clean, discoverable, and user-friendly API that feels natural to Rust developers, supporting method chaining and strong static typing.
- **Composability & Extensibility:** Features a modular design, allowing components to be independent and easily combinable, and is built to be easily extendable.

## Key Features

### Core Data Structures

- **DataFrame:** A columnar data store supporting heterogeneous data types per column (i32, f64, bool, String, DateTime). Efficient storage and handling of missing values.
- **Series (or Column):** A single-typed, named column of data within a DataFrame, providing type-specific operations.

### Data Ingestion & Loading

- **From `Vec<Vec<T>>` / Iterator:** Basic in-memory construction from Rust native collections.
- **CSV Support:** Minimalistic, highly efficient CSV parser for loading data.
- **JSON Support:** Efficient parsing for common JSON structures.
- **Custom Data Sources:** Traits/interfaces for users to implement their own data loading mechanisms.

### Data Cleaning & Preparation

- `drop_nulls()`: Remove rows with any null values.
- `fill_nulls(value)`: Fill nulls with a specified value (type-aware, including DateTime).
- `interpolate_nulls()`: Basic linear interpolation for numeric and DateTime series.
- **Type Casting:** Efficient conversion between compatible data types for Series (e.g., i32 to f64).
- `rename_column(old_name, new_name)`: Rename columns.

### Data Transformation & Manipulation

- **Selection:** `select_columns(names)`, `drop_columns(names)`.
- **Filtering:** Predicate-based row selection using logical (`AND`, `OR`, `NOT`) and comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`).
- **Projection:** `with_column(new_name, expression)`, `apply()` for user-defined functions.
- **Sorting:** Sort DataFrame by one or more columns (ascending/descending).
- **Joining:** Basic inner, left, and right join operations on common keys.
- **Concatenation/Append:** Combine DataFrames vertically.

### Aggregation & Reduction

- **Simple Aggregations:** `sum()`, `mean()`, `median()`, `min()`, `max()`, `count()`, `std_dev()`.
- **Group By:** Perform aggregations on groups defined by one or more columns.
- **Unique Values:** `unique()` for a Series or DataFrame columns.

### Basic Analytics & Statistics

- `describe()`: Provides summary statistics for numeric columns (count, mean, std, min, max, quartiles).
- `correlation()`: Calculate Pearson correlation between two numeric Series.
- `covariance()`: Calculate covariance.

### Output & Export

- **To `Vec<Vec<T>>`:** Export DataFrame content back to standard Rust collections.
- **To CSV:** Efficiently write DataFrame to a CSV file.
- **Display/Pretty Print:** User-friendly console output for DataFrame and Series.

## Installation

### Rust

Veloxx is available on [crates.io](https://crates.io/crates/veloxx).

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
veloxx = "0.2.4" # Or the latest version
```

To build your Rust project with Veloxx, run:

```bash
cargo build
```

To run tests:

```bash
cargo test
```

## Usage Examples

### Rust Usage

Here's a quick example demonstrating how to create a DataFrame, filter it, and perform a group-by aggregation:

```rust
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::types::{Value, DataType};
use veloxx::conditions::Condition;
use veloxx::expressions::Expr;
use std::collections::BTreeMap;

fn main() -> Result<(), String> {
    // 1. Create a DataFrame
    let mut columns = BTreeMap::new();
    columns.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string()), Some("David".to_string())]));
    columns.insert("age".to_string(), Series::new_i32("age", vec![Some(25), Some(30), Some(22), Some(35)]));
    columns.insert("city".to_string(), Series::new_string("city", vec![Some("New York".to_string()), Some("London".to_string()), Some("New York".to_string()), Some("Paris".to_string())]));
    columns.insert("last_login".to_string(), Series::new_datetime("last_login", vec![Some(1678886400), Some(1678972800), Some(1679059200), Some(1679145600)]));

    let df = DataFrame::new(columns)?;
    println!("Original DataFrame:
{}", df);

    // 2. Filter data: age > 25 AND city == "New York"
    let condition = Condition::And(
        Box::new(Condition::Gt("age".to_string(), Value::I32(25))),
        Box::new(Condition::Eq("city".to_string(), Value::String("New York".to_string()))),
    );
    let filtered_df = df.filter(&condition)?;
    println!("
Filtered DataFrame (age > 25 AND city == \"New York\"):
{}", filtered_df);

    // 3. Add a new column: age_in_10_years = age + 10
    let expr_add_10 = Expr::Add(Box::new(Expr::Column("age".to_string())), Box::new(Expr::Literal(Value::I32(10))));
    let df_with_new_col = df.with_column("age_in_10_years", &expr_add_10)?;
    println!("
DataFrame with new column (age_in_10_years):
{}", df_with_new_col);

    // 4. Group by city and calculate average age and count of users
    let grouped_df = df.group_by(vec!["city".to_string()])?;
    let aggregated_df = grouped_df.agg(vec![("age", "mean"), ("name", "count")])?;
    println!("
Aggregated DataFrame (average age and user count by city):
{}", aggregated_df);

    // 5. Demonstrate DateTime filtering (users logged in after a specific date)
    let specific_date_timestamp = 1679000000; // Example timestamp
    let condition_dt = Condition::Gt("last_login".to_string(), Value::DateTime(specific_date_timestamp));
    let filtered_df_dt = df.filter(&condition_dt)?;
    println!("
Filtered DataFrame (users logged in after {}):
{}", specific_date_timestamp, filtered_df_dt);

    Ok(())
}
```

### Python Usage

```python
import veloxx

# 1. Create a DataFrame
df = veloxx.PyDataFrame({
    "name": veloxx.PySeries("name", ["Alice", "Bob", "Charlie", "David"]),
    "age": veloxx.PySeries("age", [25, 30, 22, 35]),
    "city": veloxx.PySeries("city", ["New York", "London", "New York", "Paris"]),
})
print("Original DataFrame:")
print(df)

# 2. Filter data: age > 25
filtered_df = df.filter([i for i, age in enumerate(df.get_column("age").to_vec_f64()) if age > 25])
print("\nFiltered DataFrame (age > 25):")
print(filtered_df)

# 3. Select columns
selected_df = df.select_columns(["name", "city"])
print("\nSelected Columns (name, city):")
print(selected_df)

# 4. Rename a column
renamed_df = df.rename_column("age", "years")
print("\nRenamed Column (age to years):")
print(renamed_df)

# 5. Series operations
age_series = df.get_column("age")
print(f"\nAge Series Sum: {age_series.sum()}")
print(f"Age Series Mean: {age_series.mean()}")
print(f"Age Series Max: {age_series.max()}")
print(f"Age Series Unique: {age_series.unique().to_vec_f64()}")
```

### WebAssembly Usage (Node.js)

```javascript
const veloxx = require('veloxx');

async function runWasmExample() {
    // 1. Create a DataFrame
    const df = new veloxx.WasmDataFrame({
        name: ["Alice", "Bob", "Charlie", "David"],
        age: [25, 30, 22, 35],
        city: ["New York", "London", "New York", "Paris"],
    });
    console.log("Original DataFrame:");
    console.log(df);

    // 2. Filter data: age > 25
    const ageSeries = df.getColumn("age");
    const filteredIndices = [];
    for (let i = 0; i < ageSeries.len; i++) {
        if (ageSeries.getValue(i) > 25) {
            filteredIndices.push(i);
        }
    }
    const filteredDf = df.filter(new Uint32Array(filteredIndices));
    console.log("\nFiltered DataFrame (age > 25):");
    console.log(filteredDf);

    // 3. Series operations
    console.log(`\nAge Series Sum: ${ageSeries.sum()}`);
    console.log(`Age Series Mean: ${ageSeries.mean()}`);
    console.log(`Age Series Unique: ${ageSeries.unique().toVecF64()}`);
}

runWasmExample();
```


