use super::LargeVector;
use std::iter::FromIterator;
impl<'a, T, const N: usize> LargeVector<T, N> {
    pub fn into_iter(self) -> IntoIter<T, N> {
        IntoIter {
            data: self,
            current: 0,
            end: N,
        }
    }

    pub fn iter(&'a self) -> Iter<'a, T, N> {
        Iter {
            data: self,
            current: 0,
            end: N,
        }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, T, N> {
        IterMut {
            data: self,
            current: 0,
            end: N,
        }
    }
}

pub struct IntoIter<T, const N: usize> {
    data: LargeVector<T, N>,
    current: usize,
    end: usize,
}

impl<T, const N: usize> Iterator for IntoIter<T, N>
where
    T: Copy,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else {
            let current = self.current;
            self.current += 1;

            return Some(self.data.data[current]);
        }
    }
}

impl<T, const N: usize> IntoIterator for LargeVector<T, N>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = IntoIter<Self::Item, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            data: self,
            current: 0,
            end: N,
        }
    }
}

pub struct Iter<'a, T, const N: usize> {
    data: &'a LargeVector<T, N>,
    current: usize,
    end: usize,
}

impl<'a, T, const N: usize> IntoIterator for &'a LargeVector<T, N>
where
    T: Copy,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            data: self,
            current: 0,
            end: N,
        }
    }
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N>
where
    T: Copy,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else {
            let current = self.current;
            self.current += 1;

            return Some(&self.data.data[current]);
        }
    }
}

pub struct IterMut<'a, T, const N: usize> {
    data: &'a mut LargeVector<T, N>,
    current: usize,
    end: usize,
}

impl<'a, T, const N: usize> IntoIterator for &'a mut LargeVector<T, N>
where
    T: Copy,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            data: self,
            current: 0,
            end: N,
        }
    }
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N>
where
    T: Copy,
{
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else {
            let current = self.current;
            self.current += 1;

            let ptr = self.data.data.as_mut_ptr();
            return Some(unsafe { &mut *ptr.add(current) });
        }
    }
}

impl<T, const N: usize> FromIterator<T> for LargeVector<T, N>
where
    T: Default + Copy,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> LargeVector<T, N> {
        let mut collector: Vec<T> = Vec::with_capacity(N);

        for item in iter {
            collector.push(item);
        }

        LargeVector { data: collector }
    }
}
