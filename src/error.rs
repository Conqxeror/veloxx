#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, PartialEq)]
pub enum VeloxxError {
    ColumnNotFound(String),
    InvalidOperation(String),
    DataTypeMismatch(String),
    FileIO(String),
    Parsing(String),
    Unsupported(String),
    MemoryError(String),
    ExecutionError(String),
    Other(String),
}

#[cfg(target_arch = "wasm32")]
impl std::fmt::Display for VeloxxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VeloxxError::ColumnNotFound(msg) => write!(f, "Column not found: {}", msg),
            VeloxxError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            VeloxxError::DataTypeMismatch(msg) => write!(f, "Data type mismatch: {}", msg),
            VeloxxError::FileIO(msg) => write!(f, "File I/O error: {}", msg),
            VeloxxError::Parsing(msg) => write!(f, "Parsing error: {}", msg),
            VeloxxError::Unsupported(msg) => write!(f, "Unsupported operation: {}", msg),
            VeloxxError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            VeloxxError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            VeloxxError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl std::error::Error for VeloxxError {}
// This file handles error types for the Veloxx library.
// Ensure that any error handling that uses non-WASM-compatible dependencies
// is feature gated and excluded from WASM builds.
#[cfg(not(target_arch = "wasm32"))]
use thiserror::Error;

/// Custom error type for the Veloxx library.
///
/// This enum unifies error handling across the library, providing specific error variants
/// for common issues like column not found, invalid operations, data type mismatches,
/// and I/O errors.
///
/// # Examples
///
/// ```rust
/// use veloxx::error::VeloxxError;
///
/// // Example of creating a ColumnNotFound error
/// let err = VeloxxError::ColumnNotFound("my_column".to_string());
/// println!("Error: {}", err);
/// // Output: Error: Column not found: my_column
///
/// // Example of creating an InvalidOperation error
/// let err = VeloxxError::InvalidOperation("Cannot divide by zero".to_string());
/// println!("Error: {}", err);
/// // Output: Error: Invalid operation: Cannot divide by zero
/// ```
#[cfg(not(target_arch = "wasm32"))]
#[derive(Error, Debug, PartialEq)]
pub enum VeloxxError {
    #[error("Column not found: {0}")]
    ColumnNotFound(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Data type mismatch: {0}")]
    DataTypeMismatch(String),
    #[error("File I/O error: {0}")]
    FileIO(String),
    #[error("Parsing error: {0}")]
    Parsing(String),
    #[error("Unsupported feature: {0}")]
    Unsupported(String),
    #[error("Memory error: {0}")]
    MemoryError(String),
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::io::Error> for VeloxxError {
    fn from(err: std::io::Error) -> Self {
        VeloxxError::FileIO(err.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::string::FromUtf8Error> for VeloxxError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        VeloxxError::Parsing(err.to_string())
    }
}

#[cfg(all(feature = "advanced_io", not(target_arch = "wasm32")))]
impl From<parquet::errors::ParquetError> for VeloxxError {
    fn from(err: parquet::errors::ParquetError) -> Self {
        VeloxxError::FileIO(err.to_string())
    }
}

#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
impl From<arrow::error::ArrowError> for VeloxxError {
    fn from(err: arrow::error::ArrowError) -> Self {
        VeloxxError::Parsing(err.to_string())
    }
}

#[cfg(all(feature = "python", not(target_arch = "wasm32")))]
impl From<VeloxxError> for pyo3::PyErr {
    fn from(err: VeloxxError) -> Self {
        pyo3::exceptions::PyValueError::new_err(err.to_string())
    }
}
