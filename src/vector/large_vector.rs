pub mod into_vec;
pub mod iterator;
pub mod math;
pub mod math_ops;

use self::into_vec::IntoVec;

use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::convert::TryInto;
use std::ops::Index;
use std::{fmt::Debug, ops::IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct LargeVector<T, const N: usize> {
    pub data: Vec<T>,
}

impl<T, const N: usize> LargeVector<T, N> {
    pub fn new(data: impl IntoVec<T, N>) -> Self {
        LargeVector {
            data: data.into_vec(),
        }
    }

    pub fn new_random() -> Self
    where
        Standard: Distribution<T>,
    {
        let mut rng = rand::thread_rng();
        let mut random_data: Vec<T> = Vec::with_capacity(N);

        for _ in 0..N {
            let random_num: T = rng.gen::<T>();
            random_data.push(random_num)
        }

        LargeVector { data: random_data }
    }
}

impl<T, const N: usize> Default for LargeVector<T, N> {
    fn default() -> Self {
        LargeVector {
            data: Vec::with_capacity(N),
        }
    }
}

impl<T, const N: usize> Index<usize> for LargeVector<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl<T, const N: usize> IndexMut<usize> for LargeVector<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

impl<T, const N: usize> LargeVector<T, N>
where
    T: Clone,
{
    pub fn to_vec(self) -> Vec<T> {
        self.data
    }

    pub fn to_array(self) -> [T; N] {
        self.data.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }
}
