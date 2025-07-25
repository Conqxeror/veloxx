# LuminarData Development Roadmap

This document outlines the development roadmap for the LuminarData library, a lightweight, high-performance data processing and analytics library for Rust.

## Phase 1: Core Functionality (MVP)

The focus of this phase is to build the minimum viable product (MVP) with the core features required for basic data manipulation and analysis.

- **[x] Core Data Structures:**
  - [x] `DataFrame` with columnar storage.
  - [x] `Series` with support for `i32`, `f64`, `bool`, `String`, and `DateTime`.
  - [x] Handling of missing values using `Option<T>`.

- **[x] Data Ingestion & Loading:**
  - [x] `from_vec_of_vec` for in-memory data.
  - [x] High-performance CSV reader.
  - [ ] **TODO:** High-performance JSON reader.

- **[x] Data Cleaning & Preparation:**
  - [x] `drop_nulls` to remove rows with null values.
  - [x] `fill_nulls` to fill nulls with a specific value.
  - [ ] **TODO:** `interpolate_nulls` for linear interpolation of numeric series.
  - [ ] **TODO:** `cast` for type casting between compatible `Series` types.
  - [x] `rename_column` to rename columns.

- **[x] Data Transformation & Manipulation:**
  - [x] `select_columns` and `drop_columns` for column selection.
  - [x] `filter` for row selection with logical and comparison operators.
  - [x] `with_column` to create new columns.
  - [x] `sort` by one or more columns.
  - [x] `join` with support for inner, left, right, and outer joins.
  - [x] `append` to concatenate `DataFrames`.

- **[ ] Aggregation & Reduction:**
  - [x] `sum`, `mean`, `median`, `min`, `max`, `count`, `std_dev`.
  - [ ] **TODO:** `group_by` with aggregations.
  - [x] `unique` to get unique values.

- **[x] Basic Analytics & Statistics:**
  - [x] `describe` for summary statistics.
  - [x] `correlation` and `covariance`.

- **[x] Output & Export:**
  - [x] `to_csv` to write to a CSV file.
  - [x] `to_parquet` to write to a Parquet file.
  - [x] `Display` for pretty-printing `DataFrames`.

## Phase 2: Advanced Features & Performance

This phase will focus on adding more advanced features and optimizing the library for performance.

- **[ ] Time-Series Functionality:**
  - [ ] `resample` to change the frequency of time-series data.
  - [ ] `rolling` for rolling window calculations.

- **[ ] Machine Learning:**
  - [x] `LinearRegression`.
  - [x] `KMeans`.
  - [x] `LogisticRegression`.
  - [ ] **TODO:** Add more models (e.g., decision trees, SVM).

- **[ ] Visualization:**
  - [x] `save_histogram`.
  - [x] `save_scatter_plot`.
  - [ ] **TODO:** Add more plot types (e.g., line, bar, box).

- **[ ] Performance Optimizations:**
  - [ ] SIMD-accelerated operations for numeric `Series`.
  - [ ] Parallel execution for more operations using `rayon`.

## Phase 3: Ecosystem & Extensibility

This phase will focus on making the library more extensible and integrating it with the broader Rust ecosystem.

- **[ ] Streaming Data:**
  - [ ] Support for processing data in a streaming fashion.

- **[ ] Foreign Function Interface (FFI):**
  - [ ] C API for integration with other languages.
  - [ ] Python bindings.

- **[ ] Persistence:**
  - [ ] Custom binary format for fast serialization/deserialization.

- **[ ] Extensibility:**
  - [ ] Traits for custom data sources and sinks.
  - [ ] Plugin system for adding new functionality.

## Phase 4: Advanced Analytics & Query Engine

This phase focuses on advanced analytics capabilities and query optimization.

- **[ ] Advanced Statistical Functions:**
  - [ ] **TODO:** Statistical hypothesis testing (t-test, chi-square, ANOVA).
  - [ ] **TODO:** Quantile calculations and percentiles.
  - [ ] **TODO:** Correlation matrix operations.
  - [ ] **TODO:** Principal Component Analysis (PCA).
  - [ ] **TODO:** Time series decomposition (trend, seasonal, residual).

- **[ ] Enhanced Time Series Analysis:**
  - [ ] **TODO:** Advanced resampling with multiple frequencies.
  - [ ] **TODO:** Lag/lead operations for time series.
  - [ ] **TODO:** Seasonal decomposition and forecasting.
  - [ ] **TODO:** Time series anomaly detection.
  - [ ] **TODO:** Auto-correlation and cross-correlation functions.

- **[ ] Query Engine & SQL Interface:**
  - [ ] **TODO:** Basic SQL parser (SELECT, WHERE, GROUP BY, ORDER BY).
  - [ ] **TODO:** Enhanced expression engine with complex predicates.
  - [ ] **TODO:** Column expressions and computed columns.
  - [ ] **TODO:** Subquery support.
  - [ ] **TODO:** Common Table Expressions (CTEs).

- **[ ] Advanced Join Operations:**
  - [ ] **TODO:** Cross joins implementation.
  - [ ] **TODO:** Anti-joins and semi-joins.
  - [ ] **TODO:** Join optimization strategies.
  - [ ] **TODO:** Broadcast joins for small tables.
  - [ ] **TODO:** Hash join algorithms.

- **[ ] Enhanced Window Functions:**
  - [x] **COMPLETED:** Basic moving averages.
  - [ ] **TODO:** Ranking functions (row_number, rank, dense_rank).
  - [ ] **TODO:** Lead/lag functions for time series.
  - [ ] **TODO:** Cumulative operations (cumsum, cummax, cummin).
  - [ ] **TODO:** Percentile window functions.

## Phase 5: Performance & Scalability

This phase focuses on high-performance computing and scalability improvements.

- **[ ] Advanced Performance Optimizations:**
  - [ ] **TODO:** SIMD vectorization for arithmetic operations.
  - [ ] **TODO:** CPU cache-friendly data layouts.
  - [ ] **TODO:** Lazy evaluation and query optimization.
  - [ ] **TODO:** Parallel query execution planning.
  - [ ] **TODO:** Memory-mapped file operations.

- **[ ] Distributed Computing Enhancements:**
  - [x] **COMPLETED:** Basic parallel processing with rayon.
  - [ ] **TODO:** Distributed DataFrame operations.
  - [ ] **TODO:** Cluster computing support.
  - [ ] **TODO:** Data partitioning strategies.
  - [ ] **TODO:** Network-based data exchange.

- **[ ] Memory Management:**
  - [ ] **TODO:** Advanced memory pooling.
  - [ ] **TODO:** Garbage collection optimization.
  - [ ] **TODO:** Memory usage profiling tools.
  - [ ] **TODO:** Out-of-core processing for large datasets.

## Phase 6: Developer Experience & Ecosystem

This phase focuses on improving developer experience and ecosystem integration.

- **[ ] Enhanced Error Handling:**
  - [ ] **TODO:** More descriptive error messages with suggestions.
  - [ ] **TODO:** Error context and stack traces.
  - [ ] **TODO:** Error recovery mechanisms.
  - [ ] **TODO:** Debugging utilities and profiling tools.

- **[ ] Comprehensive Benchmarking:**
  - [ ] **TODO:** Performance benchmarks against pandas and polars.
  - [ ] **TODO:** Memory usage benchmarking.
  - [ ] **TODO:** Performance regression testing.
  - [ ] **TODO:** Automated performance monitoring.

- **[ ] Advanced Data Quality:**
  - [x] **COMPLETED:** Basic data profiling and outlier detection.
  - [ ] **TODO:** Advanced data validation rules.
  - [ ] **TODO:** Data lineage tracking.
  - [ ] **TODO:** Schema evolution support.
  - [ ] **TODO:** Data quality metrics and reporting.

- **[ ] Enhanced Arrow Integration:**
  - [ ] **TODO:** Zero-copy data exchange with Arrow.
  - [ ] **TODO:** Arrow Flight protocol support.
  - [ ] **TODO:** Integration with Arrow-based tools.
  - [ ] **TODO:** Arrow compute kernel utilization.

## Future Considerations

- **[ ] SQL Interface:** A SQL interface for querying `DataFrames`.
- **[ ] Distributed Computing:** Support for distributed `DataFrames` using a framework like `timely-dataflow`.
- **[ ] GPU Acceleration:** Support for GPU-accelerated operations using a framework like `faer`.
## Status Update (Current Implementation)

### ✅ Recently Completed Features:
- **JSON Support**: High-performance JSON reader and writer implemented
- **Data Interpolation**: `interpolate_nulls()` method for linear interpolation
- **Type Casting**: `cast()` method for Series type conversions
- **Group By Operations**: Full `group_by()` with aggregations support
- **Time Series**: Rolling window operations (mean, sum, min, max, std)
- **Advanced I/O**: Parquet support, async operations, streaming capabilities
- **Machine Learning**: Linear regression, K-means, logistic regression, normalization
- **Data Quality**: Outlier detection, duplicate detection, data profiling
- **Visualization**: Histogram and scatter plot generation
- **Window Functions**: Moving averages and analytical functions
- **Distributed Computing**: Parallel processing and memory-mapped operations
- **Python Bindings**: PyO3 integration for Python interoperability
- **WebAssembly**: WASM bindings for web applications

### 🎯 Priority Next Steps:
1. **SQL Interface**: Basic SQL parser for DataFrame queries
2. **Advanced SIMD**: Vectorized arithmetic operations
3. **Enhanced Analytics**: Statistical tests, PCA, quantile calculations
4. **Performance Benchmarking**: Comprehensive performance testing suite
5. **Advanced Joins**: Cross joins, anti-joins, join optimization
