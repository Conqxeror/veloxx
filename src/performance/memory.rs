//! Memory optimization utilities
//!
//! This module provides memory-efficient data structures and operations
//! for improved performance and reduced memory footprint.

use crate::error::VeloxxError;
use crate::series::Series;
use std::collections::HashMap;

/// Memory usage analyzer
pub struct MemoryAnalyzer;

impl MemoryAnalyzer {
    /// Estimate memory usage of a series
    pub fn estimate_series_memory(series: &Series) -> usize {
        match series {
            Series::I32(name, values) => {
                name.len() + values.len() * std::mem::size_of::<Option<i32>>()
            }
            Series::F64(name, values) => {
                name.len() + values.len() * std::mem::size_of::<Option<f64>>()
            }
            Series::Bool(name, values) => {
                name.len() + values.len() * std::mem::size_of::<Option<bool>>()
            }
            Series::String(name, values) => {
                name.len()
                    + values
                        .iter()
                        .map(|v| match v {
                            Some(s) => s.len() + std::mem::size_of::<Option<String>>(),
                            None => std::mem::size_of::<Option<String>>(),
                        })
                        .sum::<usize>()
            }
            Series::DateTime(name, values) => {
                name.len() + values.len() * std::mem::size_of::<Option<i64>>()
            }
        }
    }

    /// Suggest compression strategy for a series
    pub fn suggest_compression(series: &Series) -> Vec<&'static str> {
        let mut suggestions = Vec::new();

        match series {
            Series::String(_, _) => {
                suggestions.push("dictionary");
            }
            Series::Bool(_, _) => {
                suggestions.push("bit_packed");
            }
            Series::DateTime(_, values) => {
                // Check if values are sequential or have small deltas
                let mut is_sequential = true;
                let mut has_small_deltas = true;

                if values.len() > 1 {
                    for i in 1..values.len() {
                        if let (Some(prev), Some(curr)) = (values[i - 1], values[i]) {
                            let delta = curr - prev;
                            if delta.abs() > 1000 {
                                has_small_deltas = false;
                            }
                            if delta != 1 {
                                is_sequential = false;
                            }
                        }
                    }
                }

                if is_sequential || has_small_deltas {
                    suggestions.push("delta_encoded");
                }
            }
            _ => {}
        }

        // Check for run-length encoding potential
        let mut consecutive_count = 1;
        let mut max_consecutive = 1;

        for i in 1..series.len() {
            if series.get_value(i) == series.get_value(i - 1) {
                consecutive_count += 1;
                max_consecutive = max_consecutive.max(consecutive_count);
            } else {
                consecutive_count = 1;
            }
        }

        if max_consecutive > 3 {
            suggestions.push("run_length");
        }

        suggestions
    }
}

/// Memory-efficient column storage using compression
pub enum CompressedColumn {
    /// Run-length encoded values
    RunLength {
        values: Vec<String>,
        counts: Vec<usize>,
    },
    /// Dictionary encoded strings
    Dictionary {
        dictionary: Vec<String>,
        indices: Vec<Option<u32>>,
    },
}

impl CompressedColumn {
    /// Create a run-length encoded column
    pub fn from_run_length(series: &Series) -> Result<Self, VeloxxError> {
        let mut values = Vec::new();
        let mut counts = Vec::new();

        if series.is_empty() {
            return Ok(CompressedColumn::RunLength { values, counts });
        }

        let mut current_value = format!("{:?}", series.get_value(0));
        let mut current_count = 1;

        for i in 1..series.len() {
            let value = format!("{:?}", series.get_value(i));
            if value == current_value {
                current_count += 1;
            } else {
                values.push(current_value);
                counts.push(current_count);
                current_value = value;
                current_count = 1;
            }
        }

        // Add the last run
        values.push(current_value);
        counts.push(current_count);

        Ok(CompressedColumn::RunLength { values, counts })
    }

    /// Create a dictionary encoded column for strings
    pub fn from_dictionary(series: &Series) -> Result<Self, VeloxxError> {
        match series {
            Series::String(_, values) => {
                let mut dictionary = Vec::new();
                let mut dict_map = HashMap::new();
                let mut indices = Vec::new();

                for value in values {
                    match value {
                        Some(s) => {
                            let index = if let Some(&idx) = dict_map.get(s) {
                                idx
                            } else {
                                let idx = dictionary.len() as u32;
                                dictionary.push(s.clone());
                                dict_map.insert(s.clone(), idx);
                                idx
                            };
                            indices.push(Some(index));
                        }
                        None => indices.push(None),
                    }
                }

                Ok(CompressedColumn::Dictionary {
                    dictionary,
                    indices,
                })
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Dictionary encoding only supported for string columns".to_string(),
            )),
        }
    }

    /// Get the compression ratio (original size / compressed size)
    pub fn compression_ratio(&self, original_series: &Series) -> f64 {
        let original_size = MemoryAnalyzer::estimate_series_memory(original_series);
        let compressed_size = self.compressed_size();

        if compressed_size == 0 {
            1.0
        } else {
            original_size as f64 / compressed_size as f64
        }
    }

    /// Estimate the size of the compressed column in bytes
    pub fn compressed_size(&self) -> usize {
        match self {
            CompressedColumn::RunLength { values, counts } => {
                values.iter().map(|s| s.len()).sum::<usize>()
                    + counts.len() * std::mem::size_of::<usize>()
            }
            CompressedColumn::Dictionary {
                dictionary,
                indices,
            } => {
                dictionary.iter().map(|s| s.len()).sum::<usize>()
                    + indices.len() * std::mem::size_of::<Option<u32>>()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::Series;

    #[test]
    fn test_run_length_compression() {
        let series = Series::new_i32(
            "test",
            vec![Some(1), Some(1), Some(1), Some(2), Some(2), Some(3)],
        );

        let compressed = CompressedColumn::from_run_length(&series).unwrap();

        match compressed {
            CompressedColumn::RunLength { values, counts } => {
                assert_eq!(values.len(), 3);
                assert_eq!(counts, vec![3, 2, 1]);
            }
            _ => panic!("Expected run-length encoding"),
        }
    }

    #[test]
    fn test_dictionary_compression() {
        let series = Series::new_string(
            "test",
            vec![
                Some("apple".to_string()),
                Some("banana".to_string()),
                Some("apple".to_string()),
                Some("cherry".to_string()),
                Some("banana".to_string()),
            ],
        );

        let compressed = CompressedColumn::from_dictionary(&series).unwrap();

        match compressed {
            CompressedColumn::Dictionary {
                dictionary,
                indices,
            } => {
                assert_eq!(dictionary.len(), 3); // apple, banana, cherry
                assert_eq!(indices.len(), 5);
            }
            _ => panic!("Expected dictionary encoding"),
        }
    }

    #[test]
    fn test_memory_analyzer() {
        let series = Series::new_i32("test", vec![Some(1), Some(2), Some(3)]);
        let memory_usage = MemoryAnalyzer::estimate_series_memory(&series);
        assert!(memory_usage > 0);

        // Test with string series which should have dictionary suggestion
        let string_series = Series::new_string(
            "test",
            vec![
                Some("apple".to_string()),
                Some("banana".to_string()),
                Some("apple".to_string()),
            ],
        );
        let suggestions = MemoryAnalyzer::suggest_compression(&string_series);
        assert!(!suggestions.is_empty());
        assert!(suggestions.contains(&"dictionary"));
    }
}
