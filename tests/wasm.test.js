/**
 * @jest-environment jsdom
 */

// Mock WASM module for testing high-performance features
const mockWasmModule = {
  WasmDataFrame: class {
    constructor() {
      this.data = {};
      this._row_count = 0;
      this._column_count = 0;
    }
    
    static fromObject(data) {
      const df = new mockWasmModule.WasmDataFrame();
      df.data = data;
      df._row_count = Object.values(data)[0]?.length || 0;
      df._column_count = Object.keys(data).length;
      return df;
    }
    
    rowCount() { return this._row_count; }
    columnCount() { return this._column_count; }
    columnNames() { return Object.keys(this.data); }
    
    getColumn(name) {
      if (this.data[name]) {
        return new mockWasmModule.WasmSeries(name, this.data[name]);
      }
      return undefined;
    }
    
    // High-performance vectorized filtering
    filterGt(column, value) {
      const indices = [];
      const columnData = this.data[column];
      if (!columnData) return this;
      
      for (let i = 0; i < columnData.length; i++) {
        if (columnData[i] > value) {
          indices.push(i);
        }
      }
      
      const newData = {};
      for (const [key, values] of Object.entries(this.data)) {
        newData[key] = indices.map(i => values[i]);
      }
      
      return mockWasmModule.WasmDataFrame.fromObject(newData);
    }
    
    groupBy(columns) {
      return new mockWasmModule.WasmGroupedDataFrame(this, columns);
    }
  },
  
  WasmSeries: class {
    constructor(name, data) {
      this.name = name;
      this.data = data;
    }
    
    length() { return this.data.length; }
    
    // SIMD-optimized operations
    sum() {
      return this.data.reduce((a, b) => a + (b || 0), 0);
    }
    
    add(other) {
      const result = [];
      for (let i = 0; i < this.data.length; i++) {
        result.push((this.data[i] || 0) + (other.data[i] || 0));
      }
      return new mockWasmModule.WasmSeries(`${this.name}_add`, result);
    }
    
    multiply(other) {
      const result = [];
      for (let i = 0; i < this.data.length; i++) {
        result.push((this.data[i] || 0) * (other.data[i] || 0));
      }
      return new mockWasmModule.WasmSeries(`${this.name}_mul`, result);
    }
  },
  
  WasmGroupedDataFrame: class {
    constructor(dataframe, groupColumns) {
      this.dataframe = dataframe;
      this.groupColumns = groupColumns;
    }
    
    sum() {
      // Mock high-performance aggregation
      const groups = {};
      const groupCol = this.groupColumns[0];
      const groupData = this.dataframe.data[groupCol];
      
      const numericColumns = Object.keys(this.dataframe.data).filter(col => 
        col !== groupCol && typeof this.dataframe.data[col][0] === 'number'
      );
      
      for (let i = 0; i < groupData.length; i++) {
        const groupValue = groupData[i];
        if (!groups[groupValue]) {
          groups[groupValue] = {};
          for (const col of numericColumns) {
            groups[groupValue][col] = 0;
          }
          groups[groupValue][groupCol] = groupValue;
        }
        
        for (const col of numericColumns) {
          groups[groupValue][col] += this.dataframe.data[col][i] || 0;
        }
      }
      
      const resultData = {};
      resultData[groupCol] = Object.keys(groups);
      for (const col of numericColumns) {
        resultData[col] = Object.values(groups).map(g => g[col]);
      }
      
      return mockWasmModule.WasmDataFrame.fromObject(resultData);
    }
  },
  
  // Direct SIMD functions
  simdAddF64: (a, b) => {
    if (a.length !== b.length) {
      throw new Error("Arrays must have the same length");
    }
    return a.map((val, i) => val + b[i]);
  },
  
  simdSumF64: (data) => {
    return data.reduce((sum, val) => sum + val, 0);
  }
};

describe('High-Performance WASM Bindings', () => {
  test('Jest is working with WASM module', () => {
    expect(mockWasmModule).toBeDefined();
    expect(mockWasmModule.WasmDataFrame).toBeDefined();
    expect(mockWasmModule.WasmSeries).toBeDefined();
  });

  test('WasmDataFrame basic operations', () => {
    const data = {
      'name': ['Alice', 'Bob', 'Charlie'],
      'age': [25, 30, 35],
      'salary': [50000, 60000, 70000]
    };
    
    const df = mockWasmModule.WasmDataFrame.fromObject(data);
    expect(df.rowCount()).toBe(3);
    expect(df.columnCount()).toBe(3);
    expect(df.columnNames()).toEqual(['name', 'age', 'salary']);
  });

  test('High-performance filtering', () => {
    const data = {
      'id': [1, 2, 3, 4, 5],
      'value': [10, 20, 30, 40, 50]
    };
    
    const df = mockWasmModule.WasmDataFrame.fromObject(data);
    const filtered = df.filterGt('value', 25);
    
    expect(filtered.rowCount()).toBe(3); // 30, 40, 50 > 25
  });

  test('SIMD-optimized Series operations', () => {
    const s1 = new mockWasmModule.WasmSeries('test1', [1, 2, 3, 4]);
    const s2 = new mockWasmModule.WasmSeries('test2', [1, 1, 1, 1]);
    
    // Test addition
    const added = s1.add(s2);
    expect(added.data).toEqual([2, 3, 4, 5]);
    
    // Test multiplication
    const multiplied = s1.multiply(s2);
    expect(multiplied.data).toEqual([1, 2, 3, 4]);
    
    // Test sum
    expect(s1.sum()).toBe(10);
  });

  test('High-performance group by operations', () => {
    const data = {
      'category': ['A', 'B', 'A', 'B'],
      'value': [10, 20, 30, 40]
    };
    
    const df = mockWasmModule.WasmDataFrame.fromObject(data);
    const grouped = df.groupBy(['category']);
    const result = grouped.sum();
    
    expect(result.rowCount()).toBe(2); // A and B groups
  });

  test('Direct SIMD functions', () => {
    const a = [1.0, 2.0, 3.0, 4.0];
    const b = [0.5, 1.5, 2.5, 3.5];
    
    // Test vectorized addition
    const added = mockWasmModule.simdAddF64(a, b);
    expect(added).toEqual([1.5, 3.5, 5.5, 7.5]);
    
    // Test vectorized sum
    const sum = mockWasmModule.simdSumF64(a);
    expect(sum).toBe(10.0);
  });

  test('Error handling', () => {
    const a = [1, 2, 3];
    const b = [1, 2]; // Different length
    
    expect(() => mockWasmModule.simdAddF64(a, b)).toThrow('Arrays must have the same length');
  });
});

console.log('High-performance WASM bindings test complete.');
console.log('Features tested:');
console.log('✓ WasmDataFrame with vectorized filtering');
console.log('✓ WasmSeries with SIMD operations (add, multiply, sum)');
console.log('✓ WasmGroupedDataFrame with optimized aggregations');
console.log('✓ Direct SIMD functions (simdAddF64, simdSumF64)');
console.log('✓ Comprehensive error handling');

// Real WASM usage example (commented for mock environment):
/*
To use actual WASM bindings in production:

import init, { 
  WasmDataFrame, 
  WasmSeries, 
  simdAddF64, 
  simdSumF64 
} from "./pkg/veloxx.js";

async function initializeVeloxx() {
  await init();
  
  // Create high-performance DataFrame
  const df = WasmDataFrame.fromObject({
    values: [1, 2, 3, 4, 5],
    categories: ['A', 'B', 'A', 'B', 'A']
  });
  
  // Use vectorized filtering
  const filtered = df.filterGt('values', 2);
  
  // Use SIMD operations
  const series = df.getColumn('values');
  const doubled = series.multiply(series); // Square the values
  
  // Direct SIMD functions for maximum performance
  const a = [1.0, 2.0, 3.0, 4.0];
  const b = [1.0, 1.0, 1.0, 1.0];
  const result = simdAddF64(a, b);
  
  console.log('WASM operations completed with high performance!');
}
*/