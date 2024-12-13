use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Matrix<const C: usize, const R: usize, T>([[T; C]; R])
where
    T: Clone + Copy;

impl<const C: usize, const R: usize, T> Matrix<C, R, T>
where
    T: Clone + Copy,
{
    pub fn new(value: T) -> Self {
        Self([[value; C]; R])
    }
}

impl<const C: usize, const R: usize, T> Deref for Matrix<C, R, T>
where
    T: Clone + Copy,
{
    type Target = [[T; C]; R];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const C: usize, const R: usize, T> DerefMut for Matrix<C, R, T>
where
    T: Clone + Copy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
