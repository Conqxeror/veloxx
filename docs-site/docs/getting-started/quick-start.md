# Quick Start

Get up and running with Veloxx in just 5 minutes! Experience **up to 4x performance improvements** over traditional data processing with our SIMD-accelerated library.

## Why Choose Veloxx?

✅ **1.6-4x faster** than traditional data processing  
✅ **38-45% memory reduction** through optimized layouts  
✅ **Zero-copy operations** for maximum efficiency  
✅ **Production-ready** with 128 comprehensive tests  

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
veloxx = "0.3.1"
```

For advanced features with maximum performance:

```toml title="Cargo.toml"
[dependencies]
veloxx = { version = "0.3.1", features = ["all"] }
```

## Your First High-Performance DataFrame

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
    println!("{}
", df);

    // Group by department and calculate average salary
    println!("📊 Average Salary by Department:");
    let grouped = df.group_by(vec!["department".to_string()])?;
    let avg_salaries = grouped.agg(vec![("salary", "mean")])?;
    println!("{}
", avg_salaries);

    // Find employees aged 30 or older
    println!("👴 Employees 30 or Older:");
    let condition = Condition::GreaterThanOrEqual("age".to_string(), Value::I32(30));
    let mature_employees = df.filter(&condition)?;
    println!("{}", mature_employees);

    Ok(())
}
```

## Python Quick Start

Veloxx provides high-performance Python bindings.

### Installation

Make sure you have Veloxx installed for Python:

```bash
pip install veloxx
```

### Your First Python DataFrame

```python title="your_first_df.py"
import veloxx as vx

# Create a DataFrame from scratch
df = vx.PyDataFrame({
    "name": vx.PySeries("name", ["Alice", "Bob", "Charlie", "Diana"]),
    "age": vx.PySeries("age", [25, 30, 35, 28]),
    "city": vx.PySeries("city", ["New York", "London", "New York", "Paris"]),
})

print("📊 Our Employee DataFrame (Python):")
print(df)
```

Run your Python script:

```bash
python your_first_df.py
```

### Basic Python Operations

```python title="basic_ops.py"
import veloxx as vx

# Create the DataFrame (same as before)
df = vx.PyDataFrame({
    "name": vx.PySeries("name", ["Alice", "Bob", "Charlie", "Diana"]),
    "age": vx.PySeries("age", [25, 30, 35, 28]),
    "city": vx.PySeries("city", ["New York", "London", "New York", "Paris"]),
    "salary": vx.PySeries("salary", [50000.0, 60000.0, 75000.0, 55000.0]),
})

# 1. Basic DataFrame info
print("📊 DataFrame Info (Python):")
print(f"Rows: {df.row_count()}, Columns: {df.column_count()}")
print(f"Columns: {df.column_names()}
")

# 2. Filter employees with age > 28
print("👴 Employees older than 28:")
age_series = df.get_column("age")
filtered_indices = [i for i, age in enumerate(age_series.to_list()) if age is not None and age > 28]
filtered_df = df.filter(filtered_indices)
print(f"{filtered_df}
")

# 3. Select specific columns
print("👥 Names and Cities Only:")
names_cities = df.select_columns(["name", "city"])
print(f"{names_cities}
")

# 4. Group by city and calculate average salary
print("📊 Average Salary by City:")
grouped_df = df.group_by(["city"])
avg_salaries = grouped_df.agg([("salary", "mean")])
print(f"{avg_salaries}
")

# 5. Basic statistics on a Series
print("📊 Salary Statistics:")
salary_series = df.get_column("salary")
print(f"Mean Salary: ${salary_series.mean():.2f}")
print(f"Max Salary: ${salary_series.max():.2f}")
print(f"Min Salary: ${salary_series.min():.2f}")
```

Run your Python script:

```bash
python basic_ops.py
```

## JavaScript/WASM Quick Start

Veloxx also provides WebAssembly bindings for JavaScript environments (browser and Node.js).

### Installation

Make sure you have Veloxx installed for JavaScript:

```bash
npm install veloxx
```

### Your First JavaScript/WASM DataFrame

```javascript title="your_first_df.js"
import init, { WasmDataFrame, WasmSeries } from 'veloxx';

async function run() {
    await init(); // Initialize the WASM module

    // Create a DataFrame from scratch
    const df = new WasmDataFrame({
        name: ["Alice", "Bob", "Charlie", "Diana"],
        age: [25, 30, 35, 28],
        salary: [50000, 60000, 75000, 55000],
    });

    console.log("📊 Our Employee DataFrame (JavaScript/WASM):");
    console.log(df);
}

run().catch(console.error);
```

Run your JavaScript file (Node.js):

```bash
node your_first_df.js
```

### Basic JavaScript/WASM Operations

```javascript title="basic_ops.js"
import init, { WasmDataFrame, WasmSeries } from 'veloxx';

async function run() {
    await init(); // Initialize the WASM module

    // Create the DataFrame (same as before)
    const df = new WasmDataFrame({
        name: ["Alice", "Bob", "Charlie", "Diana"],
        age: [25, 30, 35, 28],
        city: ["New York", "London", "New York", "Paris"],
        salary: [50000, 60000, 75000, 55000],
    });

    // 1. Basic DataFrame info
    console.log("📊 DataFrame Info (JavaScript/WASM):");
    console.log(`Rows: ${df.rowCount()}, Columns: ${df.columnCount()}`);
    console.log(`Columns: ${df.columnNames()}
`);

    // 2. Filter employees with age > 28
    console.log("👴 Employees older than 28:");
    const ageSeries = df.getColumn("age");
    const filteredIndices = [];
    for (let i = 0; i < ageSeries.len; i++) {
        if (ageSeries.getValue(i) > 28) {
            filteredIndices.push(i);
        }
    }
    const filteredDf = df.filter(new Uint32Array(filteredIndices));
    console.log(`${filteredDf}
`);

    // 3. Select specific columns
    console.log("👥 Names and Cities Only:");
    const namesCities = df.selectColumns(["name", "city"]);
    console.log(`${namesCities}
`);

    // 4. Group by city and calculate average salary
    console.log("📊 Average Salary by City:");
    const groupedDf = df.groupBy(["city"]);
    const avgSalaries = groupedDf.agg([["salary", "mean"]]);
    console.log(`${avgSalaries}
`);

    // 5. Basic statistics on a Series
    console.log("📊 Salary Statistics:");
    const salarySeries = df.getColumn("salary");
    console.log(`Mean Salary: ${salarySeries.mean().toFixed(2)}`);
    console.log(`Max Salary: ${salarySeries.max().toFixed(2)}`);
    console.log(`Min Salary: ${salarySeries.min().toFixed(2)}`);
}

run().catch(console.error);


## Next Steps

Congratulations! You've learned the basics of Veloxx. Here's what to explore next:

### 📚 Learning Resources

- **[Complete API Reference](/docs/api/rust)**: Explore all available methods for Rust, Python, and JavaScript/WASM.
- **[Tutorials & Guides](/docs/tutorials/general_tutorial)**: Dive deeper into specific topics like advanced I/O, data quality, and more.
- **[Examples Repository](https://github.com/conqxeror/veloxx/tree/main/examples)**: See real-world usage patterns and complete code examples.
- **[Performance Guide](/docs/performance/benchmarks)**: Learn how to optimize your data processing workflows.

### 💡 Common Patterns

Veloxx operations are designed to be intuitive and chainable, allowing you to build complex data pipelines efficiently.

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

- **[GitHub Discussions](https://github.com/conqxeror/veloxx/discussions)**: Ask questions and share ideas.
- **[Issues](https://github.com/conqxeror/veloxx/issues)**: Report bugs or request features.
- **[Contributing Guide](https://github.com/conqxeror/veloxx/blob/main/CONTRIBUTING.md)**: Learn how you can contribute to Veloxx.

:::tip Pro Tip
Start small with simple operations and gradually explore more advanced features. The Veloxx API is designed to be intuitive and chainable for building complex data processing pipelines.
:::

:::info Performance Note
Veloxx is optimized for performance with columnar storage and lazy evaluation. For large datasets, consider using features like chunked processing and streaming I/O.
:::