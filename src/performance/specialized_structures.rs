// src/performance/specialized_structures.rs
use crate::VeloxxError;
use std::collections::HashMap;

/// Specialized data structures optimized for specific operations
pub struct SpecializedStructures;

/// Bit-packed boolean array for efficient filtering
#[derive(Debug, Clone)]
pub struct BitPackedArray {
    bits: Vec<u64>,
    length: usize,
}

impl BitPackedArray {
    pub fn new(capacity: usize) -> Self {
        let num_u64s = capacity.div_ceil(64);
        Self {
            bits: vec![0u64; num_u64s],
            length: 0,
        }
    }

    pub fn push(&mut self, value: bool) {
        if self.length >= self.bits.len() * 64 {
            self.bits.push(0);
        }

        let word_index = self.length / 64;
        let bit_index = self.length % 64;

        if value {
            self.bits[word_index] |= 1u64 << bit_index;
        }

        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.length {
            return None;
        }

        let word_index = index / 64;
        let bit_index = index % 64;
        let bit = (self.bits[word_index] >> bit_index) & 1;
        Some(bit == 1)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Iterator over the bits in the array
    pub fn iter(&self) -> BitPackedIterator {
        BitPackedIterator {
            array: self,
            index: 0,
        }
    }

    /// Count the number of true bits using popcount
    pub fn count_ones(&self) -> usize {
        let full_words = self.length / 64;
        let mut count = 0;

        // Count full words
        for i in 0..full_words {
            count += self.bits[i].count_ones() as usize;
        }

        // Handle partial word
        let remaining_bits = self.length % 64;
        if remaining_bits > 0 && full_words < self.bits.len() {
            let mask = (1u64 << remaining_bits) - 1;
            count += (self.bits[full_words] & mask).count_ones() as usize;
        }

        count
    }

    /// Apply filter to a data array efficiently
    pub fn filter_f64(&self, data: &[f64]) -> Result<Vec<f64>, VeloxxError> {
        if data.len() != self.length {
            return Err(VeloxxError::InvalidOperation(
                "Data and filter arrays must have same length".to_string(),
            ));
        }

        let mut result = Vec::with_capacity(self.count_ones());

        for (i, &item) in data.iter().enumerate().take(self.length) {
            if self.get(i).unwrap() {
                result.push(item);
            }
        }

        Ok(result)
    }
}

/// Iterator for BitPackedArray
pub struct BitPackedIterator<'a> {
    array: &'a BitPackedArray,
    index: usize,
}

impl<'a> Iterator for BitPackedIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let result = self.array.get(self.index);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

/// Compressed sparse row format for sparse operations
#[derive(Debug)]
pub struct SparseArray {
    indices: Vec<usize>,
    values: Vec<f64>,
    length: usize,
}

impl SparseArray {
    pub fn new(length: usize) -> Self {
        Self {
            indices: Vec::new(),
            values: Vec::new(),
            length,
        }
    }

    pub fn push(&mut self, index: usize, value: f64) -> Result<(), VeloxxError> {
        if index >= self.length {
            return Err(VeloxxError::InvalidOperation(
                "Index out of bounds".to_string(),
            ));
        }

        // Maintain sorted order by index
        match self.indices.binary_search(&index) {
            Ok(pos) => {
                // Update existing value
                self.values[pos] = value;
            }
            Err(pos) => {
                // Insert new value at correct position
                self.indices.insert(pos, index);
                self.values.insert(pos, value);
            }
        }

        Ok(())
    }

    pub fn get(&self, index: usize) -> f64 {
        match self.indices.binary_search(&index) {
            Ok(pos) => self.values[pos],
            Err(_) => 0.0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn nnz(&self) -> usize {
        self.indices.len()
    }

    /// Sum all non-zero values
    pub fn sum(&self) -> f64 {
        self.values.iter().sum()
    }

    /// Add two sparse arrays
    pub fn add(&self, other: &SparseArray) -> Result<SparseArray, VeloxxError> {
        if self.length != other.length {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have same length".to_string(),
            ));
        }

        let mut result = SparseArray::new(self.length);
        let mut i = 0;
        let mut j = 0;

        while i < self.indices.len() || j < other.indices.len() {
            if i >= self.indices.len() {
                // Only other has remaining elements
                result.indices.push(other.indices[j]);
                result.values.push(other.values[j]);
                j += 1;
            } else if j >= other.indices.len() {
                // Only self has remaining elements
                result.indices.push(self.indices[i]);
                result.values.push(self.values[i]);
                i += 1;
            } else if self.indices[i] < other.indices[j] {
                result.indices.push(self.indices[i]);
                result.values.push(self.values[i]);
                i += 1;
            } else if self.indices[i] > other.indices[j] {
                result.indices.push(other.indices[j]);
                result.values.push(other.values[j]);
                j += 1;
            } else {
                // Same index, add values
                let sum = self.values[i] + other.values[j];
                if sum != 0.0 {
                    result.indices.push(self.indices[i]);
                    result.values.push(sum);
                }
                i += 1;
                j += 1;
            }
        }

        Ok(result)
    }
}

/// Hash-based groupby structure for efficient aggregation
#[derive(Debug)]
pub struct HashGroupBy {
    groups: HashMap<u64, GroupAccumulator>,
}

#[derive(Debug, Clone)]
struct GroupAccumulator {
    sum: f64,
    count: usize,
    min: f64,
    max: f64,
}

impl Default for HashGroupBy {
    fn default() -> Self {
        Self::new()
    }
}

impl HashGroupBy {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }

    pub fn add_value(&mut self, group_key: u64, value: f64) {
        let entry = self.groups.entry(group_key).or_insert(GroupAccumulator {
            sum: 0.0,
            count: 0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        });

        entry.sum += value;
        entry.count += 1;
        entry.min = entry.min.min(value);
        entry.max = entry.max.max(value);
    }

    pub fn get_sums(&self) -> Vec<(u64, f64)> {
        self.groups
            .iter()
            .map(|(&key, acc)| (key, acc.sum))
            .collect()
    }

    pub fn get_counts(&self) -> Vec<(u64, usize)> {
        self.groups
            .iter()
            .map(|(&key, acc)| (key, acc.count))
            .collect()
    }

    pub fn get_averages(&self) -> Vec<(u64, f64)> {
        self.groups
            .iter()
            .map(|(&key, acc)| (key, acc.sum / acc.count as f64))
            .collect()
    }

    pub fn get_min_max(&self) -> Vec<(u64, f64, f64)> {
        self.groups
            .iter()
            .map(|(&key, acc)| (key, acc.min, acc.max))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_packed_array() {
        let mut bits = BitPackedArray::new(10);
        bits.push(true);
        bits.push(false);
        bits.push(true);
        bits.push(true);
        bits.push(false);

        assert_eq!(bits.get(0), Some(true));
        assert_eq!(bits.get(1), Some(false));
        assert_eq!(bits.get(2), Some(true));
        assert_eq!(bits.count_ones(), 3);

        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let filtered = bits.filter_f64(&data).unwrap();
        assert_eq!(filtered, vec![1.0, 3.0, 4.0]);
    }

    #[test]
    fn test_sparse_array() {
        let mut sparse = SparseArray::new(10);
        sparse.push(2, 5.0).unwrap();
        sparse.push(7, 10.0).unwrap();
        sparse.push(1, 3.0).unwrap();

        assert_eq!(sparse.get(0), 0.0);
        assert_eq!(sparse.get(1), 3.0);
        assert_eq!(sparse.get(2), 5.0);
        assert_eq!(sparse.get(7), 10.0);
        assert_eq!(sparse.sum(), 18.0);
        assert_eq!(sparse.nnz(), 3);
    }

    #[test]
    fn test_sparse_array_add() {
        let mut a = SparseArray::new(5);
        a.push(1, 3.0).unwrap();
        a.push(3, 5.0).unwrap();

        let mut b = SparseArray::new(5);
        b.push(1, 2.0).unwrap();
        b.push(2, 4.0).unwrap();

        let result = a.add(&b).unwrap();
        assert_eq!(result.get(1), 5.0); // 3.0 + 2.0
        assert_eq!(result.get(2), 4.0);
        assert_eq!(result.get(3), 5.0);
    }

    #[test]
    fn test_hash_groupby() {
        let mut groupby = HashGroupBy::new();
        groupby.add_value(1, 10.0);
        groupby.add_value(1, 20.0);
        groupby.add_value(2, 5.0);
        groupby.add_value(2, 15.0);
        groupby.add_value(2, 25.0);

        let sums = groupby.get_sums();
        let counts = groupby.get_counts();
        let averages = groupby.get_averages();

        // Note: HashMap doesn't guarantee order, so we need to sort for testing
        let mut sums_sorted = sums;
        sums_sorted.sort_by_key(|&(k, _)| k);
        assert_eq!(sums_sorted, vec![(1, 30.0), (2, 45.0)]);

        let mut counts_sorted = counts;
        counts_sorted.sort_by_key(|&(k, _)| k);
        assert_eq!(counts_sorted, vec![(1, 2), (2, 3)]);

        let mut avg_sorted = averages;
        avg_sorted.sort_by_key(|&(k, _)| k);
        assert_eq!(avg_sorted, vec![(1, 15.0), (2, 15.0)]);
    }
}
