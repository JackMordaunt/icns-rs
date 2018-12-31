use std::fs::File;
use std::io::BufWriter;
use num_complex;
use icns::encode::Encoder;
use image::{self, ConvertBuffer};

fn main() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy.
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Generate gradient. 
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // Generate fractal. 
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let data = (*pixel as image::Rgb<u8>).data;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Open output file.
    let mut output = BufWriter::new(File::create("fractal.icns")
        .expect("creating output file"));
     
    // Encode the image as icns. 
    // Note that we use ConvertBuffer trait to convert from RGB to RGBA. 
    Encoder::new(&mut output)
        .encode(&imgbuf.convert())
        .expect("encoding icns");
}