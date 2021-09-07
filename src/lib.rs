//! Math crate targeted at machine learning.
//!
//! Provides a fast and scalable Vector and Matrix implementation.

/// Implement your own Vector type
mod math_vector;
pub use math_vector::MathVector;
/// Multiple vectors wrapped in an array
mod matrix;
pub use matrix::Matrix;
/// Supports both `SmallVector` and `LargeVector`
mod vector;
pub use vector::Vector;
