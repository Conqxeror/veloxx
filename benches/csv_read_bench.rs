use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::hint::black_box;
use std::io::{BufWriter, Write};
use veloxx::dataframe::DataFrame;

/// Generate a large CSV file for benchmarking
fn generate_large_csv(path: &str, rows: usize) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Write header
    writeln!(writer, "id,name,value,category,timestamp")?;

    // Write data rows
    for i in 0..rows {
        let name = format!("name_{}", i % 1000);
        let value = (i as f64) * 1.5;
        let category = format!("category_{}", i % 50);
        let timestamp = 1609459200 + i; // Start from 2021-01-01

        writeln!(
            writer,
            "{},{},{},{},{}",
            i, name, value, category, timestamp
        )?;
    }

    writer.flush()?;
    Ok(())
}

fn bench_csv_read_small(c: &mut Criterion) {
    // Generate test data (10K rows)
    generate_large_csv("small_test.csv", 10_000).unwrap();

    c.bench_function("csv_read_small_10k", |b| {
        b.iter(|| {
            let df = DataFrame::from_csv(black_box("small_test.csv"));
            let _ = black_box(df);
        })
    });
}

fn bench_csv_read_medium(c: &mut Criterion) {
    // Generate test data (100K rows - reduced from 1M to save memory)
    generate_large_csv("medium_test.csv", 100_000).unwrap();

    c.bench_function("csv_read_medium_100k", |b| {
        b.iter(|| {
            let df = DataFrame::from_csv(black_box("medium_test.csv"));
            let _ = black_box(df);
        })
    });
}

criterion_group!(benches, bench_csv_read_small, bench_csv_read_medium);
criterion_main!(benches);
