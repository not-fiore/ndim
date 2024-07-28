// MIT License

// Copyright (c) 2024 Abhishek

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! NDim is an open-source Rust library for n-dimensional array storage, similar to NumPy in Python and ndarray in Rust. It aims to assist in scientific computation by providing efficient and flexible n-dimensional array data structures and operations. The library is designed to be performant and easy to use, making it an excellent choice for high-performance computing tasks in Rust.
//!
//! **Note:** This project is still under development, and contributions are welcome!
//!
//! ```
//! use ndim::core::NdArray;
//!
//! fn main() {
//!     // Create an NdArray filled with a specific value
//!     let shape = [3, 2];
//!     let array = NdArray::<i32, 2>::zeros(shape);
//!
//!     // Print the array
//!     for i in 0..array.shape()[0] {
//!         for j in 0..array.shape()[1] {
//!             // access the value from memory
//!             println!("{}", array[[i, j]]);
//!         }
//!     }
//! }
//! ```
//!
//! ## Features
//! + **N-Dimensional Array Storage:** Efficient storage and manipulation of n-dimensional arrays.
//! + **Generic Data Types:** Supports various numeric types, including integers and floating-point numbers.
//! + **Basic Array Operations:** Provides basic operations such as array creation, indexing, and element-wise operations.
//!
//! ## Upcoming Features
//!
//! + **Fancy printing:** Print the n-dimensional array conforming to its shape.
//! + **Axes mutation:** Change values of the n-dimensional array in an axis and much more with axes.
//! + **Mapping and other looping support:** Loop over the n-dimensional array by using (viz.) map, reduce, filter, etc. methods.
//! + **BLAS Support:** Integration with Basic Linear Algebra Subprograms (BLAS) for advanced linear algebra operations.
//!

/// API to create N-dimensional array
///
/// ## Types
///
/// + [`NdArray<T, N>`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html)
/// + [`Array<N>`](https://docs.rs/ndim/latest/ndim/core/type.Array.html)
/// + [`Array2<N>`](https://docs.rs/ndim/latest/ndim/core/type.Array2.html)
/// + [`Array3<N>`](https://docs.rs/ndim/latest/ndim/core/type.Array3.html)
/// + [`Array4<N>`](https://docs.rs/ndim/latest/ndim/core/type.Array4.html)
///
/// ## APIs (available in NdArray)
///
/// + [`NdArray::<T, N>::new()`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html#method.new) 
///     + Create an empty NdArray
/// + [`NdArray::<T, N>::from(arr: &[T], shape: [usize; N])`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html#method.from)
///     + Create an NdArray from a sized array with a shape
/// + [`NdArray::<T, N>::reshape(&mut self, shape: [usize; N])`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html#method.reshape)
///     + Reshape an NdArray
///
/// + [`NdArray::<T, N>::arange(range: usize)`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html#method.arange)
///     + Create an NdArray from 0 to an end value for type T
/// + [`NdArray::<T, N>::arange_with_step(range: usize, step: usize)`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html#method.arange_with_step)
///     + Create an NdArray from 0 to an end value with a step value for type T
/// + [`NdArray::<T, N>::ranges(range: (isize, isize))`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html#method.ranges)
///     + Create an NdArray from a start value, and a end value for type T
/// + [`NdArray::<T, N>::ranges_with_step(range: (isize, isize), step: usize)`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html#method.ranges_with_step)
///     + Create an NdArray from a start value, end value, and a step value for type T
///
/// + [`NdArray::<T, N>::zeros(shape: [usize; N])`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html#method.zeros)
///     + Create an NdArray with zeros
/// + [`NdArray::<T, N>::ones(shape: [usize; N])`](https://docs.rs/ndim/latest/ndim/core/struct.NdArray.html#method.ones)
///     + Create an NdArray with ones
pub mod core;
