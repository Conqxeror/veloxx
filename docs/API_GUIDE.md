# Velox API Guide

This comprehensive guide covers all Velox features and their APIs, organized by functionality.

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

The `DataFrame` is the primary data structure in Velox, representing a columnar data table with heterogeneous data types.

#### Creation

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
```

#### Core Methods

- `row_count()` - Get number of rows
- `column_count()` - Get number of columns
- `column_names()` - Get all column names
- `get_column(name)` - Get a specific column
- `head(n)` - Get first n rows
- `tail(n)` - Get last n rows

### Series

The `Series` represents a single column of data with a specific type.

#### Creation

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

- `len()` - Get length of series
- `data_type()` - Get the data type
- `get_value(index)` - Get value at index
- `is_null(index)` - Check if value is null
- `null_count()` - Count null values

## Basic Operations

### Filtering

```rust
use veloxx::conditions::Condition;
use veloxx::types::Value;

// Simple conditions
let condition = Condition::Gt("age".to_string(), Value::I32(25));
let filtered_df = df.filter(&condition)?;

// Complex conditions
let complex_condition = Condition::And(
    Box::new(Condition::Gt("age".to_string(), Value::I32(25))),
    Box::new(Condition::Eq("city".to_string(), Value::String("New York".to_string())))
);
let filtered_df = df.filter(&complex_condition)?;
```

### Column Operations

```rust
// Select specific columns
let selected_df = df.select_columns(vec!["name".to_string(), "age".to_string()])?;

// Drop columns
let dropped_df = df.drop_columns(vec!["unwanted_col".to_string()])?;

// Rename columns
let renamed_df = df.rename_column("old_name", "new_name")?;

// Add new columns with expressions
use veloxx::expressions::Expr;
let expr = Expr::Add(
    Box::new(Expr::Column("age".to_string())),
    Box::new(Expr::Literal(Value::I32(10)))
);
let df_with_new_col = df.with_column("age_plus_10", &expr)?;
```

### Aggregation and Grouping

```rust
// Group by operations
let grouped_df = df.group_by(vec!["city".to_string()])?;
let aggregated_df = grouped_df.agg(vec![
    ("age", "mean"),
    ("salary", "sum"),
    ("name", "count")
])?;

// Series aggregations
let age_series = df.get_column("age").unwrap();
let mean_age = age_series.mean()?;
let sum_age = age_series.sum()?;
let max_age = age_series.max()?;
let min_age = age_series.min()?;
```

### Sorting and Joining

```rust
// Sort by columns
let sorted_df = df.sort(vec!["age".to_string()], true)?; // ascending
let sorted_desc_df = df.sort(vec!["age".to_string()], false)?; // descending

// Join operations
let joined_df = df1.join(&df2, &["id".to_string()], "inner")?;
let left_joined_df = df1.join(&df2, &["id".to_string()], "left")?;
```

## Advanced I/O Operations

*Available with `advanced_io` feature flag*

### Parquet Operations

```rust
use veloxx::advanced_io::{ParquetReader, ParquetWriter, CompressionType};

// Writing to Parquet
let writer = ParquetWriter::new();
writer.write_dataframe(&df, "data.parquet").await?;
writer.write_dataframe_compressed(&df, "data_compressed.parquet", CompressionType::Snappy).await?;

// Reading from Parquet
let reader = ParquetReader::new();
let loaded_df = reader.read_dataframe("data.parquet").await?;

// Streaming read for large files
let streaming_dfs = reader.read_dataframe_streaming("large_data.parquet", 1000).await?;
```

### Database Connectivity

```rust
use veloxx::advanced_io::DatabaseConnector;

// Connect to different databases
let sqlite_connector = DatabaseConnector::new("sqlite://database.db");
let postgres_connector = DatabaseConnector::new("postgresql://user:password@localhost/db");
let mysql_connector = DatabaseConnector::new("mysql://user:password@localhost/db");

// Create table from DataFrame schema
connector.create_table_from_dataframe(&df, "my_table").await?;

// Insert DataFrame data
connector.insert_dataframe(&df, "my_table").await?;

// Query data
let result_df = connector.query("SELECT * FROM my_table WHERE age > 25").await?;
```

### JSON Streaming

```rust
use veloxx::advanced_io::JsonStreamer;

let streamer = JsonStreamer::new();

// Stream from string
let json_data = r#"[{"id": 1, "name": "Alice"}]"#;
let dataframes = streamer.stream_from_string(json_data, 100).await?;

// Stream from file
let file_dataframes = streamer.stream_from_file("data.json", 100).await?;
```

### Async File Operations

```rust
use veloxx::advanced_io::AsyncFileOps;

// Async CSV operations
AsyncFileOps::write_csv_async(&df, "output.csv").await?;
let loaded_df = AsyncFileOps::read_csv_async("input.csv").await?;

// Async JSON operations
AsyncFileOps::write_json_async(&df, "output.json").await?;
let json_df = AsyncFileOps::read_json_async("input.json").await?;
```

## Data Quality & Validation

*Available with `data_quality` feature flag*

### Schema Validation

```rust
use veloxx::data_quality::{Schema, ColumnSchema, Constraint, SchemaValidator};
use veloxx::types::DataType;

// Define schema
let mut schema = Schema { columns: BTreeMap::new() };
schema.columns.insert("age".to_string(), ColumnSchema {
    name: "age".to_string(),
    data_type: DataType::I32,
    nullable: false,
    constraints: vec![
        Constraint::MinValue(Value::I32(0)),
        Constraint::MaxValue(Value::I32(120)),
    ],
});

// Validate DataFrame against schema
let validator = SchemaValidator::new();
let validation_result = validator.validate_dataframe(&df, &schema)?;
```

### Data Profiling

```rust
use veloxx::data_quality::DataProfiler;

let profiler = DataProfiler::new();
let profile = profiler.profile_dataframe(&df)?;

// Access profile information
println!("Null percentage: {}", profile.null_percentage);
println!("Data types: {:?}", profile.column_types);
println!("Unique value counts: {:?}", profile.unique_counts);
```

### Anomaly Detection

```rust
use veloxx::data_quality::AnomalyDetector;

let detector = AnomalyDetector::new();

// Detect outliers using IQR method
let outliers = detector.detect_outliers(&df, "sales")?;

// Detect anomalies using Z-score
let z_score_anomalies = detector.detect_anomalies_zscore(&df, "sales", 2.0)?;

// Statistical anomaly detection
let statistical_anomalies = detector.detect_statistical_anomalies(&df, "sales")?;
```

### Data Validation

```rust
use veloxx::data_quality::DataValidator;

let validator = DataValidator::new();

// Validate email format
let email_validation = validator.validate_email_format(&df, "email")?;

// Validate phone numbers
let phone_validation = validator.validate_phone_format(&df, "phone")?;

// Custom pattern validation
let pattern_validation = validator.validate_pattern(&df, "product_code", r"^[A-Z]{2}\d{4}$")?;
```

## Window Functions & Analytics

*Available with `window_functions` feature flag*

### Window Specifications

```rust
use veloxx::window_functions::{WindowSpec, WindowFrame, FrameBound};

// Create window specification
let window_spec = WindowSpec::new()
    .partition_by(vec!["region".to_string()])
    .order_by(vec!["sales".to_string()])
    .frame(WindowFrame::new(
        FrameBound::UnboundedPreceding,
        FrameBound::CurrentRow
    ));
```

### Ranking Functions

```rust
use veloxx::window_functions::{WindowFunction, RankingFunction};

// Row number
let row_number_df = WindowFunction::apply_ranking(&df, &RankingFunction::RowNumber, &window_spec)?;

// Rank and dense rank
let rank_df = WindowFunction::apply_ranking(&df, &RankingFunction::Rank, &window_spec)?;
let dense_rank_df = WindowFunction::apply_ranking(&df, &RankingFunction::DenseRank, &window_spec)?;

// Percentile rank
let percentile_rank_df = WindowFunction::apply_ranking(&df, &RankingFunction::PercentRank, &window_spec)?;
```

### Lag and Lead Functions

```rust
use veloxx::window_functions::OffsetFunction;

// Lag function (previous value)
let lag_df = WindowFunction::apply_offset(&df, &OffsetFunction::Lag(1), "sales", &window_spec)?;

// Lead function (next value)
let lead_df = WindowFunction::apply_offset(&df, &OffsetFunction::Lead(1), "sales", &window_spec)?;

// First and last value in window
let first_value_df = WindowFunction::apply_offset(&df, &OffsetFunction::FirstValue, "sales", &window_spec)?;
let last_value_df = WindowFunction::apply_offset(&df, &OffsetFunction::LastValue, "sales", &window_spec)?;
```

### Moving Averages and Rolling Statistics

```rust
use veloxx::window_functions::AggregateFunction;

// Moving average
let moving_avg_df = WindowFunction::apply_aggregate(&df, &AggregateFunction::Mean, "sales", &window_spec)?;

// Rolling sum
let rolling_sum_df = WindowFunction::apply_aggregate(&df, &AggregateFunction::Sum, "sales", &window_spec)?;

// Rolling standard deviation
let rolling_std_df = WindowFunction::apply_aggregate(&df, &AggregateFunction::StdDev, "sales", &window_spec)?;
```

### Time-based Windows

```rust
use veloxx::window_functions::TimeWindow;
use chrono::Duration;

// Time-based window specification
let time_window = TimeWindow::new(Duration::days(7)); // 7-day window

// Apply time-based aggregation
let time_agg_df = WindowFunction::apply_time_aggregate(&df, &AggregateFunction::Mean, "sales", "timestamp", &time_window)?;
```

## Performance Optimization

### Parallel Operations

```rust
use veloxx::performance::ParallelOps;

// Parallel filtering
let parallel_filtered = ParallelOps::parallel_filter(&df, &condition)?;

// Parallel aggregation
let parallel_agg = ParallelOps::parallel_group_by(&df, vec!["category".to_string()])?;

// Parallel sorting
let parallel_sorted = ParallelOps::parallel_sort(&df, vec!["sales".to_string()], true)?;
```

### Memory Optimization

```rust
use veloxx::performance::MemoryOps;

// Optimize memory usage
let optimized_df = MemoryOps::optimize_memory(&df)?;

// Compress DataFrame
let compressed_df = MemoryOps::compress_dataframe(&df)?;

// Memory usage statistics
let memory_stats = MemoryOps::memory_usage(&df)?;
```

### SIMD Operations

```rust
use veloxx::performance::SimdOps;

// SIMD-accelerated operations (when available)
let simd_sum = SimdOps::simd_sum(&numeric_series)?;
let simd_mean = SimdOps::simd_mean(&numeric_series)?;
```

## Error Handling

All Velox operations return `Result<T, VeloxxError>`. Common error types:

```rust
use veloxx::error::VeloxxError;

match result {
    Ok(dataframe) => println!("Success: {}", dataframe),
    Err(VeloxxError::ColumnNotFound(name)) => println!("Column '{}' not found", name),
    Err(VeloxxError::TypeMismatch(msg)) => println!("Type mismatch: {}", msg),
    Err(VeloxxError::InvalidOperation(msg)) => println!("Invalid operation: {}", msg),
    Err(VeloxxError::IoError(err)) => println!("I/O error: {}", err),
    Err(VeloxxError::ParseError(msg)) => println!("Parse error: {}", msg),
}
```

## Best Practices

1. **Feature Flags**: Only enable features you need to minimize dependencies
2. **Error Handling**: Always handle `VeloxxError` appropriately
3. **Memory Management**: Use streaming operations for large datasets
4. **Performance**: Leverage parallel operations for compute-intensive tasks
5. **Type Safety**: Prefer strongly-typed operations over generic ones

## Examples Directory

See the `examples/` directory for complete working examples:

- `basic_dataframe_operations.rs` - Core DataFrame operations
- `advanced_io.rs` - Advanced I/O with Parquet, databases, and async operations
- `data_quality_validation.rs` - Data quality and validation examples
- `window_functions_analytics.rs` - Window functions and advanced analytics
- `performance_optimization.rs` - Performance optimization techniques
- `machine_learning.rs` - ML integration examples
- `data_visualization.rs` - Visualization examples