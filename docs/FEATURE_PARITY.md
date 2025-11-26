# Feature Parity Matrix: Veloxx vs. Polars

This document tracks the feature parity between Veloxx and its primary competitor, Polars.

| Category | Feature | Veloxx Implementation | Polars Implementation | Gap / Status |
|----------|---------|-----------------------|-----------------------|--------------|
| **Core Structure** | Column Storage | `IndexMap<String, Series>` (Ordered) | `Vec<Series>` (Ordered) | ✅ **Parity**: Deterministic column order now guaranteed. |
| **Joins** | Inner Join | ✅ Supported (Parallelized) | ✅ Supported | Parity (Single Column) |
| | Left/Right Join | ✅ Supported | ✅ Supported | Parity (Single Column) |
| | Outer Join | ✅ Supported | ✅ Supported | ✅ **Completed** |
| | Cross Join | ❌ Missing | ✅ Supported | Low Priority |
| | Multi-key Join | ❌ Missing | ✅ Supported | Medium Priority |
| | Asof Join | ❌ Missing | ✅ Supported | Future |
| **GroupBy** | GroupBy | ✅ Multi-column | ✅ Multi-column | Parity |
| | Aggregations | `sum`, `mean`, `min`, `max`, `count` | Extensive (inc. `std`, `var`, `quantile`, etc.) | **Medium Priority**: Missing statistical aggs. |
| | Pivot | ✅ Supported | ✅ Supported | ✅ **Completed** |
| **Reshaping** | Melt | ❌ Missing | ✅ Supported | High Priority |
| | Explode | ❌ Missing | ✅ Supported | Medium Priority |
| **Lazy Evaluation** | Query Optimizer | ✅ Predicate Pushdown | Mature (Predicate/Projection Pushdown, CSE) | ✅ **Improved**: Basic optimizer active. |
| **I/O** | CSV | ✅ Supported (Memmap/Parallel) | ✅ Supported (Multi-threaded) | Parity |
| | Parquet | ✅ Supported (Zero-copy Arrow) | ✅ Supported | Parity |
| | JSON | ✅ Supported | ✅ Supported | Parity |
| **Ergonomics** | Expression API | ✅ Partial (`Expr` enum) | ✅ Extensive DSL | Gap: Polars DSL is more expressive. |

## Summary of Recent Improvements

1.  **Deterministic Column Order**: Switched to `IndexMap` to ensure predictable API behavior.
2.  **Pivot Support**: Implemented `Pivot` trait for reshaping DataFrames.
3.  **Full Outer Join**: Added `JoinType::Outer` with parallel execution.
4.  **Performance**: Massive SIMD speedups for numerical aggregations and optimized parallel execution strategies.