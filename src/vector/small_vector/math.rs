use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

use crate::{math_vector::MathVector, matrix::Matrix};
use num::{integer::Roots, FromPrimitive, ToPrimitive};

use super::SmallVector;

impl<T, const N: usize> MathVector<T, N> for SmallVector<T, N>
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
        let mut scaled_array: [T; N] = [T::default(); N];

        let scalar: T = FromPrimitive::from_isize(scalar).expect("Expected isize");
        for (idx, num) in self.data.iter().enumerate() {
            scaled_array[idx] = *num * scalar;
        }

        SmallVector { data: scaled_array }
    }

    fn scalar_mut(&mut self, scalar: isize) {
        let scalar: T = FromPrimitive::from_isize(scalar).expect("Expected isize");
        for num in self.data.iter_mut() {
            *num = *num * scalar;
        }
    }

    fn dot(&self, rhs: &(impl MathVector<T, N> + Index<usize, Output = T>)) -> isize {
        let mut acc: T = T::default();

        for idx in 0..N {
            acc += self.data[idx] * rhs[idx];
        }

        ToPrimitive::to_isize(&acc).expect("Type of T is not supported")
    }

    fn add_vector(&self, rhs: &(impl MathVector<T, N> + Index<usize, Output = T>)) -> Self {
        let mut added_array: [T; N] = [T::default(); N];

        for (idx, num) in added_array.iter_mut().enumerate() {
            *num = self.data[idx] + rhs[idx];
        }

        SmallVector { data: added_array }
    }

    fn add_vector_mut(&mut self, rhs: &(impl MathVector<T, N> + Index<usize, Output = T>)) {
        for (idx, num) in self.iter_mut().enumerate() {
            *num += rhs[idx];
        }
    }

    fn sub_vector(&self, rhs: &(impl MathVector<T, N> + Index<usize, Output = T>)) -> Self {
        let mut subtracted_array: [T; N] = [T::default(); N];

        for (idx, num) in subtracted_array.iter_mut().enumerate() {
            *num = self.data[idx] - rhs[idx];
        }

        SmallVector {
            data: subtracted_array,
        }
    }

    fn sub_vector_mut(&mut self, rhs: &(impl MathVector<T, N> + Index<usize, Output = T>)) {
        for (idx, num) in self.iter_mut().enumerate() {
            *num -= rhs[idx];
        }
    }

    fn entrywise(&self, rhs: &(impl MathVector<T, N> + Index<usize, Output = T>)) -> Self {
        let mut multiplied_array: [T; N] = [T::default(); N];

        for (idx, num) in multiplied_array.iter_mut().enumerate() {
            *num = self.data[idx] * rhs[idx];
        }

        SmallVector {
            data: multiplied_array,
        }
    }

    fn entrywise_mut(&mut self, rhs: &(impl MathVector<T, N> + Index<usize, Output = T>)) {
        for (idx, num) in self.iter_mut().enumerate() {
            *num *= rhs[idx];
        }
    }

    fn cross(&self, rhs: &(impl MathVector<T, N> + Index<usize, Output = T>)) -> Self {
        if N != 3 {
            panic!("The cross product requires that the length of both vectors must be 3");
        }

        let mut crossed_array: [T; N] = [T::default(); N];

        crossed_array[0] = self.data[1] * rhs[2] - self.data[2] * rhs[1];
        crossed_array[1] = self.data[2] * rhs[0] - self.data[0] * rhs[2];
        crossed_array[2] = self.data[0] * rhs[1] - self.data[1] * rhs[0];

        SmallVector {
            data: crossed_array,
        }
    }

    fn cross_mut(&mut self, rhs: &(impl MathVector<T, N> + Index<usize, Output = T>)) {
        if N != 3 {
            panic!("The cross product requires that the length of both vectors must be 3");
        }

        let data = self.data.clone();
        self.data[0] = data[1] * rhs[2] - data[2] * rhs[1];
        self.data[1] = data[2] * rhs[0] - data[0] * rhs[2];
        self.data[2] = data[0] * rhs[1] - data[1] * rhs[0];
    }

    fn tensor_prod<const M: usize>(
        &self,
        rhs: &(impl MathVector<T, N> + Index<usize, Output = T>),
    ) -> Matrix<T, M, N> {
        let mut tensor_product: Matrix<T, M, N> = Matrix::new([[T::default(); N]; M]);

        for (row_idx, row) in tensor_product.iter_mut().enumerate() {
            for (col_idx, col) in row.iter_mut().enumerate() {
                *col = self.data[row_idx] * rhs[col_idx];
            }
        }

        tensor_product
    }

    fn magnitude(&self) -> usize {
        let mut acc: T = T::default();

        for num in self.iter() {
            acc += *num * *num;
        }

        let isize_acc = ToPrimitive::to_usize(&acc)
            .expect("Valid integers are required to calculate the magnitude");

        isize_acc.sqrt()
    }

    fn sum(&self) -> isize {
        let mut acc: T = T::default();

        for num in self.iter() {
            acc += *num;
        }

        ToPrimitive::to_isize(&acc).expect("Valid integers are required to calculate the sum")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_vector_scalar() {
        let small_vector = SmallVector { data: [1, 2, 3, 4] };

        let scaled_small_vector: SmallVector<u8, 4> = SmallVector {
            data: [3, 6, 9, 12],
        };

        assert_eq!(small_vector.scalar(3), scaled_small_vector);
    }

    #[test]
    fn small_vector_scalar_mut() {
        let mut small_vector: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        small_vector.scalar_mut(3);

        let scaled_small_vector: SmallVector<u8, 4> = SmallVector {
            data: [3, 6, 9, 12],
        };

        assert_eq!(small_vector, scaled_small_vector);
    }

    #[test]
    fn small_vector_dot() {
        let small_vector_1: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        assert_eq!(small_vector_1.dot(&small_vector_2), 70);
    }

    #[test]
    fn small_vector_add_vector() {
        let small_vector_1: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        let added_small_vector: SmallVector<u8, 4> = SmallVector {
            data: [6, 8, 10, 12],
        };

        assert_eq!(
            small_vector_1.add_vector(&small_vector_2),
            added_small_vector
        );
    }

    #[test]
    fn small_vector_add_vector_mut() {
        let mut small_vector: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        small_vector.add_vector_mut(&small_vector_2);

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

        assert_eq!(
            small_vector_1.sub_vector(&small_vector_2),
            subtracted_small_vector
        );
    }

    #[test]
    fn small_vector_sub_vector_mut() {
        let mut small_vector: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        small_vector.sub_vector_mut(&small_vector_2);

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

        assert_eq!(
            small_vector_1.entrywise(&small_vector_2),
            multiplied_small_vector
        );
    }

    #[test]
    fn small_vector_entrywise_mut() {
        let mut small_vector: SmallVector<u8, 4> = SmallVector { data: [1, 2, 3, 4] };

        let small_vector_2: SmallVector<u8, 4> = SmallVector { data: [5, 6, 7, 8] };

        small_vector.entrywise_mut(&small_vector_2);

        let multiplied_small_vector: SmallVector<u8, 4> = SmallVector {
            data: [5, 12, 21, 32],
        };
        assert_eq!(small_vector, multiplied_small_vector);
    }

    #[test]
    fn small_vector_cross() {
        let small_vector_1: SmallVector<i8, 3> = SmallVector { data: [1, 2, 3] };

        let small_vector_2: SmallVector<i8, 3> = SmallVector { data: [4, 5, 6] };

        let crossed_small_vector: SmallVector<i8, 3> = small_vector_1.cross(&small_vector_2);

        assert_eq!(SmallVector { data: [-3, 6, -3] }, crossed_small_vector);
    }

    #[test]
    fn small_vector_cross_mut() {
        let mut small_vector: SmallVector<i8, 3> = SmallVector { data: [1, 2, 3] };

        let small_vector_2: SmallVector<i8, 3> = SmallVector { data: [4, 5, 6] };

        small_vector.cross_mut(&small_vector_2);

        let crossed_small_vector: SmallVector<i8, 3> = SmallVector { data: [-3, 6, -3] };
        assert_eq!(small_vector, crossed_small_vector);
    }

    #[test]
    fn small_vector_tensor_prod() {
        let small_vector_1: SmallVector<u8, 3> = SmallVector { data: [1, 2, 3] };

        let small_vector_2: SmallVector<u8, 3> = SmallVector { data: [4, 5, 6] };

        let crossed_matrix_data = [[4, 5, 6], [8, 10, 12], [12, 15, 18]];

        let tensor_product: Matrix<u8, 3, 3> = Matrix::new(crossed_matrix_data);

        assert_eq!(small_vector_1.tensor_prod(&small_vector_2), tensor_product);
    }

    #[test]
    fn small_vector_magnitude() {
        let small_vector: SmallVector<i8, 2> = SmallVector { data: [2, 2] };

        assert_eq!(small_vector.magnitude(), 2);
    }
    #[test]
    fn small_vector_sum() {
        let small_vector: SmallVector<i8, 3> = SmallVector { data: [1, 2, 3] };

        assert_eq!(small_vector.sum(), 6);
    }
}
