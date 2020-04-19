use icns::Encoder;
use image::buffer::ConvertBuffer;
use num_complex;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let imgx = 1024;
    let imgy = 1024;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy.
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

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
            let data = (*pixel as image::Rgb<u8>).0;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Open output file.
    let mut output = BufWriter::new(File::create("fractal.icns").expect("creating output file"));

    // Encode the image as icns.
    // Note that we use ConvertBuffer trait to convert from RGB to RGBA.
    Encoder::new(&mut output)
        .encode(&imgbuf.convert())
        .expect("encoding icns");
}
