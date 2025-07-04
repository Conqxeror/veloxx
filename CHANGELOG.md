# Changelog

## 0.2.1 - 2025-07-02

### Improved
- Major performance improvements across all core DataFrame and Series operations, including:
  - Optimized join, filter, sort, and aggregation logic.
  - Faster unique value extraction and null interpolation.
  - Type-specific apply methods for Series.
  - More efficient CSV/JSON ingestion and type inference.
- All benchmarks show significant speedups (see README and benchmarks).

## 0.2.0 - 2025-07-02

### Added
- New `DateTime` data type and `Value` variant.
- Extended expression capabilities with comparison and logical operators (`Equals`, `NotEquals`, `GreaterThan`, `LessThan`, `GreaterThanOrEqual`, `LessThanOrEqual`, `And`, `Or`, `Not`).

### Changed
- Updated `Series` and `DataFrame` methods to support the new `DateTime` type.
- Improved type inference and serialization for `DateTime` in CSV and JSON I/O.
- Enhanced `fill_nulls`, `sort`, `with_column`, `describe`, `agg`, and `Display` implementations to handle `DateTime`.

### Fixed
- Resolved `Expr::Not` evaluation bug in `test_expression_evaluation`.

### Other
- Ran `cargo clippy`, `cargo fmt`, and `cargo doc` to ensure code quality and documentation consistency.
- Updated `Cargo.toml` version to `0.2.0`.
- Updated `README.md` to reflect new features and usage examples.
