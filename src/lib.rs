//! # Veloxx
//!
//! A high-performance, lightweight dataframe library for Rust, focusing on efficient
//! data manipulation with minimal overhead.
// pub use distributed::global_aggregate::GlobalAggregate;
// pub use distributed::global_sort::GlobalSort;
// pub use analytics::pca::PCA;
#[cfg(not(target_arch = "wasm32"))]
pub mod distributed; // Only available for non-WASM targets
                     // pub mod analytics; // Remove or comment out unresolved module

// Core exports
pub use crate::conditions::Condition;
pub use crate::dataframe::DataFrame;
pub use crate::series::Series;
pub use crate::types::{DataType, Value};

// WASM exports
#[cfg(target_arch = "wasm32")]
pub use wasm_bindings::{WasmDataFrame, WasmGroupedDataFrame, WasmSeries};

// Core modules
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
pub mod arrow;
pub mod conditions;
#[cfg(feature = "data_quality")]
pub mod data_quality;
pub mod dataframe;
pub mod error;
pub mod io;
#[cfg(feature = "ml")]
pub mod ml;
pub mod performance;
pub mod query;
pub mod series;
pub mod types;
#[cfg(feature = "visualization")]
pub mod visualization;
#[cfg(feature = "window_functions")]
pub mod window_functions;

// Additional modules needed for compilation
pub mod expressions;
pub mod lazy;

// Optional modules that only build for native targets
#[cfg(not(target_arch = "wasm32"))]
pub mod advanced_io;
#[cfg(not(target_arch = "wasm32"))]
pub mod audit;
#[cfg(not(target_arch = "wasm32"))]
// pub mod distributed; // Remove duplicate
#[cfg(all(not(target_arch = "wasm32"), feature = "python"))]
pub mod python_bindings;

// Re-export the main error type
pub use error::VeloxxError;

// WASM Bindings are implemented in a dedicated module to keep API consistent.
// Force Rust to use the directory module (wasm_bindings/mod.rs) even if a
// legacy file `wasm_bindings.rs` exists.
#[cfg(target_arch = "wasm32")]
#[path = "wasm_bindings/mod.rs"]
pub mod wasm_bindings;
