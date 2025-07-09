# Project Issues

This document outlines several issues identified in the project that require attention.

## 1. Redundant Wasm Bindings File

**Issue:** The file `bindings/wasm/mod.rs` appears to be a redundant and outdated version of the Wasm bindings. The primary and more up-to-date Wasm bindings are located in `src/wasm_bindings/mod.rs`. This duplication can lead to confusion and maintenance overhead.

**Recommendation:** Delete the `bindings/wasm/mod.rs` file and remove any corresponding references in the build configuration to ensure that only `src/wasm_bindings/mod.rs` is used.

## 2. Incorrect Python Bindings Path in `src/lib.rs`

**Issue:** The `src/lib.rs` file incorrectly points to `python_bindings.rs` for the Python bindings. The actual file is located at `bindings/python/mod.rs`. This will cause a compilation error when building the Python bindings.

**Recommendation:** Correct the path in `src/lib.rs` to `#[path = "bindings/python/mod.rs"]` to ensure the Python bindings compile correctly.

## 3. Inconsistent Module System in Wasm Tests

**Issue:** The Wasm tests in `pkg/test_wasm.test.js` use `require('veloxx')`, which is a CommonJS module import. However, the `pkg/package.json` specifies `"type": "module"`, indicating that the project should be using ES Modules (`import veloxx from 'veloxx'`). This inconsistency can cause issues with the test runner and module resolution.

**Recommendation:** Update the Wasm tests to use ES Module syntax (`import`) to align with the `package.json` configuration.

## 4. Missing Mock File for Wasm Tests

**Issue:** The Jest configuration in `pkg/jest.config.cjs` includes a `moduleNameMapper` that points to a mock file for the `veloxx` module: `^veloxx$: '<rootDir>/__mocks__/veloxx.js'`. However, this mock file does not exist in the `pkg/__mocks__/` directory, which will cause the Wasm tests to fail.

**Recommendation:** Create the `pkg/__mocks__/veloxx.js` file and implement a proper mock for the Wasm module to allow the tests to run correctly.

## 5. Temporary Scripts in Examples Directory

**Issue:** The `examples` directory contains two temporary-looking Python scripts: `temp_inspect_veloxx.py` and `temp_inspect_veloxx_lib.py`. These files seem to be for debugging or inspection purposes and are not part of the documented examples.

**Recommendation:** Remove these temporary scripts from the `examples` directory to keep it clean and focused on demonstrating the library's usage.

## 6. Use of Unstable Rust Edition

**Issue:** The `Cargo.toml` file specifies `edition = "2024"`. The 2024 edition of Rust is not yet stable and is only available on the nightly toolchain. Using an unstable edition can lead to breaking changes and compatibility issues.

**Recommendation:** Change the Rust edition in `Cargo.toml` to the latest stable version, which is `2021`, to ensure project stability and compatibility with the stable Rust toolchain.

## 7. Missing Functionalities in Python Bindings

**Issue:** The Python bindings (`PyDataFrame` and `PySeries`) are missing many of the core functionalities available in the Rust library, severely limiting their usefulness.

**Missing `PyDataFrame` functionalities:**
- **I/O Operations:** `from_csv`, `to_csv`, `from_json`.
- **Advanced Filtering:** Condition-based filtering is not exposed.
- **Joining:** `join` method is missing.
- **Grouping and Aggregation:** `group_by` and `agg` methods are missing.
- **Column Creation:** `with_column` method is missing.
- **Descriptive Statistics:** `describe`, `correlation`, and `covariance` methods are missing.
- **Data Appending:** `append` method is missing.

**Missing `PySeries` functionalities:**
- `apply` methods for custom transformations.
- `append` method.

**Recommendation:** Implement the missing methods in the Python bindings to provide feature parity with the core Rust library.

## 8. Missing Functionalities in JavaScript/Wasm Bindings

**Issue:** The JavaScript/Wasm bindings (`WasmDataFrame` and `WasmSeries`) are also missing many of the core functionalities available in the Rust library.

**Missing `WasmDataFrame` functionalities:**
- **I/O Operations:** `from_csv`, `to_csv`, `from_json`.
- **Advanced Filtering:** Condition-based filtering is not exposed.
- **Joining:** `join` method is missing.
- **Grouping and Aggregation:** `group_by` and `agg` methods are missing.
- **Column Creation:** `with_column` method is missing.
- **Descriptive Statistics:** `describe`, `correlation`, and `covariance` methods are missing.
- **Data Appending:** `append` method is missing.

**Missing `WasmSeries` functionalities:**
- `apply` methods for custom transformations.
- `append` method.

**Recommendation:** Implement the missing methods in the JavaScript/Wasm bindings to provide feature parity with the core Rust library.

## 9. Missing Tests in Python Bindings

**Issue:** The Python tests in `test_veloxx.py` do not cover all the functionalities that are currently exposed in the `PySeries` class.

**Untested `PySeries` methods:**
- `set_name`
- `filter`
- `count`
- `median`
- `correlation`
- `covariance`

**Recommendation:** Add tests for the missing `PySeries` methods to ensure the reliability and stability of the Python bindings.

## 10. Missing Tests in JavaScript/Wasm Bindings

**Issue:** The JavaScript tests in `test_wasm.test.js` have significant gaps in coverage for both `WasmDataFrame` and `WasmSeries`.

**Untested `WasmDataFrame` methods:**
- `dropNulls`
- `fillNulls`
- `sort`

**Untested `WasmSeries` methods:**
- `isEmpty`
- `dataType`
- `setName`
- `filter`
- `fillNulls`
- `cast`
- `append`
- `count`
- `min`
- `max`
- `median`
- `stdDev`
- `correlation`
- `covariance`

**Recommendation:** Add tests for the missing `WasmDataFrame` and `WasmSeries` methods to ensure the reliability and stability of the JavaScript/Wasm bindings.
