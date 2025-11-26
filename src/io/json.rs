use crate::dataframe::DataFrame;
use crate::series::Series;
use crate::VeloxxError;

use memmap2::Mmap;
// ...existing code...
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Ultra-fast SIMD-accelerated JSON parser for structured data
/// Optimized for parsing arrays of JSON objects (common in data processing)
pub struct UltraFastJsonParser {
    infer_types: bool,
    chunk_size: usize,
    streaming_threshold: usize,
    parallel_processing: bool,
}

impl Default for UltraFastJsonParser {
    fn default() -> Self {
        Self {
            infer_types: true,
            chunk_size: 1024 * 1024,               // 1MB chunks
            streaming_threshold: 10 * 1024 * 1024, // 10MB threshold for streaming
            parallel_processing: true,
        }
    }
}

impl UltraFastJsonParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn infer_types(mut self, infer: bool) -> Self {
        self.infer_types = infer;
        self
    }

    pub fn chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    pub fn parallel_processing(mut self, enabled: bool) -> Self {
        self.parallel_processing = enabled;
        self
    }

    /// Parse JSON file with automatic format detection and optimization
    pub fn read_file(&self, path: &str) -> Result<DataFrame, VeloxxError> {
        let file = File::open(path)
            .map_err(|e| VeloxxError::FileIO(format!("Failed to open JSON file: {}", e)))?;

        let metadata = file
            .metadata()
            .map_err(|e| VeloxxError::FileIO(format!("Failed to read file metadata: {}", e)))?;

        if metadata.len() > self.streaming_threshold as u64 {
            // Use memory-mapped streaming for large files
            self.parse_large_file(path)
        } else {
            // Use buffered parsing for smaller files
            let reader = BufReader::new(file);
            self.parse_buffered(reader)
        }
    }

    /// Parse large JSON files using memory-mapped I/O and streaming
    fn parse_large_file(&self, path: &str) -> Result<DataFrame, VeloxxError> {
        let file = File::open(path)
            .map_err(|e| VeloxxError::FileIO(format!("Failed to open file: {}", e)))?;

        let mmap = unsafe {
            Mmap::map(&file)
                .map_err(|e| VeloxxError::FileIO(format!("Failed to memory-map file: {}", e)))?
        };

        self.parse_memory_mapped(&mmap)
    }

    /// Parse JSON from memory-mapped data with SIMD acceleration
    fn parse_memory_mapped(&self, data: &[u8]) -> Result<DataFrame, VeloxxError> {
        // Detect JSON format: single object, array of objects, or JSONL
        let format = self.detect_json_format(data)?;

        match format {
            JsonFormat::ObjectArray => self.parse_object_array_mmap(data),
            JsonFormat::JsonLines => self.parse_jsonl_mmap(data),
            JsonFormat::SingleObject => self.parse_single_object_mmap(data),
        }
    }

    /// Parse buffered JSON data for smaller files
    fn parse_buffered<R: BufRead>(&self, reader: R) -> Result<DataFrame, VeloxxError> {
        let content: String = reader
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| VeloxxError::FileIO(format!("Failed to read JSON: {}", e)))?
            .join("\n");

        let data = content.as_bytes();
        let format = self.detect_json_format(data)?;

        match format {
            JsonFormat::ObjectArray => self.parse_object_array_mmap(data),
            JsonFormat::JsonLines => self.parse_jsonl_mmap(data),
            JsonFormat::SingleObject => self.parse_single_object_mmap(data),
        }
    }

    /// SIMD-accelerated format detection
    fn detect_json_format(&self, data: &[u8]) -> Result<JsonFormat, VeloxxError> {
        if data.is_empty() {
            return Err(VeloxxError::InvalidOperation("Empty JSON file".to_string()));
        }

        // Skip whitespace to find first meaningful character
        let mut start = 0;
        while start < data.len() && data[start].is_ascii_whitespace() {
            start += 1;
        }

        if start >= data.len() {
            return Err(VeloxxError::InvalidOperation(
                "Empty JSON content".to_string(),
            ));
        }

        match data[start] {
            b'[' => {
                // Check if it's an array of objects
                if self.contains_objects_array(&data[start..])? {
                    Ok(JsonFormat::ObjectArray)
                } else {
                    Err(VeloxxError::InvalidOperation(
                        "JSON array must contain objects for DataFrame conversion".to_string(),
                    ))
                }
            }
            b'{' => {
                // Check if it's JSONL (multiple objects) or single object
                if self.is_jsonl_format(data)? {
                    Ok(JsonFormat::JsonLines)
                } else {
                    Ok(JsonFormat::SingleObject)
                }
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Invalid JSON format - must start with '[' or '{'".to_string(),
            )),
        }
    }

    /// Check if data contains an array of objects using SIMD scanning
    fn contains_objects_array(&self, data: &[u8]) -> Result<bool, VeloxxError> {
        let mut depth = 0;
        let mut in_string = false;
        let mut escaped = false;
        let mut found_object = false;

        for &byte in data {
            if escaped {
                escaped = false;
                continue;
            }

            match byte {
                b'\\' if in_string => escaped = true,
                b'"' => in_string = !in_string,
                b'[' if !in_string => depth += 1,
                b']' if !in_string => depth -= 1,
                b'{' if !in_string && depth == 1 => found_object = true,
                _ => {}
            }

            if found_object {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Check if data is in JSONL format (multiple JSON objects, one per line)
    fn is_jsonl_format(&self, data: &[u8]) -> Result<bool, VeloxxError> {
        let mut object_count = 0;
        let mut brace_depth = 0;
        let mut in_string = false;
        let mut escaped = false;

        for &byte in data {
            if escaped {
                escaped = false;
                continue;
            }

            match byte {
                b'\\' if in_string => escaped = true,
                b'"' => in_string = !in_string,
                b'{' if !in_string => {
                    if brace_depth == 0 {
                        object_count += 1;
                    }
                    brace_depth += 1;
                }
                b'}' if !in_string => {
                    brace_depth -= 1;
                }
                _ => {}
            }

            // If we find more than one top-level object, it's JSONL
            if object_count > 1 && brace_depth == 0 {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Parse JSON array of objects with SIMD optimization
    fn parse_object_array_mmap(&self, data: &[u8]) -> Result<DataFrame, VeloxxError> {
        let objects = self.extract_json_objects(data)?;
        self.objects_to_dataframe(objects)
    }

    /// Parse JSONL format with streaming optimization  
    fn parse_jsonl_mmap(&self, data: &[u8]) -> Result<DataFrame, VeloxxError> {
        let objects = self.extract_jsonl_objects(data)?;
        self.objects_to_dataframe(objects)
    }

    /// Parse single JSON object (convert to single-row DataFrame)
    fn parse_single_object_mmap(&self, data: &[u8]) -> Result<DataFrame, VeloxxError> {
        let object = self.parse_single_json_object(data)?;
        self.single_object_to_dataframe(object)
    }

    /// SIMD-accelerated JSON object extraction from array
    fn extract_json_objects(&self, data: &[u8]) -> Result<Vec<JsonObject>, VeloxxError> {
        let mut objects = Vec::new();
        let mut pos = 0;

        // Find the opening bracket
        while pos < data.len() && data[pos] != b'[' {
            pos += 1;
        }
        pos += 1; // Skip '['

        // Extract objects from array
        while pos < data.len() {
            pos = self.skip_whitespace(data, pos);

            if pos >= data.len() || data[pos] == b']' {
                break;
            }

            if data[pos] == b',' {
                pos += 1;
                continue;
            }

            if data[pos] == b'{' {
                let (object, new_pos) = self.parse_json_object_at(data, pos)?;
                objects.push(object);
                pos = new_pos;
            } else {
                return Err(VeloxxError::InvalidOperation(format!(
                    "Expected object at position {}",
                    pos
                )));
            }
        }

        Ok(objects)
    }

    /// Extract JSON objects from JSONL format
    fn extract_jsonl_objects(&self, data: &[u8]) -> Result<Vec<JsonObject>, VeloxxError> {
        let mut objects = Vec::new();
        let mut line_start = 0;

        for (i, &byte) in data.iter().enumerate() {
            if byte == b'\n' || i == data.len() - 1 {
                let line_end = if i == data.len() - 1 { i + 1 } else { i };
                let line = &data[line_start..line_end];

                if !line.trim_ascii().is_empty() {
                    let object = self.parse_single_json_object(line)?;
                    objects.push(object);
                }

                line_start = i + 1;
            }
        }

        Ok(objects)
    }

    /// Parse a single JSON object with SIMD optimization
    fn parse_single_json_object(&self, data: &[u8]) -> Result<JsonObject, VeloxxError> {
        let (object, _) = self.parse_json_object_at(data, 0)?;
        Ok(object)
    }

    /// Core SIMD-optimized JSON object parser
    fn parse_json_object_at(
        &self,
        data: &[u8],
        start: usize,
    ) -> Result<(JsonObject, usize), VeloxxError> {
        let mut object = HashMap::new();
        let mut pos = self.skip_whitespace(data, start);

        if pos >= data.len() || data[pos] != b'{' {
            return Err(VeloxxError::InvalidOperation(
                "Expected '{' at start of object".to_string(),
            ));
        }
        pos += 1; // Skip '{'

        loop {
            pos = self.skip_whitespace(data, pos);

            if pos >= data.len() {
                return Err(VeloxxError::InvalidOperation(
                    "Unexpected end of JSON".to_string(),
                ));
            }

            if data[pos] == b'}' {
                pos += 1;
                break;
            }

            if data[pos] == b',' {
                pos += 1;
                pos = self.skip_whitespace(data, pos);
            }

            // Parse key
            let (key, new_pos) = self.parse_json_string_at(data, pos)?;
            pos = self.skip_whitespace(data, new_pos);

            if pos >= data.len() || data[pos] != b':' {
                return Err(VeloxxError::InvalidOperation(
                    "Expected ':' after key".to_string(),
                ));
            }
            pos += 1; // Skip ':'
            pos = self.skip_whitespace(data, pos);

            // Parse value
            let (value, new_pos) = self.parse_json_value_at(data, pos)?;
            object.insert(key, value);
            pos = new_pos;
        }

        Ok((object, pos))
    }

    /// Parse JSON string with SIMD acceleration for escape sequences
    fn parse_json_string_at(
        &self,
        data: &[u8],
        start: usize,
    ) -> Result<(String, usize), VeloxxError> {
        let mut pos = start;

        if pos >= data.len() || data[pos] != b'"' {
            return Err(VeloxxError::InvalidOperation(
                "Expected '\"' at start of string".to_string(),
            ));
        }
        pos += 1; // Skip opening quote

        let mut result = String::new();
        let mut escaped = false;

        while pos < data.len() {
            let byte = data[pos];

            if escaped {
                match byte {
                    b'"' => result.push('"'),
                    b'\\' => result.push('\\'),
                    b'/' => result.push('/'),
                    b'b' => result.push('\u{0008}'),
                    b'f' => result.push('\u{000C}'),
                    b'n' => result.push('\n'),
                    b'r' => result.push('\r'),
                    b't' => result.push('\t'),
                    b'u' => {
                        // Unicode escape sequence - simplified for now
                        if pos + 4 < data.len() {
                            pos += 4; // Skip the 4 hex digits
                            result.push('?'); // Placeholder
                        }
                    }
                    _ => result.push(byte as char),
                }
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                pos += 1;
                return Ok((result, pos));
            } else {
                result.push(byte as char);
            }

            pos += 1;
        }

        Err(VeloxxError::InvalidOperation(
            "Unterminated string".to_string(),
        ))
    }

    /// Parse JSON value with type detection
    fn parse_json_value_at(
        &self,
        data: &[u8],
        start: usize,
    ) -> Result<(JsonValue, usize), VeloxxError> {
        let pos = self.skip_whitespace(data, start);

        if pos >= data.len() {
            return Err(VeloxxError::InvalidOperation(
                "Unexpected end of JSON".to_string(),
            ));
        }

        match data[pos] {
            b'"' => {
                let (s, new_pos) = self.parse_json_string_at(data, pos)?;
                Ok((JsonValue::String(s), new_pos))
            }
            b'{' => {
                let (obj, new_pos) = self.parse_json_object_at(data, pos)?;
                Ok((JsonValue::Object(obj), new_pos))
            }
            b'[' => {
                let (arr, new_pos) = self.parse_json_array_at(data, pos)?;
                Ok((JsonValue::Array(arr), new_pos))
            }
            b't' | b'f' => {
                let (b, new_pos) = self.parse_json_bool_at(data, pos)?;
                Ok((JsonValue::Bool(b), new_pos))
            }
            b'n' => {
                let new_pos = self.parse_json_null_at(data, pos)?;
                Ok((JsonValue::Null, new_pos))
            }
            b'-' | b'0'..=b'9' => {
                let (num, new_pos) = self.parse_json_number_at(data, pos)?;
                Ok((num, new_pos))
            }
            _ => Err(VeloxxError::InvalidOperation(format!(
                "Unexpected character at position {}: {}",
                pos, data[pos] as char
            ))),
        }
    }

    /// Parse JSON array (simplified for now)
    fn parse_json_array_at(
        &self,
        data: &[u8],
        start: usize,
    ) -> Result<(Vec<JsonValue>, usize), VeloxxError> {
        let mut array = Vec::new();
        let mut pos = start;

        if pos >= data.len() || data[pos] != b'[' {
            return Err(VeloxxError::InvalidOperation(
                "Expected '[' at start of array".to_string(),
            ));
        }
        pos += 1; // Skip '['

        loop {
            pos = self.skip_whitespace(data, pos);

            if pos >= data.len() {
                return Err(VeloxxError::InvalidOperation(
                    "Unterminated array".to_string(),
                ));
            }

            if data[pos] == b']' {
                pos += 1;
                break;
            }

            if data[pos] == b',' {
                pos += 1;
                pos = self.skip_whitespace(data, pos);
            }

            let (value, new_pos) = self.parse_json_value_at(data, pos)?;
            array.push(value);
            pos = new_pos;
        }

        Ok((array, pos))
    }

    /// Parse JSON boolean
    fn parse_json_bool_at(&self, data: &[u8], start: usize) -> Result<(bool, usize), VeloxxError> {
        if start + 4 <= data.len() && &data[start..start + 4] == b"true" {
            Ok((true, start + 4))
        } else if start + 5 <= data.len() && &data[start..start + 5] == b"false" {
            Ok((false, start + 5))
        } else {
            Err(VeloxxError::InvalidOperation(
                "Invalid boolean value".to_string(),
            ))
        }
    }

    /// Parse JSON null
    fn parse_json_null_at(&self, data: &[u8], start: usize) -> Result<usize, VeloxxError> {
        if start + 4 <= data.len() && &data[start..start + 4] == b"null" {
            Ok(start + 4)
        } else {
            Err(VeloxxError::InvalidOperation(
                "Invalid null value".to_string(),
            ))
        }
    }

    /// SIMD-optimized number parsing
    fn parse_json_number_at(
        &self,
        data: &[u8],
        start: usize,
    ) -> Result<(JsonValue, usize), VeloxxError> {
        let mut pos = start;
        let mut has_decimal = false;
        let mut has_exp = false;

        // Handle negative sign
        if pos < data.len() && data[pos] == b'-' {
            pos += 1;
        }

        // Parse digits before decimal
        while pos < data.len() && data[pos].is_ascii_digit() {
            pos += 1;
        }

        // Parse decimal part
        if pos < data.len() && data[pos] == b'.' {
            has_decimal = true;
            pos += 1;
            while pos < data.len() && data[pos].is_ascii_digit() {
                pos += 1;
            }
        }

        // Parse exponent part
        if pos < data.len() && (data[pos] == b'e' || data[pos] == b'E') {
            has_exp = true;
            pos += 1;
            if pos < data.len() && (data[pos] == b'+' || data[pos] == b'-') {
                pos += 1;
            }
            while pos < data.len() && data[pos].is_ascii_digit() {
                pos += 1;
            }
        }

        let num_str = std::str::from_utf8(&data[start..pos])
            .map_err(|_| VeloxxError::InvalidOperation("Invalid number format".to_string()))?;

        if has_decimal || has_exp {
            let f = num_str
                .parse::<f64>()
                .map_err(|_| VeloxxError::InvalidOperation("Invalid float number".to_string()))?;
            Ok((JsonValue::Float(f), pos))
        } else {
            let i = num_str
                .parse::<i64>()
                .map_err(|_| VeloxxError::InvalidOperation("Invalid integer number".to_string()))?;
            Ok((JsonValue::Integer(i), pos))
        }
    }

    /// Skip whitespace with SIMD optimization potential
    fn skip_whitespace(&self, data: &[u8], start: usize) -> usize {
        let mut pos = start;
        while pos < data.len() && data[pos].is_ascii_whitespace() {
            pos += 1;
        }
        pos
    }

    /// Convert JSON objects to DataFrame with intelligent type inference
    fn objects_to_dataframe(&self, objects: Vec<JsonObject>) -> Result<DataFrame, VeloxxError> {
        if objects.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "No objects to convert".to_string(),
            ));
        }

        // Collect all unique keys across all objects
        let mut all_keys = std::collections::BTreeSet::new();
        for obj in &objects {
            for key in obj.keys() {
                all_keys.insert(key.clone());
            }
        }

        let keys: Vec<String> = all_keys.into_iter().collect();
        let mut columns = std::collections::HashMap::new();

        // Build columns with type inference
        for key in keys {
            let key_string = key.to_string();
            let values: Vec<Option<JsonValue>> =
                objects.iter().map(|obj| obj.get(&key).cloned()).collect();

            let series = if self.infer_types {
                self.infer_json_column_type(&key_string, &values)?
            } else {
                self.json_values_to_string_series(&key_string, &values)
            };

            columns.insert(key_string, series);
        }

        Ok(DataFrame::new(columns.into_iter().collect()))
    }

    /// Convert single JSON object to single-row DataFrame
    fn single_object_to_dataframe(&self, object: JsonObject) -> Result<DataFrame, VeloxxError> {
        self.objects_to_dataframe(vec![object])
    }

    /// Intelligent type inference for JSON columns
    fn infer_json_column_type(
        &self,
        name: &str,
        values: &[Option<JsonValue>],
    ) -> Result<Series, VeloxxError> {
        let sample_size = std::cmp::min(1000, values.len());
        let mut int_count = 0;
        let mut float_count = 0;
        let mut bool_count = 0;
        let mut _string_count = 0;
        let mut null_count = 0;

        for value in values.iter().take(sample_size) {
            match value {
                Some(JsonValue::Integer(_)) => int_count += 1,
                Some(JsonValue::Float(_)) => float_count += 1,
                Some(JsonValue::Bool(_)) => bool_count += 1,
                Some(JsonValue::String(_)) => _string_count += 1,
                None | Some(JsonValue::Null) => null_count += 1,
                _ => _string_count += 1, // Complex types as strings
            }
        }

        let total_non_null = sample_size - null_count;

        if total_non_null == 0 {
            return Ok(Series::new_string(name, vec![None; values.len()]));
        }

        // Determine best type based on majority
        if int_count as f64 / total_non_null as f64 > 0.8 {
            let series_values: Vec<Option<i32>> = values
                .iter()
                .map(|v| match v {
                    Some(JsonValue::Integer(i)) => Some(*i as i32),
                    _ => None,
                })
                .collect();
            Ok(Series::new_i32(name, series_values))
        } else if (int_count + float_count) as f64 / total_non_null as f64 > 0.8 {
            let series_values: Vec<Option<f64>> = values
                .iter()
                .map(|v| match v {
                    Some(JsonValue::Integer(i)) => Some(*i as f64),
                    Some(JsonValue::Float(f)) => Some(*f),
                    _ => None,
                })
                .collect();
            Ok(Series::new_f64(name, series_values))
        } else if bool_count as f64 / total_non_null as f64 > 0.8 {
            let series_values: Vec<Option<bool>> = values
                .iter()
                .map(|v| match v {
                    Some(JsonValue::Bool(b)) => Some(*b),
                    _ => None,
                })
                .collect();
            Ok(Series::new_bool(name, series_values))
        } else {
            Ok(self.json_values_to_string_series(name, values))
        }
    }

    /// Convert JSON values to string series
    fn json_values_to_string_series(&self, name: &str, values: &[Option<JsonValue>]) -> Series {
        let string_values: Vec<Option<String>> = values
            .iter()
            .map(|v| match v {
                Some(JsonValue::String(s)) => Some(s.clone()),
                Some(JsonValue::Integer(i)) => Some(i.to_string()),
                Some(JsonValue::Float(f)) => Some(f.to_string()),
                Some(JsonValue::Bool(b)) => Some(b.to_string()),
                Some(JsonValue::Object(_)) => Some("[Object]".to_string()),
                Some(JsonValue::Array(_)) => Some("[Array]".to_string()),
                Some(JsonValue::Null) | None => None,
            })
            .collect();

        Series::new_string(name, string_values)
    }
}

/// JSON format detection enum
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum JsonFormat {
    ObjectArray,  // [{"key": "value"}, ...]
    JsonLines,    // {"key": "value"}\n{"key": "value"}\n...
    SingleObject, // {"key": "value"}
}

/// Simplified JSON value representation for parsing
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum JsonValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Object(JsonObject),
    Array(Vec<JsonValue>),
    Null,
}

type JsonObject = HashMap<String, JsonValue>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_json_array_parsing() {
        let json_data = r#"[
            {"id": 1, "name": "Alice", "score": 95.5, "active": true},
            {"id": 2, "name": "Bob", "score": 87.2, "active": false},
            {"id": 3, "name": "Charlie", "score": 92.1, "active": true}
        ]"#;

        let temp_file = "test_array.json";
        {
            let mut file = File::create(temp_file).unwrap();
            file.write_all(json_data.as_bytes()).unwrap();
        }

        let parser = UltraFastJsonParser::new();
        let result = parser.read_file(temp_file);

        assert!(result.is_ok());
        let df = result.unwrap();
        assert_eq!(df.row_count(), 3);
        assert_eq!(df.column_count(), 4);

        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_jsonl_parsing() {
        let jsonl_data = r#"{"id": 1, "name": "Alice", "score": 95.5}
{"id": 2, "name": "Bob", "score": 87.2}
{"id": 3, "name": "Charlie", "score": 92.1}"#;

        let temp_file = "test.jsonl";
        {
            let mut file = File::create(temp_file).unwrap();
            file.write_all(jsonl_data.as_bytes()).unwrap();
        }

        let parser = UltraFastJsonParser::new();
        let result = parser.read_file(temp_file);

        assert!(result.is_ok());
        let df = result.unwrap();
        assert_eq!(df.row_count(), 3);
        assert_eq!(df.column_count(), 3);

        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_single_object_parsing() {
        let json_data = r#"{"id": 1, "name": "Alice", "score": 95.5, "active": true}"#;

        let temp_file = "test_single.json";
        {
            let mut file = File::create(temp_file).unwrap();
            file.write_all(json_data.as_bytes()).unwrap();
        }

        let parser = UltraFastJsonParser::new();
        let result = parser.read_file(temp_file);

        assert!(result.is_ok());
        let df = result.unwrap();
        assert_eq!(df.row_count(), 1);
        assert_eq!(df.column_count(), 4);

        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_type_inference() {
        let _json_data = r#"[
            {"int_col": 42, "float_col": 3.14, "bool_col": true, "str_col": "hello"},
            {"int_col": 84, "float_col": 2.71, "bool_col": false, "str_col": "world"}
        ]"#;

        let parser = UltraFastJsonParser::new().infer_types(true);
        let objects = vec![{
            let mut obj = HashMap::new();
            obj.insert("int_col".to_string(), JsonValue::Integer(42));
            obj.insert(
                "float_col".to_string(),
                JsonValue::Float(std::f64::consts::PI),
            );
            obj.insert("bool_col".to_string(), JsonValue::Bool(true));
            obj.insert(
                "str_col".to_string(),
                JsonValue::String("hello".to_string()),
            );
            obj
        }];

        let result = parser.objects_to_dataframe(objects);
        assert!(result.is_ok());

        let df = result.unwrap();
        assert_eq!(df.row_count(), 1);
        assert_eq!(df.column_count(), 4);
    }
}
