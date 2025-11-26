use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use indexmap::IndexMap;
use std::fs::File;
use std::sync::Arc;
use veloxx::dataframe::DataFrame;
use veloxx::io::arrow::{read_parquet_to_dataframe, write_parquet_from_dataframe};
use veloxx::series::Series;

fn create_test_dataframe(size: usize) -> DataFrame {
    let mut columns = IndexMap::new();
    let data: Vec<Option<i32>> = (0..size).map(|i| Some(i as i32)).collect();
    columns.insert("a".to_string(), Series::new_i32("a", data.clone()));
    columns.insert("b".to_string(), Series::new_i32("b", data));
    DataFrame::new(columns)
}

fn bench_parquet_io(c: &mut Criterion) {
    let size = 100_000;
    let df = create_test_dataframe(size);
    let file_path = "test_bench.parquet";

    // Write once for read bench
    let _ = write_parquet_from_dataframe(&df, file_path);
    let file_size = std::fs::metadata(file_path).unwrap().len();

    let mut group = c.benchmark_group("parquet_io");
    group.throughput(Throughput::Bytes(file_size));

    group.bench_function("parquet_write_100k", |b| {
        b.iter(|| write_parquet_from_dataframe(&df, "test_bench_write.parquet").unwrap())
    });

    group.bench_function("parquet_read_100k", |b| {
        b.iter(|| read_parquet_to_dataframe(file_path).unwrap())
    });

    group.finish();

    // Cleanup
    let _ = std::fs::remove_file(file_path);
    let _ = std::fs::remove_file("test_bench_write.parquet");
}

criterion_group!(benches, bench_parquet_io);
criterion_main!(benches);
