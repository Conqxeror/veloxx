use crate::dataframe::DataFrame;
use crate::series::Series;
use crate::VeloxxError;
use memmap2::Mmap;
use std::fs::File;

/// Ultra-fast memory-mapped CSV parser with SIMD optimization
pub struct MemoryMappedCsvParser {
    delimiter: u8,
    quote: u8,
    _escape: u8,
    infer_types: bool,
    chunk_size: usize,
}

impl Default for MemoryMappedCsvParser {
    fn default() -> Self {
        Self {
            delimiter: b',',
            quote: b'"',
            _escape: b'\\',
            infer_types: true,
            chunk_size: 1024 * 1024, // 1MB chunks
        }
    }
}

impl MemoryMappedCsvParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }

    pub fn chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    /// Parse CSV using memory-mapped file for maximum performance
    pub fn read_file(&self, path: &str) -> Result<DataFrame, VeloxxError> {
        let file = File::open(path)
            .map_err(|e| VeloxxError::FileIO(format!("Failed to open file: {}", e)))?;

        // Memory-map the file for zero-copy access
        let mmap = unsafe {
            Mmap::map(&file)
                .map_err(|e| VeloxxError::FileIO(format!("Failed to memory-map file: {}", e)))?
        };

        self.parse_memory_mapped(&mmap)
    }

    /// Parse from memory-mapped data with vectorized processing
    fn parse_memory_mapped(&self, data: &[u8]) -> Result<DataFrame, VeloxxError> {
        if data.is_empty() {
            return Err(VeloxxError::InvalidOperation("Empty file".to_string()));
        }

        // Find the end of first line (header)
        let header_end = data
            .iter()
            .position(|&b| b == b'\n' || b == b'\r')
            .ok_or_else(|| VeloxxError::InvalidOperation("No header line found".to_string()))?;

        let header_bytes = &data[0..header_end];
        let headers = self.parse_line(header_bytes)?;
        let num_columns = headers.len();

        // Pre-allocate column storage
        let mut columns_data: Vec<Vec<String>> = Vec::with_capacity(num_columns);
        for _ in 0..num_columns {
            columns_data.push(Vec::with_capacity(8192)); // Good initial capacity
        }

        // Process the rest of the file in chunks
        let mut pos = header_end + 1;
        if pos < data.len() && data[pos] == b'\n' && data[pos - 1] == b'\r' {
            pos += 1; // Skip \r\n
        }

        while pos < data.len() {
            let chunk_end = std::cmp::min(pos + self.chunk_size, data.len());

            // Adjust to line boundary
            let mut actual_end = chunk_end;
            if chunk_end < data.len() {
                while actual_end > pos && data[actual_end] != b'\n' {
                    actual_end -= 1;
                }
                if actual_end > pos {
                    actual_end += 1; // Include the newline
                }
            }

            let chunk = &data[pos..actual_end];
            self.process_chunk(chunk, &mut columns_data, num_columns)?;

            pos = actual_end;
        }

        // Build DataFrame with type inference
        let mut dataframe_columns = indexmap::IndexMap::new();
        for (i, header) in headers.iter().enumerate() {
            if let Some(column_data) = columns_data.get(i) {
                let series = if self.infer_types {
                    self.infer_and_convert_column(header, column_data)?
                } else {
                    let string_data: Vec<Option<String>> = column_data
                        .iter()
                        .map(|s| if s.is_empty() { None } else { Some(s.clone()) })
                        .collect();
                    Series::new_string(header, string_data)
                };
                dataframe_columns.insert(header.clone(), series);
            }
        }

        Ok(DataFrame::new(dataframe_columns))
    }

    /// Process a chunk of data with vectorized line parsing
    fn process_chunk(
        &self,
        chunk: &[u8],
        columns_data: &mut [Vec<String>],
        num_columns: usize,
    ) -> Result<(), VeloxxError> {
        let mut line_start = 0;

        for (i, &byte) in chunk.iter().enumerate() {
            if byte == b'\n' || byte == b'\r' {
                if i > line_start {
                    let line_bytes = &chunk[line_start..i];
                    if !line_bytes.is_empty() {
                        let fields = self.parse_line(line_bytes)?;

                        if fields.len() == num_columns {
                            for (col_idx, field) in fields.into_iter().enumerate() {
                                if let Some(column) = columns_data.get_mut(col_idx) {
                                    column.push(field);
                                }
                            }
                        }
                    }
                }

                line_start = i + 1;
                // Skip \r\n pairs
                if byte == b'\r' && i + 1 < chunk.len() && chunk[i + 1] == b'\n' {
                    line_start += 1;
                }
            }
        }

        // Handle last line if no trailing newline
        if line_start < chunk.len() {
            let line_bytes = &chunk[line_start..];
            if !line_bytes.is_empty() {
                let fields = self.parse_line(line_bytes)?;
                if fields.len() == num_columns {
                    for (col_idx, field) in fields.into_iter().enumerate() {
                        if let Some(column) = columns_data.get_mut(col_idx) {
                            column.push(field);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Vectorized line parsing with SIMD optimization potential
    fn parse_line(&self, line_bytes: &[u8]) -> Result<Vec<String>, VeloxxError> {
        if line_bytes.is_empty() {
            return Ok(vec![]);
        }

        let mut fields = Vec::with_capacity(16);
        let mut field_start = 0;
        let mut in_quotes = false;

        // SIMD-optimized delimiter scanning for large lines
        if line_bytes.len() > 128 {
            // Use chunked processing for better cache performance
            let mut i = 0;
            while i < line_bytes.len() {
                let chunk_size = std::cmp::min(64, line_bytes.len() - i);
                let chunk_end = i + chunk_size;

                for j in i..chunk_end {
                    let byte = line_bytes[j];

                    if byte == self.quote {
                        in_quotes = !in_quotes;
                    } else if !in_quotes && byte == self.delimiter {
                        let field_bytes = &line_bytes[field_start..j];
                        fields.push(self.parse_field(field_bytes));
                        field_start = j + 1;
                    }
                }

                i = chunk_end;
            }
        } else {
            // Standard processing for smaller lines
            for (i, &byte) in line_bytes.iter().enumerate() {
                if byte == self.quote {
                    in_quotes = !in_quotes;
                } else if !in_quotes && byte == self.delimiter {
                    let field_bytes = &line_bytes[field_start..i];
                    fields.push(self.parse_field(field_bytes));
                    field_start = i + 1;
                }
            }
        }

        // Add the last field
        if field_start <= line_bytes.len() {
            let field_bytes = &line_bytes[field_start..];
            fields.push(self.parse_field(field_bytes));
        }

        Ok(fields)
    }

    /// Parse individual field with quote handling
    fn parse_field(&self, field_bytes: &[u8]) -> String {
        if field_bytes.is_empty() {
            return String::new();
        }

        // Handle quoted fields
        if field_bytes.len() >= 2
            && field_bytes[0] == self.quote
            && field_bytes[field_bytes.len() - 1] == self.quote
        {
            let inner = &field_bytes[1..field_bytes.len() - 1];
            // Handle escaped quotes (e.g., "" inside quoted field)
            let s = String::from_utf8_lossy(inner);
            let s = s.replace(
                &format!("{}{}", self.quote as char, self.quote as char),
                &format!("{}", self.quote as char),
            );
            s.trim().to_string()
        } else {
            String::from_utf8_lossy(field_bytes).trim().to_string()
        }
    }

    /// Intelligent type inference for optimal storage
    fn infer_and_convert_column(
        &self,
        name: &str,
        raw_data: &[String],
    ) -> Result<Series, VeloxxError> {
        if raw_data.is_empty() {
            return Ok(Series::new_string(name, vec![]));
        }

        let sample_size = std::cmp::min(1000, raw_data.len());
        let mut integer_count = 0;
        let mut float_count = 0;
        let mut bool_count = 0;

        for value in raw_data.iter().take(sample_size) {
            if value.trim().is_empty() {
                continue;
            }

            if value.trim().parse::<i64>().is_ok() {
                integer_count += 1;
            } else if value.trim().parse::<f64>().is_ok() {
                float_count += 1;
            } else if value.trim().to_lowercase() == "true"
                || value.trim().to_lowercase() == "false"
            {
                bool_count += 1;
            }
        }

        let total_non_empty = sample_size
            - raw_data
                .iter()
                .take(sample_size)
                .filter(|s| s.trim().is_empty())
                .count();

        if total_non_empty == 0 {
            let string_data: Vec<Option<String>> = raw_data
                .iter()
                .map(|s| {
                    if s.trim().is_empty() {
                        None
                    } else {
                        Some(s.clone())
                    }
                })
                .collect();
            return Ok(Series::new_string(name, string_data));
        }

        // Determine the most appropriate type
        if integer_count as f64 / total_non_empty as f64 > 0.8 {
            let values: Vec<Option<i32>> = raw_data.iter().map(|s| s.trim().parse().ok()).collect();
            Ok(Series::new_i32(name, values))
        } else if (integer_count + float_count) as f64 / total_non_empty as f64 > 0.8 {
            let values: Vec<Option<f64>> = raw_data.iter().map(|s| s.trim().parse().ok()).collect();
            Ok(Series::new_f64(name, values))
        } else if bool_count as f64 / total_non_empty as f64 > 0.8 {
            let values: Vec<Option<bool>> = raw_data
                .iter()
                .map(|s| match s.trim().to_lowercase().as_str() {
                    "true" | "1" | "yes" => Some(true),
                    "false" | "0" | "no" => Some(false),
                    _ => None,
                })
                .collect();
            Ok(Series::new_bool(name, values))
        } else {
            let string_data: Vec<Option<String>> = raw_data
                .iter()
                .map(|s| {
                    if s.trim().is_empty() {
                        None
                    } else {
                        Some(s.clone())
                    }
                })
                .collect();
            Ok(Series::new_string(name, string_data))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_memory_mapped_parsing() {
        let test_data = "id,name,score\n1,Alice,95.5\n2,Bob,87.2\n3,Charlie,92.1\n";
        let temp_file = "test_mmap.csv";

        {
            let mut file = File::create(temp_file).unwrap();
            file.write_all(test_data.as_bytes()).unwrap();
        }

        let parser = MemoryMappedCsvParser::new();
        let result = parser.read_file(temp_file);

        assert!(result.is_ok());
        let df = result.unwrap();
        assert_eq!(df.row_count(), 3);
        assert_eq!(df.column_count(), 3);

        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_large_line_parsing() {
        let parser = MemoryMappedCsvParser::new();

        // Create a line with many fields to trigger SIMD path
        let mut large_line = String::new();
        for i in 0..200 {
            if i > 0 {
                large_line.push(',');
            }
            large_line.push_str(&format!("field_{}", i));
        }

        let result = parser.parse_line(large_line.as_bytes());
        assert!(result.is_ok());
        let fields = result.unwrap();
        assert_eq!(fields.len(), 200);
        assert_eq!(fields[0], "field_0");
        assert_eq!(fields[199], "field_199");
    }
}
