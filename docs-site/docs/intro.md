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

### ✅ **Implemented**
- **DataFrame & Series**: Core data structures with type safety
- **CSV I/O**: Fast CSV reading and writing with automatic type inference
- **Filtering**: Complex conditions with logical operators (And, Or, Not)
- **Aggregations**: Group by operations with multiple aggregation functions
- **Column Operations**: Select, drop, rename, and computed columns
- **Sorting**: Single and multi-column sorting
- **Joins**: Inner and left joins on single or multiple columns
- **Statistics**: Mean, sum, min, max, standard deviation
- **Data Cleaning**: Handle null values with drop and fill operations

### 🚧 **In Development**
- **Advanced I/O**: JSON, Parquet, and database connectivity
- **Data Quality**: Validation, profiling, and duplicate detection  
- **Window Functions**: Moving averages, ranking, and time series analysis
- **Python Bindings**: Full Python API with pandas compatibility
- **JavaScript/WASM**: Browser and Node.js support

## Core Data Structures

### DataFrame
A columnar data table with heterogeneous data types:

```rust
use std::collections::BTreeMap;
use veloxx::{DataFrame, Series};

let mut columns = BTreeMap::new();
columns.insert("name".to_string(), Series::new_string("name", vec![
    Some("Alice".to_string()), Some("Bob".to_string())
]));
columns.insert("age".to_string(), Series::new_i32("age", vec![Some(30), Some(25)]));

let df = DataFrame::new(columns)?;
```

### Series
Single-typed columns with rich operations:

```rust
use veloxx::Series;

let ages = Series::new_i32("age", vec![Some(25), Some(30), None, Some(35)]);
let mean_age = ages.mean()?;
let null_count = ages.null_count();
```

## Data Operations

### Filtering with Conditions

```rust
use veloxx::{Condition, Value};

// Simple condition
let condition = Condition::Gt("age".to_string(), Value::I32(25));
let filtered = df.filter(&condition)?;

// Complex condition
let complex = Condition::And(
    Box::new(Condition::Gt("age".to_string(), Value::I32(25))),
    Box::new(Condition::Lt("salary".to_string(), Value::F64(100000.0)))
);
let result = df.filter(&complex)?;
```

### Aggregation and Grouping

```rust
// Group by department and calculate statistics
let summary = df
    .group_by(vec!["department".to_string()])?
    .agg(vec![
        ("salary", "mean"),
        ("salary", "count"),
        ("age", "max")
    ])?;
```

### Computed Columns

```rust
use veloxx::expressions::Expr;

// Add a bonus column (10% of salary)
let bonus_expr = Expr::Multiply(
    Box::new(Expr::Column("salary".to_string())),
    Box::new(Expr::Literal(Value::F64(0.1)))
);
let with_bonus = df.with_column("bonus", &bonus_expr)?;
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

## Feature Roadmap

### Phase 1: Core Foundation ✅
- [x] DataFrame and Series data structures
- [x] Basic I/O (CSV)
- [x] Filtering and basic operations
- [x] Aggregations and grouping
- [x] Comprehensive documentation

### Phase 2: Advanced Features 🚧
- [ ] Advanced I/O (JSON, Parquet, databases)
- [ ] Data quality and validation tools
- [ ] Window functions and time series analysis
- [ ] Performance optimizations

### Phase 3: Multi-Language Support 📋
- [ ] Python bindings with pandas compatibility
- [ ] JavaScript/WebAssembly support
- [ ] Language-specific documentation and examples

### Phase 4: Ecosystem 🔮
- [ ] Visualization integrations
- [ ] Machine learning pipeline support
- [ ] Cloud and distributed computing features

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