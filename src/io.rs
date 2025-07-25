use crate::error::VeloxxError;
use crate::DataFrame;

pub struct JsonReader;
pub struct JsonWriter;

impl JsonReader {
    pub fn new() -> Self {
        JsonReader
    }
    
    pub fn read_file(&self, _path: &str) -> Result<DataFrame, VeloxxError> {
        use std::collections::BTreeMap;
        Ok(DataFrame { columns: BTreeMap::new() })
    }
    
    pub fn read_string(&self, _s: &str) -> Option<DataFrame> {
        use std::collections::BTreeMap;
        Some(DataFrame { columns: BTreeMap::new() })
    }
    
    pub fn stream_string(&self, _s: &str, _n: usize) -> Option<DataFrame> {
        use std::collections::BTreeMap;
        Some(DataFrame { columns: BTreeMap::new() })
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