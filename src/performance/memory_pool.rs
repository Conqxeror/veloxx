//! Memory pool implementation for efficient memory management with SIMD optimization

use crate::VeloxxError;

use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    collections::HashMap,
    ptr::NonNull,
    sync::{Arc, Mutex},
};

/// Cache line size for modern x86_64 processors
pub const CACHE_LINE_SIZE: usize = 64;

/// AVX512 register alignment (64 bytes)
pub const SIMD_ALIGNMENT: usize = 64;

/// Advanced memory pool for SIMD-optimized operations with aligned allocation
pub struct MemoryPool {
    /// Pools for different sizes of memory blocks (legacy)
    pools: Arc<Mutex<HashMap<usize, Vec<Vec<u8>>>>>,
    /// Aligned memory pools for SIMD operations
    // SAFETY & lint rationale:
    // - This Arc<Mutex<...>> holds raw pointers managed only inside this module.
    // - We implement Send/Sync for MemoryPool and guard access via Mutex.
    // - Clippy warns about non-Send/Sync inner types; in our design, we never share
    //   these pointers across threads without the Mutex, so this is acceptable.
    #[allow(clippy::arc_with_non_send_sync)]
    aligned_pools: Arc<Mutex<HashMap<usize, Vec<*mut u8>>>>,
    /// Maximum size of memory blocks to pool
    max_pool_size: usize,
    /// Total allocated memory tracking
    total_allocated: std::sync::atomic::AtomicUsize,
    /// Allocation count tracking
    allocation_count: std::sync::atomic::AtomicUsize,
}

// Safety: MemoryPool manages memory pointers safely through proper synchronization
unsafe impl Send for MemoryPool {}
unsafe impl Sync for MemoryPool {}

impl MemoryPool {
    /// Create a new memory pool
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
            aligned_pools: Arc::new(Mutex::new(HashMap::new())),
            max_pool_size,
            total_allocated: std::sync::atomic::AtomicUsize::new(0),
            allocation_count: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Allocate a memory block of the specified size
    pub fn allocate(&self, size: usize) -> Result<Vec<u8>, VeloxxError> {
        if size > self.max_pool_size {
            // For large allocations, allocate directly without pooling
            return Ok(vec![0; size]);
        }

        let mut pools = self.pools.lock().map_err(|_| {
            VeloxxError::MemoryError("Failed to acquire memory pool lock".to_string())
        })?;

        // Try to get a block from the pool
        if let Some(blocks) = pools.get_mut(&size) {
            if let Some(block) = blocks.pop() {
                return Ok(block);
            }
        }

        // If no block is available, allocate a new one
        Ok(vec![0; size])
    }

    /// Deallocate a memory block, returning it to the pool if appropriate
    pub fn deallocate(&self, block: Vec<u8>) {
        let size = block.len();

        if size > self.max_pool_size {
            // Large blocks are not pooled, so just drop them
            return;
        }

        let mut pools = match self.pools.lock() {
            Ok(pools) => pools,
            Err(_) => return, // If we can't acquire the lock, just drop the block
        };

        // Add the block to the pool
        pools.entry(size).or_insert_with(Vec::new).push(block);
    }

    /// Allocate aligned memory for SIMD operations
    pub fn allocate_aligned<T>(&self, count: usize) -> Result<NonNull<T>, VeloxxError> {
        let size = count * std::mem::size_of::<T>();
        let layout = Layout::from_size_align(size, SIMD_ALIGNMENT)
            .map_err(|e| VeloxxError::MemoryError(format!("Invalid layout: {}", e)))?;

        // Try to reuse from aligned pool first
        {
            let mut pools = self.aligned_pools.lock().map_err(|_| {
                VeloxxError::MemoryError("Failed to acquire aligned pool lock".to_string())
            })?;
            if let Some(pool) = pools.get_mut(&size) {
                if let Some(ptr) = pool.pop() {
                    // Safety: We maintain type safety through the generic parameter
                    return Ok(unsafe { NonNull::new_unchecked(ptr) }.cast::<T>());
                }
            }
        }

        // Allocate new aligned memory
        let ptr = unsafe { alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(VeloxxError::MemoryError(
                "Failed to allocate aligned memory".to_string(),
            ));
        }

        self.total_allocated
            .fetch_add(size, std::sync::atomic::Ordering::Relaxed);
        self.allocation_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        // Safety: alloc_zeroed returns a valid pointer or null
        Ok(unsafe { NonNull::new_unchecked(ptr) }.cast::<T>())
    }

    /// Deallocate aligned memory back to the pool for reuse
    pub fn deallocate_aligned<T>(&self, ptr: NonNull<T>, count: usize) -> Result<(), VeloxxError> {
        let size = count * std::mem::size_of::<T>();

        // Add back to pool for reuse
        {
            let mut pools = self.aligned_pools.lock().map_err(|_| {
                VeloxxError::MemoryError("Failed to acquire aligned pool lock".to_string())
            })?;
            let pool = pools.entry(size).or_insert_with(Vec::new);

            // Limit pool size to prevent unbounded growth
            if pool.len() < 100 {
                pool.push(ptr.as_ptr() as *mut u8);
                return Ok(());
            }
        }

        // If pool is full, actually deallocate
        let layout = Layout::from_size_align(size, SIMD_ALIGNMENT).map_err(|e| {
            VeloxxError::MemoryError(format!("Invalid layout for deallocation: {}", e))
        })?;
        unsafe {
            dealloc(ptr.as_ptr() as *mut u8, layout);
        }

        self.total_allocated
            .fetch_sub(size, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    /// Get memory usage statistics
    pub fn stats(&self) -> MemoryPoolStats {
        let aligned_pool_count = self
            .aligned_pools
            .lock()
            .map(|pools| pools.len())
            .unwrap_or(0);

        MemoryPoolStats {
            total_allocated: self
                .total_allocated
                .load(std::sync::atomic::Ordering::Relaxed),
            allocation_count: self
                .allocation_count
                .load(std::sync::atomic::Ordering::Relaxed),
            pool_count: aligned_pool_count,
        }
    }

    /// Get the current memory usage of the pool (legacy method)
    pub fn memory_usage(&self) -> usize {
        let pools = match self.pools.lock() {
            Ok(pools) => pools,
            Err(_) => return 0,
        };

        pools
            .values()
            .map(|blocks| blocks.len() * blocks.first().map_or(0, |b| b.len()))
            .sum()
    }
}

/// Memory pool statistics
#[derive(Debug, Clone)]
pub struct MemoryPoolStats {
    pub total_allocated: usize,
    pub allocation_count: usize,
    pub pool_count: usize,
}

/// Global memory pool instance
static GLOBAL_MEMORY_POOL: std::sync::OnceLock<Arc<MemoryPool>> = std::sync::OnceLock::new();

/// Get the global memory pool
pub fn global_memory_pool() -> &'static Arc<MemoryPool> {
    GLOBAL_MEMORY_POOL.get_or_init(|| Arc::new(MemoryPool::default()))
}

/// RAII wrapper for aligned memory allocation
pub struct AlignedBuffer<T> {
    ptr: NonNull<T>,
    len: usize,
    pool: Arc<MemoryPool>,
}

impl<T> AlignedBuffer<T> {
    /// Create a new aligned buffer
    pub fn new(len: usize) -> Result<Self, VeloxxError> {
        let pool = global_memory_pool().clone();
        let ptr = pool.allocate_aligned::<T>(len)?;
        Ok(Self { ptr, len, pool })
    }

    /// Create a new aligned buffer with a specific pool
    pub fn with_pool(len: usize, pool: Arc<MemoryPool>) -> Result<Self, VeloxxError> {
        let ptr = pool.allocate_aligned::<T>(len)?;
        Ok(Self { ptr, len, pool })
    }

    /// Get a raw pointer to the data
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    /// Get a mutable raw pointer to the data
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Get the length of the buffer
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get a slice of the buffer
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Get a mutable slice of the buffer
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> Drop for AlignedBuffer<T> {
    fn drop(&mut self) {
        let _ = self.pool.deallocate_aligned(self.ptr, self.len);
    }
}

unsafe impl<T: Send> Send for AlignedBuffer<T> {}
unsafe impl<T: Sync> Sync for AlignedBuffer<T> {}

/// Helper trait for NUMA-aware memory allocation
pub trait NumaAware {
    /// Allocate memory on the current NUMA node
    fn allocate_numa_local<T>(count: usize) -> Result<AlignedBuffer<T>, VeloxxError> {
        // For now, use regular aligned allocation
        // In a full implementation, this would use NUMA APIs
        AlignedBuffer::new(count)
    }

    /// Get the current NUMA node
    fn current_numa_node() -> usize {
        // Placeholder - would use actual NUMA detection
        0
    }
}

impl NumaAware for MemoryPool {}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new(1024 * 1024) // 1MB max pool size by default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool_allocation() {
        let pool = MemoryPool::new(1024);

        // Allocate a block
        let block = pool.allocate(100).unwrap();
        assert_eq!(block.len(), 100);

        // Deallocate the block
        pool.deallocate(block);

        // Allocate another block of the same size
        let block2 = pool.allocate(100).unwrap();
        assert_eq!(block2.len(), 100);
    }

    #[test]
    fn test_aligned_buffer() {
        let mut buffer = AlignedBuffer::<f64>::new(100).unwrap();
        assert_eq!(buffer.len(), 100);
        assert_eq!(buffer.as_ptr() as usize % SIMD_ALIGNMENT, 0);

        let slice = buffer.as_mut_slice();
        slice[0] = 42.0;
        assert_eq!(slice[0], 42.0);
    }

    #[test]
    fn test_aligned_memory_pool_reuse() {
        let pool = MemoryPool::new(1024 * 1024);

        // Allocate and deallocate
        let ptr1 = pool.allocate_aligned::<f64>(100).unwrap();
        pool.deallocate_aligned(ptr1, 100).unwrap();

        // Allocate again - should reuse
        let ptr2 = pool.allocate_aligned::<f64>(100).unwrap();
        // Note: reuse verification would require more complex tracking

        pool.deallocate_aligned(ptr2, 100).unwrap();
    }

    #[test]
    fn test_large_allocation() {
        let pool = MemoryPool::new(1024);

        // Allocate a block larger than the max pool size
        let block = pool.allocate(2048).unwrap();
        assert_eq!(block.len(), 2048);

        // Deallocate the block (should just drop it)
        pool.deallocate(block);
    }

    #[test]
    fn test_memory_usage() {
        let pool = MemoryPool::new(1024);
        assert_eq!(pool.memory_usage(), 0);

        let block = pool.allocate(100).unwrap();
        assert_eq!(pool.memory_usage(), 0); // No blocks in pool yet

        pool.deallocate(block);
        assert_eq!(pool.memory_usage(), 100); // One block of 100 bytes in pool
    }
}
