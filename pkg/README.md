# <img src="docs/veloxx_logo.png" alt="Veloxx Logo" height="70px"> Veloxx: Lightweight Rust-Powered Data Processing & Analytics Library

[![crates.io](https://img.shields.io/crates/v/veloxx.svg)](https://crates.io/crates/veloxx)

> **New in 0.2.1:** Major performance improvements across all core operations. See CHANGELOG for details.

Veloxx is a new Rust library designed for highly performant and **extremely lightweight** in-memory data processing and analytics. It prioritizes minimal dependencies, optimal memory footprint, and compile-time guarantees, making it an ideal choice for resource-constrained environments, high-performance computing, and applications where every byte and cycle counts.

## Core Principles & Design Goals

- **Extreme Lightweighting:** Strives for zero or very few, carefully selected external crates. Focuses on minimal overhead and small binary size.
- **Performance First:** Leverages Rust's zero-cost abstractions, with potential for SIMD and parallelism. Data structures are optimized for cache efficiency.
- **Safety & Reliability:** Fully utilizes Rust's ownership and borrowing system to ensure memory safety and prevent common data manipulation errors. Unsafe code is minimized and thoroughly audited.
- **Ergonomics & Idiomatic Rust API:** Designed for a clean, discoverable, and user-friendly API that feels natural to Rust developers, supporting method chaining and strong static typing.
- **Composability & Extensibility:** Features a modular design, allowing components to be independent and easily combinable, and is built to be easily extendable.

## Key Features

### Core Data Structures

- **DataFrame:** A columnar data store supporting heterogeneous data types per column (i32, f64, bool, String, DateTime). Efficient storage and handling of missing values.
- **Series (or Column):** A single-typed, named column of data within a DataFrame, providing type-specific operations.

### Data Ingestion & Loading

- **From `Vec<Vec<T>>` / Iterator:** Basic in-memory construction from Rust native collections.
- **CSV Support:** Minimalistic, highly efficient CSV parser for loading data.
- **JSON Support:** Efficient parsing for common JSON structures.
- **Custom Data Sources:** Traits/interfaces for users to implement their own data loading mechanisms.

### Data Cleaning & Preparation

- `drop_nulls()`: Remove rows with any null values.
- `fill_nulls(value)`: Fill nulls with a specified value (type-aware, including DateTime).
- `interpolate_nulls()`: Basic linear interpolation for numeric and DateTime series.
- **Type Casting:** Efficient conversion between compatible data types for Series (e.g., i32 to f64).
- `rename_column(old_name, new_name)`: Rename columns.

### Data Transformation & Manipulation

- **Selection:** `select_columns(names)`, `drop_columns(names)`.
- **Filtering:** Predicate-based row selection using logical (`AND`, `OR`, `NOT`) and comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`).
- **Projection:** `with_column(new_name, expression)`, `apply()` for user-defined functions.
- **Sorting:** Sort DataFrame by one or more columns (ascending/descending).
- **Joining:** Basic inner, left, and right join operations on common keys.
- **Concatenation/Append:** Combine DataFrames vertically.

### Aggregation & Reduction

- **Simple Aggregations:** `sum()`, `mean()`, `median()`, `min()`, `max()`, `count()`, `std_dev()`.
- **Group By:** Perform aggregations on groups defined by one or more columns.
- **Unique Values:** `unique()` for a Series or DataFrame columns.

### Basic Analytics & Statistics

- `describe()`: Provides summary statistics for numeric columns (count, mean, std, min, max, quartiles).
- `correlation()`: Calculate Pearson correlation between two numeric Series.
- `covariance()`: Calculate covariance.

### Output & Export

- **To `Vec<Vec<T>>`:** Export DataFrame content back to standard Rust collections.
- **To CSV:** Efficiently write DataFrame to a CSV file.
- **Display/Pretty Print:** User-friendly console output for DataFrame and Series.

## Installation

You can install the WebAssembly package using `npm` after building it with `wasm-pack`:

```bash
# First, build the WebAssembly package (from the project root)
wasm-pack build --target web --out-dir pkg

# Then install the package
npm install ./pkg
```

## Usage Example

Here's a quick example demonstrating how to create a DataFrame, filter it, and perform a group-by aggregation:

```javascript
const veloxx = require('veloxx');

async function runWasmExample() {
    // 1. Create a DataFrame
    const df = new veloxx.WasmDataFrame({
        name: ['Alice', 'Bob', 'Charlie', 'David'],
        age: [25, 30, 22, 35],
        city: ['New York', 'London', 'New York', 'Paris'],
    });
    console.log("Original DataFrame:");
    console.log(df);

    // 2. Filter data: age > 25
    const ageSeries = df.getColumn("age");
    const filteredIndices = [];
    for (let i = 0; i < ageSeries.len; i++) {
        if (ageSeries.getValue(i) > 25) {
            filteredIndices.push(i);
        }
    }
    const filteredDf = df.filter(new Uint32Array(filteredIndices));
    console.log("
Filtered DataFrame (age > 25):");
    console.log(filteredDf);

    // 3. Series operations
    console.log(`
Age Series Sum: ${ageSeries.sum()}`);
    console.log(`Age Series Mean: ${ageSeries.mean()}`);
    console.log(`Age Series Unique: ${ageSeries.unique().toVecF64()}`);
}

runWasmExample();

```

## Non-Functional Requirements

- **Comprehensive Documentation:** Extensive `///` documentation for all public APIs, examples, and design choices.
- **Robust Testing:** Thorough unit and integration tests covering all functionalities and edge cases.
- **Performance Benchmarking:** Includes benchmarks to track performance and memory usage, ensuring lightweight and high-performance goals are met.
- **Cross-Platform Compatibility:** Designed to work on common operating systems (Linux, macOS, Windows).
- **Safety:** Upholds Rust's safety guarantees, with minimal and heavily justified `unsafe` code.

## Future Considerations / Roadmap

- **Streaming Data:** Support for processing data in a streaming fashion.
- **Time-Series Functionality:** Basic time-series resampling, rolling windows.
- **FFI (Foreign Function Interface):** Consider C API for integration with other languages (Python, JavaScript).
- **Simple Plotting Integration:** Provide hooks or basic data preparation for common plotting libraries.
- **Persistence:** Basic serialization/deserialization formats (e.g., custom binary format, Parquet subset).

## WebAssembly Testing

WebAssembly bindings are currently tested using `console.assert` in `test_wasm.js`. Future work includes migrating to a more robust JavaScript testing framework like Jest.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
