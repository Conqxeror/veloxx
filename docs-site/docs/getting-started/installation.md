# Installation

## Rust

Add `veloxx` to your `Cargo.toml`. We recommend enabling SIMD features for maximum performance on supported hardware.

```toml
[dependencies]
veloxx = { version = "0.4.0", features = ["advanced_io", "simd"] }
```

### Feature Flags

- `advanced_io`: Parquet reading/writing, async support.
- `simd`: Enable AVX2 optimizations (requires nightly Rust or `RUSTFLAGS="-C target-cpu=native"`).
- `ml`: Machine learning algorithms (Linear Regression, K-Means).
- `visualization`: Plotting capabilities.

## Python

Veloxx is available on PyPI as a pre-compiled wheel for major platforms (Windows, Linux, macOS).

```bash
pip install veloxx
```

**Requirements:**
- Python 3.7+
- 64-bit OS

## JavaScript / Node.js

Veloxx provides WASM bindings via npm.

```bash
npm install veloxx-wasm
```

### Webpack Configuration

You may need to enable `experiments.asyncWebAssembly` in your `webpack.config.js` to load the WASM module correctly.

```javascript
module.exports = {
  // ...
  experiments: {
    asyncWebAssembly: true,
  },
};
```
