# Python API Reference

Complete API reference for Veloxx Python bindings.

## Installation

```bash
pip install veloxx
```

## Quick Start

```python
import veloxx as vx

# Load data
df = vx.read_csv("data.csv")

# Basic operations
filtered = df.filter(df["age"] > 25)
grouped = df.groupby("department").mean()
```

## Core Classes

### `PyDataFrame`

The main data structure for working with tabular data in Python.

#### Constructors

<div className="api-section">
<div className="api-method">PyDataFrame(columns: dict)</div>

Creates a new DataFrame from a dictionary of column names to PySeries.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">columns</span>: <span className="parameter-type">dict</span> - Dictionary mapping column names to PySeries objects
</div>
</div>

**Example:**
```python
import veloxx as vx

df = vx.PyDataFrame({
    "name": vx.PySeries("name", ["Alice", "Bob", "Charlie"]),
    "age": vx.PySeries("age", [25, 30, 35]),
    "salary": vx.PySeries("salary", [50000.0, 75000.0, 60000.0])
})
```
</div>

#### Class Methods

<div className="api-section">
<div className="api-method">@classmethod from_csv(path: str) -&gt; PyDataFrame</div>

Loads a DataFrame from a CSV file with automatic type inference.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">path</span>: <span className="parameter-type">str</span> - Path to the CSV file
</div>
</div>

**Example:**
```python
df = vx.PyDataFrame.from_csv("data/employees.csv")
print(f"Loaded {df.row_count()} rows")
```
</div>

<div className="api-section">
<div className="api-method">@classmethod from_json(path: str) -&gt; PyDataFrame</div>

Loads a DataFrame from a JSON file.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">path</span>: <span className="parameter-type">str</span> - Path to the JSON file
</div>
</div>

**Example:**
```python
df = vx.PyDataFrame.from_json("data/users.json")
```
</div>

#### Properties

<div className="api-section">
<div className="api-method">row_count() -&gt; int</div>

Returns the number of rows in the DataFrame.

**Example:**
```python
print(f"DataFrame has {df.row_count()} rows")
```
</div>

<div className="api-section">
<div className="api-method">column_count() -&gt; int</div>

Returns the number of columns in the DataFrame.

**Example:**
```python
print(f"DataFrame has {df.column_count()} columns")
```
</div>

<div className="api-section">
<div className="api-method">column_names() -&gt; List[str]</div>

Returns a list of column names.

**Example:**
```python
names = df.column_names()
for name in names:
    print(f"Column: {name}")
```
</div>

#### Data Access

<div className="api-section">
<div className="api-method">get_column(name: str) -&gt; Optional[PySeries]</div>

Gets a column by name.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">str</span> - Name of the column to retrieve
</div>
</div>

**Example:**
```python
age_column = df.get_column("age")
if age_column:
    print(f"Age column has {age_column.len()} values")
```
</div>

<div className="api-section">
<div className="api-method">__getitem__(key: str) -&gt; PySeries</div>

Gets a column using bracket notation (syntactic sugar).

**Example:**
```python
# These are equivalent
age1 = df.get_column("age")
age2 = df["age"]
```
</div>

#### Data Manipulation

<div className="api-section">
<div className="api-method">filter(row_indices: List[int]) -&gt; PyDataFrame</div>

Filters rows by index positions.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">row_indices</span>: <span className="parameter-type">List[int]</span> - List of row indices to keep
</div>
</div>

**Example:**
```python
# Filter rows where age > 25
age_series = df.get_column("age")
indices = [i for i, age in enumerate(age_series.to_list()) if age and age > 25]
filtered_df = df.filter(indices)
```
</div>

<div className="api-section">
<div className="api-method">select_columns(names: List[str]) -&gt; PyDataFrame</div>

Selects specific columns from the DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">names</span>: <span className="parameter-type">List[str]</span> - Names of columns to select
</div>
</div>

**Example:**
```python
selected = df.select_columns(["name", "age"])
```
</div>

<div className="api-section">
<div className="api-method">drop_columns(names: List[str]) -&gt; PyDataFrame</div>

Removes specified columns from the DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">names</span>: <span className="parameter-type">List[str]</span> - Names of columns to drop
</div>
</div>

**Example:**
```python
without_id = df.drop_columns(["id"])
```
</div>

<div className="api-section">
<div className="api-method">rename_column(old_name: str, new_name: str) -&gt; PyDataFrame</div>

Renames a column in the DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">old_name</span>: <span className="parameter-type">str</span> - Current name of the column
</div>
<div className="api-parameter">
<span className="parameter-name">new_name</span>: <span className="parameter-type">str</span> - New name for the column
</div>
</div>

**Example:**
```python
renamed = df.rename_column("age", "years")
```
</div>

<div className="api-section">
<div className="api-method">with_column(name: str, expr: PyExpr) -&gt; PyDataFrame</div>

Adds a new column or replaces an existing one using an expression.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">str</span> - Name of the new column
</div>
<div className="api-parameter">
<span className="parameter-name">expr</span>: <span className="parameter-type">PyExpr</span> - Expression to compute the column values
</div>
</div>

**Example:**
```python
# Add a column with salary + 1000 bonus
expr = vx.PyExpr.add(
    vx.PyExpr.column("salary"),
    vx.PyExpr.literal(1000.0)
)
with_bonus = df.with_column("salary_with_bonus", expr)
```
</div>

#### Grouping and Aggregation

<div className="api-section">
<div className="api-method">group_by(by_columns: List[str]) -&gt; PyGroupedDataFrame</div>

Groups the DataFrame by specified columns.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">by_columns</span>: <span className="parameter-type">List[str]</span> - Columns to group by
</div>
</div>

**Example:**
```python
grouped = df.group_by(["department"])
result = grouped.mean()
```
</div>

<div className="api-section">
<div className="api-method">describe() -&gt; PyDataFrame</div>

Generates descriptive statistics for numeric columns.

**Example:**
```python
stats = df.describe()
print(stats)
```
</div>

#### Statistical Methods

<div className="api-section">
<div className="api-method">correlation(col1_name: str, col2_name: str) -&gt; float</div>

Calculates the Pearson correlation between two numeric columns.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">col1_name</span>: <span className="parameter-type">str</span> - Name of the first column
</div>
<div className="api-parameter">
<span className="parameter-name">col2_name</span>: <span className="parameter-type">str</span> - Name of the second column
</div>
</div>

**Example:**
```python
corr = df.correlation("age", "salary")
print(f"Age-Salary correlation: {corr:.3f}")
```
</div>

<div className="api-section">
<div className="api-method">covariance(col1_name: str, col2_name: str) -&gt; float</div>

Calculates the covariance between two numeric columns.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">col1_name</span>: <span className="parameter-type">str</span> - Name of the first column
</div>
<div className="api-parameter">
<span className="parameter-name">col2_name</span>: <span className="parameter-type">str</span> - Name of the second column
</div>
</div>

**Example:**
```python
cov = df.covariance("age", "salary")
print(f"Age-Salary covariance: {cov:.2f}")
```
</div>

#### Joining

<div className="api-section">
<div className="api-method">join(other: PyDataFrame, on_column: str, join_type: PyJoinType) -&gt; PyDataFrame</div>

Joins this DataFrame with another DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">other</span>: <span className="parameter-type">PyDataFrame</span> - DataFrame to join with
</div>
<div className="api-parameter">
<span className="parameter-name">on_column</span>: <span className="parameter-type">str</span> - Column name to join on
</div>
<div className="api-parameter">
<span className="parameter-name">join_type</span>: <span className="parameter-type">PyJoinType</span> - Type of join (Inner, Left, Right)
</div>
</div>

**Example:**
```python
joined = df1.join(df2, "user_id", vx.PyJoinType.Inner)
```
</div>

#### Sorting and Ordering

<div className="api-section">
<div className="api-method">sort(by_columns: List[str], ascending: bool = True) -&gt; PyDataFrame</div>

Sorts the DataFrame by specified columns.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">by_columns</span>: <span className="parameter-type">List[str]</span> - Columns to sort by
</div>
<div className="api-parameter">
<span className="parameter-name">ascending</span>: <span className="parameter-type">bool</span> - Sort order (default: True)
</div>
</div>

**Example:**
```python
sorted_df = df.sort(["age", "name"], ascending=True)
```
</div>

#### Data Cleaning

<div className="api-section">
<div className="api-method">drop_nulls() -&gt; PyDataFrame</div>

Removes rows containing any null values.

**Example:**
```python
clean_df = df.drop_nulls()
```
</div>

<div className="api-section">
<div className="api-method">fill_nulls(value: Any) -&gt; PyDataFrame</div>

Fills null values with a specified value.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">value</span>: <span className="parameter-type">Any</span> - Value to use for filling nulls
</div>
</div>

**Example:**
```python
filled = df.fill_nulls(0)  # Fill with 0
filled_str = df.fill_nulls("Unknown")  # Fill with string
```
</div>

#### I/O Operations

<div className="api-section">
<div className="api-method">to_csv(path: str) -&gt; None</div>

Writes the DataFrame to a CSV file.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">path</span>: <span className="parameter-type">str</span> - Output file path
</div>
</div>

**Example:**
```python
df.to_csv("output/results.csv")
```
</div>

#### Concatenation

<div className="api-section">
<div className="api-method">append(other: PyDataFrame) -&gt; PyDataFrame</div>

Appends another DataFrame vertically.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">other</span>: <span className="parameter-type">PyDataFrame</span> - DataFrame to append
</div>
</div>

**Example:**
```python
combined = df1.append(df2)
```
</div>

### `PyGroupedDataFrame`

Represents a grouped DataFrame for aggregation operations.

#### Aggregation Methods

<div className="api-section">
<div className="api-method">sum() -&gt; PyDataFrame</div>

Calculates the sum for each group.

**Example:**
```python
grouped = df.group_by(["department"])
sums = grouped.sum()
```
</div>

<div className="api-section">
<div className="api-method">mean() -&gt; PyDataFrame</div>

Calculates the mean for each group.

**Example:**
```python
averages = grouped.mean()
```
</div>

<div className="api-section">
<div className="api-method">count() -&gt; PyDataFrame</div>

Counts values for each group.

**Example:**
```python
counts = grouped.count()
```
</div>

<div className="api-section">
<div className="api-method">min() -&gt; PyDataFrame</div>

Finds the minimum value for each group.

**Example:**
```python
minimums = grouped.min()
```
</div>

<div className="api-section">
<div className="api-method">max() -&gt; PyDataFrame</div>

Finds the maximum value for each group.

**Example:**
```python
maximums = grouped.max()
```
</div>

<div className="api-section">
<div className="api-method">agg(aggregations: List[Tuple[str, str]]) -&gt; PyDataFrame</div>

Performs custom aggregations.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">aggregations</span>: <span className="parameter-type">List[Tuple[str, str]]</span> - List of (column, aggregation_function) tuples
</div>
</div>

**Example:**
```python
result = grouped.agg([
    ("salary", "mean"),
    ("age", "count"),
    ("experience", "max")
])
```
</div>

### `PySeries`

Represents a single column of data.

#### Constructors

<div className="api-section">
<div className="api-method">PySeries(name: str, data: List[Any])</div>

Creates a new Series with automatic type inference.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">str</span> - Name of the series
</div>
<div className="api-parameter">
<span className="parameter-name">data</span>: <span className="parameter-type">List[Any]</span> - List of values (supports None for nulls)
</div>
</div>

**Example:**
```python
# Integer series
ages = vx.PySeries("age", [25, 30, None, 35])

# String series  
names = vx.PySeries("name", ["Alice", "Bob", None, "Charlie"])

# Float series
salaries = vx.PySeries("salary", [50000.0, 75000.0, 60000.0])

# Boolean series
active = vx.PySeries("is_active", [True, False, True])
```
</div>

#### Properties

<div className="api-section">
<div className="api-method">name() -&gt; str</div>

Returns the name of the Series.

**Example:**
```python
print(f"Series name: {series.name()}")
```
</div>

<div className="api-section">
<div className="api-method">len() -&gt; int</div>

Returns the length of the Series.

**Example:**
```python
print(f"Series has {series.len()} values")
```
</div>

<div className="api-section">
<div className="api-method">is_empty() -&gt; bool</div>

Checks if the Series is empty.

**Example:**
```python
if series.is_empty():
    print("Series is empty")
```
</div>

<div className="api-section">
<div className="api-method">data_type() -&gt; PyDataType</div>

Returns the data type of the Series.

**Example:**
```python
dtype = series.data_type()
print(f"Series type: {dtype}")
```
</div>

#### Data Access

<div className="api-section">
<div className="api-method">get_value(index: int) -&gt; Any</div>

Gets the value at a specific index.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">index</span>: <span className="parameter-type">int</span> - Index of the value to retrieve
</div>
</div>

**Example:**
```python
first_value = series.get_value(0)
print(f"First value: {first_value}")
```
</div>

<div className="api-section">
<div className="api-method">to_list() -&gt; List[Any]</div>

Converts the Series to a Python list.

**Example:**
```python
values = series.to_list()
for value in values:
    if value is not None:
        print(value)
```
</div>

#### Statistical Methods

<div className="api-section">
<div className="api-method">sum() -&gt; float</div>

Calculates the sum of numeric values.

**Example:**
```python
total = series.sum()
print(f"Sum: {total}")
```
</div>

<div className="api-section">
<div className="api-method">mean() -&gt; float</div>

Calculates the mean of numeric values.

**Example:**
```python
average = series.mean()
print(f"Average: {average}")
```
</div>

<div className="api-section">
<div className="api-method">median() -&gt; float</div>

Calculates the median of numeric values.

**Example:**
```python
median = series.median()
print(f"Median: {median}")
```
</div>

<div className="api-section">
<div className="api-method">min() -&gt; Any</div>

Finds the minimum value.

**Example:**
```python
minimum = series.min()
print(f"Minimum: {minimum}")
```
</div>

<div className="api-section">
<div className="api-method">max() -&gt; Any</div>

Finds the maximum value.

**Example:**
```python
maximum = series.max()
print(f"Maximum: {maximum}")
```
</div>

<div className="api-section">
<div className="api-method">std() -&gt; float</div>

Calculates the standard deviation.

**Example:**
```python
std_dev = series.std()
print(f"Standard deviation: {std_dev}")
```
</div>

<div className="api-section">
<div className="api-method">count() -&gt; int</div>

Counts non-null values.

**Example:**
```python
non_null_count = series.count()
print(f"Non-null values: {non_null_count}")
```
</div>

<div className="api-section">
<div className="api-method">unique() -&gt; PySeries</div>

Returns a Series with unique values.

**Example:**
```python
unique_values = series.unique()
print(f"Unique values: {unique_values.len()}")
```
</div>

#### Data Manipulation

<div className="api-section">
<div className="api-method">filter(row_indices: List[int]) -&gt; PySeries</div>

Filters the Series by index positions.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">row_indices</span>: <span className="parameter-type">List[int]</span> - List of indices to keep
</div>
</div>

**Example:**
```python
filtered = series.filter([0, 2, 4])  # Keep indices 0, 2, 4
```
</div>

<div className="api-section">
<div className="api-method">fill_nulls(value: Any) -&gt; PySeries</div>

Fills null values with a specified value.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">value</span>: <span className="parameter-type">Any</span> - Value to use for filling nulls
</div>
</div>

**Example:**
```python
filled = series.fill_nulls(0)
```
</div>

### `PyExpr`

Represents expressions for computed columns.

#### Static Methods

<div className="api-section">
<div className="api-method">@staticmethod column(name: str) -&gt; PyExpr</div>

Creates a column reference expression.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">str</span> - Name of the column to reference
</div>
</div>

**Example:**
```python
expr = vx.PyExpr.column("salary")
```
</div>

<div className="api-section">
<div className="api-method">@staticmethod literal(value: Any) -&gt; PyExpr</div>

Creates a literal value expression.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">value</span>: <span className="parameter-type">Any</span> - The literal value
</div>
</div>

**Example:**
```python
expr = vx.PyExpr.literal(1000.0)
```
</div>

#### Arithmetic Operations

<div className="api-section">
<div className="api-method">@staticmethod add(left: PyExpr, right: PyExpr) -&gt; PyExpr</div>

Creates an addition expression.

**Example:**
```python
expr = vx.PyExpr.add(
    vx.PyExpr.column("base_salary"),
    vx.PyExpr.column("bonus")
)
```
</div>

<div className="api-section">
<div className="api-method">@staticmethod subtract(left: PyExpr, right: PyExpr) -&gt; PyExpr</div>

Creates a subtraction expression.

**Example:**
```python
expr = vx.PyExpr.subtract(
    vx.PyExpr.column("revenue"),
    vx.PyExpr.column("costs")
)
```
</div>

<div className="api-section">
<div className="api-method">@staticmethod multiply(left: PyExpr, right: PyExpr) -&gt; PyExpr</div>

Creates a multiplication expression.

**Example:**
```python
expr = vx.PyExpr.multiply(
    vx.PyExpr.column("quantity"),
    vx.PyExpr.column("price")
)
```
</div>

<div className="api-section">
<div className="api-method">@staticmethod divide(left: PyExpr, right: PyExpr) -&gt; PyExpr</div>

Creates a division expression.

**Example:**
```python
expr = vx.PyExpr.divide(
    vx.PyExpr.column("total_sales"),
    vx.PyExpr.column("num_customers")
)
```
</div>

### `PyJoinType`

Enumeration for join types.

```python
class PyJoinType:
    Inner = "Inner"
    Left = "Left" 
    Right = "Right"
```

**Example:**
```python
joined = df1.join(df2, "user_id", vx.PyJoinType.Left)
```

## Convenience Functions

### Data Loading

<div className="api-section">
<div className="api-method">read_csv(path: str) -&gt; PyDataFrame</div>

Convenience function to load CSV files.

**Example:**
```python
import veloxx as vx

df = vx.read_csv("data.csv")
```
</div>

<div className="api-section">
<div className="api-method">read_json(path: str) -&gt; PyDataFrame</div>

Convenience function to load JSON files.

**Example:**
```python
df = vx.read_json("data.json")
```
</div>

## Usage Patterns

### Basic Data Analysis

```python
import veloxx as vx

# Load data
df = vx.read_csv("sales_data.csv")

# Basic info
print(f"Dataset: {df.row_count()} rows, {df.column_count()} columns")
print(f"Columns: {df.column_names()}")

# Filter high-value sales
high_value_indices = []
amount_series = df.get_column("amount")
for i, amount in enumerate(amount_series.to_list()):
    if amount and amount > 1000:
        high_value_indices.append(i)

high_value_sales = df.filter(high_value_indices)

# Group by and aggregate
summary = high_value_sales.group_by(["region"]).agg([
    ("amount", "sum"),
    ("amount", "mean"),
    ("customer_id", "count")
])

print(summary)
```

### Advanced Analytics

```python
import veloxx as vx

def analyze_customer_data():
    # Load customer data
    customers = vx.read_csv("customers.csv")
    orders = vx.read_csv("orders.csv")
    
    # Join datasets
    customer_orders = customers.join(orders, "customer_id", vx.PyJoinType.Inner)
    
    # Calculate customer lifetime value
    clv_expr = vx.PyExpr.multiply(
        vx.PyExpr.column("order_value"),
        vx.PyExpr.column("order_frequency")
    )
    
    with_clv = customer_orders.with_column("lifetime_value", clv_expr)
    
    # Segment customers
    high_value_indices = []
    clv_series = with_clv.get_column("lifetime_value")
    for i, clv in enumerate(clv_series.to_list()):
        if clv and clv > 5000:
            high_value_indices.append(i)
    
    high_value_customers = with_clv.filter(high_value_indices)
    
    # Analyze by segment
    segment_analysis = high_value_customers.group_by(["customer_segment"]).agg([
        ("lifetime_value", "mean"),
        ("order_frequency", "mean"),
        ("customer_id", "count")
    ])
    
    return segment_analysis

# Run analysis
results = analyze_customer_data()
print(results)
```

### Data Cleaning Pipeline

```python
import veloxx as vx

def clean_dataset(df):
    """Clean and prepare dataset for analysis"""
    
    # Remove rows with missing critical data
    clean_df = df.drop_nulls()
    
    # Fill missing values in optional columns
    filled_df = clean_df.fill_nulls("Unknown")
    
    # Remove outliers (example: ages > 100)
    age_series = filled_df.get_column("age")
    valid_indices = []
    for i, age in enumerate(age_series.to_list()):
        if age and 0 <= age <= 100:
            valid_indices.append(i)
    
    filtered_df = filled_df.filter(valid_indices)
    
    # Standardize column names
    standardized = filtered_df.rename_column("customer_name", "name")
    standardized = standardized.rename_column("customer_age", "age")
    
    return standardized

# Usage
raw_data = vx.read_csv("raw_customer_data.csv")
clean_data = clean_dataset(raw_data)
clean_data.to_csv("clean_customer_data.csv")
```

## Performance Tips

1. **Use appropriate data types**: Let Veloxx infer types automatically for best performance
2. **Filter early**: Apply filters before expensive operations like joins
3. **Use vectorized operations**: Leverage expressions instead of loops
4. **Process in chunks**: For very large datasets, process in smaller chunks
5. **Minimize data copying**: Chain operations when possible

## Error Handling

```python
import veloxx as vx

try:
    df = vx.read_csv("data.csv")
    result = df.group_by(["category"]).mean()
    result.to_csv("output.csv")
except FileNotFoundError:
    print("Input file not found")
except Exception as e:
    print(f"Error processing data: {e}")
```

## Integration with Pandas

Convert between Veloxx and Pandas for interoperability:

```python
import veloxx as vx
import pandas as pd

# Pandas to Veloxx
def pandas_to_veloxx(pandas_df):
    columns = {}
    for col in pandas_df.columns:
        data = pandas_df[col].tolist()
        # Convert NaN to None
        data = [None if pd.isna(x) else x for x in data]
        columns[col] = vx.PySeries(col, data)
    return vx.PyDataFrame(columns)

# Veloxx to Pandas
def veloxx_to_pandas(veloxx_df):
    data = {}
    for col_name in veloxx_df.column_names():
        series = veloxx_df.get_column(col_name)
        data[col_name] = series.to_list()
    return pd.DataFrame(data)

# Usage
pandas_df = pd.read_csv("data.csv")
veloxx_df = pandas_to_veloxx(pandas_df)

# Process with Veloxx (faster)
result = veloxx_df.group_by(["category"]).mean()

# Convert back to Pandas if needed
result_pandas = veloxx_to_pandas(result)
```