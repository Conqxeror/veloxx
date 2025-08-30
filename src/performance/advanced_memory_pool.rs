//! Advanced memory pool implementation with better performance characteristics

use crate::VeloxxError;
use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A more advanced memory pool with better performance characteristics
pub struct AdvancedMemoryPool {
    /// Pools for different sizes of memory blocks
    pools: Arc<Mutex<HashMap<usize, Vec<MemoryBlock>>>>,
    /// Maximum size of memory blocks to pool
    max_pool_size: usize,
    /// Statistics for monitoring pool usage
    stats: Arc<Mutex<PoolStats>>,
}

/// Statistics for monitoring pool usage
#[derive(Debug, Default, Copy, Clone)]
pub struct PoolStats {
    /// Total allocations served from pool
    pub allocations_from_pool: usize,
    /// Total allocations that had to be made directly
    pub direct_allocations: usize,
    /// Total deallocations returned to pool
    pub deallocations_to_pool: usize,
    /// Total deallocations that were too large for pool
    pub direct_deallocations: usize,
}

/// A memory block with its layout information
struct MemoryBlock {
    /// Pointer to the allocated memory
    ptr: *mut u8,
    /// Layout of the allocated memory
    layout: Layout,
}

unsafe impl Send for MemoryBlock {}
unsafe impl Sync for MemoryBlock {}

impl AdvancedMemoryPool {
    /// Create a new advanced memory pool
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
            max_pool_size,
            stats: Arc::new(Mutex::new(PoolStats::default())),
        }
    }

    /// Allocate a memory block of the specified size
    pub fn allocate(&self, size: usize) -> Result<(*mut u8, Layout), VeloxxError> {
        if size == 0 {
            return Err(VeloxxError::MemoryError(
                "Cannot allocate zero bytes".to_string(),
            ));
        }

        if size > self.max_pool_size {
            // For large allocations, allocate directly without pooling
            let layout = Layout::from_size_align(size, 8)
                .map_err(|e| VeloxxError::MemoryError(format!("Invalid layout: {}", e)))?;

            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                return Err(VeloxxError::MemoryError(
                    "Failed to allocate memory".to_string(),
                ));
            }

            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.direct_allocations += 1;
            }

            return Ok((ptr, layout));
        }

        let mut pools = self.pools.lock().map_err(|_| {
            VeloxxError::MemoryError("Failed to acquire memory pool lock".to_string())
        })?;

        // Try to get a block from the pool
        if let Some(blocks) = pools.get_mut(&size) {
            if let Some(block) = blocks.pop() {
                // Update statistics
                if let Ok(mut stats) = self.stats.lock() {
                    stats.allocations_from_pool += 1;
                }

                return Ok((block.ptr, block.layout));
            }
        }

        // If no block is available, allocate a new one
        let layout = Layout::from_size_align(size, 8)
            .map_err(|e| VeloxxError::MemoryError(format!("Invalid layout: {}", e)))?;

        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(VeloxxError::MemoryError(
                "Failed to allocate memory".to_string(),
            ));
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.direct_allocations += 1;
        }

        Ok((ptr, layout))
    }

    /// Deallocate a memory block, returning it to the pool if appropriate
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `ptr` was allocated by this allocator with the given `layout`
    /// - `ptr` is currently allocated (has not been deallocated already)
    /// - The memory referenced by `ptr` is not accessed after this call
    pub unsafe fn deallocate(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();

        if size == 0 || ptr.is_null() {
            return;
        }

        if size > self.max_pool_size {
            // Large blocks are not pooled, so just deallocate them directly
            unsafe { dealloc(ptr, layout) };

            // Update statistics
            if let Ok(mut stats) = self.stats.lock() {
                stats.direct_deallocations += 1;
            }

            return;
        }

        let block = MemoryBlock { ptr, layout };

        let mut pools = match self.pools.lock() {
            Ok(pools) => pools,
            Err(_) => {
                // If we can't acquire the lock, just deallocate the block directly
                unsafe { dealloc(ptr, layout) };
                return;
            }
        };

        // Add the block to the pool
        pools.entry(size).or_insert_with(Vec::new).push(block);

        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.deallocations_to_pool += 1;
        }
    }

    /// Get the current memory usage of the pool
    pub fn memory_usage(&self) -> usize {
        let pools = match self.pools.lock() {
            Ok(pools) => pools,
            Err(_) => return 0,
        };

        pools
            .values()
            .map(|blocks| blocks.len() * blocks.first().map_or(0, |b| b.layout.size()))
            .sum()
    }

    /// Get statistics about pool usage
    pub fn stats(&self) -> PoolStats {
        self.stats.lock().map(|s| *s).unwrap_or_default()
    }

    /// Clear all pooled memory blocks
    pub fn clear(&self) {
        let mut pools = match self.pools.lock() {
            Ok(pools) => pools,
            Err(_) => return,
        };

        // Deallocate all pooled blocks
        for blocks in pools.values_mut() {
            for block in blocks.drain(..) {
                unsafe { dealloc(block.ptr, block.layout) };
            }
        }
    }
}

impl Default for AdvancedMemoryPool {
    fn default() -> Self {
        Self::new(1024 * 1024) // 1MB max pool size by default
    }
}

impl Drop for AdvancedMemoryPool {
    fn drop(&mut self) {
        self.clear();
    }
}

// Implement clone manually to share the same pool
impl Clone for AdvancedMemoryPool {
    fn clone(&self) -> Self {
        Self {
            pools: Arc::clone(&self.pools),
            max_pool_size: self.max_pool_size,
            stats: Arc::clone(&self.stats),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_memory_pool_allocation() {
        let pool = AdvancedMemoryPool::new(1024);

        // Allocate a block
        let (ptr, layout) = pool.allocate(100).unwrap();
        assert!(!ptr.is_null());
        assert_eq!(layout.size(), 100);

        // Deallocate the block
        unsafe {
            pool.deallocate(ptr, layout);
        }

        // Allocate another block of the same size
        let (ptr2, layout2) = pool.allocate(100).unwrap();
        assert!(!ptr2.is_null());
        assert_eq!(layout2.size(), 100);

        // This should come from the pool
        unsafe {
            pool.deallocate(ptr2, layout2);
        }
    }

    #[test]
    fn test_large_allocation() {
        let pool = AdvancedMemoryPool::new(1024);

        // Allocate a block larger than the max pool size
        let (ptr, layout) = pool.allocate(2048).unwrap();
        assert!(!ptr.is_null());
        assert_eq!(layout.size(), 2048);

        // Deallocate the block (should just deallocate it directly)
        unsafe {
            pool.deallocate(ptr, layout);
        }
    }

    #[test]
    fn test_memory_usage() {
        let pool = AdvancedMemoryPool::new(1024);
        assert_eq!(pool.memory_usage(), 0);

        let (ptr, layout) = pool.allocate(100).unwrap();
        assert_eq!(pool.memory_usage(), 0); // No blocks in pool yet

        unsafe {
            pool.deallocate(ptr, layout);
        }
        assert_eq!(pool.memory_usage(), 100); // One block of 100 bytes in pool
    }

    #[test]
    fn test_stats() {
        let pool = AdvancedMemoryPool::new(1024);

        // Direct allocation (too large for pool)
        let (ptr1, layout1) = pool.allocate(2048).unwrap();
        unsafe {
            pool.deallocate(ptr1, layout1);
        }

        // Pool allocation - first one is direct, second one is from pool
        let (ptr2, layout2) = pool.allocate(100).unwrap(); // This is a direct allocation
        unsafe {
            pool.deallocate(ptr2, layout2);
        } // This goes to the pool
        let (ptr3, layout3) = pool.allocate(100).unwrap(); // This should come from the pool
        unsafe {
            pool.deallocate(ptr3, layout3);
        } // This goes to the pool

        let stats = pool.stats();
        assert_eq!(stats.direct_allocations, 2); // One large direct allocation, one small direct allocation
        assert_eq!(stats.direct_deallocations, 1); // One large direct deallocation
        assert_eq!(stats.allocations_from_pool, 1); // One allocation from pool (ptr3)
        assert_eq!(stats.deallocations_to_pool, 2); // Two deallocations to pool (ptr2 and ptr3)
    }
}
