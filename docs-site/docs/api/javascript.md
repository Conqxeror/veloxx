# JavaScript API Reference

Veloxx provides WebAssembly (WASM) bindings for running high-performance data processing in the browser or Node.js.

:::warning
The JavaScript/WASM API is currently in **Alpha**. Many features available in Rust/Python are not yet fully exposed.
:::

## Installation

```bash
npm install veloxx-wasm
```

## Core Classes

### `WasmDataFrame`

#### Creation

```javascript
import { WasmDataFrame } from "veloxx-wasm";

// From Object
const df = new WasmDataFrame({
  "col1": [1, 2, 3],
  "col2": ["a", "b", "c"]
});
```

#### Methods

| Method | Description |
|--------|-------------|
| `shape()` | Returns `[rows, cols]` |
| `column_names()` | Returns Array of column names |
| `get_column(name)` | Returns raw Float64Array (if numeric) |
| `add_column(name, data)` | Adds a new column |
| `head(n)` | Returns first n rows as JSON object |

### `WasmSeries`

Currently internal-only in the JS bindings. Access data via DataFrame methods.

## Examples

### Browser (React)

```javascript
import { useEffect, useState } from 'react';
import init, { WasmDataFrame } from 'veloxx-wasm';

function App() {
  const [data, setData] = useState(null);

  useEffect(() => {
    init().then(() => {
      const df = new WasmDataFrame({ "val": [10, 20, 30] });
      setData(df.head(5));
    });
  }, []);

  return <pre>{JSON.stringify(data, null, 2)}</pre>;
}
```

### Node.js

```javascript
const { WasmDataFrame } = require('veloxx-wasm');

const df = new WasmDataFrame({ "id": [1, 2, 3] });
console.log(df.column_names());
```
