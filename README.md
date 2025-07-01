# Velox: Lightweight Rust-Powered Data Processing & Analytics Library

Velox is a new Rust library designed for highly performant and **extremely lightweight** in-memory data processing and analytics. It prioritizes minimal dependencies, optimal memory footprint, and compile-time guarantees, making it an ideal choice for resource-constrained environments, high-performance computing, and applications where every byte and cycle counts.

## Core Principles & Design Goals

*   **Extreme Lightweighting:** Strives for zero or very few, carefully selected external crates. Focuses on minimal overhead and small binary size.
*   **Performance First:** Leverages Rust's zero-cost abstractions, with potential for SIMD and parallelism. Data structures are optimized for cache efficiency.
*   **Safety & Reliability:** Fully utilizes Rust's ownership and borrowing system to ensure memory safety and prevent common data manipulation errors. Unsafe code is minimized and thoroughly audited.
*   **Ergonomics & Idiomatic Rust API:** Designed for a clean, discoverable, and user-friendly API that feels natural to Rust developers, supporting method chaining and strong static typing.
*   **Composability & Extensibility:** Features a modular design, allowing components to be independent and easily combinable, and is built to be easily extendable.

## Key Features

### Core Data Structures
*   **DataFrame:** A columnar data store supporting heterogeneous data types per column (i32, f64, bool, String). Efficient storage and handling of missing values.
*   **Series (or Column):** A single-typed, named column of data within a DataFrame, providing type-specific operations.

### Data Ingestion & Loading
*   **From `Vec<Vec<T>>` / Iterator:** Basic in-memory construction from Rust native collections.
*   **CSV Support:** Minimalistic, highly efficient CSV parser for loading data.
*   **JSON Support:** Efficient parsing for common JSON structures (planned).
*   **Custom Data Sources:** Traits/interfaces for users to implement their own data loading mechanisms.

### Data Cleaning & Preparation
*   `drop_nulls()`: Remove rows with any null values.
*   `fill_nulls(value)`: Fill nulls with a specified value (type-aware).
*   `interpolate_nulls()`: Basic linear interpolation for numeric series.
*   **Type Casting:** Efficient conversion between compatible data types for Series (e.g., i32 to f64).
*   `rename_column(old_name, new_name)`: Rename columns.

### Data Transformation & Manipulation
*   **Selection:** `select_columns(names)`, `drop_columns(names)`.
*   **Filtering:** Predicate-based row selection using logical (`AND`, `OR`, `NOT`) and comparison operators (`==`, `!=`, `<`, `>`).
*   **Projection:** `with_column(new_name, expression)`, `apply()` for user-defined functions.
*   **Sorting:** Sort DataFrame by one or more columns (ascending/descending).
*   **Joining:** Basic inner, left, and right join operations on common keys.
*   **Concatenation/Append:** Combine DataFrames vertically.

### Aggregation & Reduction
*   **Simple Aggregations:** `sum()`, `mean()`, `median()`, `min()`, `max()`, `count()`, `std_dev()`.
*   **Group By:** Perform aggregations on groups defined by one or more columns.
*   **Unique Values:** `unique()` for a Series or DataFrame columns.

### Basic Analytics & Statistics
*   `describe()`: Provides summary statistics for numeric columns (count, mean, std, min, max, quartiles).
*   `correlation()`: Calculate Pearson correlation between two numeric Series.
*   `covariance()`: Calculate covariance.

### Output & Export
*   **To `Vec<Vec<T>>`:** Export DataFrame content back to standard Rust collections.
*   **To CSV:** Efficiently write DataFrame to a CSV file.
*   **Display/Pretty Print:** User-friendly console output for DataFrame and Series.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
velox = "0.1.0" # Or the latest version
```

## Usage Example

Here's a quick example demonstrating how to create a DataFrame, filter it, and perform a group-by aggregation:

```rust
use velox::dataframe::DataFrame;
use velox::series::Series;
use velox::types::{Value, DataType};
use velox::conditions::Condition;
use std::collections::HashMap;

fn main() -> Result<(), String> {
    // 1. Create a DataFrame
    let mut columns = HashMap::new();
    columns.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string()), Some("David".to_string())]));
    columns.insert("age".to_string(), Series::new_i32("age", vec![Some(25), Some(30), Some(22), Some(35)]));
    columns.insert("city".to_string(), Series::new_string("city", vec![Some("New York".to_string()), Some("London".to_string()), Some("New York".to_string()), Some("Paris".to_string())]));

    let df = DataFrame::new(columns)?;
    println!("Original DataFrame:
{}", df);

    // 2. Filter data: age > 25
    let condition = Condition::Gt("age".to_string(), Value::I32(25));
    let filtered_df = df.filter(&condition)?;
    println!("
Filtered DataFrame (age > 25):
{}", filtered_df);

    // 3. Group by city and calculate average age
    let grouped_df = df.group_by(vec!["city".to_string()])?;
    let aggregated_df = grouped_df.agg(vec![("age", "mean")])?;
    println!("
Aggregated DataFrame (average age by city):
{}", aggregated_df);

    Ok(())
}
```

## Non-Functional Requirements

*   **Comprehensive Documentation:** Extensive `///` documentation for all public APIs, examples, and design choices.
*   **Robust Testing:** Thorough unit and integration tests covering all functionalities and edge cases.
*   **Performance Benchmarking:** Includes benchmarks to track performance and memory usage, ensuring lightweight and high-performance goals are met.
*   **Cross-Platform Compatibility:** Designed to work on common operating systems (Linux, macOS, Windows).
*   **Safety:** Upholds Rust's safety guarantees, with minimal and heavily justified `unsafe` code.

## Future Considerations / Roadmap

*   **Streaming Data:** Support for processing data in a streaming fashion.
*   **Time-Series Functionality:** Basic time-series resampling, rolling windows.
*   **FFI (Foreign Function Interface):** Consider C API for integration with other languages (Python, JavaScript).
*   **Simple Plotting Integration:** Provide hooks or basic data preparation for common plotting libraries.
*   **Persistence:** Basic serialization/deserialization formats (e.g., custom binary format, Parquet subset).

## Current Development Status

All compilation and test errors have been resolved, except for JSON parsing. We are currently investigating issues with the `microjson` crate for JSON parsing and may explore alternative solutions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
