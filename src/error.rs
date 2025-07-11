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
#[derive(Error, Debug, PartialEq)]
pub enum VeloxxError {
    /// Indicates that a specified column was not found in the DataFrame.
    #[error("Column not found: {0}")]
    ColumnNotFound(String),
    /// Indicates that an operation is invalid given the current state or inputs.
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    /// Indicates a mismatch in data types during an operation (e.g., casting incompatible types).
    #[error("Data type mismatch: {0}")]
    DataTypeMismatch(String),
    /// Represents an I/O error that occurred during file operations.
    #[error("File I/O error: {0}")]
    FileIO(String),
    /// Indicates an error during parsing of data (e.g., from a string to a number).
    #[error("Parsing error: {0}")]
    Parsing(String),
    /// Indicates that a requested feature is not yet supported.
    #[error("Unsupported feature: {0}")]
    Unsupported(String),
    /// A general catch-all for other unexpected errors.
    #[error("Other error: {0}")]
    Other(String),
}

impl From<std::io::Error> for VeloxxError {
    /// Converts a `std::io::Error` into a `VeloxxError::FileIO`.
    fn from(err: std::io::Error) -> Self {
        VeloxxError::FileIO(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for VeloxxError {
    /// Converts a `std::string::FromUtf8Error` into a `VeloxxError::Parsing`.
    fn from(err: std::string::FromUtf8Error) -> Self {
        VeloxxError::Parsing(err.to_string())
    }
}

#[cfg(feature = "python")]
impl From<VeloxxError> for pyo3::PyErr {
    fn from(err: VeloxxError) -> Self {
        pyo3::exceptions::PyValueError::new_err(err.to_string())
    }
}