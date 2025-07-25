# Async JSON I/O Guide

This guide covers Velox's async JSON capabilities with practical examples.

## Key Features
- Async read/write operations
- Streaming support for large datasets
- Error handling patterns

## Basic Usage
```rust
use veloxx::io::JsonReader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = JsonReader::new();
    
    // Read from string
    let df = reader.read_string(json_data).await?;
    
    // Read from file
    let file_df = reader.read_file("data.json").await?;
    
    Ok(())
}
```

## Streaming Large JSON
```rust
async fn process_large_file() -> Result<(), Box<dyn std::error::Error>> {
    let reader = JsonReader::new();
    let mut stream = reader.stream_file("large_data.json", 1000).await?;
    
    while let Some(chunk) = stream.next().await {
        let df = chunk?;
        // Process chunk
    }
    
    Ok(())
}
```

## Best Practices
- Use streaming for files >100MB
- Handle errors with `?` operator
- Use tokio runtime for async operations

See `examples/async_json_io.rs` for complete implementation.