use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::math_vector::MathVector;

use super::InlineVector;

impl<T, const N: usize> Add for InlineVector<T, N>
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

impl<T, const N: usize> AddAssign for InlineVector<T, N>
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

impl<T, const N: usize> Sub for InlineVector<T, N>
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

impl<T, const N: usize> SubAssign for InlineVector<T, N>
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

impl<T, const N: usize> Mul for InlineVector<T, N>
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

impl<T, const N: usize> MulAssign for InlineVector<T, N>
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
    fn inline_vector_add_vector() {
        let inline_vector_1: InlineVector<u8, 4> = InlineVector { data: [1, 2, 3, 4] };

        let inline_vector_2: InlineVector<u8, 4> = InlineVector { data: [5, 6, 7, 8] };

        let added_inline_vector: InlineVector<u8, 4> = InlineVector {
            data: [6, 8, 10, 12],
        };

        assert_eq!(inline_vector_1 + inline_vector_2, added_inline_vector);
    }

    #[test]
    fn inline_vector_add_vector_mut() {
        let mut inline_vector: InlineVector<u8, 4> = InlineVector { data: [1, 2, 3, 4] };

        let inline_vector_2: InlineVector<u8, 4> = InlineVector { data: [5, 6, 7, 8] };

        inline_vector += inline_vector_2;

        let added_inline_vector: InlineVector<u8, 4> = InlineVector {
            data: [6, 8, 10, 12],
        };
        assert_eq!(inline_vector, added_inline_vector);
    }

    #[test]
    fn inline_vector_sub_vector() {
        let inline_vector_1: InlineVector<u8, 4> = InlineVector { data: [5, 6, 7, 8] };

        let inline_vector_2: InlineVector<u8, 4> = InlineVector { data: [1, 2, 3, 4] };

        let subtracted_inline_vector: InlineVector<u8, 4> = InlineVector { data: [4, 4, 4, 4] };

        assert_eq!(inline_vector_1 - inline_vector_2, subtracted_inline_vector);
    }

    #[test]
    fn inline_vector_sub_vector_mut() {
        let mut inline_vector: InlineVector<u8, 4> = InlineVector { data: [5, 6, 7, 8] };

        let inline_vector_2: InlineVector<u8, 4> = InlineVector { data: [1, 2, 3, 4] };

        inline_vector -= inline_vector_2;

        let subtracted_inline_vector: InlineVector<u8, 4> = InlineVector { data: [4, 4, 4, 4] };
        assert_eq!(inline_vector, subtracted_inline_vector);
    }

    #[test]
    fn inline_vector_entrywise() {
        let inline_vector_1: InlineVector<u8, 4> = InlineVector { data: [1, 2, 3, 4] };

        let inline_vector_2: InlineVector<u8, 4> = InlineVector { data: [5, 6, 7, 8] };

        let multiplied_inline_vector: InlineVector<u8, 4> = InlineVector {
            data: [5, 12, 21, 32],
        };

        assert_eq!(inline_vector_1 * inline_vector_2, multiplied_inline_vector);
    }

    #[test]
    fn inline_vector_entrywise_mut() {
        let mut inline_vector: InlineVector<u8, 4> = InlineVector { data: [1, 2, 3, 4] };

        let inline_vector_2: InlineVector<u8, 4> = InlineVector { data: [5, 6, 7, 8] };

        inline_vector *= inline_vector_2;

        let multiplied_inline_vector: InlineVector<u8, 4> = InlineVector {
            data: [5, 12, 21, 32],
        };
        assert_eq!(inline_vector, multiplied_inline_vector);
    }
}
