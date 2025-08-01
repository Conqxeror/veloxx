# Rust API Reference

Complete API reference for the Veloxx Rust library. This guide covers all available functionality with practical examples and best practices.

## Table of Contents

1. [Core Data Structures](#core-data-structures)
2. [Basic Operations](#basic-operations)
3. [Advanced I/O Operations](#advanced-io-operations)
4. [Data Quality & Validation](#data-quality--validation)
5. [Window Functions & Analytics](#window-functions--analytics)
6. [Performance Optimization](#performance-optimization)
7. [Visualization](#visualization)
8. [Machine Learning](#machine-learning)

## Core Data Structures

### DataFrame

The `DataFrame` is the primary data structure in Veloxx, representing a columnar data table with heterogeneous data types.

#### Creation Methods

```rust
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use std::collections::BTreeMap;

// Create from columns
let mut columns = BTreeMap::new();
columns.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string())]));
columns.insert("age".to_string(), Series::new_i32("age", vec![Some(30)]));
let df = DataFrame::new(columns)?;

// Create from Vec<Vec<String>>
let data = vec![
    vec!["Alice".to_string(), "30".to_string()],
    vec!["Bob".to_string(), "25".to_string()],
];
let column_names = vec!["name".to_string(), "age".to_string()];
let df = DataFrame::from_vec_of_vec(data, column_names)?;

// Load from CSV
let df = DataFrame::from_csv("data.csv")?;

// Load from JSON
let df = DataFrame::from_json("data.json")?;
```

#### Core Methods

##### Information Methods

```rust
// Get basic information
let row_count = df.row_count();        // Number of rows
let col_count = df.column_count();     // Number of columns
let col_names = df.column_names();     // Vector of column names

// Get specific column
if let Some(age_column) = df.get_column("age") {
    println!("Age column has {} values", age_column.len());
}

// Display first/last rows
let first_5 = df.head(5)?;   // First 5 rows
let last_5 = df.tail(5)?;    // Last 5 rows
```

##### Data Inspection

```rust
// Generate descriptive statistics
let stats = df.describe()?;
println!("Statistics:\n{}", stats);

// Check data types
for name in df.column_names() {
    if let Some(column) = df.get_column(name) {
        println!("{}: {:?}", name, column.data_type());
    }
}
```

### Series

The `Series` represents a single column of data with a specific type.

#### Creation Methods

```rust
use veloxx::series::Series;

// Different data types
let int_series = Series::new_i32("ages", vec![Some(25), Some(30), None]);
let float_series = Series::new_f64("scores", vec![Some(95.5), Some(87.2)]);
let string_series = Series::new_string("names", vec![Some("Alice".to_string())]);
let bool_series = Series::new_bool("active", vec![Some(true), Some(false)]);
let datetime_series = Series::new_datetime("timestamps", vec![Some(1678886400)]);
```

#### Core Methods

```rust
// Basic information
let length = series.len();
let data_type = series.data_type();
let name = series.name();

// Access values
let value = series.get_value(0)?;  // Get value at index
let is_null = series.get_value(0).is_none();   // Check if null
let null_count = series.len() - series.count(); // Count nulls

// Statistics (for numeric series)
let mean = series.mean()?;
let sum = series.sum()?;
let max = series.max()?;
let min = series.min()?;
let std_dev = series.std()?;
```

## Basic Operations

### Filtering

Filter rows based on conditions using the `Condition` enum:

```rust
use veloxx::conditions::Condition;
use veloxx::types::Value;

// Simple conditions
let condition = Condition::Gt("age".to_string(), Value::I32(25));
let filtered_df = df.filter(&condition)?;

// Available condition types
let eq_condition = Condition::Eq("status".to_string(), Value::String("active".to_string()));
let ne_condition = Condition::Ne("status".to_string(), Value::String("inactive".to_string()));
let lt_condition = Condition::Lt("score".to_string(), Value::F64(80.0));
let le_condition = Condition::Le("score".to_string(), Value::F64(80.0));
let gt_condition = Condition::Gt("score".to_string(), Value::F64(80.0));
let ge_condition = Condition::Ge("score".to_string(), Value::F64(80.0));

// Complex conditions
let complex_condition = Condition::And(
    Box::new(Condition::Gt("age".to_string(), Value::I32(25))),
    Box::new(Condition::Lt("age".to_string(), Value::I32(65)))
);
let working_age = df.filter(&complex_condition)?;

let or_condition = Condition::Or(
    Box::new(Condition::Eq("department".to_string(), Value::String("Engineering".to_string()))),
    Box::new(Condition::Eq("department".to_string(), Value::String("Research".to_string())))
);
let tech_teams = df.filter(&or_condition)?;
```

### Column Operations

#### Selection and Dropping

```rust
// Select specific columns
let selected_df = df.select_columns(vec!["name".to_string(), "age".to_string()])?;

// Drop columns
let dropped_df = df.drop_columns(vec!["unwanted_col".to_string()])?;

// Rename columns
let renamed_df = df.rename_column("old_name", "new_name")?;
```

#### Adding Computed Columns

```rust
use veloxx::expressions::Expr;

// Simple arithmetic
let bonus_expr = Expr::Add(
    Box::new(Expr::Column("salary".to_string())),
    Box::new(Expr::Literal(Value::F64(5000.0)))
);
let df_with_bonus = df.with_column("salary_with_bonus", &bonus_expr)?;

// Complex expressions
let total_comp = Expr::Add(
    Box::new(Expr::Multiply(
        Box::new(Expr::Column("salary".to_string())),
        Box::new(Expr::Literal(Value::F64(1.1))) // 10% increase
    )),
    Box::new(Expr::Column("bonus".to_string()))
);
let df_with_total = df.with_column("total_compensation", &total_comp)?;

// Available expression types
let add_expr = Expr::Add(Box::new(expr1), Box::new(expr2));
let subtract_expr = Expr::Subtract(Box::new(expr1), Box::new(expr2));
let multiply_expr = Expr::Multiply(Box::new(expr1), Box::new(expr2));
let divide_expr = Expr::Divide(Box::new(expr1), Box::new(expr2));
```

### Aggregation and Grouping

#### Group By Operations

```rust
// Group by single column
let grouped_df = df.group_by(vec!["department".to_string()])?;
let aggregated_df = grouped_df.agg(vec![
    ("salary", "mean"),
    ("salary", "sum"),
    ("age", "count"),
    ("age", "max"),
    ("age", "min")
])?;

// Group by multiple columns
let multi_grouped = df.group_by(vec!["department".to_string(), "level".to_string()])?;
let detailed_agg = multi_grouped.agg(vec![
    ("salary", "mean"),
    ("bonus", "sum"),
    ("performance_score", "max")
])?;
```

#### Series Aggregations

```rust
// Direct series aggregations
if let Some(salary_series) = df.get_column("salary") {
    let mean_salary = salary_series.mean()?;
    let total_salary = salary_series.sum()?;
    let max_salary = salary_series.max()?;
    let min_salary = salary_series.min()?;
    let std_salary = salary_series.std()?;
    
    println!("Salary Statistics:");
    println!("Mean: ${:.2}", mean_salary);
    println!("Total: ${:.2}", total_salary);
    println!("Range: ${:.2} - ${:.2}", min_salary, max_salary);
    println!("Std Dev: ${:.2}", std_salary);
}
```

### Sorting and Joining

#### Sorting

```rust
// Sort by single column
let sorted_df = df.sort(vec!["age".to_string()], true)?; // ascending
let sorted_desc_df = df.sort(vec!["salary".to_string()], false)?; // descending

// Sort by multiple columns
let multi_sorted = df.sort(vec!["department".to_string(), "salary".to_string()], true)?;
```

#### Joining

```rust
// Inner join
let joined_df = df1.join(&df2, "id", JoinType::Inner)?;

// Left join
let left_joined_df = df1.join(&df2, "user_id", JoinType::Left)?;

// Join on multiple columns
let multi_join = df1.join(&df2, "dept_id", JoinType::Inner)?;
```

## Advanced I/O Operations

*Available with `advanced_io` feature flag*

### File Format Support

```rust
#[cfg(feature = "advanced_io")]
use veloxx::advanced_io::{AsyncFileOps, ParquetReader, ParquetWriter, JsonStreamer, DatabaseConnector, CompressionType};

// CSV with options
let df = DataFrame::from_csv("data.csv")?;
df.to_csv("output.csv")?;

// JSON support
#[cfg(feature = "advanced_io")]
let df = DataFrame::from_json("data.json")?;
#[cfg(feature = "advanced_io")]
df.to_json("output.json")?;

// Custom data loading
let data = vec![
    vec!["Alice".to_string(), "30".to_string(), "Engineer".to_string()],
    vec!["Bob".to_string(), "25".to_string(), "Designer".to_string()],
];
let columns = vec!["name".to_string(), "age".to_string(), "role".to_string()];
let df = DataFrame::from_vec_of_vec(data, columns)?;
```

### Streaming and Large Data

```rust
// For large datasets, process in chunks
#[cfg(feature = "advanced_io")]
fn process_large_csv(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let chunk_size = 10000;
    let mut total_rows = 0;
    
    // This is a conceptual example - actual chunked reading would be implemented
    // in the advanced_io feature
    let df = DataFrame::from_csv(file_path)?;
    
    // Process in chunks
    for chunk_start in (0..df.row_count()).step_by(chunk_size) {
        let chunk_end = std::cmp::min(chunk_start + chunk_size, df.row_count());
        // Process chunk
        total_rows += chunk_end - chunk_start;
    }
    
    println!("Processed {} rows", total_rows);
    Ok(())
}
```

## Data Quality & Validation

### Handling Missing Data

```rust
// Remove rows with any null values
let clean_df = df.drop_nulls()?;

// Fill null values
let filled_df = df.fill_nulls(Value::I32(0))?;

// Check for nulls
for column_name in df.column_names() {
    if let Some(column) = df.get_column(column_name) {
        let null_count = column.null_count();
        if null_count > 0 {
            println!("Column '{}' has {} null values", column_name, null_count);
        }
    }
}
```

### Data Validation

```rust
#[cfg(feature = "data_quality")]
use veloxx::data_quality::{Schema, ColumnSchema, Constraint, SchemaValidator};
#[cfg(feature = "data_quality")]
use veloxx::types::DataType;

// Validate data ranges
#[cfg(feature = "data_quality")]
fn validate_age_range(df: &DataFrame) -> Result<bool, Box<dyn std::error::Error>> {
    if let Some(age_column) = df.get_column("age") {
        let min_age = age_column.min()?;
        let max_age = age_column.max()?;
        
        if let (Some(Value::F64(min_val)), Some(Value::F64(max_val))) = (min_age, max_age) {
            if min_val < 0.0 || max_val > 150.0 {
                println!("Warning: Age values outside expected range (0-150)");
                return Ok(false);
            }
        }
    }
    Ok(true)
}

// Check for duplicates (conceptual - would be in data_quality feature)
#[cfg(feature = "data_quality")]
fn check_duplicates(df: &DataFrame, key_columns: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    // Implementation would group by key columns and count
    // This is a placeholder for the actual feature
    Ok(0)
}
```

### Data Profiling

```rust
#[cfg(feature = "data_quality")]
use veloxx::data_quality::DataProfiler;

// Generate data profile
#[cfg(feature = "data_quality")]
fn profile_dataframe(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("DataFrame Profile:");
    println!("================");
    println!("Rows: {}", df.row_count());
    println!("Columns: {}", df.column_count());
    println!();
    
    for column_name in df.column_names() {
        if let Some(column) = df.get_column(column_name) {
            println!("Column: {}", column_name);
            println!("  Type: {:?}", column.data_type());
            println!("  Length: {}", column.len());
            println!("  Null Count: {}", column.null_count());
            println!("  Null %: {:.2}%", (column.null_count() as f64 / column.len() as f64) * 100.0);
            
            // For numeric columns, show statistics
            match column.data_type() {
                veloxx::types::DataType::I32 | veloxx::types::DataType::F64 => {
                    if let (Some(mean), Some(std)) = (column.mean()?, column.std_dev()?) {
                        println!("  Mean: {:.2}", mean);
                        println!("  Std Dev: {:.2}", std);
                        println!("  Min: {:.2}", column.min()?);
                        println!("  Max: {:.2}", column.max()?);
                    }
                }
                _ => {}
            }
            println!();
        }
    }
    
    Ok(())
}
```

## Window Functions & Analytics

*Available with `window_functions` feature flag*

### Window Operations

```rust
#[cfg(feature = "window_functions")]
use veloxx::window_functions::{WindowSpec, WindowFunction, RankingFunction, AggregateFunction, OffsetFunction, TimeWindow};
#[cfg(feature = "window_functions")]
use chrono::Duration;

// Running totals
let window_spec = WindowSpec::new()
    .partition_by(vec!["department".to_string()])
    .order_by(vec!["date".to_string()]);

// This would be the API for window functions
// let df_with_running_total = WindowFunction::apply_aggregate(
//     &df, 
//     "sales", 
//     &AggregateFunction::Sum, 
//     &window_spec
// )?;
```

### Time Series Analysis

```rust
// Time-based operations (conceptual for window_functions feature)
#[cfg(feature = "window_functions")]
fn analyze_time_series(df: &DataFrame) -> Result<DataFrame, Box<dyn std::error::Error>> {
    // Sort by timestamp
    let sorted_df = df.sort(vec!["timestamp".to_string()], true)?;
    
    // Calculate moving averages, trends, etc.
    // This would be implemented in the window_functions feature
    
    Ok(sorted_df)
}
```

## Performance Optimization

*Available with `performance` feature flag*

### Best Practices

```rust
// 1. Chain operations efficiently
let result = df
    .filter(&Condition::Gt("score".to_string(), Value::F64(80.0)))?
    .select_columns(vec!["name".to_string(), "score".to_string()])?
    .sort(vec!["score".to_string()], false)?;

// 2. Use appropriate data types
let optimized_series = Series::new_i32("count", vec![Some(1), Some(2), Some(3)]);
// Instead of Series::new_f64 for integer data

// 3. Filter early in the pipeline
let filtered_first = df
    .filter(&condition)?  // Apply filters first
    .group_by(vec!["category".to_string()])?  // Then group
    .agg(vec![("value", "sum")])?;  // Finally aggregate

// 4. Minimize data copying
let view = df.select_columns(vec!["needed_col".to_string()])?;
// Work with the view instead of the full DataFrame
```

### Memory Management

```rust
#[cfg(feature = "performance")]
use veloxx::performance::memory::{MemoryAnalyzer, CompressedColumn};

// Monitor memory usage
#[cfg(feature = "performance")]
fn process_with_memory_awareness(df: DataFrame) -> Result<DataFrame, Box<dyn std::error::Error>> {
    println!("Processing DataFrame with {} rows", df.row_count());
    
    // Example: Estimate memory usage
    let series_memory = MemoryAnalyzer::estimate_series_memory(df.get_column("some_column").unwrap());
    println!("Estimated memory for 'some_column': {} bytes", series_memory);

    // Example: Suggest compression
    let suggestions = MemoryAnalyzer::suggest_compression(df.get_column("another_column").unwrap());
    println!("Compression suggestions for 'another_column': {:?}", suggestions);

    // Process in stages to manage memory
    let stage1 = df.filter(&Condition::Ne("status".to_string(), Value::String("deleted".to_string())))?;
    
    let stage2 = stage1.select_columns(vec![
        "id".to_string(),
        "value".to_string(),
        "category".to_string()
    ])?;
    
    let result = stage2.group_by(vec!["category".to_string()])?
        .agg(vec![("value", "sum")])?;
    
    Ok(result)
}
```

### Parallel Processing

```rust
#[cfg(feature = "performance")]
use veloxx::performance::parallel::ParallelAggregations;
#[cfg(feature = "performance")]
use rayon::prelude::*;

// Veloxx automatically uses parallel processing for many operations
// No special configuration needed - operations are optimized internally

// For custom parallel processing:
#[cfg(feature = "performance")]
fn parallel_series_processing(series_list: Vec<Series>) -> Vec<f64> {
    series_list
        .par_iter()
        .map(|series| series.mean().unwrap_or(Value::F64(0.0)).as_f64().unwrap_or(0.0))
        .collect()
}
```

## Error Handling

Veloxx operations return `Result<T, VeloxxError>`. Common error types:

```rust
use veloxx::error::VeloxxError;

match result {
    Ok(dataframe) => println!("Success: {}", dataframe),
    Err(VeloxxError::ColumnNotFound(name)) => println!("Column '{}' not found", name),
    Err(VeloxxError::DataTypeMismatch(msg)) => println!("Type mismatch: {}", msg),
    Err(VeloxxError::InvalidOperation(msg)) => println!("Invalid operation: {}", msg),
    Err(VeloxxError::FileIO(err)) => println!("I/O error: {}", err),
    Err(VeloxxError::Parsing(msg)) => println!("Parse error: {}", msg),
    Err(VeloxxError::Unsupported(msg)) => println!("Unsupported feature: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Complete Example

Here's a comprehensive example demonstrating multiple features:

```rust
use veloxx::dataframe::DataFrame;
use veloxx::conditions::Condition;
use veloxx::expressions::Expr;
use veloxx::types::Value;
use std::collections::BTreeMap;

fn comprehensive_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load and inspect data
    let df = DataFrame::from_csv("sales_data.csv")?;
    println!("Loaded {} rows, {} columns", df.row_count(), df.column_count());
    
    // 2. Data cleaning
    let clean_df = df
        .filter(&Condition::Ne("status".to_string(), Value::String("cancelled".to_string())))?;
        .drop_nulls()?;
    
    // 3. Feature engineering
    let profit_expr = Expr::Subtract(
        Box::new(Expr::Column("revenue".to_string())),
        Box::new(Expr::Column("cost".to_string()))
    );
    let enriched_df = clean_df.with_column("profit", &profit_expr)?;
    
    // 4. Analysis
    let regional_analysis = enriched_df
        .group_by(vec!["region".to_string()])?;
        .agg(vec![
            ("profit", "sum"),
            ("revenue", "mean"),
            ("customer_id", "count")
        ])?;
    
    // 5. Filter for high-performing regions
    let top_regions = regional_analysis
        .filter(&Condition::Gt("profit_sum".to_string(), Value::F64(100000.0)))?;
        .sort(vec!["profit_sum".to_string()], false)?;
    
    // 6. Export results
    top_regions.to_csv("top_regions.csv")?;
    
    // 7. Summary statistics
    if let Some(profit_series) = enriched_df.get_column("profit") {
        println!("Profit Analysis:");
        println!("Total Profit: ${:.2}", profit_series.sum()?);
        println!("Average Profit: ${:.2}", profit_series.mean()?);
        println!("Profit Range: ${:.2} to ${:.2}", profit_series.min()?, profit_series.max()?);
    }
    
    Ok(())
}
```

This comprehensive API reference covers all major Veloxx functionality. For more examples and advanced usage patterns, check out the [examples repository](https://github.com/Conqxeror/veloxx/tree/main/examples) and [performance benchmarks](/docs/performance/benchmarks).

## Visualization

*Available with `visualization` feature flag*

Veloxx provides charting and plotting capabilities to visualize your data.

### Plotting DataFrames

```rust
#[cfg(feature = "visualization")]
use veloxx::visualization::{Plot, ChartType, PlotConfig};

#[cfg(feature = "visualization")]
fn create_plots(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    // Scatter Plot
    let scatter_plot = Plot::new(df, ChartType::Scatter)
        .with_columns("feature1", "feature2")
        .with_config(PlotConfig { 
            title: "Feature Correlation".to_string(), 
            x_label: "Feature 1".to_string(), 
            y_label: "Feature 2".to_string(), 
            ..Default::default() 
        });
    scatter_plot.save("scatter_plot.svg")?;

    // Bar Chart
    let bar_chart = Plot::new(df, ChartType::Bar)
        .with_columns("category", "value")
        .with_config(PlotConfig { 
            title: "Sales by Category".to_string(), 
            x_label: "Category".to_string(), 
            y_label: "Sales".to_string(), 
            ..Default::default() 
        });
    bar_chart.save("bar_chart.svg")?;

    // Histogram
    let histogram = Plot::new(df, ChartType::Histogram)
        .with_columns("age", "") // Y-column is not used for histograms
        .with_config(PlotConfig { 
            title: "Age Distribution".to_string(), 
            x_label: "Age".to_string(), 
            y_label: "Frequency".to_string(), 
            ..Default::default() 
        });
    histogram.save("age_histogram.svg")?;

    Ok(())
}
```

## Machine Learning

*Available with `ml` feature flag*

Veloxx integrates with machine learning algorithms for predictive modeling and data preprocessing.

### Linear Regression

```rust
#[cfg(feature = "ml")]
use veloxx::ml::LinearRegression;

#[cfg(feature = "ml")]
fn run_linear_regression(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let mut model = LinearRegression::new();
    let fitted_model = model.fit(df, "target_column", &["feature1", "feature2"])?;
    let predictions = fitted_model.predict(df, &["feature1", "feature2"])?;
    println!("Predictions: {:?}", predictions);
    Ok(())
}
```

### Data Preprocessing

```rust
#[cfg(feature = "ml")]
use veloxx::ml::Preprocessing;

#[cfg(feature = "ml")]
fn preprocess_data(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    // Standardize features (mean=0, std=1)
    let standardized_df = Preprocessing::standardize(df, &["numeric_feature"])?;
    println!("Standardized DataFrame:\n{}", standardized_df);

    // Normalize features (scale to 0-1 range)
    let normalized_df = Preprocessing::normalize(df, &["another_numeric_feature"])?;
    println!("Normalized DataFrame:\n{}", normalized_df);
    Ok(())
}
```