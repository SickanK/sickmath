use std::ops::Index;

use crate::matrix::Matrix;

/// A trait for mathematical vectors.
///
///  Implementing `MathVector` and `Index` on a data type makes it possible to perform mathematical
///  operations with other data types that implements `MathVector` such as `Vector`.
///
///  ## Mutable methods
///  Most methods will have a mutable alternative which will mutate the right hand side argument instead of creating
///  a new one.
///  This vastly increases the speed of the operation and should be used in most cases.
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

    /// Cross product. Will panic if vector has a length other than 3
    fn cross(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self;

    /// Mutable cross product. Will panic if vector has a length other than 3
    fn cross_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>);

    /// Tensor product. Will return a `Matrix` instead of `Self`
    fn tensor_prod<const M: usize>(
        &self,
        rhs: impl MathVector<T, N> + Index<usize, Output = T>,
    ) -> Matrix<T, M, N>;

    /// Magnitude of the vector
    fn magnitude(&self) -> usize;

    /// Sum of all items
    fn sum(&self) -> isize;
}
