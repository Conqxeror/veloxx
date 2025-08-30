//! String operations using Arrow's compute functions

#[cfg(feature = "arrow")]
use arrow_array::StringArray;

#[cfg(feature = "arrow")]
use crate::arrow::series::ArrowSeries;
#[cfg(feature = "arrow")]
use crate::VeloxxError;

/// String operations for Arrow Series
#[cfg(feature = "arrow")]
pub trait ArrowStringOps {
    /// Convert strings to uppercase
    fn to_uppercase(&self) -> Result<ArrowSeries, VeloxxError>;

    /// Convert strings to lowercase
    fn to_lowercase(&self) -> Result<ArrowSeries, VeloxxError>;
}

#[cfg(feature = "arrow")]
impl ArrowStringOps for ArrowSeries {
    fn to_uppercase(&self) -> Result<ArrowSeries, VeloxxError> {
        match self {
            ArrowSeries::String(name, array, _null_buffer) => {
                let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
                let uppercase_strings: Vec<Option<String>> =
                    arr.iter().map(|s| s.map(|s| s.to_uppercase())).collect();
                Ok(ArrowSeries::new_string(name, uppercase_strings))
            }
            _ => Err(VeloxxError::Unsupported(
                "To uppercase operation not supported for this series type".to_string(),
            )),
        }
    }

    fn to_lowercase(&self) -> Result<ArrowSeries, VeloxxError> {
        match self {
            ArrowSeries::String(name, array, _null_buffer) => {
                let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
                let lowercase_strings: Vec<Option<String>> =
                    arr.iter().map(|s| s.map(|s| s.to_lowercase())).collect();
                Ok(ArrowSeries::new_string(name, lowercase_strings))
            }
            _ => Err(VeloxxError::Unsupported(
                "To lowercase operation not supported for this series type".to_string(),
            )),
        }
    }
}
