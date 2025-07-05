use thiserror::Error;

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
    #[error("Other error: {0}")]
    Other(String),
}

impl From<std::io::Error> for VeloxxError {
    fn from(err: std::io::Error) -> Self {
        VeloxxError::FileIO(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for VeloxxError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        VeloxxError::Parsing(err.to_string())
    }
}

