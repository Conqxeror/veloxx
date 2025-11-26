# Quick Start Guide

## Rust

Add `veloxx` to your `Cargo.toml`:

```toml
[dependencies]
veloxx = { version = "0.4.0", features = ["advanced_io", "simd"] }
```

```rust
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::io::UltraFastCsvParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load Data
    let df = UltraFastCsvParser::quick_read("data.csv")?;

    // 2. Filter
    let filtered = df.filter_by_indices(&[0, 1, 2])?;

    // 3. Group & Aggregate
    let grouped = filtered.group_by(vec!["category".to_string()])?;
    let result = grouped.agg(vec![("value", "sum")])?;

    println!("{}", result);
    Ok(())
}
```

## Python

```bash
pip install veloxx
```

```python
import veloxx
from veloxx import PyCondition

# 1. Load Data
df = veloxx.read_csv("data.csv")

# 2. Filter (Efficient)
cond = PyCondition.gt("value", 100.0)
filtered = df.filter(cond)

# 3. Pivot
pivoted = filtered.pivot(
    values="sales",
    index=["region"],
    columns="quarter",
    agg_fn="sum"
)

print(pivoted.row_count())
```

## JavaScript (WASM)

```javascript
import init, { WasmDataFrame } from 'veloxx-wasm';

async function run() {
  await init(); // Initialize WASM module

  const df = new WasmDataFrame({
    "id": new Float64Array([1, 2, 3]),
    "val": new Float64Array([10.5, 20.0, 30.5])
  });

  console.log("Shape:", df.shape());
  console.log("Cols:", df.column_names());
}

run();
```
