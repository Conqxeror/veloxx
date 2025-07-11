// Simple test for WASM bindings
// This file demonstrates that the WASM bindings can be used

// Note: This is a basic example showing the API structure
// To actually run this, you would need to:
// 1. Build the WASM package with wasm-pack
// 2. Set up a proper JavaScript environment
// 3. Import the generated WASM module

// Example usage (pseudo-code):
/*
import init, { WasmDataFrame, WasmSeries, WasmValue } from './pkg/veloxx.js';

async function testVeloxWasm() {
    // Initialize the WASM module
    await init();
    
    // Create test data
    const data = {
        'name': ['Alice', 'Bob', 'Charlie'],
        'age': [25, 30, 35],
        'salary': [50000.0, 60000.0, 70000.0]
    };
    
    // Create DataFrame
    const df = new WasmDataFrame(data);
    
    console.log('Row count:', df.row_count);
    console.log('Column count:', df.column_count);
    console.log('Column names:', df.columnNames());
    
    // Get a column
    const ageColumn = df.getColumn('age');
    if (ageColumn) {
        console.log('Age column length:', ageColumn.len);
        console.log('First age value:', ageColumn.getValue(0));
    }
    
    // Group by and aggregate
    const grouped = df.groupBy(['name']);
    const aggregated = grouped.agg([['salary', 'mean']]);
    
    console.log('Aggregated result row count:', aggregated.row_count);
}

// testVeloxWasm().catch(console.error);
*/

console.log('WASM bindings test file created. See comments for usage example.');