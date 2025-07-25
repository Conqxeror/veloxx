# <img width="180" height="180" alt="Veloxx Logo" src="./docs/veloxx_logo.png" />

# Veloxx: Lightweight Rust-Powered Data Processing & Analytics Library

<p align="center">
  <a href="https://crates.io/crates/veloxx"><img src="https://img.shields.io/crates/v/veloxx.svg?label=Crates.io&logo=rust" alt="Crates.io" /></a>
  <a href="https://pypi.org/project/veloxx/"><img src="https://img.shields.io/pypi/v/veloxx?color=blue&label=PyPI&logo=python" alt="PyPI" /></a>
  <a href="https://www.npmjs.com/package/veloxx"><img src="https://img.shields.io/npm/v/veloxx?color=red&label=npm&logo=npm" alt="npm" /></a>
  <a href="https://github.com/Conqxeror/veloxx"><img src="https://img.shields.io/github/stars/Conqxeror/veloxx?style=social&label=GitHub&logo=github" alt="GitHub" /></a>
  <a href="https://conqxeror.github.io/veloxx/"><img src="https://img.shields.io/badge/docs-online-blue?logo=readthedocs" alt="Documentation" /></a>
</p>

---

> 🚀 **v0.3.1 Released!** See CHANGELOG for details.

Veloxx is a high-performance, **extremely lightweight** in-memory data processing and analytics library in Rust, with bindings for Python, WebAssembly, and more. Designed for minimal dependencies, optimal memory usage, and blazing speed, it's ideal for data science, analytics, and any environment where every byte and cycle counts.

---

## ✨ Project Links

- 🦀 [**Rust crate** (crates.io)](https://crates.io/crates/veloxx)
- 🐍 [**Python package** (PyPI)](https://pypi.org/project/veloxx/)
- 📦 [**JavaScript package** (npm)](https://www.npmjs.com/package/veloxx)
- 🌐 [**GitHub**](https://github.com/Conqxeror/veloxx)
- 📖 [**Online Documentation**](https://conqxeror.github.io/veloxx/)

## 🧩 Core Principles & Design Goals

- 🪶 **Lightweight**: Minimal dependencies and small binaries
- ⚡ **Performance First**: SIMD, parallelism, cache-friendly data structures
- 🦺 **Safety & Reliability**: Idiomatic Rust, memory safety, minimal unsafe code
- 🧑‍💻 **Ergonomics**: Discoverable, chainable, and user-friendly API
- 🧱 **Composability**: Modular, extensible, and feature-rich

## 🚩 Key Features

- **DataFrame** and **Series** for fast, type-safe tabular data
- 🚀 In-memory analytics: filtering, joining, grouping, aggregation, stats
- 📦 Data ingestion: CSV, JSON, custom sources
- 💾 Advanced I/O: Parquet, async DB, streaming *(features)*
- 🧹 Data cleaning & validation: schema checks, anomaly detection *(features)*
- 🪟 Window functions, time-series analytics *(features)*
- 📈 Charting & visualization *(features)*
- 🤖 Machine learning: linear regression, preprocessing *(features)*
- 🔄 Python & Wasm bindings

## ⚡ Quick Start

### Rust

```toml
[dependencies]
veloxx = "0.3.1"
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
filtered = df.filter([...])
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

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## 📝 License

MIT License. See [LICENSE](./LICENSE).
