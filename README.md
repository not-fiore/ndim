# NDim - Rust-based N-Dimensional Array Library

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/ndim.svg)](https://crates.io/)
[![Documentation](https://docs.rs/image/badge.svg)](https://docs.rs/ndim)
[![Build Status](https://github.com/noosiecoder/ndim/actions/workflows/rust.yaml/badge.svg)](https://github.com/noobsiecoder/ndim/actions)

## Overview

**NDim** is an open-source Rust library for n-dimensional array storage, similar to NumPy in Python and ndarray in Rust. It aims to assist in scientific computation by providing efficient and flexible n-dimensional array data structures and operations. The library is designed to be performant and easy to use, making it an excellent choice for high-performance computing tasks in Rust.

**Note:** This project is still under development, and contributions are welcome!

## Features

- **N-Dimensional Array Storage:** Efficient storage and manipulation of n-dimensional arrays.
- **Generic Data Types:** Supports various numeric types, including integers and floating-point numbers.
- **Basic Array Operations:** Provides basic operations such as array creation, indexing, and element-wise operations.

## Upcoming Features

- **Fancy printing:** - Print the n-dimensional array conforming to its shape.
- **Axes mutation:** - Change values of the n-dimensional array in an axis and much more with axes.
- **Mapping and other looping support:** Loop over the n-dimensional array by using (viz.) map, reduce, filter, etc. methods.
- **BLAS Support:** Integration with Basic Linear Algebra Subprograms (BLAS) for advanced linear algebra operations.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ndim = { git = "https://github.com/noobsiecoder/ndim.git" }
```

## Usage

Here's a simple example of how to use `NDim`:

```rust
use ndim::NdArray;

fn main() {
    // Create an NdArray filled with a specific value
    let shape = [3, 2];
    let array = NdArray::<i32, 2>::zeros(&shape);

    // Print the array
    for i in 0..array.len[0] {
        for i in 0..array.len[1] {
            println!("{}", array[[i, j]]); // access the value from memory
        }
    }
}
```

## Documentation

Detailed documentation is available on [docs.rs](https://docs.rs/ndim). You can also generate the documentation locally using `cargo doc`.

## Contributing

Contributions from the community are welcomed! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes and commit them with descriptive messages.
4. Push your branch to your forked repository.
5. Create a pull request to the main repository.

Please ensure that your code adheres to the project's coding standards and includes appropriate tests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions, issues, or suggestions, please open an issue on GitHub or contact the project maintainers.

---

Thank you for using NDim! I hope it helps you in your scientific computing endeavors.
