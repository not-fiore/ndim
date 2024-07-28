use ndim::core::NdArray;

fn main() {
    // Create an NdArray filled with zeros
    let shape: [usize; 2] = [3, 2];
    let array: NdArray<i32, 2> = NdArray::<i32, 2>::zeros(shape);

    for i in 0..array.shape()[0] {
        for j in 0..array.shape()[1] {
            // access the value from memory
            // check if the values are zero
            assert_eq!(array[[i, j]], 0);
        }
    }
    
    // Create an NdArray filled with ones
    let shape: [usize; 3] = [1, 3, 5];
    let array: NdArray<f32, 3> = NdArray::<f32, 3>::ones(shape);
    
    for i in 0..array.shape()[0] {
        for j in 0..array.shape()[1] {
            for k in 0..array.shape()[2] {
                // access the value from memory
                // check if the values are one
                assert_eq!(array[[i, j, k]], 1.0);
            }
        }
    }
}
