pub mod simd_string;
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
pub use simd_string::*;
/// Performance optimization utilities
pub mod advanced_memory_pool;
pub mod advanced_parallel;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
pub mod arrow_simd;
pub mod cache_optimization;
pub mod expression_fusion;
pub mod fast_filter;
pub mod fast_groupby;
pub mod global_aggregate;
pub mod memory;
pub mod memory_compression;
pub mod memory_pool;
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
pub mod optimized_simd;
pub mod parallel;
pub mod parallel_group_by;
pub mod series_ext;
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
pub mod simd;
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
pub mod simd_std;
pub mod specialized_structures;
pub mod ultra_fast_filter;
pub mod ultra_fast_groupby;
pub mod ultra_fast_join;
pub mod vectorized_filter;
pub mod vectorized_groupby;
// Temporarily disabled due to threading issues
// pub mod parallel_framework;
// Curated public re-exports (avoid ambiguous glob re-exports)
pub use advanced_memory_pool::*;
pub use advanced_parallel::*;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
pub use arrow_simd::*;
pub use cache_optimization::*;
pub use expression_fusion::*;
pub use fast_filter::*;
pub use fast_groupby::*;
pub use memory::*;
pub use memory_compression::{
    CompressedBuffer, CompressionAlgorithm, MemoryPoolStats as CompressionMemoryPoolStats,
    UltraFastMemoryPool,
};
pub use memory_pool::{global_memory_pool, AlignedBuffer, MemoryPool};
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
pub use optimized_simd::OptimizedSimdOps;
pub use parallel::*;
pub use parallel_group_by::*;
pub use series_ext::*;
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
pub use simd::optimized::simd_sum_optimized;
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
pub use simd::SimdOps;
#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
pub use simd_std::StdSimdOps;
pub use specialized_structures::*;
pub use ultra_fast_filter::*;
pub use ultra_fast_groupby::*;
pub use ultra_fast_join::*;
pub use vectorized_filter::*;
// Temporarily disabled due to threading issues
// pub use parallel_framework::*;
// pub use expression_fusion::*; // TODO: Implement later
