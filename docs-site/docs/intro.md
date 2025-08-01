# Welcome to Veloxx

Veloxx is a **lightning-fast**, **lightweight** Rust library for in-memory data processing and analytics. It provides a modern, ergonomic API that competes with industry leaders like pandas and Polars while maintaining excellent performance and memory efficiency.

## Why Veloxx?

### 🚀 **High Performance**
- **Optimized columnar operations** for fast data processing
- **Efficient memory usage** with minimal allocations
- **Zero-cost abstractions** leveraging Rust's performance guarantees

### 🌐 **Multi-Language Support**
- **Native Rust** library with full type safety
- **Python bindings** with familiar pandas-like API (coming soon)
- **JavaScript/WebAssembly** support for browser and Node.js (coming soon)

### 🪶 **Lightweight & Efficient**
- **Minimal dependencies** in core library
- **Small binary size** perfect for various deployment scenarios
- **Resource-efficient** for both small and large datasets

### 🛡️ **Memory Safe & Reliable**
- **Compile-time guarantees** prevent common data manipulation errors
- **No garbage collection overhead**
- **Safe Rust** implementation with careful memory management

## Quick Start

Get up and running with Veloxx in minutes:

```toml title="Cargo.toml"
[dependencies]
veloxx = "0.2.4"
```

```rust
use veloxx::dataframe::DataFrame;
use veloxx::conditions::Condition;
use veloxx::types::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a DataFrame from CSV
    let df = DataFrame::from_csv("employees.csv")?;
    
    // Filter employees with high salaries
    let high_earners = df.filter(&Condition::Gt(
        "salary".to_string(), 
        Value::F64(70000.0)
    ))?;
    
    // Group by department and calculate averages
    let dept_analysis = df
        .group_by(vec!["department".to_string()])?
        .agg(vec![("salary", "mean"), ("age", "count")])?;
    
    println!("Department Analysis:\n{}", dept_analysis);
    Ok(())
}
```

## Current Features

Veloxx offers a rich set of features for high-performance data processing and analytics:

- **DataFrame & Series**: Core data structures with type safety and columnar storage.
- **CSV & JSON I/O**: Fast reading and writing of CSV and JSON files with automatic type inference.
- **Filtering**: Powerful filtering capabilities using complex conditions with logical operators (AND, OR, NOT).
- **Aggregations & Grouping**: Flexible group-by operations with various aggregation functions (sum, mean, count, min, max, median, std_dev).
- **Column Operations**: Easy selection, dropping, renaming, and creation of computed columns using expressions.
- **Sorting**: Efficient single and multi-column sorting in ascending or descending order.
- **Joins**: Support for inner, left, and right joins to combine DataFrames.
- **Statistics**: Comprehensive descriptive statistics for numeric data.
- **Data Cleaning**: Robust handling of null values with drop, fill, and interpolation operations.
- **Advanced I/O**: Parquet file support and database connectivity.
- **Data Quality**: Validation, profiling, anomaly detection, and duplicate detection.
- **Window Functions**: SQL-style window functions for advanced analytics (e.g., moving averages, ranking, lag/lead).
- **Python Bindings**: Seamless Python API with a familiar pandas-like interface.
- **JavaScript/WASM**: High-performance WebAssembly bindings for browser and Node.js environments.
- **Machine Learning**: Basic machine learning models (e.g., linear regression, K-means, logistic regression) and preprocessing utilities.
- **Visualization**: Charting and plotting capabilities for data exploration (e.g., line, scatter, bar, histogram).

## Core Data Structures

Veloxx is built around two fundamental data structures: `DataFrame` and `Series`. These are designed for high performance and memory efficiency through columnar storage.

### DataFrame

A `DataFrame` is a tabular data structure with named columns, where each column can hold data of a different type. It's analogous to a table in a relational database or a spreadsheet. DataFrames in Veloxx are optimized for analytical workloads, allowing for fast filtering, grouping, and transformations.

```rust
use std::collections::BTreeMap;
use veloxx::{dataframe::DataFrame, series::Series};

// Example: Creating a DataFrame from individual Series
let mut columns = BTreeMap::new();
columns.insert("name".to_string(), Series::new_string("name", vec![
    Some("Alice".to_string()), 
    Some("Bob".to_string())
]));
columns.insert("age".to_string(), Series::new_i32("age", vec![Some(30), Some(25)]));

let df = DataFrame::new(columns).unwrap();
// df.row_count() -> 2
// df.column_count() -> 2
```

### Series

A `Series` represents a single column of data within a `DataFrame`. All data within a `Series` is of a single, homogeneous type. This columnar design is crucial for performance, enabling efficient operations and better cache utilization.

```rust
use veloxx::series::Series;
use veloxx::types::Value;

// Example: Creating a Series
let ages = Series::new_i32("age", vec![Some(25), Some(30), None, Some(35)]);

// Basic Series operations
let mean_age = ages.mean().unwrap(); // Some(Value::F64(30.0))
let null_count = ages.null_count(); // 1
```

### Value and DataType

At the lowest level, individual data points are represented by the `Value` enum, which can hold different primitive types (integers, floats, booleans, strings, and DateTime). The `DataType` enum explicitly defines the type of data a `Series` holds, ensuring type safety throughout the library.

```rust
use veloxx::types::{DataType, Value};

let int_value = Value::I32(42);
let float_type = DataType::F64;

assert_eq!(int_value.data_type(), DataType::I32);
assert_eq!(float_type, DataType::F64);
```

## Data Operations

Veloxx provides a rich set of operations for manipulating and analyzing your data. These operations are designed to be intuitive and chainable, allowing you to build complex data pipelines.

### Filtering with Conditions

Filter rows based on specific criteria using the `Condition` enum. Conditions can be simple comparisons or complex logical combinations.

```rust
use veloxx::{conditions::Condition, types::Value};

// Filter where age is greater than 25
let condition = Condition::Gt("age".to_string(), Value::I32(25));
let filtered_df = df.filter(&condition).unwrap();

// Filter where age > 25 AND salary < 100000.0
let complex_condition = Condition::And(
    Box::new(Condition::Gt("age".to_string(), Value::I32(25))),
    Box::new(Condition::Lt("salary".to_string(), Value::F64(100000.0)))
);
let result_df = df.filter(&complex_condition).unwrap();
```

### Aggregation and Grouping

Perform powerful aggregations by grouping your data based on one or more columns. You can calculate sums, means, counts, and more.

```rust
// Group by department and calculate mean salary, total employees, and max age
let grouped_df = df.group_by(vec!["department".to_string()]).unwrap();
let summary_df = grouped_df.agg(vec![
    ("salary", "mean"),
    ("salary", "count"),
    ("age", "max")
]).unwrap();
```

### Computed Columns with Expressions

Create new columns or transform existing ones using the `Expr` system. Expressions allow you to define calculations based on other columns or literal values. They support arithmetic, comparison, and logical operations.

```rust
use veloxx::expressions::Expr;
use veloxx::types::Value;

// Add a new column 'total_compensation' as salary + bonus
let total_comp_expr = Expr::Add(
    Box::new(Expr::Column("salary".to_string())),
    Box::new(Expr::Column("bonus".to_string()))
);
let df_with_total_comp = df.with_column("total_compensation", &total_comp_expr).unwrap();

// Create a boolean column 'is_senior' based on age and experience
let is_senior_expr = Expr::And(
    Box::new(Expr::GreaterThanOrEqual(Box::new(Expr::Column("age".to_string())), Box::new(Expr::Literal(Value::I32(30))))),
    Box::new(Expr::GreaterThanOrEqual(Box::new(Expr::Column("experience".to_string())), Box::new(Expr::Literal(Value::I32(5)))))
);
let df_with_seniority = df.with_column("is_senior", &is_senior_expr).unwrap();
```

## What's Next?

<div className="row">
  <div className="col col--6">
    <div className="card">
      <div className="card__header">
        <h3>📚 Learn the Basics</h3>
      </div>
      <div className="card__body">
        <p>Start with our comprehensive tutorial covering DataFrames, Series, and core operations.</p>
      </div>
      <div className="card__footer">
        <a href="/docs/getting-started/installation" className="button button--primary">Get Started</a>
      </div>
    </div>
  </div>
  <div className="col col--6">
    <div className="card">
      <div className="card__header">
        <h3>🔍 Explore the API</h3>
      </div>
      <div className="card__body">
        <p>Dive deep into the complete API reference with examples and best practices.</p>
      </div>
      <div className="card__footer">
        <a href="/docs/api/rust" className="button button--secondary">API Docs</a>
      </div>
    </div>
  </div>
</div>

<div className="row" style={{marginTop: '1rem'}}>
  <div className="col col--6">
    <div className="card">
      <div className="card__header">
        <h3>🚀 Quick Start</h3>
      </div>
      <div className="card__body">
        <p>Get up and running with Veloxx in just 5 minutes with our hands-on tutorial.</p>
      </div>
      <div className="card__footer">
        <a href="/docs/getting-started/quick-start" className="button button--outline">Quick Start</a>
      </div>
    </div>
  </div>
  <div className="col col--6">
    <div className="card">
      <div className="card__header">
        <h3>💡 Examples</h3>
      </div>
      <div className="card__body">
        <p>Learn from practical examples covering real-world data processing scenarios.</p>
      </div>
      <div className="card__footer">
        <a href="https://github.com/Conqxeror/veloxx/tree/main/examples" className="button button--outline">See Examples</a>
      </div>
    </div>
  </div>
</div>

## Community & Support

- 🐛 **Found a bug?** [Report it on GitHub](https://github.com/Conqxeror/veloxx/issues)
- 💬 **Have questions?** [Join our discussions](https://github.com/Conqxeror/veloxx/discussions)
- 🤝 **Want to contribute?** [Read our contributing guide](https://github.com/Conqxeror/veloxx/blob/main/CONTRIBUTING.md)
- 📦 **Check out the code** [on GitHub](https://github.com/Conqxeror/veloxx)

## Performance Philosophy

Veloxx is designed with performance in mind:

- **Columnar Storage**: Efficient memory layout for analytical workloads
- **Lazy Evaluation**: Optimize query execution by combining operations
- **Zero-Copy Operations**: Minimize memory allocations where possible
- **Parallel Processing**: Leverage multiple CPU cores for computations
- **Memory Efficiency**: Careful memory management to reduce overhead

:::tip Getting Started
Ready to try Veloxx? Start with our [installation guide](/docs/getting-started/installation) and then follow the [quick start tutorial](/docs/getting-started/quick-start) to build your first data processing pipeline.
:::

:::info Development Status
Veloxx is actively developed with a focus on stability and performance. The core features are production-ready, with advanced features being added based on community feedback and real-world usage patterns.
:::