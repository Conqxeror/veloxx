# Veloxx Tutorial: A Quick Tour

This tutorial provides a quick tour of the Veloxx library, covering the basics of creating DataFrames, performing common data manipulation tasks, and using the library's more advanced features.

## Creating a DataFrame

You can create a DataFrame from a variety of sources, including CSV files and in-memory data.

### From a CSV file

```rust
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;

fn main() -> Result<(), VeloxxError> {
    let df = DataFrame::from_csv("my_data.csv")?;
    println!("{}", df);
    Ok(())
}
```

### From in-memory data

```rust
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::error::VeloxxError;
use std::collections::BTreeMap;

fn main() -> Result<(), VeloxxError> {
    let mut columns = BTreeMap::new();
    columns.insert("col1".to_string(), Series::new_i32("col1", vec![Some(1), Some(2), Some(3)]));
    columns.insert("col2".to_string(), Series::new_f64("col2", vec![Some(1.0), Some(2.0), Some(3.0)]));
    let df = DataFrame::new(columns)?;
    println!("{}", df);
    Ok(())
}
```

### From Vec of Vecs (Common for CSV parsing)

```rust
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;

fn main() -> Result<(), VeloxxError> {
    let data = vec![
        vec!["Alice".to_string(), "30".to_string(), "75000.0".to_string()],
        vec!["Bob".to_string(), "25".to_string(), "65000.0".to_string()],
        vec!["Charlie".to_string(), "35".to_string(), "85000.0".to_string()],
    ];
    let column_names = vec!["name".to_string(), "age".to_string(), "salary".to_string()];
    let df = DataFrame::from_vec_of_vec(data, column_names)?;
    println!("{}", df);
    Ok(())
}
```

## Data Manipulation

Veloxx provides a rich set of tools for manipulating DataFrames.

### Filtering

```rust
use veloxx::dataframe::DataFrame;
use veloxx::conditions::Condition;
use veloxx::types::Value;
use veloxx::error::VeloxxError;

fn main() -> Result<(), VeloxxError> {
    let df = DataFrame::from_csv("my_data.csv")?;
    
    // Simple condition: age > 25
    let condition = Condition::Gt("age".to_string(), Value::I32(25));
    let filtered_df = df.filter(&condition)?;
    println!("Filtered DataFrame:\n{}", filtered_df);
    
    // Complex condition: age > 25 AND salary < 80000
    let complex_condition = Condition::And(
        Box::new(Condition::Gt("age".to_string(), Value::I32(25))),
        Box::new(Condition::Lt("salary".to_string(), Value::F64(80000.0)))
    );
    let complex_filtered_df = df.filter(&complex_condition)?;
    println!("Complex filtered DataFrame:\n{}", complex_filtered_df);
    
    Ok(())
}
```

### Selecting and Manipulating Columns

```rust
use veloxx::dataframe::DataFrame;
use veloxx::expressions::Expr;
use veloxx::types::Value;
use veloxx::error::VeloxxError;

fn main() -> Result<(), VeloxxError> {
    let df = DataFrame::from_csv("my_data.csv")?;
    
    // Select specific columns
    let selected_df = df.select_columns(vec!["name".to_string(), "age".to_string()])?;
    println!("Selected columns:\n{}", selected_df);
    
    // Drop columns
    let dropped_df = df.drop_columns(vec!["unwanted_col".to_string()])?;
    println!("After dropping columns:\n{}", dropped_df);
    
    // Rename columns
    let renamed_df = df.rename_column("age", "years_old")?;
    println!("After renaming:\n{}", renamed_df);
    
    // Add new column with expression
    let age_plus_10 = Expr::Add(
        Box::new(Expr::Column("age".to_string())),
        Box::new(Expr::Literal(Value::I32(10)))
    );
    let with_new_col = df.with_column("age_plus_10", &age_plus_10)?;
    println!("With new column:\n{}", with_new_col);
    
    Ok(())
}
```

### Aggregation and Grouping

```rust
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;

fn main() -> Result<(), VeloxxError> {
    let df = DataFrame::from_csv("my_data.csv")?;
    
    // Series aggregations
    if let Some(age_series) = df.get_column("age") {
        println!("Average age: {:.1}", age_series.mean()?);
        println!("Max age: {}", age_series.max()?);
        println!("Min age: {}", age_series.min()?);
        println!("Sum of ages: {}", age_series.sum()?);
    }
    
    // Group by operations (assuming you have a categorical column like "department")
    let grouped_df = df.group_by(vec!["department".to_string()])?;
    let aggregated_df = grouped_df.agg(vec![
        ("salary", "mean"),
        ("age", "max"),
        ("name", "count")
    ])?;
    println!("Grouped and aggregated:\n{}", aggregated_df);
    
    Ok(())
}
```

### Sorting and Joining

```rust
use veloxx::dataframe::DataFrame;
use veloxx::dataframe::join::JoinType;
use veloxx::error::VeloxxError;

fn main() -> Result<(), VeloxxError> {
    let df1 = DataFrame::from_csv("employees.csv")?;
    let df2 = DataFrame::from_csv("departments.csv")?;
    
    // Sort by age (ascending)
    let sorted_df = df1.sort(vec!["age".to_string()], true)?;
    println!("Sorted by age:\n{}", sorted_df);
    
    // Sort by salary (descending)
    let sorted_desc = df1.sort(vec!["salary".to_string()], false)?;
    println!("Sorted by salary (desc):\n{}", sorted_desc);
    
    // Join operations
    let inner_joined = df1.join(&df2, "department_id", JoinType::Inner)?;
    println!("Inner joined:\n{}", inner_joined);
    
    let left_joined = df1.join(&df2, "department_id", JoinType::Left)?;
    println!("Left joined:\n{}", left_joined);
    
    Ok(())
}
```

## Advanced Features

### JSON Operations

```rust
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;

fn main() -> Result<(), VeloxxError> {
    // Read from JSON
    let df = DataFrame::from_json("data.json")?;
    println!("Loaded from JSON:\n{}", df);
    
    // Write to JSON
    df.to_json("output.json")?;
    println!("Saved to JSON file");
    
    Ok(())
}
```

### Data Quality Operations

```rust
#[cfg(feature = "data_quality")]
use veloxx::data_quality::{DataProfiler, AnomalyDetector};
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;

#[cfg(feature = "data_quality")]
fn main() -> Result<(), VeloxxError> {
    let df = DataFrame::from_csv("my_data.csv")?;
    
    // Profile your data
    let profiler = DataProfiler::new();
    let profile = profiler.profile_dataframe(&df)?;
    println!("Data profile: {:?}", profile);
    
    // Detect anomalies
    let detector = AnomalyDetector::new();
    let outliers = detector.detect_outliers(&df, "salary")?;
    println!("Found {} outliers in salary data", outliers.len());
    
    Ok(())
}

#[cfg(not(feature = "data_quality"))]
fn main() {
    println!("Enable 'data_quality' feature to run this example");
}
```

### Machine Learning

```rust
#[cfg(feature = "ml")]
use veloxx::ml::{LinearRegression, KMeans, LogisticRegression};
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;

#[cfg(feature = "ml")]
fn main() -> Result<(), VeloxxError> {
    let df = DataFrame::from_csv("my_data.csv")?;
    
    // Linear Regression
    let mut lr_model = LinearRegression::new();
    lr_model.fit(&df, "target", &["feature1", "feature2"])?;
    let predictions = lr_model.predict(&df, &["feature1", "feature2"])?;
    println!("Linear regression predictions: {:?}", predictions);
    
    // K-Means Clustering
    let mut kmeans = KMeans::new(3); // 3 clusters
    let clusters = kmeans.fit_predict(&df, &["feature1", "feature2"])?;
    println!("K-means clusters: {:?}", clusters);
    
    // Logistic Regression
    let mut log_reg = LogisticRegression::new();
    log_reg.fit(&df, "binary_target", &["feature1", "feature2"])?;
    let log_predictions = log_reg.predict(&df, &["feature1", "feature2"])?;
    println!("Logistic regression predictions: {:?}", log_predictions);
    
    Ok(())
}

#[cfg(not(feature = "ml"))]
fn main() {
    println!("Enable 'ml' feature to run this example");
}
```

### Visualization

```rust
#[cfg(feature = "visualization")]
use veloxx::visualization::{save_scatter_plot, save_histogram};
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;

#[cfg(feature = "visualization")]
fn main() -> Result<(), VeloxxError> {
    let df = DataFrame::from_csv("my_data.csv")?;
    
    // Create scatter plot
    save_scatter_plot(
        &df, 
        "feature1", 
        "feature2", 
        "scatter_plot.png", 
        "Feature Relationship", 
        "Feature 1", 
        "Feature 2"
    )?;
    println!("Scatter plot saved to scatter_plot.png");
    
    // Create histogram
    save_histogram(
        &df, 
        "salary", 
        "salary_histogram.png", 
        "Salary Distribution", 
        "Salary", 
        "Frequency"
    )?;
    println!("Histogram saved to salary_histogram.png");
    
    Ok(())
}

#[cfg(not(feature = "visualization"))]
fn main() {
    println!("Enable 'visualization' feature to run this example");
}
```

### Window Functions and Time Series

```rust
#[cfg(feature = "window_functions")]
use veloxx::window_functions::{WindowSpec, WindowFunction, RankingFunction};
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;

#[cfg(feature = "window_functions")]
fn main() -> Result<(), VeloxxError> {
    let df = DataFrame::from_csv("time_series_data.csv")?;
    
    // Create window specification
    let window_spec = WindowSpec::new()
        .partition_by(vec!["category".to_string()])
        .order_by(vec!["date".to_string()]);
    
    // Add row numbers
    let with_row_nums = WindowFunction::apply_ranking(
        &df, 
        &RankingFunction::RowNumber, 
        &window_spec
    )?;
    println!("With row numbers:\n{}", with_row_nums);
    
    // Add rankings based on sales
    let sales_window = WindowSpec::new()
        .order_by(vec!["sales".to_string()]);
    let with_ranks = WindowFunction::apply_ranking(
        &df, 
        &RankingFunction::Rank, 
        &sales_window
    )?;
    println!("With sales rankings:\n{}", with_ranks);
    
    Ok(())
}

#[cfg(not(feature = "window_functions"))]
fn main() {
    println!("Enable 'window_functions' feature to run this example");
}
```

## Error Handling

Veloxx uses a comprehensive error handling system:

```rust
use veloxx::dataframe::DataFrame;
use veloxx::error::VeloxxError;

fn main() {
    match DataFrame::from_csv("nonexistent.csv") {
        Ok(df) => println!("Loaded DataFrame: {}"),
        Err(VeloxxError::IoError(err)) => println!("File error: {}", err),
        Err(VeloxxError::ParseError(msg)) => println!("Parse error: {}", msg),
        Err(VeloxxError::ColumnNotFound(name)) => println!("Column '{}' not found", name),
        Err(VeloxxError::TypeMismatch(msg)) => println!("Type mismatch: {}", msg),
        Err(e) => println!("Other error: {}", e),
    }
}
```

## Best Practices

1. **Use feature flags** to include only the functionality you need
2. **Handle errors properly** using Veloxx's comprehensive error types
3. **Leverage type safety** by using strongly-typed operations
4. **Chain operations** for readable and efficient data processing
5. **Profile your data** before performing complex operations
6. **Use parallel operations** for large datasets when available

## Next Steps

- Explore the [API Guide](/docs/api/rust) for comprehensive documentation
- Check out the [examples directory](https://github.com/Conqxeror/veloxx/tree/main/examples) for working code samples
- Read the [Getting Started Guide](/docs/getting-started/installation) for installation and setup
- Visit the [online documentation](https://conqxeror.github.io/veloxx/) for the latest updates

Happy data processing with Veloxx! 🚀

```