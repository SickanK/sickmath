//! Math crate targeted at machine learning.
//!
//! Provides fast and scalable Vector and Matrix implementation which
//! can be used both on the stack as well as on the heap.

/// Implement your own Vector type
mod math_vector;
pub use math_vector::MathVector;
/// Multiple vectors wrapped in an array
mod matrix;
pub use matrix::Matrix;
/// Supports both Inline and Heap vectors
mod vector;
pub use vector::Vector;
