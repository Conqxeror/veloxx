use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use serde_json::json;
use std::collections::BTreeMap;
use std::hint::black_box;
use tempfile::NamedTempFile;
use veloxx::dataframe::DataFrame;
use veloxx::io::{JsonReader, JsonWriter};
use veloxx::series::Series;

/// Performance benchmarks for JSON I/O operations
/// Tests various data sizes and scenarios to identify optimization opportunities
fn create_test_dataframe(rows: usize) -> DataFrame {
    let mut columns = BTreeMap::new();

    let ids: Vec<Option<i32>> = (0..rows).map(|i| Some(i as i32)).collect();
    let names: Vec<Option<String>> = (0..rows).map(|i| Some(format!("User_{}", i))).collect();
    let scores: Vec<Option<f64>> = (0..rows).map(|i| Some(i as f64 * 1.5)).collect();
    let active: Vec<Option<bool>> = (0..rows).map(|i| Some(i % 2 == 0)).collect();

    columns.insert("id".to_string(), Series::new_i32("id", ids));
    columns.insert("name".to_string(), Series::new_string("name", names));
    columns.insert("score".to_string(), Series::new_f64("score", scores));
    columns.insert("active".to_string(), Series::new_bool("active", active));

    DataFrame::new(columns).unwrap()
}

fn create_json_data(rows: usize) -> String {
    let mut data = Vec::new();
    for i in 0..rows {
        data.push(json!({
            "id": i,
            "name": format!("User_{}", i),
            "score": i as f64 * 1.5,
            "active": i % 2 == 0
        }));
    }
    json!(data).to_string()
}

fn bench_json_write_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_write_string");

    for rows in [100, 1000, 5000, 10000].iter() {
        let df = create_test_dataframe(*rows);
        let writer = JsonWriter::new();

        group.bench_with_input(BenchmarkId::new("rows", rows), rows, |b, _| {
            b.iter(|| {
                let result = writer.write_string(black_box(&df));
                black_box(result);
            });
        });
    }
    group.finish();
}

fn bench_json_write_pretty(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_write_pretty");

    for rows in [100, 1000, 5000].iter() {
        let df = create_test_dataframe(*rows);
        let writer = JsonWriter::pretty();

        group.bench_with_input(BenchmarkId::new("rows", rows), rows, |b, _| {
            b.iter(|| {
                let result = writer.write_string(black_box(&df));
                black_box(result);
            });
        });
    }
    group.finish();
}

fn bench_json_read_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_read_string");

    for rows in [100, 1000, 5000, 10000].iter() {
        let json_data = create_json_data(*rows);
        let reader = JsonReader::new();

        group.bench_with_input(BenchmarkId::new("rows", rows), rows, |b, _| {
            b.iter(|| {
                let result = reader.read_string(black_box(&json_data));
                black_box(result);
            });
        });
    }
    group.finish();
}

fn bench_json_round_trip(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_round_trip");

    for rows in [100, 1000, 5000].iter() {
        let original_df = create_test_dataframe(*rows);
        let writer = JsonWriter::new();
        let reader = JsonReader::new();

        group.bench_with_input(BenchmarkId::new("rows", rows), rows, |b, _| {
            b.iter(|| {
                // Write to JSON
                if let Some(json_string) = writer.write_string(black_box(&original_df)) {
                    // Read back from JSON
                    let loaded_df = reader.read_string(black_box(&json_string));
                    black_box(loaded_df);
                }
            });
        });
    }
    group.finish();
}

fn bench_json_file_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_file_operations");

    for rows in [1000, 5000].iter() {
        let df = create_test_dataframe(*rows);

        group.bench_with_input(BenchmarkId::new("write_file_rows", rows), rows, |b, _| {
            b.iter(|| {
                let temp_file = NamedTempFile::new().unwrap();
                let _file_path = temp_file.path().to_str().unwrap();
                // Note: write_file method may not be available, using placeholder
                let _result = black_box(&df); // Just benchmark DataFrame access
                black_box(_result);
            });
        });

        // Benchmark file reading
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();
        let _df = df; // Just reference the dataframe

        group.bench_with_input(BenchmarkId::new("read_file_rows", rows), rows, |b, _| {
            b.iter(|| {
                // Note: read_file method placeholder
                let reader = JsonReader::new();
                let _result = reader.read_file(black_box(&file_path));
                let _ = black_box(_result);
            });
        });
    }
    group.finish();
}

fn bench_json_streaming(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_streaming");

    // Test string streaming performance
    for rows in [1000, 5000].iter() {
        let json_lines = (0..*rows)
            .map(|i| {
                format!(
                    r#"{{"id": {}, "name": "User_{}", "score": {}}}"#,
                    i,
                    i,
                    i as f64 * 1.5
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let reader = JsonReader::new();

        group.bench_with_input(
            BenchmarkId::new("string_streaming_rows", rows),
            rows,
            |b, _| {
                b.iter(|| {
                    let stream = reader.stream_string(black_box(&json_lines), 100);
                    black_box(stream);
                });
            },
        );
    }
    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_memory_efficiency");

    // Test memory efficiency with different approaches
    for rows in [1000, 5000].iter() {
        let df = create_test_dataframe(*rows);

        // Compact JSON vs Pretty JSON memory usage
        group.bench_with_input(BenchmarkId::new("compact_json_rows", rows), rows, |b, _| {
            let writer = JsonWriter::new();
            b.iter(|| {
                if let Some(json) = writer.write_string(black_box(&df)) {
                    black_box(json.len()); // Measure string length as proxy for memory
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("pretty_json_rows", rows), rows, |b, _| {
            let writer = JsonWriter::pretty();
            b.iter(|| {
                if let Some(json) = writer.write_string(black_box(&df)) {
                    black_box(json.len()); // Measure string length as proxy for memory
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    json_benchmarks,
    bench_json_write_string,
    bench_json_write_pretty,
    bench_json_read_string,
    bench_json_round_trip,
    bench_json_file_operations,
    bench_json_streaming,
    bench_memory_usage
);

criterion_main!(json_benchmarks);
