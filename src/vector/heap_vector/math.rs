use num::integer::Roots;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::{math_vector::MathVector, matrix::Matrix};

use super::HeapVector;

impl<T, const N: usize> MathVector<T, N> for HeapVector<T, N>
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

        HeapVector { data: scaled_vec }
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

        HeapVector { data: added_vec }
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

        HeapVector {
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

        HeapVector {
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

        HeapVector { data: crossed_vec }
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
    fn heap_vector_scalar() {
        let heap_vector = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        let scaled_heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![3, 6, 9, 12],
        };
        assert_eq!(heap_vector.scalar(3), scaled_heap_vector);
    }

    #[test]
    fn heap_vector_scalar_mut() {
        let mut heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        heap_vector.scalar_mut(3);

        let scaled_heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![3, 6, 9, 12],
        };
        assert_eq!(heap_vector, scaled_heap_vector);
    }

    #[test]
    fn heap_vector_dot() {
        let heap_vector_1: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        assert_eq!(heap_vector_1.dot(heap_vector_2), 70);
    }

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

        assert_eq!(heap_vector_1.add_vector(heap_vector_2), added_heap_vector);
    }

    #[test]
    fn heap_vector_add_vector_mut() {
        let mut heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        heap_vector.add_vector_mut(heap_vector_2);

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

        assert_eq!(
            heap_vector_1.sub_vector(heap_vector_2),
            subtracted_heap_vector
        );
    }

    #[test]
    fn heap_vector_sub_vector_mut() {
        let mut heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        heap_vector.sub_vector_mut(heap_vector_2);

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

        assert_eq!(
            heap_vector_1.entrywise(heap_vector_2),
            multiplied_heap_vector
        );
    }

    #[test]
    fn heap_vector_entrywise_mut() {
        let mut heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![1, 2, 3, 4],
        };

        let heap_vector_2: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 6, 7, 8],
        };

        heap_vector.entrywise_mut(heap_vector_2);

        let multiplied_heap_vector: HeapVector<u8, 4> = HeapVector {
            data: vec![5, 12, 21, 32],
        };
        assert_eq!(heap_vector, multiplied_heap_vector);
    }

    #[test]
    fn heap_vector_cross() {
        let heap_vector_1: HeapVector<i8, 3> = HeapVector {
            data: vec![1, 2, 3],
        };

        let heap_vector_2: HeapVector<i8, 3> = HeapVector {
            data: vec![4, 5, 6],
        };

        let heap_vector_2_2: HeapVector<i8, 3> = HeapVector {
            data: vec![4, 5, 6],
        };

        let d = heap_vector_1.cross(heap_vector_2_2);
        println!("{:?}", d);

        let crossed_heap_vector: HeapVector<i8, 3> = HeapVector {
            data: vec![-3, 6, -3],
        };

        assert_eq!(heap_vector_1.cross(heap_vector_2), crossed_heap_vector);
    }

    #[test]
    fn heap_vector_cross_mut() {
        let mut heap_vector: HeapVector<i8, 3> = HeapVector {
            data: vec![1, 2, 3],
        };

        let heap_vector_2: HeapVector<i8, 3> = HeapVector {
            data: vec![4, 5, 6],
        };

        heap_vector.cross_mut(heap_vector_2);

        let crossed_heap_vector: HeapVector<i8, 3> = HeapVector {
            data: vec![-3, 6, -3],
        };
        assert_eq!(heap_vector, crossed_heap_vector);
    }

    #[test]
    fn heap_vector_tensor_prod() {
        let heap_vector_1: HeapVector<u8, 3> = HeapVector {
            data: vec![1, 2, 3],
        };

        let heap_vector_2: HeapVector<u8, 3> = HeapVector {
            data: vec![4, 5, 6],
        };

        let crossed_matrix_data = vec![vec![4, 5, 6], vec![8, 10, 12], vec![12, 15, 18]];

        let tensor_product: Matrix<u8, 3, 3> = Matrix::new(crossed_matrix_data);

        assert_eq!(heap_vector_1.tensor_prod(heap_vector_2), tensor_product);
    }

    #[test]
    fn heap_vector_magnitude() {
        let heap_vector: HeapVector<i8, 2> = HeapVector { data: vec![2, 2] };

        assert_eq!(heap_vector.magnitude(), 2);
    }
    #[test]
    fn heap_vector_sum() {
        let heap_vector: HeapVector<i8, 3> = HeapVector {
            data: vec![1, 2, 3],
        };

        assert_eq!(heap_vector.sum(), 6);
    }
}
