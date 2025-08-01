# Installation

Get Veloxx up and running in your environment quickly and easily.

## System Requirements

### Minimum Requirements
- **Operating System**: Windows 10+, macOS 10.15+, Linux (Ubuntu 18.04+, CentOS 7+)
- **Memory**: 512MB RAM (2GB+ recommended for large datasets)
- **Storage**: 50MB free space
- **Architecture**: x86_64, ARM64 (Apple Silicon supported)

### Recommended Requirements
- **Memory**: 4GB+ RAM for optimal performance
- **CPU**: Multi-core processor for parallel operations
- **Storage**: SSD for faster I/O operations

## Rust Installation

### Using Cargo

Add Veloxx to your `Cargo.toml`:

```toml title="Cargo.toml"
[dependencies]
veloxx = "0.3.1"
```

For the latest development version:

```toml title="Cargo.toml"
[dependencies]
veloxx = { git = "https://github.com/Conqxeror/veloxx" }
```

### Features

Enable optional features as needed:

```toml title="Cargo.toml"
[dependencies]
veloxx = { version = "0.2.4", features = ["python", "wasm"] }
```

Available features:
- `python`: Python bindings support
- `wasm`: WebAssembly/JavaScript bindings

### Verify Installation

Create a simple test file:

```rust title="src/main.rs"
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut columns = BTreeMap::new();
    columns.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![Some("Alice".to_string()), Some("Bob".to_string())],
        ),
    );
    columns.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(30), Some(25)]),
    );

    let df = DataFrame::new(columns)?;
    println!("Veloxx is working! DataFrame:
{}", df);

    Ok(())
}
```

Run the test:

```bash
cargo run
```

## Python Installation

### Using pip (Recommended)

Install from PyPI:

```bash
pip install veloxx
```

For the latest development version:

```bash
pip install git+https://github.com/Conqxeror/veloxx.git
```

### Using conda

```bash
conda install -c conda-forge veloxx
```

### From Source

If you need to build from source:

```bash
# Install Rust first
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and install
git clone https://github.com/Conqxeror/veloxx.git
cd veloxx
pip install maturin
maturin build --release
pip install target/wheels/veloxx-*-py3-none-any.whl
```

### Virtual Environment (Recommended)

Create an isolated environment:

```bash
# Using venv
python -m venv veloxx-env
source veloxx-env/bin/activate  # On Windows: veloxx-env\Scripts\activate
pip install veloxx

# Using conda
conda create -n veloxx-env python=3.11
conda activate veloxx-env
pip install veloxx
```

### Verify Installation

Test your Python installation:

```python title="test_veloxx.py"
import veloxx as vx

# Create a simple DataFrame
df = vx.PyDataFrame({
    "name": vx.PySeries("name", ["Alice", "Bob", "Charlie"]),
    "age": vx.PySeries("age", [25, 30, 35])
})

print("Veloxx Python bindings are working!")
print(f"DataFrame: {df.row_count()} rows, {df.column_count()} columns")
print(df)
```

Run the test:

```bash
python test_veloxx.py
```

## JavaScript/Node.js Installation

### Using npm

```bash
npm install veloxx
```

### Using yarn

```bash
yarn add veloxx
```

### Using pnpm

```bash
pnpm add veloxx
```

### From Source

Build WebAssembly bindings from source:

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Clone and build
git clone https://github.com/Conqxeror/veloxx.git
cd veloxx
wasm-pack build --target nodejs --features wasm
```

### Verify Installation

Test your JavaScript installation:

```javascript title="test_veloxx.js"
import init, { WasmDataFrame, WasmSeries } from 'veloxx';

async function test() {
    await init();
    // Create a simple DataFrame
    const df = new WasmDataFrame({
        name: ["Alice", "Bob", "Charlie"],
        age: [25, 30, 35]
    });
    
    console.log("Veloxx JavaScript bindings are working!");
    console.log(`DataFrame: ${df.rowCount()} rows`);
}

test().catch(console.error);
```

Run the test:

```bash
node test_veloxx.js
```

## Browser Installation

### Using CDN

```html title="index.html"
<!DOCTYPE html>
<html>
<head>
    <title>Veloxx in Browser</title>
</head>
<body>
    <script type="module">
        import init, { WasmDataFrame } from 'https://cdn.jsdelivr.net/npm/veloxx@latest/veloxx.js';
        
        async function run() {
            await init();
            
            const df = new WasmDataFrame({
                name: ["Alice", "Bob"],
                age: [25, 30]
            });
            
            console.log("Veloxx working in browser!");
            console.log(df);
        }
        
        run();
    </script>
</body>
</html>
```

### Using Bundlers

For Webpack, Vite, or other bundlers:

```javascript title="src/main.js"
import init, { WasmDataFrame, WasmSeries } from 'veloxx';

async function main() {
    await init();
    
    const df = new WasmDataFrame({
        sales: [100, 200, 150],
        region: ["North", "South", "East"]
    });
    
    console.log("DataFrame created:", df);
}

main();
```

## Docker Installation

### Building from Source

If you want to build Veloxx within a Docker container:

```dockerfile title="Dockerfile"
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y 
    ca-certificates 
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/veloxx /usr/local/bin/veloxx

CMD ["/usr/local/bin/veloxx"]
```

### Using Docker Compose

For multi-service applications or local development environments:

```yaml title="docker-compose.yml"
version: '3.8'
services:
  veloxx-app:
    build: .
    volumes:
      - ./data:/app/data
    environment:
      - RUST_LOG=info
```

## Development Setup

### For Contributors

Set up a development environment:

```bash
# Clone the repository
git clone https://github.com/Conqxeror/veloxx.git
cd veloxx

# Install Rust toolchain
rustup install stable
rustup default stable

# Install development dependencies
cargo install cargo-watch
cargo install cargo-tarpaulin  # For coverage
cargo install cargo-audit      # For security audits

# Install Python development tools
pip install maturin pytest black isort mypy

# Install Node.js development tools
npm install -g wasm-pack

# Run tests
cargo test --all-features
```

### IDE Setup

#### Visual Studio Code

Recommended extensions:
- `rust-analyzer`: Rust language support
- `Python`: Python language support
- `Error Lens`: Inline error highlighting
- `CodeLLDB`: Debugging support

```json title=".vscode/settings.json"
{
    "rust-analyzer.cargo.features": ["python", "wasm"],
    "python.defaultInterpreterPath": "./venv/bin/python",
    "editor.formatOnSave": true
}
```

#### IntelliJ IDEA / CLion

Install plugins:
- Rust plugin
- Python plugin
- TOML plugin

## Troubleshooting

### Common Issues

#### Rust Compilation Errors

**Issue**: `error: Microsoft Visual C++ 14.0 is required` (Windows)

**Solution**: Install Visual Studio Build Tools:
```bash
# Download and install Visual Studio Build Tools
# Or install Visual Studio Community with C++ tools
```

**Issue**: `error: linker 'cc' not found` (Linux)

**Solution**: Install build essentials:
```bash
# Ubuntu/Debian
sudo apt update && sudo apt install build-essential

# CentOS/RHEL
sudo yum groupinstall "Development Tools"
```

#### Python Binding Issues

**Issue**: `ImportError: No module named 'veloxx'`

**Solution**: Verify installation and Python path:
```bash
pip list | grep veloxx
python -c "import sys; print(sys.path)"
```

**Issue**: `ModuleNotFoundError: No module named 'veloxx._veloxx'`

**Solution**: Reinstall with correct architecture or ensure `maturin develop` was run correctly:
```bash
pip uninstall veloxx
pip install --force-reinstall veloxx
# Or if developing, ensure you ran: maturin develop --features python
```

#### WebAssembly Issues

**Issue**: `WebAssembly.instantiate(): Import #0 module="env" error`

**Solution**: Ensure proper WASM initialization:
```javascript
import init from 'veloxx';

// Always call init() first
await init();
```

**Issue**: Browser compatibility errors

**Solution**: Check browser support and use polyfills:
```html
<script src="https://cdn.jsdelivr.net/npm/@webassembly/wasi@latest/lib/browser.js"></script>
```

### Performance Issues

#### Slow Compilation

**Solution**: Use faster linker and parallel compilation:
```toml title=".cargo/config.toml"
[build]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[cargo]
jobs = 4  # Adjust based on your CPU cores
```

#### Memory Issues

**Solution**: Increase available memory and use streaming:
```rust
// Process large datasets in chunks
for chunk in dataset.chunks(100_000) {
    let result = chunk.process()?;
    // Handle chunk result
}
```

### Getting Help

If you encounter issues not covered here:

1. **Check the [GitHub Issues](https://github.com/Conqxeror/veloxx/issues)** for known problems
2. **Search [GitHub Discussions](https://github.com/Conqxeror/veloxx/discussions)** for community help
3. **Create a new issue** with:
   - Your operating system and version
   - Rust/Python/Node.js versions
   - Complete error messages
   - Minimal reproducible example

### Version Compatibility

| Veloxx Version | Rust Version | Python Version | Node.js Version |
|----------------|--------------|----------------|-----------------|
| 0.3.1 | 1.70+ | 3.8+ | 18+ |
| 0.2.4 | 1.70+ | 3.8+ | 16+ |
| 0.2.3 | 1.70+ | 3.8+ | 16+ |
| 0.2.2 | 1.65+ | 3.7+ | 14+ |
| 0.2.1 | 1.65+ | 3.7+ | 14+ |

## Next Steps

Now that you have Veloxx installed:

1. 📖 **[Quick Start Guide](/docs/getting-started/quick-start)** - Learn the basics in 5 minutes
2. 🧠 **[Core Concepts](/docs/intro)** - Understand DataFrames and Series
3. 📚 **[API Reference](/docs/api/rust)** - Explore the complete API
4. 🚀 **[Examples](/docs/intro)** - See real-world usage patterns
5. ⚡ **[Benchmarks](/docs/performance/benchmarks)** - Compare performance with other libraries

:::tip Pro Tip
Start with the [Quick Start Guide](/docs/getting-started/quick-start) to get familiar with Veloxx's core concepts, then explore the [examples](/docs/intro) for real-world usage patterns.
:::