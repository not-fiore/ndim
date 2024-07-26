use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

/// Type alias for `1usize`. Used while intializing as default values in `shape` and `strides`
const USIZE_ONE: usize = 1;
/// Type alias for `[usize; N]`. Typically used in `shape` and `strides` of an NdArray object
type SizedArray<const N: usize> = [usize; N];

/// Used in the creation of N-dimensional arrays
///
/// ## Examples
///
/// #### Create a two dimensional (2D) array
///
/// ```
/// use ndim::core::{NdArray, Array2};
///
/// # fn main() {
/// let data: NdArray<u16, 2> = Array2::<u16>::new();
/// # }
/// ```
///
/// #### Create using Range function
///
/// ```
/// use ndim::core::{NdArray, Array3};
///
/// # fn main() {
/// let range: usize = u16::MAX as usize;
/// let data: NdArray<u16, 3> = Array3::<u16>::arange(range);
/// # }
/// ```
///
/// #### Reshape, Access data from an index of the NdArray
///
/// ```
/// use ndim::core::{NdArray, Array3};
///
/// # fn main() {
/// let range: usize = u16::MAX as usize;
/// let mut shape: [usize; 3] = [1, 1, range];
/// let mut data: NdArray<u16, 3> = Array3::<u16>::arange(range); // create a NdArray ranging from 0..66535
/// assert_eq!(*data.shape(), shape);
///
/// shape = [3, 5, 4369];
/// data.reshape(shape); // reshaping NdArray's shape
/// assert_eq!(data[[1, 2, 1234]], 31817); // accessing value from memory
/// # }
/// ```
///
/// For more examples, view this link on [github](https://github.com/noobsiecoder/ndim)
#[derive(Debug)]
pub struct NdArray<T, const N: usize> {
    ptr: *mut T,
    len: usize,
    shape: SizedArray<N>,
    strides: SizedArray<N>,
}

/// Type alias for a one Dimensional (1-D) array
pub type Array<T> = NdArray<T, 1>;
/// Type alias for a two Dimensional (2-D) array
pub type Array2<T> = NdArray<T, 2>;
/// Type alias for a three Dimensional (3-D) array
pub type Array3<T> = NdArray<T, 3>;
/// Type alias for a four Dimensional (4-D) array
pub type Array4<T> = NdArray<T, 4>;

impl<T: Debug + Copy + Default, const N: usize> NdArray<T, N> {
    /// Calculate the stride of the array from the given `shape` and return in type `SizedArray<N>`
    /// Helps in index navigation and the explanation is shown [here](https://github.com/noobsiecoder/ndim)
    fn stride(shape: &SizedArray<N>) -> SizedArray<N> {
        let mut strides: SizedArray<N> = [1usize; N];
        strides[N - 1] = std::mem::size_of::<T>();
        for idx in (0..N - 1).rev() {
            // shape: [1, 2, 3]
            // strides[0] = size_of<T>
            // strides[1] = strides[0] * shape[0] -> size_of<T> * 3
            // strides[2] = strides[1] * shape[1] -> size_of<T> * 3 * 2

            // For (i, j, k):
            // Index = i * strides[0] + j * strides[1] + k * strides[2]
            // This `Index` will be used to access the memory location of the sized array
            strides[idx] = strides[idx + 1] * shape[idx + 1];
        }

        strides
    }

    /// Calulate the size of the array from the given `shape` and return in `usize`
    fn size_from_shape(shape: &SizedArray<N>) -> usize {
        let mut t_size: usize = 1;
        for val in *shape {
            t_size *= val;
        }

        t_size
    }

    /// Calulate the size of the array from the given `range`, `step` and return in `usize`
    /// Explanation behind the calculation can be viewed [here](https://github.com/noobsiecoder/ndim)
    fn size_from_range(pos: (isize, isize), step: usize) -> usize {
        let range: usize = (pos.1 - pos.0).abs() as usize;
        // Avoid Zero Division Error
        if step == 0 {
            return range;
        }

        // Even Range and step
        // range    = -1..5 (6ct)
        // step     = 2
        // values   = 4

        // Even Range and an odd step
        // range    = -1..5 (6ct)
        // step     = 3
        // values   = 3

        // Odd Range and an even step
        // range    = -1..6 (7ct)
        // step     = 2
        // values   = 4

        // Odd Range and step
        // range    = -1..6 (7ct)
        // step     = 3
        // values   = 3
        if range % 2 == 0 {
            if step % 2 == 0 {
                return (range / step) + 1;
            } else {
                return range / step;
            }
        } else {
            if step % 2 == 0 {
                return range / step;
            } else {
                return (range / step) + 1;
            }
        }
    }

    /// Returns the length of the NdArray object's sized array
    pub fn len(&self) -> &usize {
        &self.len
    }

    /// Returns the shape of the NdArray object
    pub fn shape(&self) -> &SizedArray<N> {
        &self.shape
    }

    /// Returns stride of the NDArray object
    pub fn strides(&self) -> &SizedArray<N> {
        &self.strides
    }

    /// Creates an empty NdArray object. Requires shape size of N` to determine the dimension of the array
    ///
    /// ## Example
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// // Creates a null pointer for the sized array
    /// // Hence, length is zero and the shape and strides are iniialized with 1's of size `N`
    /// let arr = NdArray::<i8, 4>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        NdArray {
            ptr: std::ptr::null_mut(),
            len: 0,
            shape: [USIZE_ONE; N],
            strides: [USIZE_ONE; N],
        }
    }

    /// Creates a NdArray object from a sized T. Requires shape of size `N`
    ///
    /// ## Panics
    /// If shape is not equivalent to current array size (or length), panics, and returns **Shape(`shape`) don't match with current Size(`size`)**
    ///
    /// ## Example
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let vec: Vec<i8> = (-2..22).collect();
    /// let shape: [usize; 4] = [2, 2, 3, 2];
    /// let arr = NdArray::<i8, 4>::from(&vec, shape);
    /// # }
    /// ```
    pub fn from(arr: &[T], shape: SizedArray<N>) -> Self {
        let len: usize = arr.len();
        if len != Self::size_from_shape(&shape) {
            panic!("Shape({:?}) don't match with array Size({})", shape, len);
        }

        let slice_as_ptr: *const T = arr.as_ptr();
        let ptr: *mut T = unsafe { std::mem::transmute(slice_as_ptr) }; // converts pointer type from *const T to *mut T by reinterpreting its bits
        let strides: SizedArray<N> = Self::stride(&shape);

        NdArray {
            ptr,
            len,
            shape,
            strides,
        }
    }

    /// Reshape the sized array for a new shape of type `SizedArray<N>`
    ///
    /// ## Panics
    /// If new (given as an argument) shape is not equivalent to current array size (or length), panics, and returns **New Shape(`shape`) don't match with current Size(`size`)**
    ///
    /// ## Example
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let shape: [usize; 3] = [1, 1, 15];
    /// let mut arr = NdArray::<i8, 3>::zeros(shape);
    /// assert_eq!(*arr.shape(), shape);
    ///
    /// let new_shape = [1, 3, 5];
    /// arr.reshape(new_shape);
    /// assert_eq!(*arr.shape(), new_shape);
    /// # }
    /// ```
    pub fn reshape(&mut self, shape: SizedArray<N>) {
        if Self::size_from_shape(&shape) != self.len {
            panic!(
                "New Shape({:?}) don't match with current Size({})",
                shape, self.len
            )
        }

        self.shape = shape;
        self.strides = Self::stride(&shape);
    }

    /// Helper function to create a sized array from a range containing `start` and an `end` value along with a `step` value
    ///
    /// ## Note
    /// - Accepts both positive and negative integers
    /// - This is a private method in the implementation and cannot (and should never) be used outside this `impl` block
    /// - `end` will not be included while creating the array. Hence the array range is `start..=(end - 1)`
    ///
    /// ## Panics
    /// - May panic if `start > end`, and returns **Index out of bound**
    /// - If `T::from(i)` conversion fails, panics, and returns **Unable to convert to type T**
    ///
    /// ## Example
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let range: usize = 5; // `arr` ranges from 0 to 4 w/o step
    /// let step: usize = 2;
    /// let arr = NdArray::<i8, 2>::arange(range); // uses range(...) to construct a sized array
    /// assert_eq!(*arr.len(), 5);
    /// # }
    /// ```
    fn range(range: (isize, isize, usize)) -> Self
    where
        T: num_traits::NumCast + num_traits::ToPrimitive,
    {
        if range.0 > range.1 {
            panic!("Index out of bound");
        }

        let end_range: usize = Self::size_from_range((range.0, range.1), range.2);
        let mut arr: Vec<T> = Vec::<T>::with_capacity(end_range);
        if range.2 == 0 {
            for i in range.0..range.1 {
                let val: T = T::from(i).expect("Unable to convert to type T"); // panics if it cannot construct to type T
                arr.push(val);
            }
        } else {
            for i in (range.0..range.1).step_by(range.2) {
                let val: T = T::from(i).expect("Unable to convert to type T"); // panics if it cannot construct to type T
                arr.push(val);
            }
        }

        let len: usize = arr.len();
        let ptr: *mut T = arr[..].as_mut_ptr();
        std::mem::forget(arr); // prevents the Vec<T> from being dropped, ensuring the buffer remains valid

        let mut shape: SizedArray<N> = [USIZE_ONE; N];
        shape[N - 1] = len; // [1, .., x]: row-wise contiguous storage format
        let mut strides: SizedArray<N> = [USIZE_ONE; N];
        strides[N - 1] = std::mem::size_of::<T>(); // [1, .., x_stride]

        NdArray {
            ptr,
            len,
            shape,
            strides,
        }
    }

    /// Create a sized array with an `end` value starting from 0 within `usize` range
    ///
    /// ## Note
    /// - Accepts only positive integers
    /// - `end` will not be included while creating the array. Hence the array range is `start..=(end - 1)`
    ///
    /// ## Panics
    /// Check `NdArray<T, N>::range(...)`
    ///
    /// ## Example
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let range: usize = 5; // `arr` ranges from 0 to 4 w/o step
    /// let step: usize = 2;
    /// let arr = NdArray::<i8, 2>::arange(range);
    /// assert_eq!(*arr.len(), 5);
    /// # }
    /// ```
    pub fn arange(range: usize) -> Self
    where
        T: num_traits::NumCast + num_traits::ToPrimitive,
    {
        Self::range((0, range as isize, 0))
    }

    /// Create a sized array with an `end` value starting from 0 within `usize` range and a step value of range `usize`
    ///
    /// ## Note
    /// - Accepts only positive integers
    /// - `end` will not be included while creating the array. Hence the array range is `start..=(end - 1)`
    ///
    /// ## Example
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let range: usize = 5; // `arr` ranges from 0 to 4 w/o step
    /// let step: usize = 2;
    /// let arr = NdArray::<i8, 2>::arange_with_step(range, step);
    /// assert_eq!(*arr.len(), 3);
    /// # }
    /// ```
    pub fn arange_with_step(range: usize, step: usize) -> Self
    where
        T: num_traits::NumCast + num_traits::ToPrimitive + Default + Copy,
    {
        Self::range((0, range as isize, step))
    }

    /// Create a sized array with `start` and `end` values within `isize` range
    ///
    /// ## Note
    /// - `end` will not be included while creating the array. Hence the array range is `start..=(end - 1)`
    ///
    /// ## Example
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let ranges: (isize, isize) = (-1, 5); // `arr` ranges from -1 to 4
    /// let arr = NdArray::<i8, 2>::ranges(ranges);
    /// assert_eq!(*arr.len(), 6);
    /// # }
    /// ```
    pub fn ranges(ranges: (isize, isize)) -> Self
    where
        T: num_traits::NumCast + num_traits::ToPrimitive + Default + Copy,
    {
        Self::range((ranges.0, ranges.1, 0))
    }

    /// Create a sized array with `start` and `end` values within `isize` range and a step value of range `usize`
    ///
    /// ## Note
    /// - `end` will not be included while creating the array. Hence the array range is `start..=(end - 1)`
    ///
    /// ## Example
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let ranges: (isize, isize) = (-1, 5); // `arr` ranges from -1 to 4 w/o step
    /// let step: usize = 2;
    /// let arr = NdArray::<i8, 2>::ranges_with_step(ranges, step);
    /// assert_eq!(*arr.len(), 3);
    /// # }
    /// ```
    pub fn ranges_with_step(ranges: (isize, isize), step: usize) -> Self
    where
        T: num_traits::NumCast + num_traits::ToPrimitive,
    {
        Self::range((ranges.0, ranges.1, step))
    }

    /// Helper method in implementation to fill any `value` of size `X` (total size of array derived from shape)
    ///
    /// ## Note
    /// This is a private method in the implementation and cannot (and should never) be used outside this `impl` block
    ///
    /// ## Example
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let shape: [usize; 2] = [3, 2];
    /// let arr = NdArray::<u16, 2>::zeros(shape); // uses value(...)
    /// # }
    /// ```
    fn values(val: T, shape: SizedArray<N>) -> Self {
        let size: usize = Self::size_from_shape(&shape);

        let mut vec: Vec<T> = vec![val; size];
        let len: usize = vec.len();
        let ptr: *mut T = vec[..].as_mut_ptr();
        let strides: SizedArray<N> = Self::stride(&shape);
        std::mem::forget(vec);

        NdArray {
            ptr,
            len,
            shape,
            strides,
        }
    }

    /// Create a sized array completely filled with numeral zero or `0`. Requires shape of size `N`
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let shape: [usize; 2] = [3, 2];
    /// let arr = NdArray::<u16, 2>::zeros(shape);
    /// for i in 0..arr.shape()[0] {
    ///     for j in 0..arr.shape()[1] {
    ///         assert_eq!(arr[[i, j]], 0);
    ///     }
    /// }
    /// # }
    /// ```
    pub fn zeros(shape: SizedArray<N>) -> Self
    where
        T: Default,
    {
        Self::values(T::default(), shape)
    }

    /// Create a sized array completely filled with numeral one or `1`. Requires shape of size `N`
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ndim::core::NdArray;
    /// #
    /// # fn main() {
    /// let shape: [usize; 2] = [3, 2];
    /// let arr = NdArray::<u16, 2>::ones(shape);
    /// for i in 0..arr.shape()[0] {
    ///     for j in 0..arr.shape()[1] {
    ///         assert_eq!(arr[[i, j]], 1);
    ///     }
    /// }
    /// # }
    /// ```
    pub fn ones(shape: SizedArray<N>) -> Self
    where
        T: num_traits::One,
    {
        Self::values(T::one(), shape)
    }
}

/// Calculate the index using strides and the given index. Returns a value which can be used to access the memory of the 1-d sized array
///
/// ## Example
///
/// ```
/// use ndim::core::NdArray;
///
/// fn main() {
///     let shape: [usize; 2] = [2, 2];
///     let mut arr = NdArray::<u16, 2>::zeros(shape);
///     // mutable pointer is accessed and the value is changed pointing to [1, 1]
///     // `get_index<..>(index: &.., strides: &..)` is used here to access the memory from the 1-d contiguous array
///     arr[[1, 1]] = 12;
///     assert_eq!(arr[[1, 1]], 12);
/// }
/// ```
fn get_index<T, const N: usize>(index: &SizedArray<N>, strides: &SizedArray<N>) -> usize {
    let mut idx: usize = 0;
    for i in 0..N {
        idx += index[i] * strides[i]
    }

    idx / strides[N - 1]
}

/// Use for indexing immutable NdArray
///
/// ## Note
/// - Calls `get_index::<..>(index: &.., strides: &..)` to access the data from the contiguous sized 1-d array
/// - `Panics` if the index is larger than the length of the sized array
/// - Unsafe block accesses the value in the memory by calculating the offset from the pointer: `*mut T + idx`
///
/// ## Example
///
/// ```
/// use ndim::core::NdArray;
///
/// fn main() {
///     let shape: [usize; 2] = [2, 2];
///     let mut arr = NdArray::<u16, 2>::zeros(shape);
///     arr[[1, 1]] = 12; // mutable pointer is accessed and the value is changed pointing to [1, 1]
/// }
/// ```
impl<T, const N: usize> Index<SizedArray<N>> for NdArray<T, N> {
    type Output = T;

    fn index(&self, index: SizedArray<N>) -> &Self::Output {
        let idx = get_index::<T, N>(&index, &self.strides);
        if idx >= self.len {
            panic!("Index out of bounds")
        }
        unsafe { &*self.ptr.add(idx) }
    }
}

/// Use for indexing mutable NdArray
impl<T, const N: usize> IndexMut<SizedArray<N>> for NdArray<T, N> {
    fn index_mut(&mut self, index: SizedArray<N>) -> &mut Self::Output {
        let idx = get_index::<T, N>(&index, &self.strides);
        if idx >= self.len {
            panic!("Index out of bounds")
        }
        unsafe { &mut *self.ptr.add(idx) }
    }
}

/// Drop *mut T when it goes out of scope
///
/// Manually dropping by getting a slice from the pointer. Total size allocated by the pointer is also taken to get this slice. Then, the drop is performed.
///
/// ## Note
/// - Pointer is not dropped if null
/// - Though drop occurs inside unsafe block, it takes the right size of the array. This does not mean that undefined behaviour cannot happen
///
/// ## Example
///
/// ```
/// use ndim::core::NdArray;
///
/// fn main() {
///     {
///         let arr = NdArray::<u16, 2>::new();
///         // `arr` as it goes out of scope here
///         // Drop implementation is called
///     }
///     // `arr` is not accessible anymore
/// }
/// ```
impl<T, const N: usize> Drop for NdArray<T, N> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let slice: &mut [T] = std::slice::from_raw_parts_mut(self.ptr, self.len);
                std::ptr::drop_in_place(slice);
            }
        }
    }
}

//
#[cfg(test)]
mod core_ndim_t {
    use crate::core::{Array, Array2, Array3, NdArray};

    // Test for zeros creation in a 1-D sized array
    // Try to access the value in the memory at location (x, y) and mutate it
    #[test]
    fn zeros_2dim_t() {
        let shape: [usize; 2] = [2, 2];
        let mut data: NdArray<u32, 2> = Array2::<u32>::zeros(shape);
        assert_eq!(data[[1, 1]], 0);

        data[[1, 1]] = 12;
        assert_eq!(data[[1, 1]], 12);
    }

    // Test for ones creation in a 2-D sized array
    // Try to access the value in the memory at location (x, y) and mutate it
    #[test]
    fn ones_2dim_t() {
        let shape: [usize; 2] = [2, 2];
        let mut data: NdArray<u32, 2> = Array2::<u32>::ones(shape);
        assert_eq!(data[[1, 1]], 1);

        data[[1, 1]] = 12;
        assert_eq!(data[[1, 1]], 12);
    }

    // Test NdArray<T, N>::from(...) for a 3-D sized array of type u32
    // Check if the memory set with shape is correct
    #[test]
    fn from_3dim_u32_t() {
        let arr: [u32; 6] = [0, 1, 2, 3, 4, 5];
        let ndim_arr: [[[i32; 3]; 2]; 1] = [[[0, 1, 2], [3, 4, 5]]];
        let shape: [usize; 3] = [1, 2, 3];

        let data: NdArray<u32, 3> = Array3::<u32>::from(&arr, shape);
        let strides: [usize; 3] = [24, 12, 4];
        assert_eq!(*data.strides(), strides);

        let mut idx: usize = 0;
        for i in 0..ndim_arr.len() {
            for j in 0..ndim_arr[i].len() {
                for k in 0..ndim_arr[i][j].len() {
                    let index: [usize; 3] = [i, j, k] as [usize; 3];
                    assert_eq!(data[index], arr[idx]);
                    idx += 1;
                }
            }
        }
    }

    // Test NdArray<T, N>::from(...) for a 3-D sized array of type i32
    // Check if strides created are correct
    // Check if the memory set with shape is correct
    #[test]
    fn from_3dim_i32_t() {
        let arr: [i32; 6] = [0, -1, 2, -3, 4, -5];
        let ndim_arr: [[i32; 2]; 3] = [[0, -1], [2, -3], [4, -5]];
        let shape: [usize; 2] = [3, 2];

        let data: NdArray<i32, 2> = Array2::<i32>::from(&arr, shape);
        let strides: [usize; 2] = [8, 4];
        assert_eq!(*data.strides(), strides);

        let mut idx: usize = 0;
        for i in 0..ndim_arr.len() {
            for j in 0..ndim_arr[i].len() {
                let index: [usize; 2] = [i, j] as [usize; 2];
                assert_eq!(data[index], arr[idx]);
                idx += 1;
            }
        }
    }

    // Test NdArray<T, N>::from(...) for a 3-D sized array of type f32
    // Check if strides created are correct
    // Check if the memory set with shape is correct
    #[test]
    fn from_3dim_f32_t() {
        let arr: [f32; 6] = [0.0, -1.2, 2.1, -3.75, 4.004, -5.65];
        let ndim_arr: [[f32; 2]; 3] = [[0.0, -1.2], [2.1, -3.75], [4.004, -5.65]];
        let shape: [usize; 2] = [3, 2];

        let data: NdArray<f32, 2> = Array2::<f32>::from(&arr, shape);
        let strides: [usize; 2] = [8, 4];
        assert_eq!(*data.strides(), strides);

        let mut idx: usize = 0;
        for i in 0..ndim_arr.len() {
            for j in 0..ndim_arr[i].len() {
                let index: [usize; 2] = [i, j] as [usize; 2];
                assert_eq!(data[index], arr[idx]);
                idx += 1;
            }
        }
    }

    // TODO T > u16: needs faster processing time
    // Test NdArray<T, N>::arange(...) for a 1-D sized array
    // Check if length and shape created are correct
    // Check if the memory set with shape is correct
    #[test]
    fn arange_1dim_t() {
        let range: usize = u16::MAX as usize;
        let arr: Vec<u16> = (0..u16::MAX).collect::<Vec<u16>>();
        let size: &usize = &range;

        let data: NdArray<u16, 1> = Array::<u16>::arange(range);
        assert_eq!(*data.len(), *size);
        assert_eq!(*data.shape(), [*size]);

        for i in 0..data.shape()[0] {
            let index: [usize; 1] = [i] as [usize; 1];
            assert_eq!(data[index], arr[i]);
        }
    }

    // Test NdArray<T, N>::arange(...) for a 2-D sized array
    // Check if length and shape created are correct
    // Check if the memory set with shape is correct
    #[test]
    fn arange_2dim_t() {
        let range: usize = u16::MAX as usize;
        let arr: Vec<u16> = (0..u16::MAX).collect::<Vec<u16>>();
        let size: &usize = &range;

        let data: NdArray<u16, 2> = Array2::<u16>::arange(range);
        assert_eq!(*data.len(), *size);
        assert_eq!(*data.shape(), [1, *size]);

        let mut idx: usize = 0;
        for i in 0..data.shape()[0] {
            for j in 0..data.shape()[1] {
                let index: [usize; 2] = [i, j] as [usize; 2];
                assert_eq!(data[index], arr[idx]);

                idx += 1;
            }
        }
    }

    // Test NdArray<T, N>::arange(...) for a 3-D sized array
    // Reshape the NdArray and check with the new shape
    // Check if length created is correct
    // Check if the memory set with shape is correct
    #[test]
    fn arange_3dim_t() {
        let range: usize = u16::MAX as usize;
        let arr: Vec<f32> = (0u16..u16::MAX)
            .step_by(2)
            .map(f32::from)
            .collect::<Vec<f32>>();
        let size: usize = if range % 2 == 0 {
            range / 2
        } else {
            (range / 2) + 1
        };

        let mut data: NdArray<f32, 3> = Array3::<f32>::arange_with_step(range, 2);
        let new_shape: [usize; 3] = [1, size / 128, 128]; // (1, 256, 128)
        data.reshape(new_shape);

        assert_eq!(*data.len(), size);
        assert_eq!(*data.shape(), new_shape);

        let mut idx: usize = 0;
        for i in 0..data.shape()[0] {
            for j in 0..data.shape()[1] {
                for k in 0..data.shape()[2] {
                    let index: [usize; 3] = [i, j, k] as [usize; 3];
                    assert_eq!(data[index], arr[idx]);

                    idx += 1;
                }
            }
        }
    }

    // Test NdArray<T, N>::reshape(...) for a 2-D sized array
    #[test]
    fn reshape_2dim_t() {
        let range: usize = u16::MAX as usize;
        let size: &usize = &range;

        let mut data: NdArray<u16, 2> = Array2::<u16>::arange(range);
        assert_eq!(*data.len(), *size);
        assert_eq!(*data.shape(), [1, *size]);

        let mut new_shape: [usize; 2] = [*size / 5, 5];
        data.reshape(new_shape);
        assert_eq!(*data.shape(), new_shape);

        new_shape = [*size / 15, 15];
        data.reshape(new_shape);
        assert_eq!(*data.shape(), new_shape);

        new_shape = [*size / 257, 257];
        data.reshape(new_shape);
        assert_eq!(*data.shape(), new_shape);
    }

    // Test NdArray<T, N>::reshape(...) for a 3-D sized array
    #[test]
    fn reshape_3dim_t() {
        let range: usize = u16::MAX as usize;
        let size: &usize = &range;

        let mut data: NdArray<u16, 3> = Array3::<u16>::arange(range);
        assert_eq!(*data.len(), *size);
        assert_eq!(*data.shape(), [1, 1, *size]);

        let t_size: usize = std::mem::size_of::<u16>();
        let mut new_shape: [usize; 3] = [3, 5, 4369];
        let mut new_strides: [usize; 3] = [
            new_shape[1] * new_shape[2] * t_size,
            new_shape[2] * t_size,
            t_size,
        ];
        data.reshape(new_shape);
        assert_eq!(*data.shape(), new_shape);
        assert_eq!(*data.strides(), new_strides);

        new_shape = [1, 17, 3855];
        new_strides = [
            new_shape[1] * new_shape[2] * t_size,
            new_shape[2] * t_size,
            t_size,
        ];
        data.reshape(new_shape);
        assert_eq!(*data.shape(), new_shape);
        assert_eq!(*data.strides(), new_strides);
    }
}
