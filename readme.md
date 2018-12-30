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

## Features  
- [x] Encode icns (mvp) `DynamicImage -> .icns`
    - [x] Parallel resizing (thanks [rayon](https://github.com/rayon-rs/rayon))  
    - [x] Lanczos3 interpolation  
- [ ] Decode largest image from icns 
- [ ] Simple and robust cli app
    - [ ] Flag based use  
    - [ ] Shell pipe use   
- [ ] Unit tests  