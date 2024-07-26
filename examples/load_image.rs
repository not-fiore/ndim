// uses image crate to load a png image from the system
use image::{GenericImageView, ImageReader, ImageResult};
// ndim crate to hold the pixel values of the image
use ndim::core::NdArray;

fn main() -> ImageResult<()> {
    // DynamicImage object from the image crate loaded into `img` variable
    let img: image::DynamicImage = ImageReader::open("asset/scenery.png")?.decode()?;
    // pixel value from the loaded image is initialized
    let pixels: image::Pixels<image::DynamicImage> = img.pixels();

    // shape of the array which will determine the size of the array
    let shape: [usize; 2] = [img.height() as usize, img.width() as usize];
    
    // sized array to hold the pixel values in a dynamic 1-d array
    // the index depends on the size of the array dimension `N`
    let mut pix_arr: NdArray<[u8; 4], 2> = NdArray::zeros(shape);
    // load into a NdArray object as a 2-D sized array (contiguous 1-d sized array in memory)
    pixels.for_each(|px: (u32, u32, image::Rgba<u8>)| {
        let idx: [usize; 2] = [px.1 as usize, px.0 as usize];
        let rgba: [u8; 4] = px.2 .0;
        // to access the value in the memory location, use ptr[[<index here>]]
        pix_arr[idx] = rgba;
    });
    // check pixel values present in the pixel coordinate
    assert_eq!(pix_arr[[1, 12]], [170, 180, 192, 255]);

    Ok(())
}
