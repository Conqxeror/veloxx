//! Arrow integration for Veloxx
//!
//! This module provides integration with Apache Arrow for improved performance
//! and memory efficiency.

#[cfg(feature = "arrow")]
pub mod aggregate;
#[cfg(feature = "arrow")]
pub mod array;
#[cfg(feature = "arrow")]
pub mod dataframe;
#[cfg(feature = "arrow")]
pub mod filter;
#[cfg(feature = "arrow")]
pub mod ops;
#[cfg(feature = "arrow")]
pub mod series;
#[cfg(all(feature = "arrow", feature = "simd"))]
pub mod simd;
#[cfg(all(feature = "arrow", feature = "simd"))]
pub mod simd_enhanced;
#[cfg(feature = "arrow")]
pub mod string_ops;

// Re-export the main types for convenience
#[cfg(feature = "arrow")]
pub use aggregate::*;
#[cfg(feature = "arrow")]
pub use array::*;
#[cfg(feature = "arrow")]
pub use dataframe::*;
#[cfg(feature = "arrow")]
pub use filter::*;
#[cfg(feature = "arrow")]
pub use ops::*;
#[cfg(feature = "arrow")]
pub use series::*;
#[cfg(all(feature = "arrow", feature = "simd"))]
pub use simd::*;
#[cfg(all(feature = "arrow", feature = "simd"))]
pub use simd_enhanced::*;
#[cfg(feature = "arrow")]
pub use string_ops::*;
