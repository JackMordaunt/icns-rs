use byteorder::{BigEndian, WriteBytesExt};
use image::imageops::{resize, Lanczos3};
use image::{self, DynamicImage, GenericImageView, RgbaImage};
use rayon::{self, prelude::*};
use std::cmp::max;
use std::io::Write;

use crate::os_type::OSType;

/// Encoder encodes icns image into the provided writer.
pub struct Encoder<W: Write> {
    w: W,
}

impl<W: Write> Encoder<W> {
    /// Create a new encoder that writes to `w`.
    pub fn new(w: W) -> Self {
        Encoder { w }
    }
    /// Encode encode an icns into the writer, using the source image.
    pub fn encode<Img>(&mut self, img: Img) -> Result<(), Box<dyn std::error::Error>>
    where
        Img: Into<IconSet>,
    {
        // Note(jfm): CPU intensive work is being done in `From` trait.
        // This is probably not good practice since it hides the actual work.
        img.into().write_to(self.w.by_ref())
    }
}

/// IconSet encodes a vector of icons.
pub struct IconSet {
    icons: Vec<Icon>,
}

/// Magic bytes that denote an icns file. These bytes appear at index 0.
const ICONSET_MAGIC: &'static str = "icns";

impl IconSet {
    /// Write the encoded iconset to writer `w`.
    pub fn write_to(self, mut wr: impl Write) -> Result<(), Box<dyn std::error::Error>> {
        // Pre-buffer the encoded icons so we can calculate the final size.
        let mut buffer: Vec<u8> = vec![];
        for icon in self.icons {
            icon.write_to(&mut buffer)?;
        }
        // Write the 4-byte magic bytes to identify this as an icns image.
        wr.write_all(ICONSET_MAGIC.as_bytes())?;
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
    /// Write the encoded icon to writer `w`.
    fn write_to(self, mut wr: impl Write) -> Result<(), Box<dyn std::error::Error>> {
        // Pre-buffer the png image so we can calculate size total.
        let (width, height) = (self.image.width(), self.image.height());
        let mut buffer: Vec<u8> = vec![];
        PNGEncoder::new(&mut buffer).encode(
            self.image.into_raw().as_ref(),
            width,
            height,
            image::ColorType::Rgba8,
        )?;
        // Write the 4-byte OSType identifier.
        wr.write_all(&self.kind.header().as_bytes())?;
        // Write the 4-byte icon size in bytes (data.len + header.len).
        wr.write_u32::<BigEndian>((buffer.len() + 8) as u32)?;
        // Write the image data.
        wr.write_all(&buffer)?;
        Ok(())
    }
}

/// PNGEncoder is a convenience wrapper around `png::Encoder`.
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
        ct: image::ColorType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        image::png::PNGEncoder::new(self.w).encode(data, width, height, ct)?;
        Ok(())
    }
}

/// Create an IconSet from the provided image.
/// If width != height, the image will be resized using the largest side
/// without preserving the aspect ratio.
impl From<&DynamicImage> for IconSet {
    fn from(img: &DynamicImage) -> Self {
        let kind = OSType::nearest(max(img.width(), img.height()));
        let icons: Vec<Icon> = kind
            .smaller_variants()
            .into_par_iter()
            .map(|v| {
                let size = v.size();
                Icon {
                    kind: v,
                    image: resize(img, size, size, Lanczos3),
                }
            })
            .collect();
        IconSet { icons }
    }
}

/// Create an IconSet from the provided image.
/// If width != height, the image will be resized using the largest side
/// without preserving the aspect ratio.
impl From<&RgbaImage> for IconSet {
    fn from(img: &RgbaImage) -> Self {
        let kind = OSType::nearest(max(img.width(), img.height()));
        let icons: Vec<Icon> = kind
            .smaller_variants()
            .into_par_iter()
            .map(|v| {
                let size = v.size();
                Icon {
                    kind: v,
                    image: resize(img, size, size, Lanczos3),
                }
            })
            .collect();
        IconSet { icons }
    }
}
