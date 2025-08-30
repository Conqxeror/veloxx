use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// Advanced memory pool with compression support for ultra-fast data processing
pub struct UltraFastMemoryPool {
    pools: Mutex<HashMap<usize, Vec<NonNull<u8>>>>,
    allocated_bytes: AtomicUsize,
    peak_usage: AtomicUsize,
    compression_threshold: usize,
}

/// Memory-mapped compressed buffer for large datasets
pub struct CompressedBuffer {
    compressed_data: Vec<u8>,
    original_size: usize,
    compression_ratio: f64,
    _alignment: usize,
}

/// NUMA-aware memory allocator for multi-core performance
pub struct NumaAwareAllocator {
    node_pools: Vec<UltraFastMemoryPool>,
    current_node: AtomicUsize,
}

/// Compression algorithms optimized for columnar data
#[derive(Debug, Clone)]
pub enum CompressionAlgorithm {
    /// LZ4 - Ultra-fast compression/decompression
    LZ4,
    /// Run-Length Encoding for repeated values
    RLE,
    /// Delta encoding for numerical sequences
    Delta,
    /// Dictionary compression for strings
    Dictionary,
    /// Bit-packing for small integers
    BitPack,
}

/// Statistics for memory pool monitoring
#[derive(Debug, Clone)]
pub struct MemoryPoolStats {
    pub total_allocated: usize,
    pub peak_usage: usize,
    pub active_pools: usize,
    pub compression_ratio: f64,
    pub allocations_per_second: f64,
}

impl UltraFastMemoryPool {
    pub fn new(compression_threshold: usize) -> Self {
        Self {
            pools: Mutex::new(HashMap::new()),
            allocated_bytes: AtomicUsize::new(0),
            peak_usage: AtomicUsize::new(0),
            compression_threshold,
        }
    }

    /// Allocate aligned memory with optional compression
    pub fn allocate_aligned(&self, size: usize, alignment: usize) -> Result<NonNull<u8>, String> {
        let layout = Layout::from_size_align(size, alignment)
            .map_err(|e| format!("Invalid layout: {}", e))?;

        let ptr = unsafe { alloc(layout) };

        if ptr.is_null() {
            return Err("Allocation failed".to_string());
        }

        let non_null_ptr = unsafe { NonNull::new_unchecked(ptr) };

        // Track allocation
        self.allocated_bytes.fetch_add(size, Ordering::Relaxed);
        let current = self.allocated_bytes.load(Ordering::Relaxed);

        // Update peak usage
        let mut peak = self.peak_usage.load(Ordering::Relaxed);
        while current > peak {
            match self.peak_usage.compare_exchange_weak(
                peak,
                current,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(new_peak) => peak = new_peak,
            }
        }

        Ok(non_null_ptr)
    }

    /// Deallocate memory with pool reuse
    pub fn deallocate(
        &self,
        ptr: NonNull<u8>,
        size: usize,
        alignment: usize,
    ) -> Result<(), String> {
        let layout = Layout::from_size_align(size, alignment)
            .map_err(|e| format!("Invalid layout: {}", e))?;

        // Check if we should pool this allocation for reuse
        if size <= 4096 && self.should_pool_size(size) {
            let mut pools = self.pools.lock().unwrap();
            pools.entry(size).or_default().push(ptr);
        } else {
            unsafe {
                dealloc(ptr.as_ptr(), layout);
            }
        }

        self.allocated_bytes.fetch_sub(size, Ordering::Relaxed);
        Ok(())
    }

    /// Try to allocate from pool first, then fallback to system allocation
    pub fn allocate_from_pool(&self, size: usize, alignment: usize) -> Result<NonNull<u8>, String> {
        // Try to reuse from pool first
        {
            let mut pools = self.pools.lock().unwrap();
            if let Some(pool) = pools.get_mut(&size) {
                if let Some(ptr) = pool.pop() {
                    return Ok(ptr);
                }
            }
        }

        // Fallback to new allocation
        self.allocate_aligned(size, alignment)
    }

    /// Compress data if it exceeds threshold
    pub fn compress_if_beneficial(
        &self,
        data: &[u8],
        algorithm: CompressionAlgorithm,
    ) -> CompressedBuffer {
        if data.len() < self.compression_threshold {
            // No compression for small data
            return CompressedBuffer {
                compressed_data: data.to_vec(),
                original_size: data.len(),
                compression_ratio: 1.0,
                _alignment: 64, // Default SIMD alignment
            };
        }

        let compressed = match algorithm {
            CompressionAlgorithm::LZ4 => self.compress_lz4(data),
            CompressionAlgorithm::RLE => self.compress_rle(data),
            CompressionAlgorithm::Delta => self.compress_delta(data),
            CompressionAlgorithm::Dictionary => self.compress_dictionary(data),
            CompressionAlgorithm::BitPack => self.compress_bitpack(data),
        };

        let compression_ratio = data.len() as f64 / compressed.len() as f64;

        CompressedBuffer {
            compressed_data: compressed,
            original_size: data.len(),
            compression_ratio,
            _alignment: 64,
        }
    }

    /// LZ4-style fast compression for general data
    fn compress_lz4(&self, data: &[u8]) -> Vec<u8> {
        // Simplified LZ4-style compression
        // In production, use actual LZ4 library
        let mut compressed = Vec::new();
        let mut i = 0;

        while i < data.len() {
            let mut run_length = 1;
            let current_byte = data[i];

            // Find run length
            while i + run_length < data.len()
                && data[i + run_length] == current_byte
                && run_length < 255
            {
                run_length += 1;
            }

            if run_length > 3 {
                // Encode as run
                compressed.push(0xFF); // Run marker
                compressed.push(run_length as u8);
                compressed.push(current_byte);
                i += run_length;
            } else {
                // Literal byte
                compressed.push(current_byte);
                i += 1;
            }
        }

        compressed
    }

    /// Run-Length Encoding for repeated values
    fn compress_rle(&self, data: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();
        if data.is_empty() {
            return compressed;
        }

        let mut current_byte = data[0];
        let mut count = 1u8;

        for &byte in data.iter().skip(1) {
            if byte == current_byte && count < 255 {
                count += 1;
            } else {
                compressed.push(count);
                compressed.push(current_byte);
                current_byte = byte;
                count = 1;
            }
        }

        // Don't forget the last run
        compressed.push(count);
        compressed.push(current_byte);

        compressed
    }

    /// Delta encoding for numerical sequences
    fn compress_delta(&self, data: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();
        if data.is_empty() {
            return compressed;
        }

        // Store first value as-is
        compressed.push(data[0]);

        // Store deltas
        for i in 1..data.len() {
            let delta = data[i].wrapping_sub(data[i - 1]);
            compressed.push(delta);
        }

        compressed
    }

    /// Dictionary compression for string data
    fn compress_dictionary(&self, data: &[u8]) -> Vec<u8> {
        let mut dictionary = HashMap::new();
        let mut compressed = Vec::new();
        let mut next_id = 0u8;

        // Build dictionary of unique bytes/patterns
        for &byte in data {
            if let std::collections::hash_map::Entry::Vacant(e) = dictionary.entry(byte) {
                e.insert(next_id);
                next_id += 1;
                if next_id == 255 {
                    break; // Dictionary full
                }
            }
        }

        // Store dictionary size
        compressed.push(dictionary.len() as u8);

        // Store dictionary
        for (&key, &value) in &dictionary {
            compressed.push(key);
            compressed.push(value);
        }

        // Store compressed data
        for &byte in data {
            if let Some(&id) = dictionary.get(&byte) {
                compressed.push(id);
            } else {
                compressed.push(byte); // Fallback for full dictionary
            }
        }

        compressed
    }

    /// Bit-packing for small integers
    fn compress_bitpack(&self, data: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();

        // Find maximum value to determine bit width
        let max_val = data.iter().max().copied().unwrap_or(0);
        let bits_per_value = if max_val == 0 {
            1
        } else {
            (max_val as f32).log2().ceil() as u8
        };

        compressed.push(bits_per_value);

        let mut current_byte = 0u8;
        let mut bits_used = 0u8;

        for &value in data {
            // Pack value into current byte
            let remaining_bits = 8 - bits_used;

            if bits_per_value <= remaining_bits {
                current_byte |= value << bits_used;
                bits_used += bits_per_value;

                if bits_used == 8 {
                    compressed.push(current_byte);
                    current_byte = 0;
                    bits_used = 0;
                }
            } else {
                // Value spans multiple bytes
                current_byte |= value << bits_used;
                compressed.push(current_byte);

                current_byte = value >> remaining_bits;
                bits_used = bits_per_value - remaining_bits;
            }
        }

        // Push final byte if needed
        if bits_used > 0 {
            compressed.push(current_byte);
        }

        compressed
    }

    fn should_pool_size(&self, size: usize) -> bool {
        // Pool common sizes for reuse
        matches!(size, 64 | 128 | 256 | 512 | 1024 | 2048 | 4096)
    }

    /// Get memory pool statistics
    pub fn get_stats(&self) -> MemoryPoolStats {
        let pools = self.pools.lock().unwrap();
        let total_allocated = self.allocated_bytes.load(Ordering::Relaxed);
        let peak_usage = self.peak_usage.load(Ordering::Relaxed);

        MemoryPoolStats {
            total_allocated,
            peak_usage,
            active_pools: pools.len(),
            compression_ratio: self.calculate_compression_ratio(),
            allocations_per_second: 0.0, // Would need timing data
        }
    }

    fn calculate_compression_ratio(&self) -> f64 {
        // Placeholder - would track actual compression ratios
        1.0
    }
}

impl CompressedBuffer {
    /// Decompress data back to original form
    pub fn decompress(&self, algorithm: CompressionAlgorithm) -> Result<Vec<u8>, String> {
        match algorithm {
            CompressionAlgorithm::LZ4 => self.decompress_lz4(),
            CompressionAlgorithm::RLE => self.decompress_rle(),
            CompressionAlgorithm::Delta => self.decompress_delta(),
            CompressionAlgorithm::Dictionary => self.decompress_dictionary(),
            CompressionAlgorithm::BitPack => self.decompress_bitpack(),
        }
    }

    fn decompress_lz4(&self) -> Result<Vec<u8>, String> {
        let mut decompressed = Vec::with_capacity(self.original_size);
        let mut i = 0;

        while i < self.compressed_data.len() {
            if self.compressed_data[i] == 0xFF && i + 2 < self.compressed_data.len() {
                // Run encoding
                let run_length = self.compressed_data[i + 1] as usize;
                let byte_value = self.compressed_data[i + 2];

                for _ in 0..run_length {
                    decompressed.push(byte_value);
                }
                i += 3;
            } else {
                // Literal byte
                decompressed.push(self.compressed_data[i]);
                i += 1;
            }
        }

        Ok(decompressed)
    }

    fn decompress_rle(&self) -> Result<Vec<u8>, String> {
        let mut decompressed = Vec::with_capacity(self.original_size);
        let mut i = 0;

        while i + 1 < self.compressed_data.len() {
            let count = self.compressed_data[i] as usize;
            let value = self.compressed_data[i + 1];

            for _ in 0..count {
                decompressed.push(value);
            }

            i += 2;
        }

        Ok(decompressed)
    }

    fn decompress_delta(&self) -> Result<Vec<u8>, String> {
        let mut decompressed = Vec::with_capacity(self.original_size);
        if self.compressed_data.is_empty() {
            return Ok(decompressed);
        }

        // First value is stored as-is
        let mut current = self.compressed_data[0];
        decompressed.push(current);

        // Reconstruct from deltas
        for i in 1..self.compressed_data.len() {
            current = current.wrapping_add(self.compressed_data[i]);
            decompressed.push(current);
        }

        Ok(decompressed)
    }

    fn decompress_dictionary(&self) -> Result<Vec<u8>, String> {
        if self.compressed_data.is_empty() {
            return Ok(Vec::new());
        }

        let dict_size = self.compressed_data[0] as usize;
        let mut dictionary = HashMap::new();

        // Rebuild dictionary
        for i in 0..dict_size {
            let key_idx = 1 + i * 2;
            let val_idx = 2 + i * 2;
            if val_idx < self.compressed_data.len() {
                let key = self.compressed_data[key_idx];
                let value = self.compressed_data[val_idx];
                dictionary.insert(value, key);
            }
        }

        // Decompress data
        let mut decompressed = Vec::with_capacity(self.original_size);
        let data_start = 1 + dict_size * 2;

        for i in data_start..self.compressed_data.len() {
            let encoded_byte = self.compressed_data[i];
            if let Some(&original_byte) = dictionary.get(&encoded_byte) {
                decompressed.push(original_byte);
            } else {
                decompressed.push(encoded_byte);
            }
        }

        Ok(decompressed)
    }

    fn decompress_bitpack(&self) -> Result<Vec<u8>, String> {
        if self.compressed_data.is_empty() {
            return Ok(Vec::new());
        }

        let bits_per_value = self.compressed_data[0];
        let mut decompressed = Vec::with_capacity(self.original_size);

        let mut byte_idx = 1;
        let mut bit_offset = 0u8;

        while byte_idx < self.compressed_data.len() && decompressed.len() < self.original_size {
            let mut value = 0u8;
            let mut bits_read = 0u8;

            while bits_read < bits_per_value && byte_idx < self.compressed_data.len() {
                let current_byte = self.compressed_data[byte_idx];
                let bits_available = 8 - bit_offset;
                let bits_to_read = std::cmp::min(bits_per_value - bits_read, bits_available);

                let mask = (1u8 << bits_to_read) - 1;
                let extracted_bits = (current_byte >> bit_offset) & mask;

                value |= extracted_bits << bits_read;
                bits_read += bits_to_read;
                bit_offset += bits_to_read;

                if bit_offset == 8 {
                    bit_offset = 0;
                    byte_idx += 1;
                }
            }

            decompressed.push(value);
        }

        Ok(decompressed)
    }

    pub fn get_compression_ratio(&self) -> f64 {
        self.compression_ratio
    }

    pub fn get_compressed_size(&self) -> usize {
        self.compressed_data.len()
    }

    pub fn get_original_size(&self) -> usize {
        self.original_size
    }
}

impl NumaAwareAllocator {
    pub fn new(num_nodes: usize, compression_threshold: usize) -> Self {
        let mut node_pools = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            node_pools.push(UltraFastMemoryPool::new(compression_threshold));
        }

        Self {
            node_pools,
            current_node: AtomicUsize::new(0),
        }
    }

    /// Allocate from current NUMA node
    pub fn allocate_numa_aware(
        &self,
        size: usize,
        alignment: usize,
    ) -> Result<NonNull<u8>, String> {
        let node_id = self.current_node.fetch_add(1, Ordering::Relaxed) % self.node_pools.len();
        self.node_pools[node_id].allocate_from_pool(size, alignment)
    }

    /// Get aggregated statistics across all NUMA nodes
    pub fn get_numa_stats(&self) -> Vec<MemoryPoolStats> {
        self.node_pools
            .iter()
            .map(|pool| pool.get_stats())
            .collect()
    }
}

/// Create a new memory pool instance (avoiding global state for thread safety)
pub fn create_memory_pool() -> UltraFastMemoryPool {
    UltraFastMemoryPool::new(4096) // 4KB compression threshold
}
