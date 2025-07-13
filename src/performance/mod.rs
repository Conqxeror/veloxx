//! Performance optimization module for Velox
//!
//! This module provides high-performance implementations of common data operations
//! using SIMD instructions, parallel processing, and memory-efficient algorithms.

pub mod memory;
pub mod parallel;
pub mod series_ext;
pub mod simd;

pub use memory::*;
pub use parallel::*;
pub use series_ext::*;
pub use simd::*;
