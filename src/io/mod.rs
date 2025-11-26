#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
pub mod arrow;
pub mod csv;
pub mod json;
pub mod mmap_csv;

use crate::dataframe::DataFrame;
use crate::VeloxxError;

// Re-export the new ultra-fast parsers
pub use csv::UltraFastCsvParser;
pub use json::UltraFastJsonParser;
pub use mmap_csv::MemoryMappedCsvParser;

#[derive(Default)]
pub struct CsvReader;
#[derive(Default)]
pub struct JsonWriter;

impl CsvReader {
    pub fn new() -> Self {
        CsvReader
    }

    pub fn read_file(&self, _path: &str) -> Result<DataFrame, VeloxxError> {
        #[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
        {
            arrow::read_csv_to_dataframe(_path)
        }
        #[cfg(any(target_arch = "wasm32", not(feature = "arrow")))]
        {
            Err(VeloxxError::Unsupported(
                "File I/O not supported in WASM builds".to_string(),
            ))
        }
    }

    pub fn read_string(&self, _s: &str) -> Option<DataFrame> {
        // ...existing code...
        Some(DataFrame::new(indexmap::IndexMap::new()))
    }

    pub fn stream_string(&self, _s: &str, _n: usize) -> Option<DataFrame> {
        // ...existing code...
        Some(DataFrame::new(indexmap::IndexMap::new()))
    }
}

#[derive(Default)]
pub struct ParquetReader;

impl ParquetReader {
    pub fn new() -> Self {
        ParquetReader
    }

    #[cfg(feature = "advanced_io")]
    pub fn read_file(&self, path: &str) -> Result<DataFrame, VeloxxError> {
        #[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
        {
            arrow::read_parquet_to_dataframe(path)
        }
        #[cfg(any(target_arch = "wasm32", not(feature = "arrow")))]
        {
            Err(VeloxxError::Unsupported(
                "File I/O not supported in WASM builds".to_string(),
            ))
        }
    }

    #[cfg(not(feature = "advanced_io"))]
    pub fn read_file(&self, _path: &str) -> Result<DataFrame, VeloxxError> {
        Err(VeloxxError::Unsupported(
            "Parquet support requires advanced_io feature".to_string(),
        ))
    }
}

impl JsonWriter {
    pub fn new() -> Self {
        JsonWriter
    }

    pub fn pretty() -> Self {
        JsonWriter
    }

    pub fn write_file(&self, _df: &DataFrame, _path: &str) -> Result<(), VeloxxError> {
        Ok(())
    }

    pub fn write_string(&self, _df: &DataFrame) -> Option<String> {
        Some(String::new())
    }
}
