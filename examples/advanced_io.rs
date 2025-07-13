//! Example demonstrating advanced I/O operations in Velox
//!
//! This example shows how to use Parquet files, JSON streaming,
//! database connectivity, and async I/O operations.

use std::collections::BTreeMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

#[cfg(feature = "advanced_io")]
use veloxx::advanced_io::{
    AsyncFileOps, CompressionType, DatabaseConnector, JsonStreamer, ParquetReader, ParquetWriter,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use tokio runtime
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Velox Advanced I/O Operations Example");
    println!("====================================");

    // Create sample data
    let sample_df = create_sample_data()?;
    println!("Sample data created:");
    println!("{}", sample_df);

    // Demonstrate Parquet operations
    parquet_operations(&sample_df).await?;

    // Demonstrate JSON streaming
    json_streaming_operations().await?;

    // Demonstrate database operations
    database_operations(&sample_df).await?;

    // Demonstrate async file operations
    async_file_operations(&sample_df).await?;

    println!("\nAdvanced I/O operations examples completed!");
    println!("Note: Enable the 'advanced_io' feature to use actual I/O operations:");
    println!("cargo run --example advanced_io --features advanced_io");

    Ok(())
}

fn create_sample_data() -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut columns = BTreeMap::new();
    columns.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]),
    );
    columns.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
                Some("Diana".to_string()),
                Some("Eve".to_string()),
            ],
        ),
    );
    columns.insert(
        "sales".to_string(),
        Series::new_f64(
            "sales",
            vec![
                Some(1000.0),
                Some(1500.0),
                Some(800.0),
                Some(2200.0),
                Some(950.0),
            ],
        ),
    );
    columns.insert(
        "region".to_string(),
        Series::new_string(
            "region",
            vec![
                Some("North".to_string()),
                Some("South".to_string()),
                Some("East".to_string()),
                Some("West".to_string()),
                Some("North".to_string()),
            ],
        ),
    );

    Ok(DataFrame::new(columns)?)
}

#[allow(unused_variables)]
async fn parquet_operations(sample_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. Parquet Operations");
    println!("--------------------");

    #[cfg(feature = "advanced_io")]
    {
        // Write to Parquet file
        let writer = ParquetWriter::new();
        writer
            .write_dataframe(sample_df, "sample_data.parquet")
            .await?;
        println!("✓ Data written to Parquet file: sample_data.parquet");

        // Write with compression
        writer
            .write_dataframe_compressed(
                sample_df,
                "sample_data_compressed.parquet",
                CompressionType::Snappy,
            )
            .await?;
        println!("✓ Data written to compressed Parquet file with Snappy compression");

        // Read from Parquet file
        let reader = ParquetReader::new();
        let loaded_df = reader.read_dataframe("sample_data.parquet").await?;
        println!("✓ Data loaded from Parquet file:");
        println!("{}", loaded_df);

        // Streaming read for large files
        let streaming_dfs = reader
            .read_dataframe_streaming("sample_data.parquet", 2)
            .await?;
        println!(
            "✓ Streaming read completed, {} batches loaded",
            streaming_dfs.len()
        );
    }

    #[cfg(not(feature = "advanced_io"))]
    {
        println!("✗ Advanced I/O feature not enabled - Parquet operations not available");
    }

    Ok(())
}

async fn json_streaming_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. JSON Streaming Operations");
    println!("----------------------------");

    #[cfg(feature = "advanced_io")]
    {
        let streamer = JsonStreamer::new();

        // Sample JSON data
        let json_data = r#"
        [
            {"id": 1, "name": "Product A", "price": 29.99},
            {"id": 2, "name": "Product B", "price": 39.99},
            {"id": 3, "name": "Product C", "price": 19.99}
        ]
        "#;

        // Stream JSON from string
        let dataframes = streamer.stream_from_string(json_data, 2).await?;
        println!(
            "✓ JSON streaming from string completed, {} DataFrames created",
            dataframes.len()
        );

        for (i, df) in dataframes.iter().enumerate() {
            println!("Batch {}: {} rows", i + 1, df.row_count());
        }

        // Create a temporary JSON file for streaming demo
        tokio::fs::write("temp_data.json", json_data).await?;
        let file_dataframes = streamer.stream_from_file("temp_data.json", 2).await?;
        println!(
            "✓ JSON streaming from file completed, {} DataFrames created",
            file_dataframes.len()
        );

        // Cleanup
        let _ = tokio::fs::remove_file("temp_data.json").await;
    }

    #[cfg(not(feature = "advanced_io"))]
    {
        println!("✗ Advanced I/O feature not enabled - JSON streaming not available");
    }

    Ok(())
}

#[allow(unused_variables)]
async fn database_operations(sample_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. Database Operations");
    println!("---------------------");

    #[cfg(feature = "advanced_io")]
    {
        // SQLite database operations
        let connector = DatabaseConnector::new("sqlite://sample.db");

        // Create table from DataFrame schema
        connector
            .create_table_from_dataframe(sample_df, "sales_data")
            .await?;
        println!("✓ Table 'sales_data' created from DataFrame schema");

        // Insert DataFrame data
        connector.insert_dataframe(sample_df, "sales_data").await?;
        println!("✓ DataFrame data inserted into 'sales_data' table");

        // Query data back
        let query_result = connector
            .query("SELECT * FROM sales_data WHERE sales > 1000")
            .await?;
        println!("✓ Query executed successfully:");
        println!("{}", query_result);

        // PostgreSQL example (would need actual database)
        let _pg_connector = DatabaseConnector::new("postgresql://user:password@localhost/database");
        println!("✓ PostgreSQL connector created (connection not tested)");

        // MySQL example (would need actual database)
        let _mysql_connector = DatabaseConnector::new("mysql://user:password@localhost/database");
        println!("✓ MySQL connector created (connection not tested)");
    }

    #[cfg(not(feature = "advanced_io"))]
    {
        println!("✗ Advanced I/O feature not enabled - database operations not available");
    }

    Ok(())
}

#[allow(unused_variables)]
async fn async_file_operations(sample_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n4. Async File Operations");
    println!("------------------------");

    #[cfg(feature = "advanced_io")]
    {
        // Async CSV operations
        AsyncFileOps::write_csv_async(sample_df, "async_sample.csv").await?;
        println!("✓ DataFrame written to CSV asynchronously");

        let loaded_df = AsyncFileOps::read_csv_async("async_sample.csv").await?;
        println!("✓ CSV file read asynchronously:");
        println!("{}", loaded_df);

        // Async JSON operations
        AsyncFileOps::write_json_async(sample_df, "async_sample.json").await?;
        println!("✓ DataFrame written to JSON asynchronously");

        let json_df = AsyncFileOps::read_json_async("async_sample.json").await?;
        println!("✓ JSON file read asynchronously:");
        println!("{}", json_df);

        // Cleanup
        let _ = tokio::fs::remove_file("async_sample.csv").await;
        let _ = tokio::fs::remove_file("async_sample.json").await;
        let _ = tokio::fs::remove_file("sample_data.parquet").await;
        let _ = tokio::fs::remove_file("sample_data_compressed.parquet").await;
    }

    #[cfg(not(feature = "advanced_io"))]
    {
        println!("✗ Advanced I/O feature not enabled - async file operations not available");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sample_data_creation() {
        let df = create_sample_data().unwrap();
        assert_eq!(df.row_count(), 5);
        assert_eq!(df.column_count(), 4);
        assert!(df.column_names().contains(&&"id".to_string()));
        assert!(df.column_names().contains(&&"name".to_string()));
        assert!(df.column_names().contains(&&"sales".to_string()));
        assert!(df.column_names().contains(&&"region".to_string()));
    }
}
