use std::ops::Index;

use crate::matrix::Matrix;

/// A trait for types that are a mathematical vector.
///
///  Implementing `MathVector` and `Index` on a data type makes it possible to perform mathematical
///  operations with other data types that implements `MathVector`.
///
///  ## Mutable methods
///  Most methods will have a mutable alternative which will mutate the right hand side vector instead of creating
///  a new one. This makes the operation faster and should be used in most cases.
pub trait MathVector<T, const N: usize> {
    /// Scalar multiplication
    fn scalar(&self, scalar: isize) -> Self;

    /// Mutable scalar multiplication
    fn scalar_mut(&mut self, scalar: isize);

    /// Dot product. Will return an `isize`
    fn dot(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> isize;

    /// Vector addition
    fn add_vector(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self;

    /// Mutable vector addition
    fn add_vector_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>);

    /// Vector subtraction
    fn sub_vector(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self;

    /// Mutable vector subtraction
    fn sub_vector_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>);

    /// Entrywise vector multiplication
    fn entrywise(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self;

    /// Mutable entrywise vector multiplication
    fn entrywise_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>);

    /// Cross product.
    /// * Note: Will panic of vector hasn't a length of 3
    fn cross(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self;

    /// Mutable cross product.
    /// * Note: Will panic of vector hasn't a length of 3
    fn cross_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>);

    /// Tensor product. Will return a `Matrix` instead of itself
    fn tensor_prod<const M: usize>(
        &self,
        rhs: impl MathVector<T, N> + Index<usize, Output = T>,
    ) -> Matrix<T, M, N>;

    /// Calculate the magnitude of the vector
    fn magnitude(&self) -> usize;

    /// Calculate the sum of all items
    fn sum(&self) -> isize;
}
