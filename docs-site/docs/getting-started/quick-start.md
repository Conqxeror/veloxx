# Quick Start

Get up and running with Veloxx in just 5 minutes! This guide will walk you through creating your first DataFrame and performing basic operations.

## Prerequisites

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

## Create a New Project

```bash
cargo new velox-quickstart
cd velox-quickstart
```

## Add Veloxx to Your Project

Add Veloxx to your `Cargo.toml`:

```toml title="Cargo.toml"
[dependencies]
veloxx = "0.2.4"
```

For additional features:

```toml title="Cargo.toml"
[dependencies]
veloxx = { version = "0.2.4", features = ["advanced_io", "data_quality", "window_functions"] }
```

## Your First DataFrame

Replace the contents of `src/main.rs` with:

```rust title="src/main.rs"
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
            Some("Diana".to_string()),
        ]),
    );
    
    columns.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(30), Some(25), Some(35), Some(28)]),
    );
    
    columns.insert(
        "salary".to_string(),
        Series::new_f64("salary", vec![
            Some(75000.0), 
            Some(65000.0), 
            Some(85000.0), 
            Some(72000.0)
        ]),
    );

    columns.insert(
        "department".to_string(),
        Series::new_string("department", vec![
            Some("Engineering".to_string()),
            Some("Marketing".to_string()),
            Some("Engineering".to_string()),
            Some("Sales".to_string()),
        ]),
    );

    let df = DataFrame::new(columns)?;
    println!("📊 Our Employee DataFrame:");
    println!("{}", df);

    Ok(())
}
```

Run your program:

```bash
cargo run
```

You should see output like:

```
📊 Our Employee DataFrame:
age            department     name           salary         
--------------- --------------- --------------- --------------- 
30             Engineering    Alice          75000.00       
25             Marketing      Bob            65000.00       
35             Engineering    Charlie        85000.00       
28             Sales          Diana          72000.00       
```

## Basic Operations

Now let's explore some basic operations. Update your `main.rs`:

```rust title="src/main.rs"
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::conditions::Condition;
use veloxx::types::Value;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the DataFrame (same as before)
    let mut columns = BTreeMap::new();
    columns.insert(
        "name".to_string(),
        Series::new_string("name", vec![
            Some("Alice".to_string()),
            Some("Bob".to_string()),
            Some("Charlie".to_string()),
            Some("Diana".to_string()),
        ]),
    );
    columns.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(30), Some(25), Some(35), Some(28)]),
    );
    columns.insert(
        "salary".to_string(),
        Series::new_f64("salary", vec![
            Some(75000.0), 
            Some(65000.0), 
            Some(85000.0), 
            Some(72000.0)
        ]),
    );
    columns.insert(
        "department".to_string(),
        Series::new_string("department", vec![
            Some("Engineering".to_string()),
            Some("Marketing".to_string()),
            Some("Engineering".to_string()),
            Some("Sales".to_string()),
        ]),
    );

    let df = DataFrame::new(columns)?;

    // 1. Basic DataFrame info
    println!("📊 DataFrame Info:");
    println!("Rows: {}, Columns: {}", df.row_count(), df.column_count());
    println!("Columns: {:?}\n", df.column_names());

    // 2. Filter employees with salary > 70000
    println!("💰 High Earners (Salary > $70,000):");
    let high_salary_condition = Condition::Gt("salary".to_string(), Value::F64(70000.0));
    let high_earners = df.filter(&high_salary_condition)?;
    println!("{}\n", high_earners);

    // 3. Select specific columns
    println!("👥 Names and Ages Only:");
    let names_ages = df.select_columns(vec!["name".to_string(), "age".to_string()])?;
    println!("{}\n", names_ages);

    // 4. Filter Engineering department
    println!("🔧 Engineering Team:");
    let eng_condition = Condition::Eq(
        "department".to_string(), 
        Value::String("Engineering".to_string())
    );
    let engineering_team = df.filter(&eng_condition)?;
    println!("{}\n", engineering_team);

    // 5. Sort by age (descending)
    println!("📈 Sorted by Age (Oldest First):");
    let sorted_by_age = df.sort(vec!["age".to_string()], false)?;
    println!("{}\n", sorted_by_age);

    // 6. Basic statistics
    println!("📊 Salary Statistics:");
    if let Some(salary_series) = df.get_column("salary") {
        println!("Mean Salary: ${:.2}", salary_series.mean()?);
        println!("Max Salary: ${:.2}", salary_series.max()?);
        println!("Min Salary: ${:.2}", salary_series.min()?);
    }

    Ok(())
}
```

Run this enhanced example:

```bash
cargo run
```

## Working with CSV Files

Veloxx can easily load data from CSV files. Create a sample CSV file:

```csv title="employees.csv"
name,age,salary,department
Alice,30,75000,Engineering
Bob,25,65000,Marketing
Charlie,35,85000,Engineering
Diana,28,72000,Sales
Eve,32,78000,Engineering
Frank,29,68000,Marketing
```

Then load and process it:

```rust title="src/main.rs"
use veloxx::dataframe::DataFrame;
use veloxx::conditions::Condition;
use veloxx::types::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load DataFrame from CSV
    let df = DataFrame::from_csv("employees.csv")?;
    
    println!("📂 Loaded from CSV:");
    println!("{}\n", df);

    // Group by department and calculate average salary
    println!("📊 Average Salary by Department:");
    let grouped = df.group_by(vec!["department".to_string()])?;
    let avg_salaries = grouped.agg(vec![("salary", "mean")])?;
    println!("{}\n", avg_salaries);

    // Find employees aged 30 or older
    println!("👴 Employees 30 or Older:");
    let condition = Condition::Gte("age".to_string(), Value::I32(30));
    let mature_employees = df.filter(&condition)?;
    println!("{}", mature_employees);

    Ok(())
}
```

## Next Steps

Congratulations! You've learned the basics of Veloxx. Here's what to explore next:

### 🚀 Advanced Features

- **[Advanced I/O](/docs/api/rust#advanced-io-operations)**: Work with JSON, Parquet, and other formats
- **[Data Quality](/docs/api/rust#data-quality--validation)**: Validate and clean your data
- **[Window Functions](/docs/api/rust#window-functions--analytics)**: Perform advanced analytics
- **[Joins](/docs/api/rust#joining)**: Combine multiple DataFrames

### 📚 Learning Resources

- **[Complete API Reference](/docs/api/rust)**: Explore all available methods
- **[Examples Repository](https://github.com/Conqxeror/veloxx/tree/main/examples)**: Real-world usage patterns
- **[Performance Guide](/docs/performance/benchmarks)**: Optimize your data processing

### 🔧 Integration

- **[Python Bindings](/docs/api/python)**: Use Veloxx from Python
- **[JavaScript/WASM](/docs/api/javascript)**: Run Veloxx in the browser or Node.js

### 💡 Common Patterns

```rust
// Chain operations for data pipeline
let result = df
    .filter(&age_condition)?
    .select_columns(vec!["name".to_string(), "salary".to_string()])?
    .sort(vec!["salary".to_string()], false)?;

// Handle missing data
let clean_df = df.drop_nulls()?;
let filled_df = df.fill_nulls(Value::I32(0))?;

// Export results
df.to_csv("output.csv")?;
```

### 🤝 Community

- **[GitHub Discussions](https://github.com/Conqxeror/veloxx/discussions)**: Ask questions and share ideas
- **[Issues](https://github.com/Conqxeror/veloxx/issues)**: Report bugs or request features
- **[Contributing Guide](https://github.com/Conqxeror/veloxx/blob/main/CONTRIBUTING.md)**: Help improve Veloxx

:::tip Pro Tip
Start small with simple operations and gradually explore more advanced features. The Veloxx API is designed to be intuitive and chainable for building complex data processing pipelines.
:::

:::info Performance Note
Veloxx is optimized for performance with columnar storage and lazy evaluation. For large datasets, consider using features like chunked processing and streaming I/O.
:::