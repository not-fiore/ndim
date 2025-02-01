use crate::core::NdArray as Array;

pub trait ArrayLike<T> {
    fn into_array<const N: usize>(self, shape: &[usize; N]) -> Array<T, N>;
    fn array<const N: usize>(&self, shape: &[usize; N]) -> Array<T, N>;
}

// TODO: actually write this generic trait impl
// impl<T, A: AsRef<[T]>> ArrayLike<T> for A {
//     fn into_array<const N: usize>(self, shape: &[usize; N]) -> Array<T, N> {}
//     fn array<const N: usize>(&self, shape: &[usize; N]) -> Array<T, N> {}
// }
