use crate::vector::small_vector::SmallVector;

pub trait IntoVec<T, const N: usize> {
    fn into_vec(self) -> Vec<T>;
}

impl<T, const N: usize> IntoVec<T, N> for Vec<T> {
    fn into_vec(self) -> Vec<T> {
        self
    }
}

impl<T, const N: usize> IntoVec<T, N> for [T; N]
where
    T: Copy,
{
    fn into_vec(self) -> Vec<T> {
        let mut converted_vec: Vec<T> = Vec::with_capacity(N);
        for t in self.iter() {
            converted_vec.push(*t);
        }

        converted_vec
    }
}

impl<T, const N: usize> IntoVec<T, N> for SmallVector<T, N>
where
    T: Copy,
{
    fn into_vec(self) -> Vec<T> {
        self.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_vec_from_vec() {
        let vec = vec![1, 2, 3, 4];

        assert_eq!(IntoVec::<u8, 4>::into_vec(vec), vec![1, 2, 3, 4]);
    }

    #[test]
    fn into_vec_from_array() {
        let array = [1, 2, 3, 4];

        assert_eq!(array.into_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn into_vec_from_small_vector() {
        let small_vector = SmallVector { data: [1, 2, 3, 4] };

        assert_eq!(small_vector.into_vec(), vec![1, 2, 3, 4]);
    }
}
