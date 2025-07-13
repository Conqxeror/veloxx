//! Performance optimization module for Velox
//! 
//! This module provides high-performance implementations of common data operations
//! using SIMD instructions, parallel processing, and memory-efficient algorithms.

pub mod simd;
pub mod parallel;
pub mod memory;
pub mod series_ext;

pub use simd::*;
pub use parallel::*;
pub use memory::*;
pub use series_ext::*;