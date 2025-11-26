# Python API Reference

Veloxx provides comprehensive Python bindings via `pyo3`, offering high-performance DataFrame operations with a NumPy-compatible API.

## Installation

```bash
pip install veloxx
```

## Core Classes

### `PyDataFrame`

The primary data structure for tabular data processing.

#### Creation

```python
import veloxx

# From Dictionary
df = veloxx.PyDataFrame({"col1": veloxx.PySeries("col1", [1, 2, 3])})

# From CSV
df = veloxx.read_csv("data.csv")
```

#### Methods

| Method | Description |
|--------|-------------|
| `row_count()` | Returns number of rows |
| `column_count()` | Returns number of columns |
| `column_names()` | Returns list of column names |
| `get_column(name)` | Returns a `PySeries` by name |
| `select(columns)` | Returns new DataFrame with selected columns |
| `drop_columns(columns)` | Returns new DataFrame without specified columns |
| `rename_column(old, new)` | Renames a column |
| `filter(condition)` | Filters rows based on `PyCondition` or indices |
| `filter_gt(col, val)` | Fast filter for greater-than condition |
| `sort(columns, ascending)` | Sorts DataFrame by columns |
| `head(n)` | Returns first n rows |
| `tail(n)` | Returns last n rows |
| `with_column(name, expr)` | Adds a computed column based on expression |
| `join(other, on, type)` | Joins with another DataFrame (`Inner`, `Left`, `Right`, `Outer`) |
| `fast_inner_join(other, left_on, right_on)` | **SIMD-optimized** inner join for integer keys |
| `pivot(values, index, columns, agg_fn)` | Reshapes DataFrame from long to wide format |
| `group_by(columns)` | Creates a `PyGroupedDataFrame` for aggregation |
| `to_csv(path)` | Exports to CSV |

### `PySeries`

Typed columnar data container.

#### Methods

| Method | Description |
|--------|-------------|
| `len()` | Returns length |
| `sum()` | Returns sum (SIMD optimized) |
| `mean()` | Returns mean |
| `min()` | Returns minimum value |
| `max()` | Returns maximum value |
| `std_dev()` | Returns standard deviation |
| `median()` | Returns median value |
| `unique()` | Returns unique values |
| `add(other)` | Element-wise addition (SIMD) |
| `multiply(other)` | Element-wise multiplication (SIMD) |
| `cast(dtype)` | Casts to another `PyDataType` |
| `correlation(other)` | Computes Pearson correlation |

### `PyCondition`

Expression builder for efficient filtering.

```python
from veloxx import PyCondition

# Create conditions
cond1 = PyCondition.gt("age", 25)
cond2 = PyCondition.eq("status", "active")

# Apply
filtered = df.filter(cond1)
```

### `PyExpr`

Expression builder for column transformations.

```python
from veloxx import PyExpr

# Create expression: col("a") + col("b")
expr = PyExpr.column("a").add(PyExpr.column("b"))

# Apply
df = df.with_column("sum_ab", expr)
```

## Enums

### `PyJoinType`
- `Inner`
- `Left`
- `Right`
- `Outer`

### `PyDataType`
- `I32`
- `F64`
- `String`
- `Bool`
- `DateTime`
