use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::math_vector::MathVector;

use super::HeapVector;

impl<T, const N: usize> Add for HeapVector<T, N>
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
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add_vector(rhs)
    }
}

impl<T, const N: usize> AddAssign for HeapVector<T, N>
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
    fn add_assign(&mut self, rhs: Self) {
        self.add_vector_mut(rhs)
    }
}

impl<T, const N: usize> Sub for HeapVector<T, N>
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
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.sub_vector(rhs)
    }
}

impl<T, const N: usize> SubAssign for HeapVector<T, N>
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
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_vector_mut(rhs)
    }
}

impl<T, const N: usize> Mul for HeapVector<T, N>
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
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.entrywise(rhs)
    }
}

impl<T, const N: usize> MulAssign for HeapVector<T, N>
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
    fn mul_assign(&mut self, rhs: Self) {
        self.entrywise_mut(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_vector_add_vector() {
        let heap_vector_1: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        let added_heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![6, 8, 10, 12],
        };

        assert_eq!(heap_vector_1 + heap_vector_2, added_heap_vector);
    }

    #[test]
    fn heap_vector_add_vector_mut() {
        let mut heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        heap_vector += heap_vector_2;

        let added_heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![6, 8, 10, 12],
        };
        assert_eq!(heap_vector, added_heap_vector);
    }

    #[test]
    fn heap_vector_sub_vector() {
        let heap_vector_1: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        let subtracted_heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![4, 4, 4, 4],
        };

        assert_eq!(heap_vector_1 - heap_vector_2, subtracted_heap_vector);
    }

    #[test]
    fn heap_vector_sub_vector_mut() {
        let mut heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        heap_vector -= heap_vector_2;

        let subtracted_heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![4, 4, 4, 4],
        };
        assert_eq!(heap_vector, subtracted_heap_vector);
    }

    #[test]
    fn heap_vector_entrywise() {
        let heap_vector_1: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        let multiplied_heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 12, 21, 32],
        };

        assert_eq!(heap_vector_1 * heap_vector_2, multiplied_heap_vector);
    }

    #[test]
    fn heap_vector_entrywise_mut() {
        let mut heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        heap_vector *= heap_vector_2;

        let multiplied_heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 12, 21, 32],
        };
        assert_eq!(heap_vector, multiplied_heap_vector);
    }
}
