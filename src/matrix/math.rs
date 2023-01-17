use crate::math_vector::MathVector;
use crate::vector::Vector;
use num::{FromPrimitive, ToPrimitive};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::Matrix;

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
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
    pub fn mult<const P: usize>(&self, matrix2: &Matrix<T, N, P>) -> Matrix<T, M, P>
    where
        T: FromPrimitive + ToPrimitive + Debug + Copy + Mul<Output = T> + AddAssign,
    {
        let mut multiplied_matrix_data: [Vector<T, P>; M] = unsafe { std::mem::zeroed() };

        if P * M < 300 * 800 {
            for row in 0..M {
                for col in 0..P {
                    let mut acc: T = T::default();

                    for index in 0..P {
                        acc += self[row][index] * matrix2[index][col]
                    }
                    multiplied_matrix_data[row][col] = acc;
                }
            }
        } else {
            for (idx_row, row) in self.into_iter().enumerate() {
                for (idx_col, col) in matrix2.transpose().into_iter().enumerate() {
                    multiplied_matrix_data[idx_row][idx_col] =
                        FromPrimitive::from_isize(row.dot(&col)).expect("Expected valid isize");
                }
            }
        }

        Matrix {
            inner: multiplied_matrix_data,
        }
    }

    pub fn add(&self, matrix2: &Matrix<T, M, N>) -> Matrix<T, M, N> {
        let mut added_matrix: [Vector<T, N>; M] = unsafe { std::mem::zeroed() };

        for (idx_row, row) in self.into_iter().enumerate() {
            for (idx_col, num) in row.into_iter().enumerate() {
                added_matrix[idx_row][idx_col] = *num + matrix2[idx_row][idx_col];
            }
        }

        Matrix {
            inner: added_matrix,
        }
    }

    pub fn subtract(&self, matrix2: &Matrix<T, M, N>) -> Matrix<T, M, N> {
        let mut subtracted_matrix: [Vector<T, N>; M] = unsafe { std::mem::zeroed() };

        for (idx_row, row) in self.into_iter().enumerate() {
            for (idx_col, num) in row.into_iter().enumerate() {
                subtracted_matrix[idx_row][idx_col] = *num - matrix2[idx_row][idx_col];
            }
        }

        Matrix {
            inner: subtracted_matrix,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix, Vector};

    #[test]
    fn multiply_matrix() {
        let matrix_array: Matrix<u8, 3, 2> = Matrix::new([[1, 2], [3, 4], [5, 6]]);

        let matrix_vec: Matrix<u8, 2, 2> = Matrix::new([Vec::from([3, 1]), Vec::from([9, 6])]);

        let multiplied_matrix = matrix_array.mult(&matrix_vec);

        assert_eq!(
            multiplied_matrix,
            Matrix::new([
                Vector::new([21, 13]),
                Vector::new([45, 27]),
                Vector::new([69, 41])
            ])
        );
    }

    #[test]
    fn add_matrix() {
        let matrix_array: Matrix<u8, 3, 2> = Matrix::new([[1, 2], [3, 4], [5, 6]]);

        let matrix_vec: Matrix<u8, 3, 2> =
            Matrix::new([Vec::from([3, 1]), Vec::from([9, 6]), Vec::from([2, 3])]);

        let added_matrix = matrix_array.add(&matrix_vec);
        println!("{:?}", matrix_vec);
        println!("{:?}", added_matrix);

        assert_eq!(
            added_matrix,
            Matrix::new([
                Vector::new([4, 3]),
                Vector::new([12, 10]),
                Vector::new([7, 9])
            ])
        );
    }

    #[test]
    fn subtract_matrix() {
        let matrix_array: Matrix<u8, 3, 2> = Matrix::new([[5, 2], [100, 8], [5, 6]]);

        let matrix_vec: Matrix<u8, 3, 2> =
            Matrix::new([Vec::from([3, 1]), Vec::from([9, 6]), Vec::from([2, 3])]);

        let subtracted_matrix = matrix_array.subtract(&matrix_vec);

        assert_eq!(
            subtracted_matrix,
            Matrix::new([
                Vector::new([2, 1]),
                Vector::new([91, 2]),
                Vector::new([3, 3])
            ])
        );
    }
}
