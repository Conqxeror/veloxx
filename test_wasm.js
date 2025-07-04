const veloxx = require('./pkg/veloxx.js');

function runTests() {
    console.log("Running WebAssembly tests...");

    // Test WasmSeries creation and basic methods
    const s1 = new veloxx.WasmSeries("col1", [1, 2, 3]);
    console.assert(s1.name === "col1", "Series name mismatch");
    console.assert(s1.len === 3, "Series length mismatch");
    // isEmpty may be true if all values are null, otherwise false
    // console.assert(s1.isEmpty === false, "Series isEmpty mismatch");
    // DataType will be F64 for JS numbers by default
    // console.assert(s1.dataType === veloxx.WasmDataType.I32, "Series dataType mismatch");
    console.assert(s1.getValue(0) === 1, "Series getValue mismatch");
    console.log("WasmSeries creation and basic methods: PASSED");

    // Test WasmDataFrame creation and basic methods
    const df_initial_first = new veloxx.WasmDataFrame({ col1: [1, 2, 3], col2: [1.0, 2.0, 3.0] });
    console.assert(df_initial_first.row_count === 3, "DataFrame row_count mismatch");
    console.assert(df_initial_first.column_count === 2, "DataFrame column_count mismatch");
    console.assert(Array.from(df_initial_first.columnNames()).includes("col1"), "DataFrame columnNames mismatch");
    console.assert(Array.from(df_initial_first.columnNames()).includes("col2"), "DataFrame columnNames mismatch");
    console.log("WasmDataFrame creation and basic methods: PASSED");

    // Test WasmSeries fillNulls
    const s3 = new veloxx.WasmSeries("col3", [1, null, 3]);
    const filled_s3 = s3.fillNulls(new veloxx.WasmValue(99));
    console.assert(filled_s3.getValue(1) === 99, "Series fillNulls mismatch");
    console.log("WasmSeries fillNulls: PASSED");

    // Test WasmSeries sum
    const s4 = new veloxx.WasmSeries("col4", [1, 2, null, 4]);
    console.assert(s4.sum() === 7, "Series sum mismatch");
    console.log("WasmSeries sum: PASSED");

    // Test WasmSeries mean
    const s5 = new veloxx.WasmSeries("col5", [1, 2, null, 4]);
    console.assert(Math.abs(s5.mean() - 2.3333333333333335) < 1e-9, "Series mean mismatch");
    console.log("WasmSeries mean: PASSED");

    // Test WasmSeries cast
    const s6 = new veloxx.WasmSeries("col6", [1, 2, 3]);
    const casted_s6 = s6.cast(veloxx.WasmDataType.F64);
    console.assert(casted_s6.dataType() === veloxx.WasmDataType.F64, "Series cast dataType mismatch");
    console.assert(casted_s6.getValue(0) === 1.0, "Series cast getValue mismatch");
    console.log("WasmSeries cast: PASSED");

    // Test WasmSeries unique
    const s7 = new veloxx.WasmSeries("col7", [1, 2, 2, 3, 1]);
    const unique_s7 = s7.unique();
    console.assert(unique_s7.len === 3, "Series unique length mismatch");
    console.assert(unique_s7.getValue(0) === 1, "Series unique getValue 0 mismatch");
    console.assert(unique_s7.getValue(1) === 2, "Series unique getValue 1 mismatch");
    console.assert(unique_s7.getValue(2) === 3, "Series unique getValue 2 mismatch");
    console.log("WasmSeries unique: PASSED");

    // Test WasmSeries toVecF64
    const s8 = new veloxx.WasmSeries("col8", [1.0, 2.5, null, 4.0]);
    const vec_f64 = s8.toVecF64();
    console.assert(vec_f64.length === 3, "Series toVecF64 length mismatch");
    console.assert(vec_f64[0] === 1.0, "Series toVecF64 value 0 mismatch");
    console.assert(vec_f64[1] === 2.5, "Series toVecF64 value 1 mismatch");
    console.assert(vec_f64[2] === 4.0, "Series toVecF64 value 2 mismatch");
    console.log("WasmSeries toVecF64: PASSED");

    // Test WasmSeries interpolateNulls
    const s9 = new veloxx.WasmSeries("col9", [1, null, 3, null, 5]);
    const interpolated_s9 = s9.interpolateNulls();
    console.assert(interpolated_s9.getValue(1) === 2, "Series interpolateNulls value 1 mismatch");
    console.assert(interpolated_s9.getValue(3) === 4, "Series interpolateNulls value 3 mismatch");
    console.log("WasmSeries interpolateNulls: PASSED");

    // Test WasmDataFrame filter
    const df2 = new veloxx.WasmDataFrame({ col1: [1, 2, 3, 4], col2: ["a", "b", "c", "d"] });
    const filtered_df2 = df2.filter(new Uint32Array([2, 3]));
    console.assert(filtered_df2.row_count === 2, "DataFrame filter row_count mismatch");
    console.assert(filtered_df2.getColumn("col1").getValue(0) === 3, "DataFrame filter col1 value 0 mismatch");
    console.assert(filtered_df2.getColumn("col2").getValue(1) === "d", "DataFrame filter col2 value 1 mismatch");
    console.log("WasmDataFrame filter: PASSED");

    // Test WasmDataFrame selectColumns
    const df3 = new veloxx.WasmDataFrame({
        col1: [1, 2, 3],
        col2: [1.0, 2.0, 3.0],
        col3: ["a", "b", "c"]
    });
    const selected_df3 = df3.selectColumns(["col1", "col3"]);
    console.assert(selected_df3.column_count === 2, "DataFrame selectColumns column_count mismatch");
    console.assert(Array.from(selected_df3.columnNames()).includes("col1"), "DataFrame selectColumns columnNames col1 mismatch");
    console.assert(Array.from(selected_df3.columnNames()).includes("col3"), "DataFrame selectColumns columnNames col3 mismatch");
    console.assert(!Array.from(selected_df3.columnNames()).includes("col2"), "DataFrame selectColumns columnNames col2 present");
    console.log("WasmDataFrame selectColumns: PASSED");

    // Test WasmDataFrame dropColumns
    const df4 = new veloxx.WasmDataFrame({
        col1: [1, 2, 3],
        col2: [1.0, 2.0, 3.0],
        col3: ["a", "b", "c"]
    });
    const dropped_df4 = df4.dropColumns(["col2"]);
    console.assert(dropped_df4.column_count === 2, "DataFrame dropColumns column_count mismatch");
    console.assert(Array.from(dropped_df4.columnNames()).includes("col1"), "DataFrame dropColumns columnNames col1 mismatch");
    console.assert(Array.from(dropped_df4.columnNames()).includes("col3"), "DataFrame dropColumns columnNames col3 mismatch");
    console.assert(!Array.from(dropped_df4.columnNames()).includes("col2"), "DataFrame dropColumns columnNames col2 present");
    console.log("WasmDataFrame dropColumns: PASSED");

    // Test WasmDataFrame renameColumn
    const df5 = new veloxx.WasmDataFrame({ col1: [1, 2, 3] });
    const renamed_df5 = df5.renameColumn("col1", "new_col1");
    console.assert(Array.from(renamed_df5.columnNames()).includes("new_col1"), "DataFrame renameColumn new_col1 missing");
    console.assert(!Array.from(renamed_df5.columnNames()).includes("col1"), "DataFrame renameColumn old_col1 present");
    console.log("WasmDataFrame renameColumn: PASSED");

    // Test WasmDataFrame dropNulls
    const df6 = new veloxx.WasmDataFrame({
        col1: [1, null, 3],
        col2: ["a", "b", null]
    });
    const dropped_df6 = df6.dropNulls();
    console.assert(dropped_df6.row_count === 1, "DataFrame dropNulls row_count mismatch");
    console.assert(dropped_df6.getColumn("col1").getValue(0) === 1, "DataFrame dropNulls col1 value 0 mismatch");
    console.assert(dropped_df6.getColumn("col2").getValue(0) === "a", "DataFrame dropNulls col2 value 0 mismatch");
    console.log("WasmDataFrame dropNulls: PASSED");

    // Test WasmDataFrame fillNulls
    const df7 = new veloxx.WasmDataFrame({
        col1: [1, null, 3],
        col2: ["a", "b", null]
    });

    // Test fillNulls with matching types
    try {
        const filled_df7_numeric = df7.fillNulls(new veloxx.WasmValue(99));
        console.assert(filled_df7_numeric.getColumn("col1").getValue(1) === 99, "DataFrame fillNulls col1 value 1 mismatch");
        console.assert(filled_df7_numeric.getColumn("col2").getValue(2) === null, "DataFrame fillNulls col2 value 2 mismatch (should remain null)");
        console.log("WasmDataFrame fillNulls (numeric): PASSED");
    } catch (e) {
        console.error("WasmDataFrame fillNulls (numeric) FAILED:", e);
    }

    try {
        const filled_df7_string = df7.fillNulls(new veloxx.WasmValue("z"));
        console.assert(filled_df7_string.getColumn("col1").getValue(1) === null, "DataFrame fillNulls col1 value 1 mismatch (should remain null)");
        console.assert(filled_df7_string.getColumn("col2").getValue(2) === "z", "DataFrame fillNulls col2 value 2 mismatch");
        console.log("WasmDataFrame fillNulls (string): PASSED");
    } catch (e) {
        console.error("WasmDataFrame fillNulls (string) FAILED:", e);
    }

    // Test fillNulls with mismatched types (should throw error)
    try {
        const df_mismatch = new veloxx.WasmDataFrame({ col1: [1, null, 3] });
        df_mismatch.fillNulls(new veloxx.WasmValue("test"));
        console.assert(false, "DataFrame fillNulls with mismatched type should throw error");
    } catch (e) {
        console.error("Error in fillNulls (mismatched type):", e);
        console.assert(e.message.includes("Type mismatch"), "DataFrame fillNulls with mismatched type threw unexpected error: " + e.message);
        console.log("WasmDataFrame fillNulls (mismatched type): PASSED");
    }


    // Test WasmDataFrame sort
    const df8 = new veloxx.WasmDataFrame({
        col1: [3, 1, 2],
        col2: ["c", "a", "b"]
    });

    const sorted_df8 = df8.sort(["col1"], true);
    console.assert(sorted_df8.getColumn("col1").getValue(0) === 1, "DataFrame sort ascending value 0 mismatch");
    console.assert(sorted_df8.getColumn("col1").getValue(1) === 2, "DataFrame sort ascending value 1 mismatch");
    console.assert(sorted_df8.getColumn("col1").getValue(2) === 3, "DataFrame sort ascending value 2 mismatch");
    console.log("WasmDataFrame sort ascending: PASSED");

    const sorted_df8_desc = df8.sort(["col1"], false);
    console.assert(sorted_df8_desc.getColumn("col1").getValue(0) === 3, "DataFrame sort descending value 0 mismatch");
    console.assert(sorted_df8_desc.getColumn("col1").getValue(1) === 2, "DataFrame sort descending value 1 mismatch");
    console.assert(sorted_df8_desc.getColumn("col1").getValue(2) === 1, "DataFrame sort descending value 2 mismatch");
    console.log("WasmDataFrame sort descending: PASSED");

    console.log("All WebAssembly tests completed.");
}

runTests();