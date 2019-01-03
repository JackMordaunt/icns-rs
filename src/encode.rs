use std::cmp::max;
use std::io::{self, Write};
use byteorder::{BigEndian, WriteBytesExt};
use image::{self, RgbaImage};
use image::imageops::{resize, Lanczos3};
use rayon::{self, prelude::*};
use png::{self, HasParameters};
use deflate;

use crate::os_type::OSType;

/// Encoder encodes icns image into the provided writer. 
pub struct Encoder<W: Write> {
    w: W
}

impl<W: Write> Encoder<W> {
    /// Create a new encoder that writes to ```w```. 
    pub fn new(w: W) -> Self {
        Encoder{w}
    }
    // Encode the icns from a source png encoded buffer.  
    pub fn encode(&mut self, img: &RgbaImage) -> io::Result<()> {
        IconSet::from(img).write_to(self.w.by_ref())
    }
}

/// IconSet encodes a vector of icons. 
struct IconSet {
    icons: Vec<Icon>,
}

/// Magic bytes that denote an icns file. These bytes appear at index 0.
const ICONSET_MAGIC: [char; 4] = ['i', 'c', 'n', 's'];

impl IconSet {
    /// Create an IconSet from the provided image. 
    /// If width != height, the image will be resized using the largest side 
    /// without preserving the aspect ratio.
    fn from(img: &RgbaImage) -> Self {
        let kind = OSType::nearest(max(img.width(), img.height()));
        let icons: Vec<Icon> = kind.smaller_variants()
            .par_iter()
            .map(|v| Icon {
                kind: v.clone(),
                image: resize(img, v.size(), v.size(), Lanczos3),
            })
            .collect();
        IconSet { icons }
    }
    /// Write the encoded iconset to writer ```w```. 
    fn write_to(self, mut wr: impl Write) -> io::Result<()> {
        // Pre-buffer the encoded icons so we can calculate the final size. 
        let mut buffer: Vec<u8> = vec![];
        for icon in self.icons {
            icon.write_to(&mut buffer)?;
        }
        // Write the 4-byte magic bytes to identify this as an icns image.
        wr.write_all(&ICONSET_MAGIC
            .into_iter()
            .map(|c| *c as u8)
            .collect::<Vec<u8>>())?;
        // Write the 4-byte container size in bytes.
        wr.write_u32::<BigEndian>((buffer.len() + 8) as u32)?;
        // Write the encoded icons. 
        wr.write_all(&buffer)?;
        Ok(())
    }
}

/// Icon encodes a single icon. 
struct Icon {
    kind: OSType,
    image: RgbaImage,
}

impl Icon {
    /// Write the encoded icon to writer ```w```.
    fn write_to(self, mut wr: impl Write) -> io::Result<()> {
        // Pre-buffer the png image so we can calculate size total. 
        let (width, height) = (self.image.width(), self.image.height());
        let mut buffer: Vec<u8> = vec![];
        PNGEncoder::new(&mut buffer)
            .encode(
                self.image.into_raw().as_ref(),
                width,
                height,
                png::ColorType::RGBA,
                png::BitDepth::Eight,
            )?;
        // Write the 4-byte OSType identifier.
        // Coerce array of characters into slice of bytes through a vec deref.
        wr.write_all(&self.kind.header()
            .into_iter()
            .map(|c| *c as u8)
            .collect::<Vec<u8>>())?;
        // Write the 4-byte icon size in bytes (data.len + header.len).
        wr.write_u32::<BigEndian>((buffer.len() + 8) as u32)?;
        // Write the image data. 
        wr.write_all(&buffer)?;
        Ok(())
    }
}

/// PNGEncoder is a convenience wrapper around ```png::Encoder```.
struct PNGEncoder<W: Write> {
    w: W,
}

impl<W: Write> PNGEncoder<W> {
    fn new(w: W) -> Self {
        PNGEncoder { w }
    }
    fn encode(
        self,
        data: &[u8],
        width: u32,
        height: u32,
        ct: png::ColorType,
        bits: png::BitDepth,
    ) -> io::Result<()> {
        let mut encoder = png::Encoder::new(self.w, width, height);
        encoder
            .set(ct)
            .set(bits)
            .set(png::Compression::Default);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(data).map_err(|e| e.into())
    }
}