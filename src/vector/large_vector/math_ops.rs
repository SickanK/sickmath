use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::math_vector::MathVector;

use super::LargeVector;

impl<T, const N: usize> Add for LargeVector<T, N>
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
        self.add_vector(&rhs)
    }
}

impl<T, const N: usize> AddAssign for LargeVector<T, N>
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
        self.add_vector_mut(&rhs)
    }
}

impl<T, const N: usize> Sub for LargeVector<T, N>
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
        self.sub_vector(&rhs)
    }
}

impl<T, const N: usize> SubAssign for LargeVector<T, N>
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
        self.sub_vector_mut(&rhs)
    }
}

impl<T, const N: usize> Mul for LargeVector<T, N>
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
        self.entrywise(&rhs)
    }
}

impl<T, const N: usize> MulAssign for LargeVector<T, N>
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
        self.entrywise_mut(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_vector_add_vector() {
        let large_vector_1: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        let added_large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![6, 8, 10, 12],
        };

        assert_eq!(large_vector_1 + large_vector_2, added_large_vector);
    }

    #[test]
    fn large_vector_add_vector_mut() {
        let mut large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        large_vector += large_vector_2;

        let added_large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![6, 8, 10, 12],
        };
        assert_eq!(large_vector, added_large_vector);
    }

    #[test]
    fn large_vector_sub_vector() {
        let large_vector_1: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        let subtracted_large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![4, 4, 4, 4],
        };

        assert_eq!(large_vector_1 - large_vector_2, subtracted_large_vector);
    }

    #[test]
    fn large_vector_sub_vector_mut() {
        let mut large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        large_vector -= large_vector_2;

        let subtracted_large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![4, 4, 4, 4],
        };
        assert_eq!(large_vector, subtracted_large_vector);
    }

    #[test]
    fn large_vector_entrywise() {
        let large_vector_1: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        let multiplied_large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 12, 21, 32],
        };

        assert_eq!(large_vector_1 * large_vector_2, multiplied_large_vector);
    }

    #[test]
    fn large_vector_entrywise_mut() {
        let mut large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        large_vector *= large_vector_2;

        let multiplied_large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 12, 21, 32],
        };
        assert_eq!(large_vector, multiplied_large_vector);
    }
}
