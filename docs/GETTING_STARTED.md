# Getting Started with Velox

Welcome to Velox! This guide will help you get up and running with the Velox data processing library quickly.

## Table of Contents

1. [Installation](#installation)
2. [Your First DataFrame](#your-first-dataframe)
3. [Basic Operations](#basic-operations)
4. [Working with Features](#working-with-features)
5. [Common Patterns](#common-patterns)
6. [Next Steps](#next-steps)

## Installation

Add Velox to your `Cargo.toml`:

```toml
[dependencies]
veloxx = "0.2.4"
```

For specific features, use:

```toml
[dependencies]
veloxx = { version = "0.2.4", features = ["advanced_io", "data_quality", "window_functions"] }
```

## Your First DataFrame

Let's create your first DataFrame and perform some basic operations:

```rust
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a DataFrame from scratch
    let mut columns = BTreeMap::new();
    
    columns.insert(
        "name".to_string(),
        Series::new_string("name", vec![
            Some("Alice".to_string()),
            Some("Bob".to_string()),
            Some("Charlie".to_string()),
        ]),
    );
    
    columns.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(30), Some(25), Some(35)]),
    );
    
    columns.insert(
        "salary".to_string(),
        Series::new_f64("salary", vec![Some(75000.0), Some(65000.0), Some(85000.0)]),
    );

    let df = DataFrame::new(columns)?;
    println!("Our DataFrame:\n{}", df);

    Ok(())
}
```

This will output:
```
age            name           salary         
--------------- --------------- --------------- 
30             Alice          75000.00       
25             Bob            65000.00       
35             Charlie        85000.00       
```

## Basic Operations

### Filtering Data

```rust
use veloxx::conditions::Condition;
use veloxx::types::Value;

// Filter employees with salary > 70000
let condition = Condition::Gt("salary".to_string(), Value::F64(70000.0));
let high_earners = df.filter(&condition)?;
println!("High earners:\n{}", high_earners);

// Complex conditions
let complex_condition = Condition::And(
    Box::new(Condition::Gt("age".to_string(), Value::I32(25))),
    Box::new(Condition::Lt("salary".to_string(), Value::F64(80000.0)))
);
let filtered = df.filter(&complex_condition)?;
```

### Selecting and Manipulating Columns

```rust
// Select specific columns
let names_and_ages = df.select_columns(vec!["name".to_string(), "age".to_string()])?;

// Drop columns
let without_salary = df.drop_columns(vec!["salary".to_string()])?;

// Rename columns
let renamed = df.rename_column("salary", "annual_salary")?;

// Add new columns with expressions
use veloxx::expressions::Expr;
let age_in_10_years = Expr::Add(
    Box::new(Expr::Column("age".to_string())),
    Box::new(Expr::Literal(Value::I32(10)))
);
let with_future_age = df.with_column("age_in_10_years", &age_in_10_years)?;
```

### Aggregation and Grouping

```rust
// Basic aggregations on Series
let age_series = df.get_column("age").unwrap();
println!("Average age: {:.1}", age_series.mean()?);
println!("Max age: {}", age_series.max()?);
println!("Min age: {}", age_series.min()?);

// Group by operations (when you have categorical data)
// This example assumes you have a "department" column
// let grouped = df.group_by(vec!["department".to_string()])?;
// let avg_salary_by_dept = grouped.agg(vec![("salary", "mean")])?;
```

### Sorting

```rust
// Sort by age (ascending)
let sorted_by_age = df.sort(vec!["age".to_string()], true)?;

// Sort by salary (descending)
let sorted_by_salary = df.sort(vec!["salary".to_string()], false)?;

// Multi-column sort
let multi_sorted = df.sort(vec!["age".to_string(), "salary".to_string()], true)?;
```

## Working with Features

Velox uses feature flags to keep the core library lightweight. Here's how to use different features:

### Advanced I/O Operations

Enable with: `features = ["advanced_io"]`

```rust
#[cfg(feature = "advanced_io")]
use veloxx::advanced_io::{ParquetWriter, ParquetReader, DatabaseConnector};

#[cfg(feature = "advanced_io")]
async fn advanced_io_example() -> Result<(), Box<dyn std::error::Error>> {
    // Write to Parquet
    let writer = ParquetWriter::new();
    writer.write_dataframe(&df, "data.parquet").await?;
    
    // Read from Parquet
    let reader = ParquetReader::new();
    let loaded_df = reader.read_dataframe("data.parquet").await?;
    
    // Database operations
    let connector = DatabaseConnector::new("sqlite://database.db");
    connector.create_table_from_dataframe(&df, "employees").await?;
    connector.insert_dataframe(&df, "employees").await?;
    
    Ok(())
}
```

### Data Quality & Validation

Enable with: `features = ["data_quality"]`

```rust
#[cfg(feature = "data_quality")]
use veloxx::data_quality::{DataProfiler, AnomalyDetector, SchemaValidator};

#[cfg(feature = "data_quality")]
fn data_quality_example(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    // Profile your data
    let profiler = DataProfiler::new();
    let profile = profiler.profile_dataframe(df)?;
    println!("Data quality score: {:.2}/100", profiler.calculate_quality_score(df)?);
    
    // Detect anomalies
    let detector = AnomalyDetector::new();
    let outliers = detector.detect_outliers(df, "salary")?;
    println!("Found {} outliers in salary data", outliers.len());
    
    Ok(())
}
```

### Window Functions & Analytics

Enable with: `features = ["window_functions"]`

```rust
#[cfg(feature = "window_functions")]
use veloxx::window_functions::{WindowSpec, WindowFunction, RankingFunction};

#[cfg(feature = "window_functions")]
fn window_functions_example(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    // Create window specification
    let window_spec = WindowSpec::new()
        .order_by(vec!["salary".to_string()]);
    
    // Add ranking
    let with_rank = WindowFunction::apply_ranking(df, &RankingFunction::Rank, &window_spec)?;
    println!("DataFrame with salary rankings:\n{}", with_rank);
    
    Ok(())
}
```

## Common Patterns

### Loading Data from CSV

```rust
// From Vec<Vec<String>> (common when parsing CSV)
let data = vec![
    vec!["Alice".to_string(), "30".to_string(), "75000.0".to_string()],
    vec!["Bob".to_string(), "25".to_string(), "65000.0".to_string()],
];
let column_names = vec!["name".to_string(), "age".to_string(), "salary".to_string()];
let df = DataFrame::from_vec_of_vec(data, column_names)?;
```

### Handling Missing Data

```rust
// Create Series with missing values
let series_with_nulls = Series::new_i32("scores", vec![Some(95), None, Some(87), Some(92)]);

// Check for nulls
println!("Null count: {}", series_with_nulls.null_count());

// DataFrames automatically handle nulls in operations
```

### Chaining Operations

```rust
// Chain multiple operations together
let result = df
    .filter(&Condition::Gt("age".to_string(), Value::I32(25)))?
    .select_columns(vec!["name".to_string(), "salary".to_string()])?
    .sort(vec!["salary".to_string()], false)?;

println!("Filtered, selected, and sorted:\n{}", result);
```

### Error Handling

```rust
use veloxx::error::VeloxxError;

match df.get_column("nonexistent") {
    Some(series) => println!("Found series: {:?}", series.data_type()),
    None => println!("Column not found"),
}

// Or handle specific errors
match df.filter(&some_condition) {
    Ok(filtered_df) => println!("Filtered successfully"),
    Err(VeloxxError::ColumnNotFound(name)) => println!("Column '{}' not found", name),
    Err(VeloxxError::TypeMismatch(msg)) => println!("Type error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Next Steps

### Run the Examples

Explore the comprehensive examples to see Velox in action:

```bash
# Basic operations
cargo run --example basic_dataframe_operations

# Advanced I/O
cargo run --example advanced_io --features advanced_io

# Data quality
cargo run --example data_quality_validation --features data_quality

# Window functions
cargo run --example window_functions_analytics --features window_functions
```

### Read the API Documentation

Generate and view the full API documentation:

```bash
cargo doc --open --features advanced_io,data_quality,window_functions
```

### Check Out the API Guide

For comprehensive API coverage, see [`docs/API_GUIDE.md`](./API_GUIDE.md).

### Performance Considerations

- Use feature flags to include only what you need
- For large datasets, consider streaming operations with `advanced_io`
- Enable parallel processing for compute-intensive operations
- Profile your data quality before heavy processing

### Getting Help

- Check the [examples directory](../examples/) for working code
- Read the API documentation for detailed method information
- Look at the test files for usage patterns
- Open an issue on GitHub for bugs or feature requests

Happy data processing with Velox! 🚀