use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Generate a large CSV file for benchmarking
pub fn generate_large_csv(path: &str, rows: usize) -> std::io::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_small_csv() {
        generate_large_csv("small_test.csv", 10000).unwrap(); // 10K rows
        assert!(Path::new("small_test.csv").exists());
    }
}
