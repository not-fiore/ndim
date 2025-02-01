use crate::core::NdArray;

pub trait ArrayLike<T> {
    fn into_array<const N: usize>(self, shape: &[usize; N]) -> NdArray<T, N>;
}
