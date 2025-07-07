# Veloxx: Python Bindings

This document provides installation and usage instructions for the Python bindings of Veloxx.

## Installation

Veloxx Python bindings are available on [PyPI](https://pypi.org/project/veloxx/).

To install the latest stable version:

```bash
pip install veloxx
```

Alternatively, if you are developing Veloxx or need to install from source, you can build the Python wheel with `maturin`:

```bash
# First, build the Python wheel (from the project root)
maturin build --release

# Then install the wheel
pip install target/wheels/veloxx-*-py3-none-any.whl
```

## Usage Examples

```python
import veloxx

# 1. Create a DataFrame
df = veloxx.PyDataFrame({
    "name": veloxx.PySeries("name", ["Alice", "Bob", "Charlie", "David"]),
    "age": veloxx.PySeries("age", [25, 30, 22, 35]),
    "city": veloxx.PySeries("city", ["New York", "London", "New York", "Paris"]),
})
print("Original DataFrame:")
print(df)

# 2. Filter data: age > 25
filtered_df = df.filter([i for i, age in enumerate(df.get_column("age").to_vec_f64()) if age > 25])
print("\nFiltered DataFrame (age > 25):")
print(filtered_df)

# 3. Select columns
selected_df = df.select_columns(["name", "city"])
print("\nSelected Columns (name, city):")
print(selected_df)

# 4. Rename a column
renamed_df = df.rename_column("age", "years")
print("\nRenamed Column (age to years):")
print(renamed_df)

# 5. Series operations
age_series = df.get_column("age")
print(f"\nAge Series Sum: {age_series.sum()}")
print(f"Age Series Mean: {age_series.mean()}")
print(f"Age Series Max: {age_series.max()}")
print(f"Age Series Unique: {age_series.unique().to_vec_f64()}")
```

```