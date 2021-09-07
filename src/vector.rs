use std::ops::{Index, IndexMut};

use rand::{distributions::Standard, prelude::Distribution};

use crate::vector::{
    large_vector::{into_vec::IntoVec, LargeVector},
    small_vector::{into_array::IntoArray, SmallVector},
};

pub mod iterator;
pub mod large_vector;
pub mod math;
pub mod math_ops;
pub mod small_vector;

/// A mathematical vector that can either be allocated on the heap or stack.
///
/// `Vector` has a fixed length determined by a const generic parameter. You can use any type tha's
/// within the requirements of `MathVector`.
///
/// ## Types
/// A `Vector` is either a `SmallVector` or a `LargeVector`. It will by default create a
/// `SmallVector` as you'll most likely never have to use `LargeVector`. However, in matrices
/// larger than 100x100 a heap allocated vector is preferably used.
///
/// ### SmallVector
/// Is allocated on the stack using the built in array data type.
/// As such it will be significatly faster than `LargeVector` while being limited by the size of
/// the stack.
///
/// ### LargeVector
/// Is allocated on the heap using [`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html).
/// Because it's heap allocated it will be slower than `SmallVector`
/// without a size limit.
/// This vector will behave just as `SmallVector` and will thus not have dynamic resizing.
///
/// ## Construction
///
/// You can either create one from another data type that implements `IntoArray` or
/// create one using random numbers.
/// ```rust
/// # use sickmath::*;
/// let vector: Vector<u8, 3> = Vector::new([1, 2, 3]);
/// let random_vector: Vector<u8, 3> = Vector::new_random();
/// ```
///
/// You will sometimes need a larger but slower vector allocated on the heap.
/// Any data type that implement IntoVec will work to create a new vector.
/// You can also create one with random numbers.
/// ```rust
/// # use sickmath::*;
/// let large_vector: Vector<u8, 3> = Vector::new_large([1, 2, 3]);
/// let large_random_vector: Vector<u8, 3> = Vector::new_large_random();
/// ```
///
/// `Vector` also supports default which will create a new `SmallVector`
/// using the default value of the chosen data type.
/// ```rust
/// # use sickmath::*;
/// let default_vector: Vector<u8, 3> = Vector::default();
/// ```
///
/// ## Example
/// ```rust
/// # use sickmath::*;
/// let vector: Vector<f64, 3> = Vector::new([0.2, 0.5, 0.8]);
/// let random_large_vector: Vector<f64, 3> = Vector::new_large_random();
///
/// println!("{:?}", vector * random_large_vector);
/// ```
#[derive(Debug, Clone)]
pub enum Vector<T, const N: usize> {
    Small(SmallVector<T, N>),
    Large(LargeVector<T, N>),
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
        Self::Small(SmallVector::new(data))
    }

    /// Create a new random `SmallVectr`
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
            Self::Small(SmallVector::new_random())
        } else {
            Self::Large(LargeVector::new_random())
        }
    }

    /// Create a new `LargeVector` from any data type that implements `IntoVec`
    /// ```rust
    /// # use sickmath::*;
    /// let large_vector: Vector<u16, 3> = Vector::new_large(vec![1, 2, 3]);
    /// ```
    pub fn new_large(data: impl IntoVec<T, N>) -> Self {
        Self::Large(LargeVector::new(data))
    }

    /// Create a new random `LargeVector`
    /// ```rust
    /// # use sickmath::*;
    /// let large_random_vector: Vector<f64, 3> = Vector::new_large_random();
    /// ```
    pub fn new_large_random() -> Self
    where
        Standard: Distribution<T>,
    {
        Self::Large(LargeVector::new_random())
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self::Small(SmallVector::default())
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Self::Small(small_vector) => &small_vector[idx],
            Self::Large(large_vector) => &large_vector[idx],
        }
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match self {
            Self::Small(small_vector) => &mut small_vector[idx],
            Self::Large(large_vector) => &mut large_vector[idx],
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
