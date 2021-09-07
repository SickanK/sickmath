use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::{math_vector::MathVector, matrix::Matrix};

use super::Vector;

impl<T, const N: usize> MathVector<T, N> for Vector<T, N>
where
    T: Default
        + Copy
        + FromPrimitive
        + ToPrimitive
        + Mul<Output = T>
        + MulAssign
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Debug,
{
    fn scalar(&self, scalar: isize) -> Self {
        match self {
            Self::Small(small_vector) => Vector::Small(small_vector.scalar(scalar)),
            Self::Large(large_vector) => Vector::Large(large_vector.scalar(scalar)),
        }
    }

    fn scalar_mut(&mut self, scalar: isize) {
        match self {
            Self::Small(small_vector) => small_vector.scalar_mut(scalar),
            Self::Large(large_vector) => large_vector.scalar_mut(scalar),
        };
    }

    fn dot(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> isize {
        match self {
            Self::Small(small_vector) => small_vector.dot(rhs),
            Self::Large(large_vector) => large_vector.dot(rhs),
        }
    }

    fn add_vector(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        match self {
            Self::Small(small_vector) => Vector::Small(small_vector.add_vector(rhs)),
            Self::Large(large_vector) => Vector::Large(large_vector.add_vector(rhs)),
        }
    }

    fn add_vector_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        match self {
            Self::Small(small_vector) => small_vector.add_vector_mut(rhs),
            Self::Large(large_vector) => large_vector.add_vector_mut(rhs),
        }
    }

    fn sub_vector(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        match self {
            Self::Small(small_vector) => Vector::Small(small_vector.sub_vector(rhs)),
            Self::Large(large_vector) => Vector::Large(large_vector.sub_vector(rhs)),
        }
    }

    fn sub_vector_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        match self {
            Self::Small(small_vector) => small_vector.sub_vector_mut(rhs),
            Self::Large(large_vector) => large_vector.sub_vector_mut(rhs),
        }
    }

    fn entrywise(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        match self {
            Self::Small(small_vector) => Vector::Small(small_vector.entrywise(rhs)),
            Self::Large(large_vector) => Vector::Large(large_vector.entrywise(rhs)),
        }
    }

    fn entrywise_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        match self {
            Self::Small(small_vector) => small_vector.entrywise_mut(rhs),
            Self::Large(large_vector) => large_vector.entrywise_mut(rhs),
        }
    }

    fn cross(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self {
        match self {
            Self::Small(small_vector) => Vector::Small(small_vector.cross(rhs)),
            Self::Large(large_vector) => Vector::Large(large_vector.cross(rhs)),
        }
    }

    fn cross_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        match self {
            Self::Small(small_vector) => small_vector.cross_mut(rhs),
            Self::Large(large_vector) => large_vector.cross_mut(rhs),
        }
    }

    fn tensor_prod<const M: usize>(
        &self,
        rhs: impl MathVector<T, N> + Index<usize, Output = T>,
    ) -> Matrix<T, M, N> {
        match self {
            Self::Small(small_vector) => small_vector.tensor_prod(rhs),
            Self::Large(large_vector) => large_vector.tensor_prod(rhs),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Small(small_vector) => small_vector.magnitude(),
            Self::Large(large_vector) => large_vector.magnitude(),
        }
    }

    fn sum(&self) -> isize {
        match self {
            Self::Small(small_vector) => small_vector.sum(),
            Self::Large(large_vector) => large_vector.sum(),
        }
    }
}
