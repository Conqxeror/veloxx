# <img src="docs/veloxx_logo.png" alt="Veloxx Logo" height="70px"> Veloxx: Lightweight Rust-Powered Data Processing & Analytics Library

[![crates.io](https://img.shields.io/crates/v/veloxx.svg)](https://crates.io/crates/veloxx)

> **New in 0.2.1:** Major performance improvements across all core operations. See CHANGELOG for details.

Veloxx is a new Rust library designed for highly performant and **extremely lightweight** in-memory data processing and analytics. It prioritizes minimal dependencies, optimal memory footprint, and compile-time guarantees, making it an ideal choice for resource-constrained environments, high-performance computing, and applications where every byte and cycle counts.



## Installation

You can install the WebAssembly package using `npm` after building it with `wasm-pack`:

```bash
# First, build the WebAssembly package (from the project root)
wasm-pack build --target web --out-dir pkg

# Then install the package
npm install ./pkg
```

## Usage Example

Here's a quick example demonstrating how to create a DataFrame, filter it, and perform a group-by aggregation:

```javascript
const veloxx = require('veloxx');

async function runWasmExample() {
    // 1. Create a DataFrame
    const df = new veloxx.WasmDataFrame({
        name: ['Alice', 'Bob', 'Charlie', 'David'],
        age: [25, 30, 22, 35],
        city: ['New York', 'London', 'New York', 'Paris'],
    });
    console.log("Original DataFrame:");
    console.log(df);

    // 2. Filter data: age > 25
    const ageSeries = df.getColumn("age");
    const filteredIndices = [];
    for (let i = 0; i < ageSeries.len; i++) {
        if (ageSeries.getValue(i) > 25) {
            filteredIndices.push(i);
        }
    }
    const filteredDf = df.filter(new Uint32Array(filteredIndices));
    console.log("
Filtered DataFrame (age > 25):");
    console.log(filteredDf);

    // 3. Series operations
    console.log(`
Age Series Sum: ${ageSeries.sum()}`);
    console.log(`Age Series Mean: ${ageSeries.mean()}`);
    console.log(`Age Series Unique: ${ageSeries.unique().toVecF64()}`);
}

runWasmExample();

```



## WebAssembly Testing

WebAssembly bindings are currently tested using `console.assert` in `test_wasm.js`. Future work includes migrating to a more robust JavaScript testing framework like Jest.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
