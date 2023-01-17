use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::math_vector::MathVector;

use super::SmallVector;

impl<T, const N: usize> Add for SmallVector<T, N>
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

impl<T, const N: usize> AddAssign for SmallVector<T, N>
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

impl<T, const N: usize> Sub for SmallVector<T, N>
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

impl<T, const N: usize> SubAssign for SmallVector<T, N>
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

impl<T, const N: usize> Mul for SmallVector<T, N>
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

impl<T, const N: usize> MulAssign for SmallVector<T, N>
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
    fn small_vector_add_vector() {
        let small_vector_1: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        let added_small_vector: SmallVector<u8, 4> = SmallVector {
            data: [6, 8, 10, 12],
        };

        assert_eq!(small_vector_1 + small_vector_2, added_small_vector);
    }

    #[test]
    fn small_vector_add_vector_mut() {
        let mut small_vector: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        small_vector += small_vector_2;

        let added_small_vector: SmallVector<u8, 4> = SmallVector {
            data: [6, 8, 10, 12],
        };
        assert_eq!(small_vector, added_small_vector);
    }

    #[test]
    fn small_vector_sub_vector() {
        let small_vector_1: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        let subtracted_small_vector: SmallVector<u8, 4> = SmallVector { data: [4, 4, 4, 4] };

        assert_eq!(small_vector_1 - small_vector_2, subtracted_small_vector);
    }

    #[test]
    fn small_vector_sub_vector_mut() {
        let mut small_vector: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        small_vector -= small_vector_2;

        let subtracted_small_vector: SmallVector<u8, 4> = SmallVector { data: [4, 4, 4, 4] };
        assert_eq!(small_vector, subtracted_small_vector);
    }

    #[test]
    fn small_vector_entrywise() {
        let small_vector_1: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        let multiplied_small_vector: SmallVector<u8, 4> = SmallVector {
            data: [5, 12, 21, 32],
        };

        assert_eq!(small_vector_1 * small_vector_2, multiplied_small_vector);
    }

    #[test]
    fn small_vector_entrywise_mut() {
        let mut small_vector: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        small_vector *= small_vector_2;

        let multiplied_small_vector: SmallVector<u8, 4> = SmallVector {
            data: [5, 12, 21, 32],
        };
        assert_eq!(small_vector, multiplied_small_vector);
    }
}
