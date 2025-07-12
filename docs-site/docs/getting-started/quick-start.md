# Quick Start Guide

Get up and running with Veloxx in just 5 minutes! This guide will walk you through the essential concepts and operations.

## What is Veloxx?

Veloxx is a **high-performance data processing library** that provides:
- 🚀 **10x faster** operations than pandas
- 🪶 **Minimal memory footprint** 
- 🌐 **Multi-language support** (Rust, Python, JavaScript)
- 🛡️ **Memory safety** with zero runtime overhead

## Choose Your Language

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs>
<TabItem value="rust" label="Rust">

### Installation

```toml title="Cargo.toml"
[dependencies]
veloxx = "0.2.4"
```

### Your First DataFrame

```rust
use veloxx::prelude::*;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a DataFrame
    let mut columns = BTreeMap::new();
    columns.insert("name".to_string(), Series::new_string("name", vec![
        Some("Alice".to_string()),
        Some("Bob".to_string()),
        Some("Charlie".to_string()),
    ]));
    columns.insert("age".to_string(), Series::new_i32("age", vec![
        Some(25), Some(30), Some(35)
    ]));
    columns.insert("salary".to_string(), Series::new_f64("salary", vec![
        Some(50000.0), Some(75000.0), Some(60000.0)
    ]));
    
    let df = DataFrame::new(columns)?;
    println!("DataFrame:\n{}", df);
    
    Ok(())
}
```

### Basic Operations

```rust
// Filter rows
let condition = Condition::Gt("age".to_string(), Value::I32(25));
let filtered = df.filter(&condition)?;

// Select columns
let selected = df.select_columns(vec!["name".to_string(), "salary".to_string()])?;

// Add computed column
let bonus_expr = Expr::Multiply(
    Box::new(Expr::Column("salary".to_string())),
    Box::new(Expr::Literal(Value::F64(0.1)))
);
let with_bonus = df.with_column("bonus", &bonus_expr)?;

// Group by and aggregate
let summary = df.group_by(vec!["department".to_string()])?
    .agg(vec![("salary", "mean"), ("age", "count")])?;
```

</TabItem>
<TabItem value="python" label="Python">

### Installation

```bash
pip install veloxx
```

### Your First DataFrame

```python
import veloxx as vx

# Create a DataFrame
df = vx.PyDataFrame({
    "name": vx.PySeries("name", ["Alice", "Bob", "Charlie"]),
    "age": vx.PySeries("age", [25, 30, 35]),
    "salary": vx.PySeries("salary", [50000.0, 75000.0, 60000.0])
})

print("DataFrame:")
print(df)
```

### Basic Operations

```python
# Filter rows (age > 25)
age_series = df.get_column("age")
indices = [i for i, age in enumerate(age_series.to_list()) if age and age > 25]
filtered = df.filter(indices)

# Select columns
selected = df.select_columns(["name", "salary"])

# Add computed column
bonus_expr = vx.PyExpr.multiply(
    vx.PyExpr.column("salary"),
    vx.PyExpr.literal(0.1)
)
with_bonus = df.with_column("bonus", bonus_expr)

# Group by and aggregate
grouped = df.group_by(["department"])
summary = grouped.agg([("salary", "mean"), ("age", "count")])
```

</TabItem>
<TabItem value="javascript" label="JavaScript">

### Installation

```bash
npm install veloxx
```

### Your First DataFrame

```javascript
const vx = require('veloxx');

async function main() {
    // Initialize WASM
    await vx.init();
    
    // Create a DataFrame
    const df = new vx.WasmDataFrame({
        name: ["Alice", "Bob", "Charlie"],
        age: [25, 30, 35],
        salary: [50000.0, 75000.0, 60000.0]
    });
    
    console.log("DataFrame created!");
    console.log(`Rows: ${df.rowCount}, Columns: ${df.columnCount}`);
}

main().catch(console.error);
```

### Basic Operations

```javascript
// Filter rows (age > 25)
const ageColumn = df.getColumn("age");
const indices = [];
for (let i = 0; i < ageColumn.len; i++) {
    if (ageColumn.getValue(i) > 25) {
        indices.push(i);
    }
}
const filtered = df.filter(new Uint32Array(indices));

// Add computed column
const bonusExpr = vx.WasmExpr.multiply(
    vx.WasmExpr.column("salary"),
    vx.WasmExpr.literal(new vx.WasmValue(0.1))
);
const withBonus = df.withColumn("bonus", bonusExpr);

// Aggregate operations
const salaryColumn = df.getColumn("salary");
console.log(`Average salary: ${salaryColumn.mean()}`);
console.log(`Total salary: ${salaryColumn.sum()}`);
```

</TabItem>
</Tabs>

## Core Concepts

### DataFrames
A **DataFrame** is a 2D table with labeled columns, similar to a spreadsheet or SQL table.

```rust
// Rust example
let df = DataFrame::from_csv("employees.csv")?;
```

```python
# Python example
df = vx.read_csv("employees.csv")
```

### Series
A **Series** is a single column of data with a specific type.

```rust
// Rust example
let ages = Series::new_i32("age", vec![Some(25), Some(30), None]);
```

```python
# Python example
ages = vx.PySeries("age", [25, 30, None])
```

### Expressions
**Expressions** define computations for creating new columns.

```rust
// Rust example
let total = Expr::Add(
    Box::new(Expr::Column("base".to_string())),
    Box::new(Expr::Column("bonus".to_string()))
);
```

```python
# Python example
total = vx.PyExpr.add(
    vx.PyExpr.column("base"),
    vx.PyExpr.column("bonus")
)
```

## Common Operations

### Loading Data

<Tabs>
<TabItem value="rust" label="Rust">

```rust
// From CSV
let df = DataFrame::from_csv("data.csv")?;

// From JSON
let df = DataFrame::from_json("data.json")?;

// From vectors
let data = vec![
    vec!["Alice".to_string(), "25".to_string()],
    vec!["Bob".to_string(), "30".to_string()],
];
let columns = vec!["name".to_string(), "age".to_string()];
let df = DataFrame::from_vec_of_vec(data, columns)?;
```

</TabItem>
<TabItem value="python" label="Python">

```python
# From CSV
df = vx.read_csv("data.csv")

# From JSON
df = vx.read_json("data.json")

# From dictionary
df = vx.PyDataFrame({
    "name": vx.PySeries("name", ["Alice", "Bob"]),
    "age": vx.PySeries("age", [25, 30])
})
```

</TabItem>
<TabItem value="javascript" label="JavaScript">

```javascript
// From arrays
const df = new vx.WasmDataFrame({
    name: ["Alice", "Bob"],
    age: [25, 30]
});

// Note: CSV/JSON loading in browser requires fetch
async function loadCsv(url) {
    const response = await fetch(url);
    const text = await response.text();
    // Parse CSV text and create DataFrame
}
```

</TabItem>
</Tabs>

### Filtering Data

<Tabs>
<TabItem value="rust" label="Rust">

```rust
// Simple condition
let condition = Condition::Gt("age".to_string(), Value::I32(25));
let filtered = df.filter(&condition)?;

// Complex condition
let complex = Condition::And(
    Box::new(Condition::Gt("age".to_string(), Value::I32(25))),
    Box::new(Condition::Lt("age".to_string(), Value::I32(65)))
);
let working_age = df.filter(&complex)?;
```

</TabItem>
<TabItem value="python" label="Python">

```python
# Filter by condition
age_series = df.get_column("age")
indices = [i for i, age in enumerate(age_series.to_list()) 
           if age and age > 25]
filtered = df.filter(indices)

# Multiple conditions
indices = [i for i, age in enumerate(age_series.to_list()) 
           if age and 25 < age < 65]
working_age = df.filter(indices)
```

</TabItem>
<TabItem value="javascript" label="JavaScript">

```javascript
// Filter by condition
const ageColumn = df.getColumn("age");
const indices = [];
for (let i = 0; i < ageColumn.len; i++) {
    const age = ageColumn.getValue(i);
    if (age > 25 && age < 65) {
        indices.push(i);
    }
}
const workingAge = df.filter(new Uint32Array(indices));
```

</TabItem>
</Tabs>

### Grouping and Aggregation

<Tabs>
<TabItem value="rust" label="Rust">

```rust
// Group by single column
let grouped = df.group_by(vec!["department".to_string()])?;
let summary = grouped.agg(vec![("salary", "mean")])?;

// Group by multiple columns
let grouped = df.group_by(vec!["department".to_string(), "level".to_string()])?;
let detailed = grouped.agg(vec![
    ("salary", "mean"),
    ("salary", "count"),
    ("age", "max")
])?;
```

</TabItem>
<TabItem value="python" label="Python">

```python
# Group by single column
grouped = df.group_by(["department"])
summary = grouped.mean()

# Custom aggregations
summary = grouped.agg([
    ("salary", "mean"),
    ("salary", "count"),
    ("age", "max")
])
```

</TabItem>
<TabItem value="javascript" label="JavaScript">

```javascript
// Group by and aggregate
const grouped = df.groupBy(["department"]);
const summary = grouped.agg([
    ["salary", "mean"],
    ["salary", "count"]
]);
```

</TabItem>
</Tabs>

### Adding Computed Columns

<Tabs>
<TabItem value="rust" label="Rust">

```rust
// Simple arithmetic
let bonus_expr = Expr::Multiply(
    Box::new(Expr::Column("salary".to_string())),
    Box::new(Expr::Literal(Value::F64(0.1)))
);
let with_bonus = df.with_column("bonus", &bonus_expr)?;

// Complex expression
let total_comp = Expr::Add(
    Box::new(Expr::Column("salary".to_string())),
    Box::new(Expr::Column("bonus".to_string()))
);
let with_total = with_bonus.with_column("total_compensation", &total_comp)?;
```

</TabItem>
<TabItem value="python" label="Python">

```python
# Simple arithmetic
bonus_expr = vx.PyExpr.multiply(
    vx.PyExpr.column("salary"),
    vx.PyExpr.literal(0.1)
)
with_bonus = df.with_column("bonus", bonus_expr)

# Complex expression
total_expr = vx.PyExpr.add(
    vx.PyExpr.column("salary"),
    vx.PyExpr.column("bonus")
)
with_total = with_bonus.with_column("total_compensation", total_expr)
```

</TabItem>
<TabItem value="javascript" label="JavaScript">

```javascript
// Simple arithmetic
const bonusExpr = vx.WasmExpr.multiply(
    vx.WasmExpr.column("salary"),
    vx.WasmExpr.literal(new vx.WasmValue(0.1))
);
const withBonus = df.withColumn("bonus", bonusExpr);
```

</TabItem>
</Tabs>

## Real-World Example

Let's analyze employee data to find insights:

<Tabs>
<TabItem value="rust" label="Rust">

```rust
use veloxx::prelude::*;

fn analyze_employees() -> Result<(), Box<dyn std::error::Error>> {
    // Load employee data
    let df = DataFrame::from_csv("employees.csv")?;
    
    println!("Dataset overview:");
    println!("Rows: {}, Columns: {}", df.row_count(), df.column_count());
    
    // Calculate statistics
    let stats = df.describe()?;
    println!("Statistics:\n{}", stats);
    
    // Find high performers (salary > 70k)
    let high_performers = df.filter(&Condition::Gt(
        "salary".to_string(), 
        Value::F64(70000.0)
    ))?;
    
    // Analyze by department
    let dept_analysis = df
        .group_by(vec!["department".to_string()])?
        .agg(vec![
            ("salary", "mean"),
            ("salary", "count"),
            ("age", "mean")
        ])?;
    
    println!("Department Analysis:\n{}", dept_analysis);
    
    // Calculate salary bands
    let salary_band_expr = Expr::Divide(
        Box::new(Expr::Column("salary".to_string())),
        Box::new(Expr::Literal(Value::F64(10000.0)))
    );
    
    let with_bands = df.with_column("salary_band", &salary_band_expr)?;
    
    // Export results
    dept_analysis.to_csv("department_analysis.csv")?;
    high_performers.to_csv("high_performers.csv")?;
    
    Ok(())
}
```

</TabItem>
<TabItem value="python" label="Python">

```python
import veloxx as vx

def analyze_employees():
    # Load employee data
    df = vx.read_csv("employees.csv")
    
    print(f"Dataset overview:")
    print(f"Rows: {df.row_count()}, Columns: {df.column_count()}")
    
    # Calculate statistics
    stats = df.describe()
    print(f"Statistics:\n{stats}")
    
    # Find high performers (salary > 70k)
    salary_series = df.get_column("salary")
    high_perf_indices = [
        i for i, salary in enumerate(salary_series.to_list())
        if salary and salary > 70000
    ]
    high_performers = df.filter(high_perf_indices)
    
    # Analyze by department
    dept_analysis = df.group_by(["department"]).agg([
        ("salary", "mean"),
        ("salary", "count"),
        ("age", "mean")
    ])
    
    print(f"Department Analysis:\n{dept_analysis}")
    
    # Calculate salary bands
    salary_band_expr = vx.PyExpr.divide(
        vx.PyExpr.column("salary"),
        vx.PyExpr.literal(10000.0)
    )
    
    with_bands = df.with_column("salary_band", salary_band_expr)
    
    # Export results
    dept_analysis.to_csv("department_analysis.csv")
    high_performers.to_csv("high_performers.csv")

# Run analysis
analyze_employees()
```

</TabItem>
<TabItem value="javascript" label="JavaScript">

```javascript
const vx = require('veloxx');

async function analyzeEmployees() {
    await vx.init();
    
    // Create sample data (in real app, load from file)
    const df = new vx.WasmDataFrame({
        name: ["Alice", "Bob", "Charlie", "Diana"],
        department: ["Engineering", "Sales", "Engineering", "Marketing"],
        salary: [75000, 65000, 80000, 70000],
        age: [28, 32, 26, 30]
    });
    
    console.log(`Dataset: ${df.rowCount} rows, ${df.columnCount} columns`);
    
    // Find high performers
    const salaryColumn = df.getColumn("salary");
    const highPerfIndices = [];
    for (let i = 0; i < salaryColumn.len; i++) {
        if (salaryColumn.getValue(i) > 70000) {
            highPerfIndices.push(i);
        }
    }
    const highPerformers = df.filter(new Uint32Array(highPerfIndices));
    
    // Calculate statistics
    console.log(`Average salary: ${salaryColumn.mean()}`);
    console.log(`Max salary: ${salaryColumn.max()}`);
    console.log(`High performers: ${highPerformers.rowCount}`);
}

analyzeEmployees().catch(console.error);
```

</TabItem>
</Tabs>

## Performance Tips

1. **Load data efficiently**: Use CSV/JSON readers for best performance
2. **Filter early**: Apply filters before expensive operations
3. **Use appropriate types**: Let Veloxx infer types automatically
4. **Chain operations**: Combine multiple operations for optimization
5. **Process in chunks**: For very large datasets, process incrementally

## Next Steps

Now that you've learned the basics:

1. 🧠 **[Core Concepts](/docs/intro)** - Deep dive into DataFrames and Series
2. 📖 **[Data Operations](/docs/intro)** - Learn advanced data manipulation
3. 🔍 **[API Reference](/docs/api/rust)** - Explore the complete API
4. 🚀 **[Examples](/docs/intro)** - See real-world usage patterns
5. ⚡ **[Benchmarks](/docs/performance/benchmarks)** - Compare performance with other libraries

:::tip Pro Tip
Veloxx shines with large datasets and complex operations. Try it on your real data to see the performance benefits!
:::

## Common Patterns

### Data Pipeline

```rust
// Rust data processing pipeline
let result = DataFrame::from_csv("raw_data.csv")?
    .filter(&Condition::Ne("status".to_string(), Value::String("inactive".to_string())))?
    .with_column("profit", &profit_expression)?
    .group_by(vec!["region".to_string()])?
    .agg(vec![("profit", "sum"), ("customers", "count")])?
    .sort(vec!["profit_sum".to_string()], false)?;

result.to_csv("analysis_results.csv")?;
```

### Error Handling

```rust
// Robust error handling
match DataFrame::from_csv("data.csv") {
    Ok(df) => {
        // Process successfully loaded data
        let result = df.group_by(vec!["category".to_string()])?;
        println!("Analysis complete: {} groups", result.row_count());
    }
    Err(e) => {
        eprintln!("Failed to load data: {}", e);
        // Handle error appropriately
    }
}
```

Ready to dive deeper? Check out our [comprehensive examples](/docs/intro) or explore the [API documentation](/docs/api/rust)!