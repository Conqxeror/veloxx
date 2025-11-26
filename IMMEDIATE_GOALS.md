This document outlines the immediate goals and roadmap for the Veloxx project.

## Release Preparation (v0.4.0)

- [x] **Core Refactoring**:
    - [x] Switch `DataFrame` internal storage from `HashMap` to `IndexMap` for deterministic column ordering.
    - [x] Update all affected modules (`src/dataframe/mod.rs`, `src/io/*`, `src/performance/*`, etc.).
    - [x] Fix compilation errors and run `cargo check` to verify.

- [x] **Performance Optimization**:
    - [x] **SIMD**: Implement explicit AVX2 optimizations for aggregations (`sum`, `mean`) and arithmetic.
    - [x] **Parallelism**: Implement hybrid parallel/scalar strategy based on dataset size threshold.
    - [x] **I/O**: Optimize CSV reader with memory mapping and parallel chunking.
    - [x] **Lazy Engine**: Fix query optimizer and expression evaluation.

- [x] **New Features**:
    - [x] **Pivot**: Implement `Pivot` trait and logic.
    - [x] **Outer Join**: Implement full outer join support.
    - [x] **Python Bindings**: Expose new features (Pivot, Outer Join) to Python via PyO3.

- [x] **Testing & QA**:
    - [x] **Benchmarks**: Update benchmark suite (`benches/`).
    - [x] **Fuzz Testing**: Add `proptest` based fuzz tests for joins and groupby.
    - [x] **Regression Testing**: Ensure all existing tests pass (`cargo test`).
    - [x] **Linting**: Run `cargo clippy` and fix warnings.

- [ ] **Release**:
    - [ ] Update `Cargo.toml` version to `0.4.0`. (Done locally)
    - [ ] Update `CHANGELOG.md`. (Done locally)
    - [ ] Create release tag `v0.4.0`.
    - [ ] Publish to crates.io.
    - [ ] Publish Python wheels.
    - [ ] Publish WASM package.

## Future Goals (v0.5.0+)

- **SQL Interface**: Implement SQL parsing and execution engine.
- **Distributed Computing**: Explore multi-node capabilities.
- **GPU Support**: CUDA/OpenCL integration for massive datasets.