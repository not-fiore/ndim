use thiserror::Error;

use crate::core::NdArray;

/// Implement this trait on types that can be converted into an `N`-dimensional array.
pub trait ArrayLike<T, const N: usize>: Sized {
    fn into_array(self, shape: &[usize; N]) -> Result<NdArray<T, N>, ShapeError>;
    fn array(&self, shape: &[usize; N]) -> Result<NdArray<T, N>, ShapeError>;
}

// TODO: actually write this generic trait impl
// impl<T, const N: usize, A: AsRef<[T]>> ArrayLike<T, N> for A {
//     fn into_array(self, shape: &[usize; N]) -> Result<NdArray<T, N>, ShapeError> {}
//     fn array(&self, shape: &[usize; N]) -> Result<NdArray<T, N>, ShapeError> {}
// }

#[derive(Error, Debug)]
pub enum ShapeError {
    #[error("the data exceeds the given shape by {0} elements.")]
    TooLong(usize),
    #[error("the data cant fill the given shape by {0} elements.")]
    TooShort(usize),
}
