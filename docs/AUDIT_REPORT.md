# Phase 1: Audit Report

## 1. API Ergonomics Review

### Critical Finding: Non-Deterministic Column Order
**Location:** `src/dataframe/mod.rs`
```rust
pub struct DataFrame {
    pub columns: HashMap<String, Series>,
}
```
**Issue:** Using `HashMap` for column storage means the order of columns is undefined and unstable. Operations like `select`, `print`, or `head` will display columns in random order.
**Recommendation:** Refactor `DataFrame` to use `IndexMap` or a split structure (`Vec<String>` for names, `HashMap` or `Vec<Series>` for data) to preserve insertion/creation order. This is standard behavior for DataFrames (Pandas, Polars, R).

### Traits and Interfaces
*   **Iterators:** Need to verify if `DataFrame` implements `IntoIterator` for iterating over rows or columns.
*   **Display:** `Debug` is derived, but a custom `Display` implementation showing a formatted table is essential for a DataFrame library.

## 2. Dependency Audit (`Cargo.toml`)

### Heavyweight Dependencies
*   **`polars`**: Currently listed as an optional dependency.
    *   *Risk:* If this is used in core logic, we aren't a competitor, we're a wrapper.
    *   *Action:* Verify usage. If used only for benchmarks/tests, move to `[dev-dependencies]`.
*   **`sqlx`**: Heavy async runtime (Tokio) requirement.
    *   *Status:* Optional (`advanced_io`). Good.
*   **`linfa`**: Machine Learning crate.
    *   *Status:* Optional (`ml`). Good.

### Potential Optimizations
*   **`ahash` / `fxhash`**: Used for hashing. Ensure we are consistent. `fxhash` is fast but not HashDoS resistant. Acceptable for internal analytics if not exposing public endpoints processing untrusted keys, but `ahash` is generally safer/fast enough.
*   **`num-traits`**: Essential for generic numeric code. Keep.

## 3. Immediate Action Plan (Refined)

1.  **Refactor `DataFrame` Struct**: Switch storage to preserve order.
2.  **Implement `pivot`**: High-value missing feature.
3.  **Implement `Outer Join`**: High-value missing feature.
