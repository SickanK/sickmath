use num::integer::Roots;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::{math_vector::MathVector, matrix::Matrix};

use super::LargeVector;

impl<T, const N: usize> MathVector<T, N> for LargeVector<T, N>
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
        let mut scaled_vec: Vec<T> = Vec::with_capacity(N);

        for num in self.data.iter() {
            scaled_vec.push(*num * FromPrimitive::from_isize(scalar).expect("Expected isize"));
        }

        LargeVector { data: scaled_vec }
    }

    fn scalar_mut(&mut self, scalar: isize) {
        for num in self.data.iter_mut() {
            *num *= FromPrimitive::from_isize(scalar).expect("Expected isize")
        }
    }

    fn dot(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> isize {
        let mut acc: T = T::default();

        for idx in 0..N {
            acc += self.data[idx] * rhs[idx];
        }

        ToPrimitive::to_isize(&acc).expect("Type of T is not supported")
    }

    fn add_vector(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        let mut added_vec: Vec<T> = Vec::with_capacity(N);

        for idx in 0..N {
            added_vec.push(self.data[idx] + rhs[idx]);
        }

        LargeVector { data: added_vec }
    }

    fn add_vector_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        for (idx, num) in self.data.iter_mut().enumerate() {
            *num += rhs[idx];
        }
    }

    fn sub_vector(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        let mut subtracted_vec: Vec<T> = Vec::with_capacity(N);

        for idx in 0..N {
            subtracted_vec.push(self.data[idx] - rhs[idx]);
        }

        LargeVector {
            data: subtracted_vec,
        }
    }

    fn sub_vector_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        for (idx, num) in self.data.iter_mut().enumerate() {
            *num -= rhs[idx];
        }
    }

    fn entrywise(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        let mut multiplied_vec: Vec<T> = Vec::with_capacity(N);

        for idx in 0..N {
            multiplied_vec.push(self.data[idx] * rhs[idx]);
        }

        LargeVector {
            data: multiplied_vec,
        }
    }

    fn entrywise_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        for (idx, num) in self.data.iter_mut().enumerate() {
            *num *= rhs[idx];
        }
    }

    fn cross(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        if N != 3 {
            panic!("The cross product requires that the length of both vectors must be 3");
        }

        let mut crossed_vec: Vec<T> = Vec::with_capacity(N);

        crossed_vec.push(self.data[1] * rhs[2] - self.data[2] * rhs[1]);
        crossed_vec.push(self.data[2] * rhs[0] - self.data[0] * rhs[2]);
        crossed_vec.push(self.data[0] * rhs[1] - self.data[1] * rhs[0]);

        LargeVector { data: crossed_vec }
    }

    fn cross_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
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
        rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>,
    ) -> Matrix<T, M, N> {
        let mut tensor_product: Matrix<T, M, N> = Matrix::default();

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
    fn large_vector_scalar() {
        let large_vector = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        let scaled_large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![3, 6, 9, 12],
        };
        assert_eq!(large_vector.scalar(3), scaled_large_vector);
    }

    #[test]
    fn large_vector_scalar_mut() {
        let mut large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        large_vector.scalar_mut(3);

        let scaled_large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![3, 6, 9, 12],
        };
        assert_eq!(large_vector, scaled_large_vector);
    }

    #[test]
    fn large_vector_dot() {
        let large_vector_1: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        assert_eq!(large_vector_1.dot(large_vector_2), 70);
    }

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

        assert_eq!(
            large_vector_1.add_vector(large_vector_2),
            added_large_vector
        );
    }

    #[test]
    fn large_vector_add_vector_mut() {
        let mut large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        large_vector.add_vector_mut(large_vector_2);

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

        assert_eq!(
            large_vector_1.sub_vector(large_vector_2),
            subtracted_large_vector
        );
    }

    #[test]
    fn large_vector_sub_vector_mut() {
        let mut large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        large_vector.sub_vector_mut(large_vector_2);

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

        assert_eq!(
            large_vector_1.entrywise(large_vector_2),
            multiplied_large_vector
        );
    }

    #[test]
    fn large_vector_entrywise_mut() {
        let mut large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![1, 2, 3, 4],
        };

        let large_vector_2: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 6, 7, 8],
        };

        large_vector.entrywise_mut(large_vector_2);

        let multiplied_large_vector: LargeVector<u8, 4> = LargeVector {
            data: vec![5, 12, 21, 32],
        };
        assert_eq!(large_vector, multiplied_large_vector);
    }

    #[test]
    fn large_vector_cross() {
        let large_vector_1: LargeVector<i8, 3> = LargeVector {
            data: vec![1, 2, 3],
        };

        let large_vector_2: LargeVector<i8, 3> = LargeVector {
            data: vec![4, 5, 6],
        };

        let crossed_large_vector: LargeVector<i8, 3> = LargeVector {
            data: vec![-3, 6, -3],
        };

        assert_eq!(large_vector_1.cross(large_vector_2), crossed_large_vector);
    }

    #[test]
    fn large_vector_cross_mut() {
        let mut large_vector: LargeVector<i8, 3> = LargeVector {
            data: vec![1, 2, 3],
        };

        let large_vector_2: LargeVector<i8, 3> = LargeVector {
            data: vec![4, 5, 6],
        };

        large_vector.cross_mut(large_vector_2);

        let crossed_large_vector: LargeVector<i8, 3> = LargeVector {
            data: vec![-3, 6, -3],
        };
        assert_eq!(large_vector, crossed_large_vector);
    }

    #[test]
    fn large_vector_tensor_prod() {
        let large_vector_1: LargeVector<u8, 3> = LargeVector {
            data: vec![1, 2, 3],
        };

        let large_vector_2: LargeVector<u8, 3> = LargeVector {
            data: vec![4, 5, 6],
        };

        let crossed_matrix_data = vec![vec![4, 5, 6], vec![8, 10, 12], vec![12, 15, 18]];

        let tensor_product: Matrix<u8, 3, 3> = Matrix::new(crossed_matrix_data);

        assert_eq!(large_vector_1.tensor_prod(large_vector_2), tensor_product);
    }

    #[test]
    fn large_vector_magnitude() {
        let large_vector: LargeVector<i8, 2> = LargeVector { data: vec![2, 2] };

        assert_eq!(large_vector.magnitude(), 2);
    }
    #[test]
    fn large_vector_sum() {
        let large_vector: LargeVector<i8, 3> = LargeVector {
            data: vec![1, 2, 3],
        };

        assert_eq!(large_vector.sum(), 6);
    }
}
