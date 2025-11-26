use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::io::Write;
use tempfile::NamedTempFile;
use veloxx::io::UltraFastCsvParser;

fn create_large_csv(rows: usize) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "id,value,category,flag").unwrap();
    for i in 0..rows {
        writeln!(
            file,
            "{},{:.2},category_{},{}",
            i,
            i as f64 * 1.5,
            i % 10,
            i % 2 == 0
        )
        .unwrap();
    }
    file
}

fn bench_csv_read(c: &mut Criterion) {
    let rows = 10_000;
    let file = create_large_csv(rows);
    let path = file.path().to_str().unwrap().to_string();
    let file_size = std::fs::metadata(&path).unwrap().len();

    let mut group = c.benchmark_group("io");
    group.throughput(Throughput::Bytes(file_size));

    group.bench_function("csv_read_10k", |b| {
        b.iter(|| UltraFastCsvParser::quick_read(&path).unwrap())
    });

    group.finish();
}

criterion_group!(benches, bench_csv_read);
criterion_main!(benches);
