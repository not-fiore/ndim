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

//! # Description
//! NDim is an open-source Rust library for n-dimensional array storage, similar to NumPy in Python and ndarray in Rust. It aims to assist in scientific computation by providing efficient and flexible n-dimensional array data structures and operations. The library is designed to be performant and easy to use, making it an excellent choice for high-performance computing tasks in Rust.
//!
//! **Note:** This project is still under development, and contributions are welcome!
//!
//! ## Features
//! - **N-Dimensional Array Storage:** Efficient storage and manipulation of n-dimensional arrays.
//! - **Generic Data Types:** Supports various numeric types, including integers and floating-point numbers.
//! **Basic Array Operations:** Provides basic operations such as array creation, indexing, and element-wise operations.
//!
//! ## Upcoming Features
//!
//! - **Fancy printing:** Print the n-dimensional array conforming to its shape.
//! - **Axes mutation:** Change values of the n-dimensional array in an axis and much more with axes.
//! - **Mapping and other looping support:** Loop over the n-dimensional array by using (viz.) map, reduce, filter, etc. methods.
//! - **BLAS Support:** Integration with Basic Linear Algebra Subprograms (BLAS) for advanced linear algebra operations.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ndim = { git = "https://github.com/noobsiecoder/ndim.git" }
//! ```
//!
//! ## Usage
//!
//! Here's a simple example of how to use `NDim`:
//!
//! ```rust
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
//!             println!("{}", array[[i, j]]); // access the value from memory
//!         }
//!     }
//! }
//! ```

/// Contains **core API to create N-dimensional** array and also helps in manipulating the values in its corresponding memory address.
///
/// View [here](https://github.com/noobsiecoder/ndim) to know more about the available API
pub mod core;
