# Changelog

## 0.2.4 - 2025-07-09

### Added

- Implemented missing functionalities in Python bindings (DataFrame I/O, advanced filtering, joining, grouping, column creation, descriptive statistics, data appending, apply methods).
- Implemented missing functionalities in JavaScript/Wasm bindings (DataFrame I/O, advanced filtering, joining, grouping, column creation, descriptive statistics, data appending, apply methods).
- Added comprehensive tests for newly implemented Python and JavaScript/Wasm functionalities.

### Changed

- Updated version to `0.2.4` across `Cargo.toml`, `pkg/package.json`, and all relevant documentation.
- Organized test files into `tests/python/` and updated `CONTRIBUTING.md`.

### Fixed

- Resolved Wasm test failures by updating import paths and mocking strategy.
- Addressed all Clippy warnings and formatted Rust code.

### Removed

- Redundant Wasm bindings file (`bindings/wasm/mod.rs`).
- Temporary Python example scripts (`temp_inspect_veloxx.py`, `temp_inspect_veloxx_lib.py`).
- `ISSUES.md` (all issues addressed).


## 0.2.3 - 2025-07-07

### Changed

- Consolidated all language-specific usage examples into the root `README.md`.
- Created dedicated `README_PYTHON.md` for Python-specific documentation (for PyPI).
- Created dedicated `README_WASM.md` for WebAssembly/JavaScript-specific documentation (for npmjs.com).
- Ensured `pkg/README.md` is consistent with `README_WASM.md`.
- Updated version to `0.2.3` across `Cargo.toml`, `package.json`, and all relevant documentation.



## 0.2.2 - 2025-07-04

### Added

- Python bindings for DataFrame and Series operations.
- WebAssembly bindings for DataFrame and Series operations.
- `CONTRIBUTING.md` file with development guidelines.
- New example files demonstrating DataFrame operations, aggregation, and manipulation.

### Improved

- Python testing with `pytest` fixtures and expanded test coverage.

### Changed

- Updated `Cargo.toml` to include `pyo3` dependency and `python` feature.
- Updated `Cargo.toml` with `test-python` and `test-wasm` commands.
- Updated `README.md` to reflect WebAssembly testing status.

### Other

- Integrated Jest for WebAssembly testing.


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