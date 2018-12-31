use image::{self, ConvertBuffer};
use std::fs;
use icns::encode::Encoder;

fn main() {
    let imgx = 800;
    let imgy = 800;

    // Create a new ImgBuf with width: imgx and height: imgy.
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image.
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // Open output file.
    let mut output = fs::File::create("fractal.icns")
        .expect("creating output file");
     
    // Encode the image as icns. 
    Encoder::new(&mut output)
        .encode(&imgbuf.convert())
        .expect("encoding icns");
}