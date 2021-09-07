use crate::vector::large_vector::into_vec::IntoVec;
use std::convert::TryInto;

use crate::vector::large_vector::LargeVector;

pub trait IntoArray<T, const N: usize> {
    fn into_array(self) -> [T; N];
}

impl<T, const N: usize> IntoArray<T, N> for [T; N]
where
    T: Copy,
{
    fn into_array(self) -> [T; N] {
        self
    }
}

impl<T, const N: usize> IntoArray<T, N> for Vec<T> {
    fn into_array(self) -> [T; N] {
        self.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }
}

impl<T, const N: usize> IntoArray<T, N> for LargeVector<T, N>
where
    T: Clone,
{
    fn into_array(self) -> [T; N] {
        self.to_array()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_array_from_array() {
        let array = [1, 2, 3, 4];

        assert_eq!(array.into_array(), [1, 2, 3, 4]);
    }

    #[test]
    fn into_array_from_vec() {
        let array = [1, 2, 3, 4];

        assert_eq!(array.into_vec(), [1, 2, 3, 4]);
    }

    #[test]
    fn into_array_from_large_vector() {
        let large_vector = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        assert_eq!(large_vector.into_array(), [1, 2, 3, 4]);
    }
}
