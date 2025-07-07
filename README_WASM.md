# Veloxx: WebAssembly Bindings

This document provides installation and usage instructions for the WebAssembly bindings of Veloxx.

## Installation

Veloxx WebAssembly bindings are available on [npm](https://www.npmjs.com/package/veloxx).

To install the latest stable version:

```bash
npm install veloxx
```

Alternatively, if you are developing Veloxx or need to build from source, you can build the WebAssembly package with `wasm-pack`:

```bash
# First, build the WebAssembly package (from the project root)
wasm-pack build --target web --out-dir pkg

# Then install the package
npm install ./pkg
```

## Usage Examples

```javascript
const veloxx = require('veloxx');

async function runWasmExample() {
    // 1. Create a DataFrame
    const df = new veloxx.WasmDataFrame({
        name: ["Alice", "Bob", "Charlie", "David"],
        age: [25, 30, 22, 35],
        city: ["New York", "London", "New York", "Paris"],
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
    console.log("\nFiltered DataFrame (age > 25):");
    console.log(filteredDf);

    // 3. Series operations
    console.log(`\nAge Series Sum: ${ageSeries.sum()}`);
    console.log(`Age Series Mean: ${ageSeries.mean()}`);
    console.log(`Age Series Unique: ${ageSeries.unique().toVecF64()}`);
}

runWasmExample();
```

## WebAssembly Testing

WebAssembly bindings are currently tested using `console.assert` in `test_wasm.js`. Future work includes migrating to a more robust JavaScript testing framework like Jest.

