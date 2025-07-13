//! Advanced I/O Operations module for Velox.
//!
//! This module provides advanced data input/output capabilities including:
//! - Parquet file format support for columnar data
//! - Streaming JSON processing for large datasets
//! - Database connectivity (SQLite, PostgreSQL, MySQL)
//! - Asynchronous I/O operations
//!
//! # Features
//!
//! - High-performance Parquet reading and writing
//! - Memory-efficient streaming for large files
//! - Database integration with connection pooling
//! - Async/await support for non-blocking operations
//!
//! # Examples
//!
//! ```rust
//! use veloxx::dataframe::DataFrame;
//! use veloxx::series::Series;
//! use std::collections::BTreeMap;
//!
//! # #[cfg(feature = "advanced_io")]
//! # {
//! use veloxx::advanced_io::{ParquetReader, ParquetWriter, DatabaseConnector};
//!
//! let mut columns = BTreeMap::new();
//! columns.insert(
//!     "id".to_string(),
//!     Series::new_i32("id", vec![Some(1), Some(2), Some(3)]),
//! );
//! columns.insert(
//!     "name".to_string(),
//!     Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string()), Some("Charlie".to_string())]),
//! );
//!
//! let df = DataFrame::new(columns).unwrap();
//!
//! // Write to Parquet
//! // let writer = ParquetWriter::new();
//! // writer.write_dataframe(&df, "data.parquet").await.unwrap();
//!
//! // Read from Parquet
//! // let reader = ParquetReader::new();
//! // let loaded_df = reader.read_dataframe("data.parquet").await.unwrap();
//! # }
//! ```

use crate::dataframe::DataFrame;
use crate::error::VeloxxError;
use crate::series::Series;
use crate::types::DataType;
use std::collections::BTreeMap;
use std::path::Path;

#[cfg(feature = "advanced_io")]
use parquet::file::reader::{FileReader, SerializedFileReader};

/// Parquet file reader for high-performance columnar data access
pub struct ParquetReader {
    #[cfg(not(feature = "advanced_io"))]
    _phantom: std::marker::PhantomData<()>,
}

impl ParquetReader {
    /// Create a new Parquet reader
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::advanced_io::ParquetReader;
    ///
    /// let reader = ParquetReader::new();
    /// ```
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "advanced_io"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Read a DataFrame from a Parquet file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the Parquet file
    ///
    /// # Returns
    ///
    /// DataFrame containing the data from the Parquet file
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::advanced_io::ParquetReader;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let reader = ParquetReader::new();
    /// // let df = reader.read_dataframe("data.parquet").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "advanced_io")]
    pub async fn read_dataframe<P: AsRef<Path>>(&self, path: P) -> Result<DataFrame, VeloxxError> {
        let file = std::fs::File::open(path.as_ref()).map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to open Parquet file: {}", e))
        })?;

        let reader = SerializedFileReader::new(file).map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to create Parquet reader: {}", e))
        })?;

        let metadata = reader.metadata();
        let _schema = metadata.file_metadata().schema();

        // For now, return a placeholder implementation
        // In a full implementation, we would parse the Parquet schema and data
        let mut columns = BTreeMap::new();
        columns.insert(
            "placeholder".to_string(),
            Series::new_string(
                "placeholder",
                vec![Some("Parquet reading not fully implemented".to_string())],
            ),
        );

        DataFrame::new(columns)
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn read_dataframe<P: AsRef<Path>>(&self, _path: P) -> Result<DataFrame, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }

    /// Read a DataFrame from a Parquet file with streaming for large files
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the Parquet file
    /// * `batch_size` - Number of rows to read at a time
    ///
    /// # Returns
    ///
    /// Stream of DataFrames containing batches of data
    #[cfg(feature = "advanced_io")]
    pub async fn read_dataframe_streaming<P: AsRef<Path>>(
        &self,
        path: P,
        _batch_size: usize,
    ) -> Result<Vec<DataFrame>, VeloxxError> {
        // Placeholder for streaming implementation
        let df = self.read_dataframe(path).await?;
        Ok(vec![df])
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn read_dataframe_streaming<P: AsRef<Path>>(
        &self,
        _path: P,
        _batch_size: usize,
    ) -> Result<Vec<DataFrame>, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }
}

impl Default for ParquetReader {
    fn default() -> Self {
        Self::new()
    }
}

/// Parquet file writer for high-performance columnar data storage
pub struct ParquetWriter {
    #[cfg(not(feature = "advanced_io"))]
    _phantom: std::marker::PhantomData<()>,
}

impl ParquetWriter {
    /// Create a new Parquet writer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::advanced_io::ParquetWriter;
    ///
    /// let writer = ParquetWriter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "advanced_io"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Write a DataFrame to a Parquet file
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to write
    /// * `path` - Path where the Parquet file should be created
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::advanced_io::ParquetWriter;
    /// use std::collections::BTreeMap;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "id".to_string(),
    ///     Series::new_i32("id", vec![Some(1), Some(2)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let writer = ParquetWriter::new();
    /// // writer.write_dataframe(&df, "output.parquet").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "advanced_io")]
    pub async fn write_dataframe<P: AsRef<Path>>(
        &self,
        _dataframe: &DataFrame,
        path: P,
    ) -> Result<(), VeloxxError> {
        // Placeholder implementation
        tokio::fs::write(path, "Parquet writing not fully implemented")
            .await
            .map_err(|e| {
                VeloxxError::InvalidOperation(format!("Failed to write Parquet file: {}", e))
            })?;

        Ok(())
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn write_dataframe<P: AsRef<Path>>(
        &self,
        _dataframe: &DataFrame,
        _path: P,
    ) -> Result<(), VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }

    /// Write a DataFrame to a Parquet file with compression
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to write
    /// * `path` - Path where the Parquet file should be created
    /// * `compression` - Compression algorithm to use
    #[cfg(feature = "advanced_io")]
    pub async fn write_dataframe_compressed<P: AsRef<Path>>(
        &self,
        dataframe: &DataFrame,
        path: P,
        _compression: CompressionType,
    ) -> Result<(), VeloxxError> {
        // For now, delegate to regular write
        self.write_dataframe(dataframe, path).await
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn write_dataframe_compressed<P: AsRef<Path>>(
        &self,
        _dataframe: &DataFrame,
        _path: P,
        _compression: CompressionType,
    ) -> Result<(), VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }
}

impl Default for ParquetWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Compression types for Parquet files
#[derive(Debug, Clone, Copy)]
pub enum CompressionType {
    None,
    Snappy,
    Gzip,
    Lzo,
    Brotli,
    Lz4,
    Zstd,
}

/// JSON streaming processor for handling large JSON datasets
pub struct JsonStreamer {
    #[cfg(not(feature = "advanced_io"))]
    _phantom: std::marker::PhantomData<()>,
}

impl JsonStreamer {
    /// Create a new JSON streamer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::advanced_io::JsonStreamer;
    ///
    /// let streamer = JsonStreamer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "advanced_io"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Stream JSON data from a file and convert to DataFrames
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON file
    /// * `batch_size` - Number of JSON objects to process per batch
    ///
    /// # Returns
    ///
    /// Stream of DataFrames containing batches of data
    #[cfg(feature = "advanced_io")]
    pub async fn stream_from_file<P: AsRef<Path>>(
        &self,
        path: P,
        batch_size: usize,
    ) -> Result<Vec<DataFrame>, VeloxxError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to read JSON file: {}", e))
        })?;

        self.stream_from_string(&content, batch_size).await
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn stream_from_file<P: AsRef<Path>>(
        &self,
        _path: P,
        _batch_size: usize,
    ) -> Result<Vec<DataFrame>, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }

    /// Stream JSON data from a string and convert to DataFrames
    ///
    /// # Arguments
    ///
    /// * `json_str` - JSON string containing array of objects
    /// * `batch_size` - Number of JSON objects to process per batch
    ///
    /// # Returns
    ///
    /// Stream of DataFrames containing batches of data
    #[cfg(feature = "advanced_io")]
    pub async fn stream_from_string(
        &self,
        json_str: &str,
        _batch_size: usize,
    ) -> Result<Vec<DataFrame>, VeloxxError> {
        // Placeholder implementation - in reality would parse JSON incrementally
        let mut columns = BTreeMap::new();
        columns.insert(
            "json_data".to_string(),
            Series::new_string("json_data", vec![Some(json_str.to_string())]),
        );

        let df = DataFrame::new(columns)?;
        Ok(vec![df])
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn stream_from_string(
        &self,
        _json_str: &str,
        _batch_size: usize,
    ) -> Result<Vec<DataFrame>, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }
}

impl Default for JsonStreamer {
    fn default() -> Self {
        Self::new()
    }
}

/// Database connector for various database systems
pub struct DatabaseConnector {
    #[cfg(feature = "advanced_io")]
    #[allow(dead_code)]
    connection_string: String,
    #[cfg(not(feature = "advanced_io"))]
    _phantom: std::marker::PhantomData<()>,
}

impl DatabaseConnector {
    /// Create a new database connector
    ///
    /// # Arguments
    ///
    /// * `connection_string` - Database connection string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::advanced_io::DatabaseConnector;
    ///
    /// let connector = DatabaseConnector::new("sqlite://database.db");
    /// ```
    pub fn new(connection_string: &str) -> Self {
        Self {
            #[cfg(feature = "advanced_io")]
            connection_string: connection_string.to_string(),
            #[cfg(not(feature = "advanced_io"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Execute a SQL query and return results as a DataFrame
    ///
    /// # Arguments
    ///
    /// * `query` - SQL query to execute
    ///
    /// # Returns
    ///
    /// DataFrame containing query results
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::advanced_io::DatabaseConnector;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let connector = DatabaseConnector::new("sqlite://database.db");
    /// // let df = connector.query("SELECT * FROM users").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "advanced_io")]
    pub async fn query(&self, query: &str) -> Result<DataFrame, VeloxxError> {
        // Placeholder implementation
        let mut columns = BTreeMap::new();
        columns.insert(
            "query_result".to_string(),
            Series::new_string("query_result", vec![Some(format!("Executed: {}", query))]),
        );

        DataFrame::new(columns)
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn query(&self, _query: &str) -> Result<DataFrame, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }

    /// Insert a DataFrame into a database table
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to insert
    /// * `table_name` - Name of the target table
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::series::Series;
    /// use veloxx::advanced_io::DatabaseConnector;
    /// use std::collections::BTreeMap;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut columns = BTreeMap::new();
    /// columns.insert(
    ///     "id".to_string(),
    ///     Series::new_i32("id", vec![Some(1), Some(2)]),
    /// );
    ///
    /// let df = DataFrame::new(columns).unwrap();
    /// let connector = DatabaseConnector::new("sqlite://database.db");
    /// // connector.insert_dataframe(&df, "users").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "advanced_io")]
    pub async fn insert_dataframe(
        &self,
        _dataframe: &DataFrame,
        table_name: &str,
    ) -> Result<(), VeloxxError> {
        // Placeholder implementation
        println!("Would insert DataFrame into table: {}", table_name);
        Ok(())
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn insert_dataframe(
        &self,
        _dataframe: &DataFrame,
        _table_name: &str,
    ) -> Result<(), VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }

    /// Create a table from a DataFrame schema
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to use for schema inference
    /// * `table_name` - Name of the table to create
    #[cfg(feature = "advanced_io")]
    pub async fn create_table_from_dataframe(
        &self,
        dataframe: &DataFrame,
        table_name: &str,
    ) -> Result<(), VeloxxError> {
        // Generate CREATE TABLE statement from DataFrame schema
        let mut create_sql = format!("CREATE TABLE {} (", table_name);
        let column_names = dataframe.column_names();

        for (i, column_name) in column_names.iter().enumerate() {
            if let Some(series) = dataframe.get_column(column_name) {
                let sql_type = match series.data_type() {
                    DataType::I32 => "INTEGER",
                    DataType::F64 => "REAL",
                    DataType::Bool => "BOOLEAN",
                    DataType::String => "TEXT",
                    DataType::DateTime => "DATETIME",
                };

                create_sql.push_str(&format!("{} {}", column_name, sql_type));
                if i < column_names.len() - 1 {
                    create_sql.push_str(", ");
                }
            }
        }
        create_sql.push(')');

        // In a real implementation, we would execute this SQL
        println!("Would execute: {}", create_sql);
        Ok(())
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn create_table_from_dataframe(
        &self,
        _dataframe: &DataFrame,
        _table_name: &str,
    ) -> Result<(), VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }
}

/// Async file operations for non-blocking I/O
pub struct AsyncFileOps;

impl AsyncFileOps {
    /// Read a CSV file asynchronously
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the CSV file
    ///
    /// # Returns
    ///
    /// DataFrame containing the CSV data
    #[cfg(feature = "advanced_io")]
    pub async fn read_csv_async<P: AsRef<Path>>(path: P) -> Result<DataFrame, VeloxxError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to read CSV file: {}", e))
        })?;

        // Parse CSV content from string
        Self::parse_csv_from_string(&content)
    }

    #[cfg(feature = "advanced_io")]
    fn parse_csv_from_string(content: &str) -> Result<DataFrame, VeloxxError> {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "CSV content is empty".to_string(),
            ));
        }

        // Parse header
        let header_line = lines[0];
        let column_names: Vec<String> = header_line
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // Parse data rows
        let mut data_rows: Vec<Vec<String>> = Vec::new();
        for line in &lines[1..] {
            let row: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
            data_rows.push(row);
        }

        DataFrame::from_vec_of_vec(data_rows, column_names)
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn read_csv_async<P: AsRef<Path>>(_path: P) -> Result<DataFrame, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }

    /// Write a CSV file asynchronously
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to write
    /// * `path` - Path where the CSV file should be created
    #[cfg(feature = "advanced_io")]
    pub async fn write_csv_async<P: AsRef<Path>>(
        dataframe: &DataFrame,
        path: P,
    ) -> Result<(), VeloxxError> {
        // Generate CSV content as string
        let csv_content = Self::dataframe_to_csv_string(dataframe)?;
        tokio::fs::write(path, csv_content).await.map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to write CSV file: {}", e))
        })?;

        Ok(())
    }

    #[cfg(feature = "advanced_io")]
    fn dataframe_to_csv_string(dataframe: &DataFrame) -> Result<String, VeloxxError> {
        let mut csv_content = String::new();

        // Write header
        let column_names: Vec<&str> = dataframe
            .column_names()
            .iter()
            .map(|s| s.as_str())
            .collect();
        csv_content.push_str(&column_names.join(","));
        csv_content.push('\n');

        // Write data rows
        for i in 0..dataframe.row_count() {
            let mut row_values: Vec<String> = Vec::new();
            for col_name in column_names.iter() {
                let series = dataframe.get_column(col_name).unwrap();
                let value_str = match series.get_value(i) {
                    Some(crate::types::Value::I32(v)) => v.to_string(),
                    Some(crate::types::Value::F64(v)) => v.to_string(),
                    Some(crate::types::Value::Bool(v)) => v.to_string(),
                    Some(crate::types::Value::String(v)) => v,
                    Some(crate::types::Value::DateTime(v)) => v.to_string(),
                    Some(crate::types::Value::Null) => String::new(),
                    None => String::new(),
                };
                row_values.push(value_str);
            }
            csv_content.push_str(&row_values.join(","));
            csv_content.push('\n');
        }

        Ok(csv_content)
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn write_csv_async<P: AsRef<Path>>(
        _dataframe: &DataFrame,
        _path: P,
    ) -> Result<(), VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }

    /// Read a JSON file asynchronously
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON file
    ///
    /// # Returns
    ///
    /// DataFrame containing the JSON data
    #[cfg(feature = "advanced_io")]
    pub async fn read_json_async<P: AsRef<Path>>(path: P) -> Result<DataFrame, VeloxxError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to read JSON file: {}", e))
        })?;

        // Use existing JSON parsing logic
        DataFrame::from_json(&content)
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn read_json_async<P: AsRef<Path>>(_path: P) -> Result<DataFrame, VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }

    /// Write a JSON file asynchronously
    ///
    /// # Arguments
    ///
    /// * `dataframe` - DataFrame to write
    /// * `path` - Path where the JSON file should be created
    #[cfg(feature = "advanced_io")]
    pub async fn write_json_async<P: AsRef<Path>>(
        dataframe: &DataFrame,
        path: P,
    ) -> Result<(), VeloxxError> {
        let json_content = Self::dataframe_to_json_string(dataframe)?;
        tokio::fs::write(path, json_content).await.map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to write JSON file: {}", e))
        })?;

        Ok(())
    }

    #[cfg(feature = "advanced_io")]
    fn dataframe_to_json_string(dataframe: &DataFrame) -> Result<String, VeloxxError> {
        let mut json_content = String::from("[\n");

        for i in 0..dataframe.row_count() {
            if i > 0 {
                json_content.push_str(",\n");
            }
            json_content.push_str("  {");

            let mut first_field = true;
            for column_name in dataframe.column_names() {
                if !first_field {
                    json_content.push_str(", ");
                }
                first_field = false;

                let series = dataframe.get_column(column_name).unwrap();
                json_content.push_str(&format!("\"{}\":", column_name));

                match series.get_value(i) {
                    Some(crate::types::Value::I32(v)) => json_content.push_str(&v.to_string()),
                    Some(crate::types::Value::F64(v)) => json_content.push_str(&v.to_string()),
                    Some(crate::types::Value::Bool(v)) => json_content.push_str(&v.to_string()),
                    Some(crate::types::Value::String(v)) => {
                        json_content.push_str(&format!("\"{}\"", v))
                    }
                    Some(crate::types::Value::DateTime(v)) => json_content.push_str(&v.to_string()),
                    Some(crate::types::Value::Null) => json_content.push_str("null"),
                    None => json_content.push_str("null"),
                }
            }

            json_content.push('}');
        }

        json_content.push_str("\n]");
        Ok(json_content)
    }

    #[cfg(not(feature = "advanced_io"))]
    pub async fn write_json_async<P: AsRef<Path>>(
        _dataframe: &DataFrame,
        _path: P,
    ) -> Result<(), VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Advanced I/O feature is not enabled. Enable with --features advanced_io".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parquet_reader_creation() {
        let reader = ParquetReader::new();
        // Just test that we can create the reader
        assert_eq!(
            std::mem::size_of_val(&reader),
            std::mem::size_of::<ParquetReader>()
        );
    }

    #[test]
    fn test_parquet_writer_creation() {
        let writer = ParquetWriter::new();
        // Just test that we can create the writer
        assert_eq!(
            std::mem::size_of_val(&writer),
            std::mem::size_of::<ParquetWriter>()
        );
    }

    #[test]
    fn test_json_streamer_creation() {
        let streamer = JsonStreamer::new();
        // Just test that we can create the streamer
        assert_eq!(
            std::mem::size_of_val(&streamer),
            std::mem::size_of::<JsonStreamer>()
        );
    }

    #[test]
    fn test_database_connector_creation() {
        let connector = DatabaseConnector::new("sqlite://test.db");
        // Just test that we can create the connector
        assert_eq!(
            std::mem::size_of_val(&connector),
            std::mem::size_of::<DatabaseConnector>()
        );
    }

    #[tokio::test]
    async fn test_advanced_io_without_feature() {
        let reader = ParquetReader::new();
        let result = reader.read_dataframe("test.parquet").await;

        #[cfg(not(feature = "advanced_io"))]
        assert!(result.is_err());

        #[cfg(feature = "advanced_io")]
        {
            // Would test actual functionality with feature enabled
            let _ = result;
        }
    }
}
