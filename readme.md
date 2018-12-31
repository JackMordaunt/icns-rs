# icns-rs 

> Easily create icns container images from pngs. Make your apps look sharp.  

icns is an image format for MacOS ([Apple Icon Image Format](https://en.wikipedia.org/wiki/Apple_Icon_Image_format)), which is essentially a container format that rolls together one or more png encoded images. 

These icons are used by Mac `.app` bundles, allowing them to choose the appropriate resolution icon for the given context. Typically an icns will contain several versions of a png all at different resolutions, from a max of 1024x1024 pixels down to 32x32 pixels, suitable for high-dpi retina screens. 

The most common ways to generate icns files are:  
- `iconutil`, which is an esoteric Mac native cli utility that is opaque and  cumbersome to use. 
- `ImageMagick`, which adds a large dependency to your project for such a simple use case.  

This is a Rust port of my Go project [icns](https://github.com/jackmordaunt/icns).

Where the Go project builds on the standard library `image.Image` interface, this library builds on the `DyanmicImage` trait from [Piston](https://github.com/pistondevelopers) developer's [image](https://github.com/pistondevelopers/image) crate. 

## Usage  

See the examples folder for more usage. 

### Generate a fractal and encode as icns
> Adapted from [fractal.rs](https://github.com/PistonDevelopers/image/blob/master/examples/fractal.rs)  

```rust 
use std::fs::File;
use std::io::BufWriter;
use num_complex;
use icns::encode::Encoder;
use image::{self, ConvertBuffer};

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
```

## Features  
- [x] Encode icns (mvp) `DynamicImage -> .icns`
    - [x] Parallel resizing (thanks [rayon](https://github.com/rayon-rs/rayon))  
    - [x] Lanczos3 interpolation  
- [ ] Decode largest image from icns into standalone png  
- [ ] Simple and robust cli app
    - [ ] Flag based use  
    - [ ] Shell pipe use   
- [ ] Unit tests  