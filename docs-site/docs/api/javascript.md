# JavaScript API Reference

Complete API reference for Veloxx JavaScript/WebAssembly bindings.

## Installation

### Using npm

```bash
npm install veloxx-wasm
```

### Using CDN

```html
<script src="https://unpkg.com/veloxx-wasm/veloxx.js"></script>
```

## Quick Start

```javascript
import * as veloxx from 'veloxx-wasm';

// Initialize the WASM module
await veloxx.default();

// Create a DataFrame
const df = new veloxx.WasmDataFrame({
    name: ["Alice", "Bob", "Charlie"],
    age: [25, 30, 35],
    salary: [50000.0, 75000.0, 60000.0]
});

// Basic operations
const filtered = df.filter([0, 2]); // Filter rows by indices
const selected = df.selectColumns(["name", "salary"]);
console.log(`DataFrame has ${df.row_count} rows and ${df.column_count} columns`);
```

## Core Classes

### `WasmDataFrame`

The main data structure for working with tabular data in JavaScript.

#### Constructor

<div className="api-section">
<div className="api-method">new WasmDataFrame(columns: Object)</div>

Creates a new DataFrame from an object where keys are column names and values are arrays of data.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">columns</span>: <span className="parameter-type">Object</span> - Object mapping column names to data arrays
</div>
</div>

**Example:**
```javascript
const df = new veloxx.WasmDataFrame({
    name: ["Alice", "Bob", "Charlie"],
    age: [25, 30, 35],
    active: [true, false, true],
    salary: [50000.0, 75000.0, 60000.0]
});
```
</div>

#### Properties

<div className="api-section">
<div className="api-method">row_count: number</div>

Returns the number of rows in the DataFrame.

**Example:**
```javascript
console.log(`DataFrame has ${df.row_count} rows`);
```
</div>

<div className="api-section">
<div className="api-method">column_count: number</div>

Returns the number of columns in the DataFrame.

**Example:**
```javascript
console.log(`DataFrame has ${df.column_count} columns`);
```
</div>

#### Methods

<div className="api-section">
<div className="api-method">columnNames(): string[]</div>

Returns an array of column names.

**Example:**
```javascript
const names = df.columnNames();
names.forEach(name => console.log(`Column: ${name}`));
```
</div>

<div className="api-section">
<div className="api-method">getColumn(name: string): WasmSeries | null</div>

Gets a column by name.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">string</span> - Name of the column to retrieve
</div>
</div>

**Example:**
```javascript
const ageColumn = df.getColumn("age");
if (ageColumn) {
    console.log(`Age column has ${ageColumn.len} values`);
}
```
</div>

#### Data Manipulation

<div className="api-section">
<div className="api-method">filter(rowIndices: number[]): WasmDataFrame</div>

Filters rows by index positions.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">rowIndices</span>: <span className="parameter-type">number[]</span> - Array of row indices to keep
</div>
</div>

**Example:**
```javascript
// Keep only rows 0 and 2
const filtered = df.filter([0, 2]);
```
</div>

<div className="api-section">
<div className="api-method">selectColumns(names: string[]): WasmDataFrame</div>

Selects specific columns from the DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">names</span>: <span className="parameter-type">string[]</span> - Names of columns to select
</div>
</div>

**Example:**
```javascript
const selected = df.selectColumns(["name", "age"]);
```
</div>

<div className="api-section">
<div className="api-method">dropColumns(names: string[]): WasmDataFrame</div>

Removes specified columns from the DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">names</span>: <span className="parameter-type">string[]</span> - Names of columns to drop
</div>
</div>

**Example:**
```javascript
const withoutId = df.dropColumns(["id"]);
```
</div>

<div className="api-section">
<div className="api-method">renameColumn(oldName: string, newName: string): WasmDataFrame</div>

Renames a column in the DataFrame.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">oldName</span>: <span className="parameter-type">string</span> - Current name of the column
</div>
<div className="api-parameter">
<span className="parameter-name">newName</span>: <span className="parameter-type">string</span> - New name for the column
</div>
</div>

**Example:**
```javascript
const renamed = df.renameColumn("age", "years");
```
</div>

<div className="api-section">
<div className="api-method">withColumn(name: string, expr: WasmExpr): WasmDataFrame</div>

Adds a new column or replaces an existing one using an expression.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">string</span> - Name of the new column
</div>
<div className="api-parameter">
<span className="parameter-name">expr</span>: <span className="parameter-type">WasmExpr</span> - Expression to compute the column values
</div>
</div>

**Example:**
```javascript
// Add a column with salary + 1000 bonus
const salaryCol = veloxx.WasmExpr.column("salary");
const bonus = veloxx.WasmExpr.literal(new veloxx.WasmValue(1000.0));
const expr = veloxx.WasmExpr.add(salaryCol, bonus);
const withBonus = df.withColumn("salary_with_bonus", expr);
```
</div>

#### Grouping and Aggregation

<div className="api-section">
<div className="api-method">groupBy(byColumns: string[]): WasmGroupedDataFrame</div>

Groups the DataFrame by specified columns.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">byColumns</span>: <span className="parameter-type">string[]</span> - Columns to group by
</div>
</div>

**Example:**
```javascript
const grouped = df.groupBy(["department"]);
const result = grouped.agg([["salary", "mean"], ["age", "count"]]);
```
</div>

<div className="api-section">
<div className="api-method">describe(): WasmDataFrame</div>

Generates descriptive statistics for numeric columns.

**Example:**
```javascript
const stats = df.describe();
console.log(stats);
```
</div>

#### Statistical Methods

<div className="api-section">
<div className="api-method">correlation(col1Name: string, col2Name: string): number</div>

Calculates the Pearson correlation between two numeric columns.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">col1Name</span>: <span className="parameter-type">string</span> - Name of the first column
</div>
<div className="api-parameter">
<span className="parameter-name">col2Name</span>: <span className="parameter-type">string</span> - Name of the second column
</div>
</div>

**Example:**
```javascript
const corr = df.correlation("age", "salary");
console.log(`Age-Salary correlation: ${corr.toFixed(3)}`);
```
</div>

<div className="api-section">
<div className="api-method">covariance(col1Name: string, col2Name: string): number</div>

Calculates the covariance between two numeric columns.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">col1Name</span>: <span className="parameter-type">string</span> - Name of the first column
</div>
<div className="api-parameter">
<span className="parameter-name">col2Name</span>: <span className="parameter-type">string</span> - Name of the second column
</div>
</div>

**Example:**
```javascript
const cov = df.covariance("age", "salary");
console.log(`Age-Salary covariance: ${cov.toFixed(2)}`);
```
</div>

#### Data Cleaning

<div className="api-section">
<div className="api-method">dropNulls(): WasmDataFrame</div>

Removes rows containing any null values.

**Example:**
```javascript
const cleanDf = df.dropNulls();
```
</div>

<div className="api-section">
<div className="api-method">fillNulls(value: WasmValue): WasmDataFrame</div>

Fills null values with a specified value.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">value</span>: <span className="parameter-type">WasmValue</span> - Value to use for filling nulls
</div>
</div>

**Example:**
```javascript
const filled = df.fillNulls(new veloxx.WasmValue(0)); // Fill with 0
const filledStr = df.fillNulls(new veloxx.WasmValue("Unknown")); // Fill with string
```
</div>

#### Concatenation

<div className="api-section">
<div className="api-method">append(other: WasmDataFrame): WasmDataFrame</div>

Appends another DataFrame vertically.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">other</span>: <span className="parameter-type">WasmDataFrame</span> - DataFrame to append
</div>
</div>

**Example:**
```javascript
const combined = df1.append(df2);
```
</div>

### `WasmSeries`

Represents a single column of data with a specific type.

#### Constructor

<div className="api-section">
<div className="api-method">new WasmSeries(name: string, data: any[])</div>

Creates a new Series with automatic type inference.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">string</span> - Name of the series
</div>
<div className="api-parameter">
<span className="parameter-name">data</span>: <span className="parameter-type">any[]</span> - Array of values
</div>
</div>

**Example:**
```javascript
const ages = new veloxx.WasmSeries("age", [25, 30, null, 35]);
const names = new veloxx.WasmSeries("name", ["Alice", "Bob", "Charlie"]);
const active = new veloxx.WasmSeries("active", [true, false, true]);
```
</div>

#### Properties

<div className="api-section">
<div className="api-method">name: string</div>

Returns the name of the Series.

**Example:**
```javascript
console.log(`Series name: ${series.name}`);
```
</div>

<div className="api-section">
<div className="api-method">len: number</div>

Returns the length of the Series.

**Example:**
```javascript
console.log(`Series has ${series.len} values`);
```
</div>

<div className="api-section">
<div className="api-method">isEmpty: boolean</div>

Returns true if the Series is empty.

**Example:**
```javascript
if (series.isEmpty) {
    console.log("Series is empty");
}
```
</div>

<div className="api-section">
<div className="api-method">dataType: WasmDataType</div>

Returns the data type of the Series.

**Example:**
```javascript
console.log(`Data type: ${series.dataType}`);
```
</div>

#### Methods

<div className="api-section">
<div className="api-method">getValue(index: number): any</div>

Gets the value at the specified index.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">index</span>: <span className="parameter-type">number</span> - Index position
</div>
</div>

**Example:**
```javascript
const value = series.getValue(0);
console.log(`First value: ${value}`);
```
</div>

<div className="api-section">
<div className="api-method">setName(newName: string): void</div>

Sets a new name for the Series.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">newName</span>: <span className="parameter-type">string</span> - New name for the series
</div>
</div>

**Example:**
```javascript
series.setName("new_column_name");
```
</div>

<div className="api-section">
<div className="api-method">filter(rowIndices: number[]): WasmSeries</div>

Filters the Series by index positions.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">rowIndices</span>: <span className="parameter-type">number[]</span> - Array of indices to keep
</div>
</div>

**Example:**
```javascript
const filtered = series.filter([0, 2, 4]);
```
</div>

<div className="api-section">
<div className="api-method">cast(toType: WasmDataType): WasmSeries</div>

Casts the Series to a different data type.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">toType</span>: <span className="parameter-type">WasmDataType</span> - Target data type
</div>
</div>

**Example:**
```javascript
const asFloat = series.cast(veloxx.WasmDataType.F64);
```
</div>

<div className="api-section">
<div className="api-method">append(other: WasmSeries): WasmSeries</div>

Appends another Series to this one.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">other</span>: <span className="parameter-type">WasmSeries</span> - Series to append
</div>
</div>

**Example:**
```javascript
const combined = series1.append(series2);
```
</div>

#### Statistical Methods

<div className="api-section">
<div className="api-method">count(): number</div>

Returns the count of non-null values.

**Example:**
```javascript
const nonNullCount = series.count();
```
</div>

<div className="api-section">
<div className="api-method">min(): any | null</div>

Returns the minimum value in the Series.

**Example:**
```javascript
const minValue = series.min();
console.log(`Minimum: ${minValue}`);
```
</div>

<div className="api-section">
<div className="api-method">max(): any | null</div>

Returns the maximum value in the Series.

**Example:**
```javascript
const maxValue = series.max();
console.log(`Maximum: ${maxValue}`);
```
</div>

<div className="api-section">
<div className="api-method">median(): number | null</div>

Returns the median value for numeric Series.

**Example:**
```javascript
const medianValue = series.median();
console.log(`Median: ${medianValue}`);
```
</div>

<div className="api-section">
<div className="api-method">stdDev(): number | null</div>

Returns the standard deviation for numeric Series.

**Example:**
```javascript
const stdDev = series.stdDev();
console.log(`Standard Deviation: ${stdDev}`);
```
</div>

<div className="api-section">
<div className="api-method">correlation(other: WasmSeries): number | null</div>

Calculates the correlation with another numeric Series.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">other</span>: <span className="parameter-type">WasmSeries</span> - Other series to correlate with
</div>
</div>

**Example:**
```javascript
const corr = ageSeries.correlation(salarySeries);
console.log(`Correlation: ${corr}`);
```
</div>

<div className="api-section">
<div className="api-method">covariance(other: WasmSeries): number | null</div>

Calculates the covariance with another numeric Series.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">other</span>: <span className="parameter-type">WasmSeries</span> - Other series to calculate covariance with
</div>
</div>

**Example:**
```javascript
const cov = ageSeries.covariance(salarySeries);
console.log(`Covariance: ${cov}`);
```
</div>

### `WasmGroupedDataFrame`

Represents a grouped DataFrame for aggregation operations.

#### Methods

<div className="api-section">
<div className="api-method">agg(aggregations: [string, string][]): WasmDataFrame</div>

Performs aggregation operations on grouped data.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">aggregations</span>: <span className="parameter-type">[string, string][]</span> - Array of [column, operation] pairs
</div>
</div>

**Available operations:** `"sum"`, `"mean"`, `"count"`, `"min"`, `"max"`, `"std"`

**Example:**
```javascript
const grouped = df.groupBy(["department"]);
const result = grouped.agg([
    ["salary", "mean"],
    ["age", "count"],
    ["bonus", "sum"]
]);
```
</div>

### `WasmExpr`

Represents expressions for creating computed columns.

#### Static Methods

<div className="api-section">
<div className="api-method">WasmExpr.column(name: string): WasmExpr</div>

Creates an expression that references a column.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">name</span>: <span className="parameter-type">string</span> - Column name
</div>
</div>

**Example:**
```javascript
const ageExpr = veloxx.WasmExpr.column("age");
```
</div>

<div className="api-section">
<div className="api-method">WasmExpr.literal(value: WasmValue): WasmExpr</div>

Creates an expression with a literal value.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">value</span>: <span className="parameter-type">WasmValue</span> - Literal value
</div>
</div>

**Example:**
```javascript
const literalExpr = veloxx.WasmExpr.literal(new veloxx.WasmValue(100));
```
</div>

#### Arithmetic Operations

<div className="api-section">
<div className="api-method">WasmExpr.add(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates an addition expression.

**Example:**
```javascript
const sum = veloxx.WasmExpr.add(
    veloxx.WasmExpr.column("salary"),
    veloxx.WasmExpr.literal(new veloxx.WasmValue(1000))
);
```
</div>

<div className="api-section">
<div className="api-method">WasmExpr.subtract(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a subtraction expression.

**Example:**
```javascript
const diff = veloxx.WasmExpr.subtract(
    veloxx.WasmExpr.column("revenue"),
    veloxx.WasmExpr.column("cost")
);
```
</div>

<div className="api-section">
<div className="api-method">WasmExpr.multiply(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a multiplication expression.

**Example:**
```javascript
const product = veloxx.WasmExpr.multiply(
    veloxx.WasmExpr.column("price"),
    veloxx.WasmExpr.column("quantity")
);
```
</div>

<div className="api-section">
<div className="api-method">WasmExpr.divide(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a division expression.

**Example:**
```javascript
const ratio = veloxx.WasmExpr.divide(
    veloxx.WasmExpr.column("profit"),
    veloxx.WasmExpr.column("revenue")
);
```
</div>

#### Comparison Operations

<div className="api-section">
<div className="api-method">WasmExpr.equals(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates an equality comparison expression.

**Example:**
```javascript
const isActive = veloxx.WasmExpr.equals(
    veloxx.WasmExpr.column("status"),
    veloxx.WasmExpr.literal(new veloxx.WasmValue("active"))
);
```
</div>

<div className="api-section">
<div className="api-method">WasmExpr.notEquals(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a not-equals comparison expression.
</div>

<div className="api-section">
<div className="api-method">WasmExpr.greaterThan(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a greater-than comparison expression.
</div>

<div className="api-section">
<div className="api-method">WasmExpr.lessThan(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a less-than comparison expression.
</div>

<div className="api-section">
<div className="api-method">WasmExpr.greaterThanOrEqual(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a greater-than-or-equal comparison expression.
</div>

<div className="api-section">
<div className="api-method">WasmExpr.lessThanOrEqual(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a less-than-or-equal comparison expression.
</div>

#### Logical Operations

<div className="api-section">
<div className="api-method">WasmExpr.and(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a logical AND expression.

**Example:**
```javascript
const condition = veloxx.WasmExpr.and(
    veloxx.WasmExpr.greaterThan(
        veloxx.WasmExpr.column("age"),
        veloxx.WasmExpr.literal(new veloxx.WasmValue(25))
    ),
    veloxx.WasmExpr.equals(
        veloxx.WasmExpr.column("department"),
        veloxx.WasmExpr.literal(new veloxx.WasmValue("Engineering"))
    )
);
```
</div>

<div className="api-section">
<div className="api-method">WasmExpr.or(left: WasmExpr, right: WasmExpr): WasmExpr</div>

Creates a logical OR expression.
</div>

<div className="api-section">
<div className="api-method">WasmExpr.not(expr: WasmExpr): WasmExpr</div>

Creates a logical NOT expression.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">expr</span>: <span className="parameter-type">WasmExpr</span> - Expression to negate
</div>
</div>
</div>

### `WasmValue`

Represents a typed value that can be used in expressions and operations.

#### Constructor

<div className="api-section">
<div className="api-method">new WasmValue(value: any)</div>

Creates a new WasmValue with automatic type inference.

<div className="api-parameters">
**Parameters:**
<div className="api-parameter">
<span className="parameter-name">value</span>: <span className="parameter-type">any</span> - JavaScript value (number, string, boolean, null)
</div>
</div>

**Example:**
```javascript
const intValue = new veloxx.WasmValue(42);
const floatValue = new veloxx.WasmValue(3.14);
const stringValue = new veloxx.WasmValue("hello");
const boolValue = new veloxx.WasmValue(true);
const nullValue = new veloxx.WasmValue(null);
```
</div>

#### Methods

<div className="api-section">
<div className="api-method">toJsValue(): any</div>

Converts the WasmValue back to a JavaScript value.

**Example:**
```javascript
const wasmVal = new veloxx.WasmValue(42);
const jsVal = wasmVal.toJsValue(); // Returns 42
```
</div>

### `WasmDataType`

Enumeration of supported data types.

#### Values

- `WasmDataType.I32` - 32-bit signed integer
- `WasmDataType.F64` - 64-bit floating point
- `WasmDataType.Bool` - Boolean
- `WasmDataType.String` - String
- `WasmDataType.DateTime` - DateTime (stored as timestamp)

**Example:**
```javascript
// Check data type
if (series.dataType === veloxx.WasmDataType.F64) {
    console.log("This is a numeric series");
}

// Cast to different type
const asString = series.cast(veloxx.WasmDataType.String);
```

## Advanced Examples

### Complex Data Processing Pipeline

```javascript
import * as veloxx from 'veloxx-wasm';

// Initialize WASM
await veloxx.default();

// Create sample data
const df = new veloxx.WasmDataFrame({
    employee_id: [1, 2, 3, 4, 5],
    name: ["Alice", "Bob", "Charlie", "Diana", "Eve"],
    department: ["Engineering", "Sales", "Engineering", "HR", "Sales"],
    salary: [75000, 65000, 80000, 60000, 70000],
    years_experience: [5, 3, 7, 2, 4],
    performance_score: [4.2, 3.8, 4.5, 3.9, 4.1]
});

// Calculate salary per year of experience
const salaryPerYear = veloxx.WasmExpr.divide(
    veloxx.WasmExpr.column("salary"),
    veloxx.WasmExpr.column("years_experience")
);
const enriched = df.withColumn("salary_per_year", salaryPerYear);

// Filter high performers (score > 4.0)
const highPerformers = enriched.filter(
    enriched.getColumn("performance_score")
        .filter([0, 2, 4]) // Indices where score > 4.0
);

// Group by department and calculate statistics
const departmentStats = highPerformers
    .groupBy(["department"])
    .agg([
        ["salary", "mean"],
        ["years_experience", "mean"],
        ["performance_score", "mean"]
    ]);

console.log(`High performer statistics by department:`);
console.log(departmentStats);
```

### Working with Missing Data

```javascript
// Create DataFrame with null values
const dfWithNulls = new veloxx.WasmDataFrame({
    id: [1, 2, 3, 4, 5],
    value: [10.5, null, 15.2, null, 20.1],
    category: ["A", "B", null, "A", "B"]
});

// Option 1: Drop rows with any null values
const cleanDf = dfWithNulls.dropNulls();

// Option 2: Fill nulls with specific values
const filledDf = dfWithNulls
    .fillNulls(new veloxx.WasmValue(0))  // Fill numeric nulls with 0
    .fillNulls(new veloxx.WasmValue("Unknown")); // Fill string nulls

// Option 3: Fill nulls per column type
const valueFilled = dfWithNulls.fillNulls(new veloxx.WasmValue(-1));
const categoryFilled = valueFilled.fillNulls(new veloxx.WasmValue("Missing"));
```

### Statistical Analysis

```javascript
// Create sample dataset
const salesData = new veloxx.WasmDataFrame({
    month: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
    revenue: [100000, 120000, 95000, 130000, 140000, 125000],
    costs: [60000, 70000, 55000, 75000, 80000, 70000],
    customers: [1000, 1200, 950, 1300, 1400, 1250]
});

// Calculate profit
const profit = veloxx.WasmExpr.subtract(
    veloxx.WasmExpr.column("revenue"),
    veloxx.WasmExpr.column("costs")
);
const withProfit = salesData.withColumn("profit", profit);

// Calculate correlations
const revenueCustomerCorr = withProfit.correlation("revenue", "customers");
const profitRevenueCorr = withProfit.correlation("profit", "revenue");

console.log(`Revenue-Customer correlation: ${revenueCustomerCorr.toFixed(3)}`);
console.log(`Profit-Revenue correlation: ${profitRevenueCorr.toFixed(3)}`);

// Generate descriptive statistics
const stats = withProfit.describe();
console.log("Descriptive Statistics:");
console.log(stats);
```

## Error Handling

All methods that can fail return JavaScript promises or throw exceptions. Always wrap operations in try-catch blocks:

```javascript
try {
    const df = new veloxx.WasmDataFrame({
        invalid_data: [1, "string", true] // Mixed types
    });
    
    const result = df.selectColumns(["nonexistent_column"]);
} catch (error) {
    console.error("Operation failed:", error.message);
}
```

## Performance Tips

1. **Batch Operations**: Combine multiple operations into a single chain when possible
2. **Avoid Frequent Type Conversions**: Keep data in consistent types
3. **Use Appropriate Data Types**: Choose the most specific type for your data
4. **Filter Early**: Apply filters before expensive operations like grouping
5. **Reuse Expressions**: Store complex expressions in variables for reuse

```javascript
// Good: Chain operations
const result = df
    .filter([0, 1, 2])
    .selectColumns(["name", "salary"])
    .groupBy(["department"])
    .agg([["salary", "mean"]]);

// Good: Reuse expressions
const salaryExpr = veloxx.WasmExpr.column("salary");
const bonusExpr = veloxx.WasmExpr.literal(new veloxx.WasmValue(1000));
const totalComp = veloxx.WasmExpr.add(salaryExpr, bonusExpr);
```

## Browser Compatibility

Veloxx WebAssembly bindings are compatible with:

- **Chrome/Edge**: 57+
- **Firefox**: 52+
- **Safari**: 11+
- **Node.js**: 12+

For older browsers, consider using a WebAssembly polyfill.

## TypeScript Support

Type definitions are included with the package:

```typescript
import * as veloxx from 'veloxx-wasm';

interface EmployeeData {
    name: string[];
    age: number[];
    salary: number[];
}

const data: EmployeeData = {
    name: ["Alice", "Bob"],
    age: [30, 25],
    salary: [75000, 65000]
};

const df: veloxx.WasmDataFrame = new veloxx.WasmDataFrame(data);
```