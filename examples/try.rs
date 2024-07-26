use ndim::core::NdArray;

fn main() {
    // Create an NdArray filled with a specific value
    let shape = [3, 2];
    let array = NdArray::<i32, 2>::zeros(shape);

    // Print the array
    for i in 0..array.shape()[0] {
        for j in 0..array.shape()[1] {
            println!("{}", array[[i, j]]); // access the value from memory
        }
    }
}
