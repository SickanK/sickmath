use std::ops::{Index, IndexMut};

use rand::{distributions::Standard, prelude::Distribution};

use crate::vector::{
    heap_vector::{into_vec::IntoVec, HeapVector},
    inline_vector::{into_array::IntoArray, InlineVector},
};

pub mod heap_vector;
pub mod inline_vector;
pub mod iterator;
pub mod math;
pub mod math_ops;

/// A mathematical vector that can either be allocated on the heap or stack.
///
/// A `Vector` is either an InlineVector or a HeapVector.
/// The interface for the type as a whole is a bunch of methods that just match on
/// the enum variant and then call the same method on the inner vec.
///
/// ## Construction
///
/// To create a fast inline vector you can do that from a data type or by random numbers.
/// Any data type that implement IntoArray will work to create a new vector.
/// ```rust
/// # use sickmath::*;
/// let vector: Vector<u8, 3> = Vector::new([1, 2, 3]);
/// let random_vector: Vector<u8, 3> = Vector::new_random();
/// ```
///
/// You will sometimes need a larger but slower vector allocated on the heap.
/// Any data type that implement IntoVec will work to create a new vector.
/// ```rust
/// # use sickmath::*;
/// let large_vector: Vector<u8, 3> = Vector::heap([1, 2, 3]);
/// let large_random_vector: Vector<u8, 3> = Vector::heap_random();
/// ```
///
/// Vector also supports default which will create a new inline vector using the default of the
/// chosen data type within.
/// ```rust
/// # use sickmath::*;
/// let default_vector: Vector<u8, 3> = Vector::default();
/// ```
///
/// ## Example
/// ```rust
/// # use sickmath::*;
/// let vector: Vector<u8, 3> = Vector::new([1, 2, 3]);
/// let random_large_vector: Vector<u8, 3> = Vector::heap_random();
///
/// println!("{:?}", vector * random_large_vector);
/// ```
#[derive(Debug, Clone)]
pub enum Vector<T, const N: usize> {
    Inline(InlineVector<T, N>),
    Heap(HeapVector<T, N>),
}

impl<T, const N: usize> Vector<T, N>
where
    T: Default,
{
    /// Creates a new inline vector from any data type that implements `IntoArray`
    /// ```rust
    /// # use sickmath::*;
    /// let vector: Vector<u8, 3> = Vector::new([1, 2, 3]);
    /// ```
    pub fn new(data: impl IntoArray<T, N>) -> Self {
        Self::Inline(InlineVector::new(data))
    }

    /// Creates a new random inline vector
    /// ```rust
    /// # use sickmath::*;
    /// let random_vector: Vector<u8, 3> = Vector::new_random();
    /// ```
    pub fn new_random() -> Self
    where
        T: Copy,
        Standard: Distribution<T>,
    {
        if N < 5001 {
            Self::Inline(InlineVector::new_random())
        } else {
            Self::Heap(HeapVector::new_random())
        }
    }

    /// Creates a new inline vector from any data type that implements `IntoVec`
    /// ```rust
    /// # use sickmath::*;
    /// let large_vector: Vector<u16, 3> = Vector::heap(vec![1, 2, 3]);
    /// ```
    pub fn heap(data: impl IntoVec<T, N>) -> Self {
        Self::Heap(HeapVector::new(data))
    }

    /// Creates a new random heap vector
    /// ```rust
    /// # use sickmath::*;
    /// let large_random_vector: Vector<f64, 3> = Vector::heap_random();
    /// ```
    pub fn heap_random() -> Self
    where
        Standard: Distribution<T>,
    {
        Self::Heap(HeapVector::new_random())
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self::Inline(InlineVector::default())
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Self::Inline(inline_vector) => &inline_vector[idx],
            Self::Heap(heap_vector) => &heap_vector[idx],
        }
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match self {
            Self::Inline(inline_vector) => &mut inline_vector[idx],
            Self::Heap(heap_vector) => &mut heap_vector[idx],
        }
    }
}

impl<T, const N: usize> PartialEq for Vector<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for idx in 0..N {
            if self[idx] != other[idx] {
                return false;
            }
        }

        true
    }
}
