# <img width="180" height="180" alt="Veloxx Logo" src="./docs/veloxx_logo.png" />

# Veloxx: Ultra-High Performance Data Processing & Analytics Library

<p align="center">
  <a href="https://crates.io/crates/veloxx"><img src="https://img.shields.io/crates/v/veloxx.svg?label=Crates.io&logo=rust" alt="Crates.io" /></a>
  <a href="https://pypi.org/project/veloxx/"><img src="https://img.shields.io/pypi/v/veloxx?color=blue&label=PyPI&logo=python" alt="PyPI" /></a>
  <a href="https://www.npmjs.com/package/veloxx"><img src="https://img.shields.io/npm/v/veloxx?color=red&label=npm&logo=npm" alt="npm" /></a>
  <a href="https://github.com/Conqxeror/veloxx"><img src="https://img.shields.io/github/stars/Conqxeror/veloxx?style=social&label=GitHub&logo=github" alt="GitHub" /></a>
  <a href="https://docs.rs/veloxx"><img src="https://docs.rs/veloxx/badge.svg" alt="docs.rs" /></a>
  <a href="https://github.com/Conqxeror/veloxx/actions/workflows/ci.yml"><img src="https://github.com/Conqxeror/veloxx/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License: MIT" /></a>
  <a href="https://conqxeror.github.io/veloxx/"><img src="https://img.shields.io/badge/docs-online-blue?logo=readthedocs" alt="Documentation" /></a>
</p>

---

> 🚀 **v0.4.0 Released!** Major performance overhaul with SIMD acceleration, Pivot, and Outer Join support.

Veloxx is a **blazing-fast**, ultra-lightweight data processing and analytics library in Rust, with seamless bindings for Python and WebAssembly. Built from the ground up for **maximum performance**, featuring advanced SIMD acceleration, memory optimization, and parallel processing that often **outperforms industry leaders**.

## 🏆 **Performance Highlights**

*   **SIMD Acceleration**: Vectorized aggregation (`sum`, `mean`, `min`, `max`) now **30-90x faster** than scalar implementations.
*   **Parallel Processing**: Hybrid execution strategy using Rayon for large datasets (>500k rows), achieving near-linear scaling.
*   **Optimized I/O**: Multi-threaded memory-mapped CSV reading and zero-copy Parquet integration.
*   **Lazy Evaluation**: Refined Query Optimizer with predicate pushdown for efficient filtering.

## ✨ New Features (v0.4.0)

*   **Pivot**: Reshape DataFrames from long to wide format with aggregation.
*   **Outer Join**: Full support for `Left`, `Right`, `Inner`, and `Outer` joins.
*   **Deterministic Columns**: Refactored internal storage to guarantee consistent column ordering.
*   **Python Bindings**: Updated `PyDataFrame` with `pivot` and `outer_join` support.

---

## ✨ Project Links

- 🦀 [**Rust crate** (crates.io)](https://crates.io/crates/veloxx)
- 🐍 [**Python package** (PyPI)](https://pypi.org/project/veloxx/)
- 📦 [**JavaScript package** (npm)](https://www.npmjs.com/package/veloxx)
- 🌐 [**GitHub**](https://github.com/Conqxeror/veloxx)
- 📖 [**Online Documentation**](https://conqxeror.github.io/veloxx/)

## 🧩 Core Principles & Design Goals

- 🚀 **Performance First**: Advanced SIMD, parallel processing, cache-optimized algorithms
- 🪶 **Lightweight**: Minimal dependencies, optimized memory footprint
- 🦺 **Safety & Reliability**: Memory-safe Rust, comprehensive testing
- 🧑‍💻 **Developer Experience**: Intuitive APIs, excellent documentation
- 🔧 **Production Ready**: Zero-warning compilation, extensive benchmarking

## 🚩 Key Features

### **Core Data Structures**
- **DataFrame** and **Series** for lightning-fast tabular data processing
- **SIMD-optimized** operations with AVX2/NEON acceleration
- **Memory-efficient** storage with advanced compression

### **High-Performance Operations**
- 🚀 **Ultra-fast analytics**: filtering, joining, grouping, aggregation, **pivoting**
- 📊 **Advanced statistics**: correlation, regression, time-series analysis
-  **Parallel processing**: Multi-threaded execution with work-stealing
- 🧮 **Vectorized math**: SIMD-accelerated arithmetic operations

### **Advanced I/O & Integration**
- 📂 **Multiple formats**: CSV, JSON, Parquet support
- 🔌 **Database connectivity**: SQLite, PostgreSQL, MySQL
- 🌊 **Streaming operations**: Memory-efficient large dataset processing
- ⚡ **Async I/O**: Non-blocking file and network operations

### **Data Quality & ML**
- 🧹 **Data cleaning**: Automated outlier detection, validation
- 🤖 **Machine learning**: Linear/logistic regression, clustering, preprocessing
- 📈 **Visualization**: Charts, plots, statistical graphics
- 🔍 **Data profiling**: Schema inference, quality metrics

### **Multi-Language Support**
- 🦀 **Rust**: Native, zero-cost abstractions
-  **Python**: PyO3 bindings with NumPy integration  
- 🌐 **WebAssembly**: Browser and Node.js support
- 📦 **Easy installation**: Available on crates.io, PyPI, npm

## ⚡ Quick Start

### Rust

```toml
[dependencies]
veloxx = "0.4.0"
```

```rust
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

let df = DataFrame::new_from_csv("data.csv")?;
let filtered = df.filter(&your_condition)?;
let grouped = df.group_by(vec!["category"]).agg(vec![("amount", "sum")])?;
```

### Python

```python
import veloxx

df = veloxx.PyDataFrame({"name": veloxx.PySeries("name", ["Alice", "Bob"])})
filtered = df.filter(...)
pivoted = df.pivot(values="score", index=["name"], columns="subject", agg_fn="mean")
```

### JavaScript/Wasm

```javascript
const veloxx = require("veloxx");
const df = new veloxx.WasmDataFrame({name: ["Alice", "Bob"]});
const filtered = df.filter(...);
```

## 🛠️ Feature Flags

Enable only what you need:

- `advanced_io` – Parquet, databases, async
- `data_quality` – Schema checks, anomaly detection
- `window_functions` – Window analytics
- `visualization` – Charting
- `ml` – Machine learning
- `python` – Python bindings
- `wasm` – WebAssembly

## 📚 Documentation

- [Getting Started Guide](./docs/GETTING_STARTED.md)
- [API Guide](./docs/API_GUIDE.md)
- [Rust API Docs](./docs/rust/veloxx/index.html)
- [Python API Docs](./docs/python/build/html/index.html)
- [JavaScript/Wasm Docs](./docs/js/index.html)
- [Online Docs](https://conqxeror.github.io/veloxx/)

## 🧑‍💻 Examples

Run ready-made examples:

```bash
cargo run --example basic_dataframe_operations
cargo run --example advanced_io --features advanced_io
# ... more in the examples/ folder
```

## 🤝 Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines. Please review our [Code of Conduct](./CODE_OF_CONDUCT.md).

## 💬 Support

- Issues: https://github.com/Conqxeror/veloxx/issues
- Discussions: https://github.com/Conqxeror/veloxx/discussions
- Documentation: https://conqxeror.github.io/veloxx/

## 📝 License

MIT License. See [LICENSE](./LICENSE).